extern crate piston_window;

use game::*;
use piston_window::*;
use player::*;
use sprite::*;
use traits::Collides;

#[derive(Copy, Clone, PartialEq)]
enum State {
    Active,
    Inactive,
}

pub struct Pickup {
    state: State,
    pub collider: Collider,
    time_alive: f64,
    rot: f64,
}

impl Collides for Pickup {
    fn collides_with<C: Collides>(&self, other: &C) -> bool {
        self.collider.collides_with(other.get_collider())
    }

    fn get_collider(&self) -> &Collider {
        &self.collider
    }
}

impl Pickup {
    pub fn new(mut collider: Collider) -> Pickup {
        collider.disable();
        Pickup {
            state: State::Inactive,
            collider: collider,
            time_alive: 0.0,
            rot: 0.0,
        }
    }

    pub fn update(&mut self, player: &Player, dt: f64) {
        match self.state {
            State::Active => {
                self.time_alive += dt;

                // Update position based off player movement
                self.collider.pos = self.collider.pos - player.velocity() * dt;
            }
            State::Inactive => {}
        }
    }

    pub fn draw(
        &mut self,
        sprite: &mut Sprite<G2dTexture>,
        pointer: &mut Sprite<G2dTexture>,
        c: piston_window::Context,
        g: &mut G2d,
    ) -> () {
        use offscreen;
        match self.state {
            State::Active => {
                self.set_rotation();
                sprite.set_position(self.collider.pos.x, self.collider.pos.y);
                sprite.set_rotation(self.rot);
                sprite.draw(c.transform, g);

                if let Some((pos, deg)) = offscreen::place_pointer(self.collider.pos) {
                    offscreen::draw_pointer(pointer, pos, deg, c, g);
                }
            }
            State::Inactive => {}
        }
    }

    pub fn collect(&mut self) -> () {
        self.reset();
    }

    pub fn place(&mut self, pos: Point) -> () {
        self.collider.pos = pos;
        self.state = State::Active;
        self.collider.enable();
    }

    pub fn reset(&mut self) -> () {
        self.state = State::Inactive;
        self.collider.disable();
    }

    fn set_rotation(&mut self) -> () {
        use settings::pickup;

        self.rot = 360.0 * (self.time_alive / pickup::ROTATION_PERIOD);
    }
}

pub fn initialise_pickups() -> Vec<Pickup> {
    use settings::{game, pickup};

    let mut pickups: Vec<Pickup> = vec![];

    for _ in 0..game::MAX_PICKUPS {
        let pickup = Pickup::new(Collider::new(Point::new(0.0, 0.0), pickup::COLLIDER_RADIUS));

        pickups.push(pickup);
    }

    pickups
}

fn place_pickup(pickup: &mut Pickup) -> () {
    use rand::{thread_rng, Rng};
    use settings::{pickup_generator, window};

    let mut rng = thread_rng();
    let angle = rng.gen_range(0.0, ::std::f64::consts::PI * 2.0);
    let radius = rng.gen_range(
        pickup_generator::MIN_SPAWN_RADIUS,
        pickup_generator::MAX_SPAWN_RADIUS,
    );

    let (width, height) = window::SIZE;
    let pos = Point::new(
        (width as f64) / 2.0 - (angle.cos() * radius),
        (height as f64) / 2.0 - (angle.sin() * radius),
    );

    pickup.place(pos);
}

pub struct Generator {
    time_since_last_pickup: f64,
}

impl Generator {
    pub fn new() -> Generator {
        Generator {
            time_since_last_pickup: 0.0,
        }
    }

    pub fn update(&mut self, pickups: &mut Vec<Pickup>, player: &Player, dt: f64) -> () {
        use settings::pickup_generator;

        if !player.is_active() {
            return;
        }

        let mut place_new_pickup = false;

        // Place new pickup after time
        self.time_since_last_pickup += dt;

        if self.time_since_last_pickup > pickup_generator::TIME_TO_APPEAR {
            self.time_since_last_pickup -= pickup_generator::TIME_TO_APPEAR;

            place_new_pickup = true;
        }

        if place_new_pickup {
            if let Some(idx) = pickups.iter().position(|m| m.state == State::Inactive) {
                place_pickup(&mut pickups[idx]);
            }
        }
    }

    pub fn reset_pickups(&mut self, pickups: &mut Vec<Pickup>) -> () {
        for pickup in pickups.iter_mut() {
            pickup.reset();
        }

        place_pickup(&mut pickups[0]);

        self.time_since_last_pickup = 0.0;
    }
}
