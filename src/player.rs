extern crate piston_window;

use game::*;
use piston_window::*;
use sprite::Sprite;

#[derive(Copy, Clone)]
enum Action {
    NoMove,
    Left,
    Right,
}

#[derive(Copy, Clone)]
enum State {
    Active(Action),
    Exploding,
    Inactive,
}

pub struct Player {
    state: State,
    sprites: [Sprite<G2dTexture>; 3],
    pub collider: Collider,
    explosion: Animation,
    pub rot: f64,
}

impl Player {
    pub const SPEED: f64 = 800.0;

    pub fn new(
        collider: Collider,
        sprites: [Sprite<G2dTexture>; 3],
        explosion: Animation,
    ) -> Player {
        Player {
            state: State::Active(Action::NoMove),
            sprites: sprites,
            collider: collider,
            explosion: explosion,
            rot: 0.0,
        }
    }

    pub fn update(&mut self, dt: f64) {
        let added_rotation = 270.0 as f64;
        match self.state {
            State::Active(action) => match action {
                Action::Left => self.rot = self.rot - added_rotation * dt,
                Action::Right => self.rot = self.rot + added_rotation * dt,
                Action::NoMove => (),
            },
            State::Exploding => {
                self.explosion.update(dt);
                if !self.explosion.is_playing() {
                    self.state = State::Inactive;
                }
            }
            State::Inactive => {}
        }
    }

    pub fn input(&mut self, left_key: KeyState, right_key: KeyState) -> () {
        match self.state {
            State::Active(_) => match (left_key, right_key) {
                (KeyState::Pressed, KeyState::Pressed) => {
                    self.state = State::Active(Action::NoMove)
                }
                (KeyState::Pressed, KeyState::NotPressed) => {
                    self.state = State::Active(Action::Left)
                }
                (KeyState::NotPressed, KeyState::Pressed) => {
                    self.state = State::Active(Action::Right)
                }
                (KeyState::NotPressed, KeyState::NotPressed) => {
                    self.state = State::Active(Action::NoMove)
                }
            },
            State::Exploding | State::Inactive => {}
        }
    }

    pub fn draw(&mut self, c: piston_window::Context, g: &mut G2d) -> () {
        match self.state {
            State::Active(action) => {
                let rot = self.rot;
                self.active_sprite(action).set_rotation(rot);
                self.active_sprite(action).draw(c.transform, g);
            }
            State::Exploding => {
                self.explosion.draw(c, g);
            }
            State::Inactive => {}
        }
    }

    pub fn explode(&mut self) -> () {
        match self.state {
            State::Active(_) => {
                self.state = State::Exploding;
                self.explosion.play();
            }
            State::Exploding | State::Inactive => {}
        }
    }

    pub fn reset(&mut self) -> () {
        self.state = State::Active(Action::NoMove);
        self.rot = 0.0;
        self.explosion.stop();
    }

    pub fn velocity(&self) -> Point {
        Point::new(self.rot.to_radians().cos(), self.rot.to_radians().sin()) * Player::SPEED
    }

    pub fn is_active(&self) -> bool {
        match self.state {
            State::Active(_) => true,
            _ => false,
        }
    }

    fn active_sprite(&mut self, action: Action) -> &mut Sprite<G2dTexture> {
        match action {
            Action::Left => &mut self.sprites[0],
            Action::NoMove => &mut self.sprites[1],
            Action::Right => &mut self.sprites[2],
        }
    }
}
