extern crate piston_window;

use game::{Animation, *};
use piston_window::*;
use player::*;
use sprite::*;

#[derive(PartialEq)]
enum State {
    Active,
    Exploding,
    Inactive,
}

pub struct Missile {
    state: State,
    pub collider: Collider,
    pub velocity: Point,
    pub explosion: Animation,
}

impl Missile {
    const MAX_SPEED: f64 = 1200.0;
    const ACCELERATION: f64 = 3000.0;

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
                self.update_velocity(player, dt);
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
        if self.state == State::Active {
            self.state = State::Exploding;
            self.explosion.play();
        }
    }

    pub fn reset(&mut self, pos: Point, velocity: Point) -> () {
        self.collider.pos = pos;
        self.velocity = velocity;
        self.state = State::Active;
    }

    fn update_position(&mut self, player: &Player, dt: f64) -> () {
        // Update position (x = x + v*dt)
        self.collider.pos = self.collider.pos + self.velocity * dt;

        // Update position based off player movement
        self.collider.pos = self.collider.pos - player.velocity() * dt;
    }

    fn update_velocity(&mut self, player: &Player, dt: f64) -> () {
        // Update velocity and cap (v = v + a*dt)
        self.velocity = self.velocity
            + (player.collider.pos - self.collider.pos).normalized() * Missile::ACCELERATION * dt;
        if self.velocity.magnitude() >= Missile::MAX_SPEED {
            self.velocity = self.velocity.normalized() * Missile::MAX_SPEED;
        }
    }

    fn update_explosion(&mut self, dt: f64) -> () {
        // Update explosion
        self.explosion.update(dt);
        self.explosion.set_pos(self.collider.pos);
    }
}
