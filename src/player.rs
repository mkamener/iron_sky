use game::*;

pub struct Player {
    pub collider: Collider,
    pub rot: f64,
}

impl Player {
    pub const SPEED: f64 = 800.0;

    pub fn new(collider: Collider) -> Player {
        Player {
            collider: collider,
            rot: 0.0,
        }
    }

    pub fn update(&mut self, action: Actions, dt: f64) {
        let added_rotation = 270.0 as f64;
        match action {
            Actions::Left => self.rot = self.rot - added_rotation * dt,
            Actions::Right => self.rot = self.rot + added_rotation * dt,
            Actions::NoMove => (),
        }
    }
}
