extern crate piston_window;

use game::{Animation, *};
use piston_window::*;
use player::*;
use sprite::*;

pub struct Missile {
    pub collider: Collider,
    pub velocity: Point,
    pub explosion: Animation,
}

impl Missile {
    const MAX_SPEED: f64 = 1200.0;
    const ACCELERATION: f64 = 3000.0;

    pub fn new(collider: Collider, velocity: Point, explosion: Animation) -> Missile {
        Missile {
            collider: collider,
            velocity: velocity,
            explosion: explosion,
        }
    }

    pub fn update(&mut self, player: &Player, dt: f64) {
        // Update position (x = x + v*dt)
        self.collider.pos = self.collider.pos + self.velocity * dt;

        // Update velocity and cap (v = v + a*dt)
        if !self.explosion.is_playing() {
            self.velocity = self.velocity
                + (player.collider.pos - self.collider.pos).normalized()
                    * Missile::ACCELERATION
                    * dt;
            if self.velocity.magnitude() >= Missile::MAX_SPEED {
                self.velocity = self.velocity.normalized() * Missile::MAX_SPEED;
            }
        }

        // Update position based off player movement
        self.collider.pos = self.collider.pos - player.velocity() * dt;

        // Update explosion
        self.explosion.update(dt);
        self.explosion.set_pos(self.collider.pos);
    }

    pub fn draw(
        &self,
        sprite: &mut Sprite<G2dTexture>,
        c: piston_window::Context,
        g: &mut G2d,
    ) -> () {
        match self.explosion.is_playing() {
            true => {
                self.explosion.draw(c, g);
            }
            false => {
                sprite.set_position(self.collider.pos.x, self.collider.pos.y);
                sprite.set_rotation(self.velocity.y.atan2(self.velocity.x).to_degrees());
                sprite.draw(c.transform, g);
            }
        }
    }
}
