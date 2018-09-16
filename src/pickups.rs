extern crate piston_window;

use game::*;
use piston_window::*;
use player::*;
use sprite::*;
use traits::Collides;
use tween::*;

#[derive(Copy, Clone, PartialEq)]
enum State {
    Active,
    Collected,
    Disappearing,
    Inactive,
}

pub struct Pickup {
    state: State,
    pub collider: Collider,
    time_alive: f64,
    rot_tween: Tween,
    collect_opacity_tween: Tween,
    collect_rot_tween: Tween,
    grow_tween: Tween,
    disappear_opacity_tween: Tween,
    shrink_tween: Tween,
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
        use settings::pickup::*;

        collider.disable();
        Pickup {
            state: State::Inactive,
            collider: collider,
            time_alive: 0.0,
            rot_tween: Tween::new(
                vec![(0.0, 0.0), (1.0, 360.0)],
                ROTATION_PERIOD,
                Easing::Linear,
                true,
            ),
            collect_opacity_tween: Tween::new(
                vec![(0.0, 1.0), (1.0, 0.0)],
                COLLECT_FADE_OUT,
                Easing::EaseOut,
                false,
            ),
            collect_rot_tween: Tween::new(
                vec![(0.0, 360.0), (1.0, 0.0)],
                COLLECT_ROTATION_PERIOD,
                Easing::Linear,
                true,
            ),
            grow_tween: Tween::new(
                vec![(0.0, SCALE), (1.0, COLLECT_SCALE)],
                COLLECT_FADE_OUT,
                Easing::EaseOut,
                false,
            ),
            disappear_opacity_tween: Tween::new(
                vec![(0.0, 1.0), (1.0, 0.0)],
                DISAPPEAR_FADE_OUT,
                Easing::EaseOut,
                false,
            ),
            shrink_tween: Tween::new(
                vec![(0.0, SCALE), (1.0, 0.0)],
                DISAPPEAR_FADE_OUT,
                Easing::EaseInOut,
                false,
            ),
        }
    }

    pub fn update(&mut self, player: &Player, dt: f64) {
        use settings::pickup;

        match self.state {
            State::Active => {
                self.time_alive += dt;
                self.rot_tween.update(dt);

                // Update position based off player movement
                self.collider.pos = self.collider.pos - player.velocity() * dt;

                if self.time_alive > pickup::MAX_TIME {
                    self.disappear();
                }
            }
            State::Collected => {
                self.collect_opacity_tween.update(dt);
                self.collect_rot_tween.update(dt);
                self.grow_tween.update(dt);

                // Update position based off player movement
                self.collider.pos = self.collider.pos - player.velocity() * dt;

                if !self.collect_opacity_tween.is_playing() {
                    self.reset();
                }
            }
            State::Disappearing => {
                self.disappear_opacity_tween.update(dt);
                self.rot_tween.update(dt);
                self.shrink_tween.update(dt);

                // Update position based off player movement
                self.collider.pos = self.collider.pos - player.velocity() * dt;

                if !self.disappear_opacity_tween.is_playing() {
                    self.reset();
                }
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
        use offscreen::draw_offscreen;
        use settings::pickup;
        use settings::pickup::POINTER_COLOR;

        match self.state {
            State::Active => {
                sprite.set_position(self.collider.pos.x, self.collider.pos.y);
                sprite.set_rotation(self.rot_tween.get_val());
                sprite.draw(c.transform, g);

                draw_offscreen(sprite, pointer, self.collider.pos, POINTER_COLOR, c, g);
            }
            State::Collected => {
                sprite.set_position(self.collider.pos.x, self.collider.pos.y);
                sprite.set_rotation(self.collect_rot_tween.get_val());
                sprite.set_scale(self.grow_tween.get_val(), self.grow_tween.get_val());
                sprite.set_opacity(self.collect_opacity_tween.get_val() as f32);
                sprite.draw(c.transform, g);

                // Reset scale and opacity
                sprite.set_scale(pickup::SCALE, pickup::SCALE);
                sprite.set_opacity(1.0);
            }
            State::Disappearing => {
                sprite.set_position(self.collider.pos.x, self.collider.pos.y);
                sprite.set_rotation(self.rot_tween.get_val());
                sprite.set_scale(self.shrink_tween.get_val(), self.shrink_tween.get_val());
                sprite.set_opacity(self.disappear_opacity_tween.get_val() as f32);
                sprite.draw(c.transform, g);

                draw_offscreen(sprite, pointer, self.collider.pos, POINTER_COLOR, c, g);

                // Reset scale and opacity
                sprite.set_scale(pickup::SCALE, pickup::SCALE);
                sprite.set_opacity(1.0);
            }
            State::Inactive => {}
        }
    }

    pub fn collect(&mut self) -> () {
        self.state = State::Collected;
        self.collider.disable();
        self.rot_tween.stop();

        self.collect_opacity_tween.reset();
        self.collect_rot_tween.reset();
        self.grow_tween.reset();
    }

    pub fn disappear(&mut self) -> () {
        self.state = State::Disappearing;
        self.collider.disable();

        self.disappear_opacity_tween.reset();
        self.shrink_tween.reset();
    }

    pub fn place(&mut self, pos: Point) -> () {
        self.collider.pos = pos;
        self.state = State::Active;
        self.collider.enable();
        self.rot_tween.reset();
        self.time_alive = 0.0;
    }

    pub fn reset(&mut self) -> () {
        self.state = State::Inactive;
        self.collider.disable();

        self.rot_tween.stop();

        self.collect_opacity_tween.stop();
        self.collect_rot_tween.stop();
        self.grow_tween.stop();
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

        self.time_since_last_pickup = 0.0;
    }
}
