extern crate piston_window;
extern crate sprite;
extern crate find_folder;

use std::rc::Rc;
use piston_window::*;
use sprite::*;

use std::ops::{Add, Sub, Mul, Div};

#[derive(Copy, Clone)]
enum Actions {
    NoMove,
    Left,
    Right,
}

#[derive(Copy, Clone)]
enum KeyState {
    Pressed,
    NotPressed,
}

#[derive(Debug, Copy, Clone)]
struct Point {
    x: f64,
    y: f64,
}

impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Point {x: self.x + other.x, y: self.y + other.y}
    }
}

impl Sub for Point {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Point {x: self.x - other.x, y: self.y - other.y}
    }
}

impl Mul<f64> for Point {
    type Output = Self;

    fn mul(self, other: f64) -> Self {
        Point {x: self.x * other, y: self.y * other}
    }
}

impl Div<f64> for Point {
    type Output = Self;

    fn div(self, other: f64) -> Self {
        if other == 0.0 {
            panic!("Tried to divide a point by zero");
        }
        Point {x: self.x / other, y: self.y / other}
    }
}

impl Point {
    fn new(x: f64, y: f64) -> Point {
        Point {
            x: x,
            y: y,
        }
    }

    fn normalize(p: & Point) -> Point {
        let magnitude = p.magnitude();
        if magnitude > 0.0 {
            Point::new(p.x / magnitude, p.y / magnitude)
        } else {
            Point::new(0.0, 0.0)
        }
    }

    fn magnitude(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()  
    }

}

struct Player {
    pos: Point,
    rot: f64,
}

impl Player {
    fn new(x: f64, y: f64) -> Player {
        Player {
            pos: Point::new(x, y),
            rot: 0.0,
        }
    }

    fn update(&mut self, action: Actions, dt: f64) {
        let added_rotation = 360.0 as f64;
        match action {
            Actions::Left => self.rot = self.rot - added_rotation*dt,
            Actions::Right => self.rot = self.rot + added_rotation*dt,
            Actions::NoMove => (),
        }
    }
}

struct Missile {
    pos: Point,
    velocity: Point,
}

impl Missile {
    const MAX_SPEED: f64 = 300.0;
    const ACCELERATION: f64 = 500.0;

    fn new(pos: Point, velocity: Point) -> Missile {
        Missile {
            pos: pos,
            velocity: velocity,
        }
    }

    fn update(&mut self, target: Point, dt: f64) {
        self.velocity = self.velocity + Point::normalize(&(target - self.pos)) * Missile::ACCELERATION * dt;
        if self.velocity.magnitude() >= Missile::MAX_SPEED {
            self.velocity = Point::normalize(&self.velocity) * Missile::MAX_SPEED;
        }
        self.pos = self.pos + self.velocity * dt;
    }
}

fn main() {
    let (width, height) = (600, 600);
    let opengl = OpenGL::V3_2;
    let mut window: PistonWindow =
        WindowSettings::new("Iron Sky", (width, height))
        .exit_on_esc(true)
        .opengl(opengl)
        .build()
        .unwrap();

    let assets = find_folder::Search::ParentsThenKids(3, 3)
        .for_folder("assets").unwrap();

    let player = Rc::new(Texture::from_path(
            &mut window.factory,
            assets.join("player.png"),
            Flip::None,
            &TextureSettings::new()
        ).unwrap());
    let player_left = Rc::new(Texture::from_path(
            &mut window.factory,
            assets.join("playerLeft.png"),
            Flip::None,
            &TextureSettings::new()
        ).unwrap());
    let player_right = Rc::new(Texture::from_path(
            &mut window.factory,
            assets.join("playerRight.png"),
            Flip::None,
            &TextureSettings::new()
        ).unwrap());
    let missile = Rc::new(Texture::from_path(
            &mut window.factory,
            assets.join("missile.png"),
            Flip::None,
            &TextureSettings::new()
        ).unwrap());

    let mut spr_player = Sprite::from_texture(player.clone());
    spr_player.set_position(width as f64 / 2.0, height as f64 / 2.0);
    spr_player.set_scale(0.8 as f64, 0.8 as f64);

    let mut spr_player_left = Sprite::from_texture(player_left.clone());
    spr_player_left.set_position(width as f64 / 2.0, height as f64 / 2.0);
    spr_player_left.set_scale(0.8 as f64, 0.8 as f64);

    let mut spr_player_right = Sprite::from_texture(player_right.clone());
    spr_player_right.set_position(width as f64 / 2.0, height as f64 / 2.0);
    spr_player_right.set_scale(0.8 as f64, 0.8 as f64);

    let mut spr_missile = Sprite::from_texture(missile.clone());

    let mut player_action = Actions::NoMove;
    let mut player = Player::new(width as f64 / 2.0, height as f64 / 2.0);

    let mut left_key = KeyState::NotPressed;
    let mut right_key = KeyState::NotPressed;

    let mut missile = Missile::new(Point::new(0.0, 0.0), Point::new(100.0, 0.0));

    while let Some(e) = window.next() {
        // Render
        window.draw_2d(&e, |c, g| {
            clear([1.0; 4], g); // Clear to white

            // Draw player sprite
            match player_action {
                Actions::Left => {
                    spr_player_left.set_rotation(player.rot);
                    spr_player_left.draw(c.transform, g);
                },
                Actions::Right => {
                    spr_player_right.set_rotation(player.rot);
                    spr_player_right.draw(c.transform, g);
                }
                Actions::NoMove => {
                    spr_player.set_rotation(player.rot);
                    spr_player.draw(c.transform, g);
                }
            }

            // Draw missile sprite
            spr_missile.set_position(missile.pos.x, missile.pos.y);
            spr_missile.set_rotation(missile.velocity.y.atan2(missile.velocity.x).to_degrees() + 90.0);
            spr_missile.draw(c.transform, g);
        });

        // Check for keyboard input
        match e.press_args() {
            Some(Button::Keyboard(Key::Left)) => left_key = KeyState::Pressed,
            Some(Button::Keyboard(Key::Right)) => right_key = KeyState::Pressed,
            _ => (),
        }

        match e.release_args() {
            Some(Button::Keyboard(Key::Left)) => left_key = KeyState::NotPressed,
            Some(Button::Keyboard(Key::Right)) => right_key = KeyState::NotPressed,
            _ => (),
        }

        // Set player action based on key presses
        match (left_key, right_key) {
            (KeyState::Pressed, KeyState::Pressed) => player_action = Actions::NoMove,
            (KeyState::Pressed, KeyState::NotPressed) => player_action = Actions::Left,
            (KeyState::NotPressed, KeyState::Pressed) => player_action = Actions::Right,
            (KeyState::NotPressed, KeyState::NotPressed) => player_action = Actions::NoMove,
        }

        if let Some(u) = e.update_args() {
            // Update player
            player.update(player_action, u.dt);

            // Update missile
            missile.update(player.pos, u.dt);
        }
    }
}

