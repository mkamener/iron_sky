extern crate piston_window;

use piston_window::*;
use std::ops::{Add, Div, Mul, Sub};

#[derive(Copy, Clone)]
pub enum Actions {
    NoMove,
    Left,
    Right,
}

#[derive(Copy, Clone)]
pub enum KeyState {
    Pressed,
    NotPressed,
}

#[derive(Debug, Copy, Clone)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub for Point {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Point {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Mul<f64> for Point {
    type Output = Self;

    fn mul(self, other: f64) -> Self {
        Point {
            x: self.x * other,
            y: self.y * other,
        }
    }
}

impl Div<f64> for Point {
    type Output = Self;

    fn div(self, other: f64) -> Self {
        if other == 0.0 {
            panic!("Tried to divide a point by zero");
        }
        Point {
            x: self.x / other,
            y: self.y / other,
        }
    }
}

impl Point {
    pub fn new(x: f64, y: f64) -> Point {
        Point { x: x, y: y }
    }

    pub fn normalized(self) -> Point {
        let magnitude = self.magnitude();
        if magnitude > 0.0 {
            Point::new(self.x / magnitude, self.y / magnitude)
        } else {
            Point::new(0.0, 0.0)
        }
    }

    pub fn magnitude(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

pub struct Collider {
    pub pos: Point,
    r: f64,
}

impl Collider {
    pub fn new(pos: Point, r: f64) -> Collider {
        if r <= 0.0 {
            panic!("Radius of collider must be greater than 0");
        }
        Collider { pos: pos, r: r }
    }

    pub fn collides_with(&self, other: &Collider) -> bool {
        let min_distance = self.r + other.r;
        let distance = (self.pos - other.pos).magnitude();
        distance < min_distance
    }

    pub fn draw_debug(&self, c: piston_window::Context, g: &mut G2d) -> () {
        let rect = [
            self.pos.x - self.r,
            self.pos.y - self.r,
            self.r * 2.0,
            self.r * 2.0,
        ];
        ellipse([1.0, 0.0, 0.0, 0.5], rect, c.transform, g);
    }
}

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

pub struct Missile {
    pub collider: Collider,
    pub velocity: Point,
}

impl Missile {
    const MAX_SPEED: f64 = 1200.0;
    const ACCELERATION: f64 = 3000.0;

    pub fn new(collider: Collider, velocity: Point) -> Missile {
        Missile {
            collider: collider,
            velocity: velocity,
        }
    }

    pub fn update(&mut self, player: &Player, dt: f64) {
        // Update position (x = x + v*dt)
        self.collider.pos = self.collider.pos + self.velocity * dt;

        // Update velocity and cap (v = v + a*dt)
        self.velocity = self.velocity
            + (player.collider.pos - self.collider.pos).normalized() * Missile::ACCELERATION * dt;
        if self.velocity.magnitude() >= Missile::MAX_SPEED {
            self.velocity = self.velocity.normalized() * Missile::MAX_SPEED;
        }

        // Update position based off player movement
        let player_dir = Point::new(player.rot.to_radians().cos(), player.rot.to_radians().sin());
        self.collider.pos = self.collider.pos - player_dir * Player::SPEED * dt;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_detect_collision() {
        let collider_1 = Collider::new(Point::new(0.0, 0.0), 1.0);
        let collider_2 = Collider::new(Point::new(1.2, 1.2), 1.0);
        assert_eq!(collider_1.collides_with(&collider_2), true);
    }

    #[test]
    fn it_should_not_detect_collision() {
        let collider_1 = Collider::new(Point::new(0.0, 0.0), 1.0);
        let collider_2 = Collider::new(Point::new(-2.0, -2.0), 1.0);
        assert_eq!(collider_1.collides_with(&collider_2), false);
    }
}
