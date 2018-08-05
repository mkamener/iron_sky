extern crate piston_window;

use game::{Animation, *};
use piston_window::*;
use player::*;
use sprite::*;

#[derive(Copy, Clone)]
enum State {
    Active,
    Exploding,
    Inactive,
}

pub struct Missile {
    state: State,
    pub collider: Collider,
    velocity: Point,
    explosion: Animation,
}

impl Missile {
    pub fn new(collider: Collider, velocity: Point, explosion: Animation) -> Missile {
        Missile {
            state: State::Active,
            collider: collider,
            velocity: velocity,
            explosion: explosion,
        }
    }

    pub fn update(&mut self, player: &Player, dt: f64) {
        match self.state {
            State::Active => {
                self.update_position(player, dt);
                if player.is_active() {
                    self.update_velocity(player, dt);
                }
            }
            State::Exploding => {
                self.update_position(player, dt);
                self.update_explosion(dt);
                if !self.explosion.is_playing() {
                    self.state = State::Inactive;
                }
            }
            State::Inactive => {}
        }
    }

    pub fn draw(
        &self,
        sprite: &mut Sprite<G2dTexture>,
        c: piston_window::Context,
        g: &mut G2d,
    ) -> () {
        match self.state {
            State::Active => {
                sprite.set_position(self.collider.pos.x, self.collider.pos.y);
                sprite.set_rotation(self.velocity.y.atan2(self.velocity.x).to_degrees());
                sprite.draw(c.transform, g);
            }
            State::Exploding => {
                self.explosion.draw(c, g);
            }
            State::Inactive => {}
        }
    }

    pub fn explode(&mut self) -> () {
        match self.state {
            State::Active => {
                self.state = State::Exploding;
                self.collider.disable();
                self.explosion.play();
            }
            State::Exploding | State::Inactive => {}
        }
    }

    pub fn reset(&mut self, pos: Point, velocity: Point) -> () {
        self.collider.pos = pos;
        self.velocity = velocity;
        self.state = State::Active;
        self.collider.enable();
        self.explosion.stop();
    }

    fn update_position(&mut self, player: &Player, dt: f64) -> () {
        // Update position (x = x + v*dt)
        self.collider.pos = self.collider.pos + self.velocity * dt;

        // Update position based off player movement
        self.collider.pos = self.collider.pos - player.velocity() * dt;
    }

    fn update_velocity(&mut self, player: &Player, dt: f64) -> () {
        use settings::missile;
        // Update velocity and cap (v = v + a*dt)
        self.velocity = self.velocity
            + (player.collider.pos - self.collider.pos).normalized() * missile::ACCELERATION * dt;
        if self.velocity.magnitude() >= missile::MAX_SPEED {
            self.velocity = self.velocity.normalized() * missile::MAX_SPEED;
        }
    }

    fn update_explosion(&mut self, dt: f64) -> () {
        // Update explosion
        self.explosion.update(dt);
        self.explosion.set_pos(self.collider.pos);
    }
}
