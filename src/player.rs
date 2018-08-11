extern crate piston_window;

use game::*;
use piston_window::*;
use sprite::Sprite;
use traits::Collides;

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
    pub collider: Collider,
    explosion: Animation,
    rot: f64,
}

impl Collides for Player {
    fn collides_with<C: Collides>(&self, other: &C) -> bool {
        self.collider.collides_with(other.get_collider())
    }

    fn get_collider(&self) -> &Collider {
        &self.collider
    }
}

impl Player {
    pub fn new(collider: Collider, explosion: Animation) -> Player {
        Player {
            state: State::Active(Action::NoMove),
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

    pub fn draw(
        &mut self,
        sprites: &mut [Sprite<G2dTexture>; 3],
        explosion_tex: &mut AnimTexture,
        c: piston_window::Context,
        g: &mut G2d,
    ) -> () {
        match self.state {
            State::Active(action) => {
                let rot = self.rot;
                let mut sprite = self.active_sprite(sprites, action);
                sprite.set_rotation(rot);
                sprite.set_position(self.collider.pos.x, self.collider.pos.y);
                sprite.draw(c.transform, g);
            }
            State::Exploding => {
                self.explosion.draw(explosion_tex, c, g);
            }
            State::Inactive => {}
        }
    }

    pub fn explode(&mut self) -> () {
        match self.state {
            State::Active(_) => {
                self.state = State::Exploding;
                self.collider.disable();
                self.explosion.play();
                self.explosion.set_pos(self.collider.pos);
            }
            State::Exploding | State::Inactive => {}
        }
    }

    pub fn reset(&mut self) -> () {
        self.state = State::Active(Action::NoMove);
        self.rot = 0.0;
        self.collider.enable();
        self.explosion.stop();
    }

    pub fn velocity(&self) -> Point {
        use settings::player;
        Point::new(self.rot.to_radians().cos(), self.rot.to_radians().sin()) * player::SPEED
    }

    pub fn is_active(&self) -> bool {
        match self.state {
            State::Active(_) => true,
            _ => false,
        }
    }

    fn active_sprite<'a>(
        &mut self,
        sprites: &'a mut [Sprite<G2dTexture>; 3],
        action: Action,
    ) -> &'a mut Sprite<G2dTexture> {
        match action {
            Action::Left => &mut sprites[0],
            Action::NoMove => &mut sprites[1],
            Action::Right => &mut sprites[2],
        }
    }
}

pub fn initialise_player_sprites(
    window: &mut PistonWindow,
    folder: &::std::path::PathBuf,
    files: [&str; 3],
    scale: f64,
) -> [Sprite<G2dTexture>; 3] {
    let mut spr_player_left = load_sprite(window, folder, files[0]);
    spr_player_left.set_scale(scale, scale);

    let mut spr_player_mid = load_sprite(window, folder, files[1]);
    spr_player_mid.set_scale(scale, scale);

    let mut spr_player_right = load_sprite(window, folder, files[2]);
    spr_player_right.set_scale(scale, scale);

    [spr_player_left, spr_player_mid, spr_player_right]
}
