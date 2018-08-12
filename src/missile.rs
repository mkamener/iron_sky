extern crate piston_window;

use game::{Animation, *};
use piston_window::*;
use player::*;
use sprite::*;
use traits::Collides;

#[derive(Copy, Clone, PartialEq)]
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

impl Collides for Missile {
    fn collides_with<C: Collides>(&self, other: &C) -> bool {
        self.collider.collides_with(other.get_collider())
    }

    fn get_collider(&self) -> &Collider {
        &self.collider
    }
}

impl Missile {
    pub fn new(mut collider: Collider, velocity: Point, explosion: Animation) -> Missile {
        collider.disable();
        Missile {
            state: State::Inactive,
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
        &mut self,
        sprite: &mut Sprite<G2dTexture>,
        explosion_tex: &mut AnimTexture,
        pointer: &mut Sprite<G2dTexture>,
        c: piston_window::Context,
        g: &mut G2d,
    ) -> () {
        use offscreen::{draw_anim_offscreen, draw_offscreen};
        match self.state {
            State::Active => {
                let rot = self.get_rotation();
                sprite.set_position(self.collider.pos.x, self.collider.pos.y);
                sprite.set_rotation(rot);
                sprite.draw(c.transform, g);

                draw_offscreen(sprite, pointer, self.collider.pos, rot, c, g);
            }
            State::Exploding => {
                self.explosion.draw(explosion_tex, c, g);

                draw_anim_offscreen(
                    &mut self.explosion,
                    explosion_tex,
                    pointer,
                    self.collider.pos,
                    c,
                    g,
                );
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
                self.explosion.set_pos(self.collider.pos);
            }
            State::Exploding | State::Inactive => {}
        }
    }

    pub fn place(&mut self, pos: Point, velocity: Point) -> () {
        self.collider.pos = pos;
        self.velocity = velocity;
        self.state = State::Active;
        self.collider.enable();
        self.explosion.stop();
    }

    pub fn reset(&mut self) -> () {
        self.state = State::Inactive;
        self.collider.disable();
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

    fn get_rotation(&self) -> f64 {
        self.velocity.y.atan2(self.velocity.x).to_degrees()
    }
}

pub fn initialise_missiles() -> Vec<Missile> {
    use settings::{game, missile};

    let mut missiles: Vec<Missile> = vec![];

    for _ in 0..game::MAX_MISSILES {
        let missile = Missile::new(
            Collider::new(Point::new(0.0, 0.0), missile::COLLIDER_RADIUS),
            Point::new(0.0, 0.0),
            Animation::new(missile::EXPLOSION_LENGTH, missile::EXPLOSION_ZOOM),
        );

        missiles.push(missile);
    }

    missiles
}

fn place_missile(missile: &mut Missile) -> () {
    use rand::{thread_rng, Rng};
    use settings::{missile_generator, window};

    let mut rng = thread_rng();
    let angle = rng.gen_range(0.0, ::std::f64::consts::PI * 2.0);

    let (width, height) = window::SIZE;
    let pos = Point::new(
        (width as f64) / 2.0 - (angle.cos() * missile_generator::SPAWN_RADIUS),
        (height as f64) / 2.0 - (angle.sin() * missile_generator::SPAWN_RADIUS),
    );
    let velocity = Point::new(0.0, 0.0);

    missile.place(pos, velocity);
}

pub struct Generator {
    time_since_last_missile: f64,
}

impl Generator {
    pub fn new() -> Generator {
        Generator {
            time_since_last_missile: 0.0,
        }
    }

    pub fn update(&mut self, missiles: &mut Vec<Missile>, player: &Player, dt: f64) -> () {
        use settings::missile_generator;

        if !player.is_active() {
            return;
        }

        let mut place_new_missile = false;

        // Place new missile after time
        self.time_since_last_missile += dt;

        if self.time_since_last_missile > missile_generator::TIME_TO_APPEAR {
            self.time_since_last_missile -= missile_generator::TIME_TO_APPEAR;

            place_new_missile = true;
        }

        if place_new_missile {
            if let Some(idx) = missiles.iter().position(|m| m.state == State::Inactive) {
                place_missile(&mut missiles[idx]);
            }
        }
    }

    pub fn reset_missiles(&mut self, missiles: &mut Vec<Missile>) -> () {
        for missile in missiles.iter_mut() {
            missile.reset();
        }

        place_missile(&mut missiles[0]);

        self.time_since_last_missile = 0.0;
    }
}
