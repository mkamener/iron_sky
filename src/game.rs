extern crate piston_window;

use piston_window::*;
use sprite::*;
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

pub struct Animation {
    texture: G2dTexture,
    frames: Vec<[f64; 4]>,
    pos: Point,
    frame_idx: usize,
    length: f64,
    duration: f64,
    playing: bool,
    zoom: f64,
}

impl Animation {
    pub fn new(
        window: &mut PistonWindow,
        folder: &::std::path::PathBuf,
        file: &str,
        pos: Point,
        rows: u32,
        cols: u32,
        length: f64,
        zoom: f64,
    ) -> Animation {
        let mut frames = vec![];
        let texture = load_texture(window, folder, file);
        let (width, height) = texture.get_size();
        let (spr_width, spr_height) = (width / cols, height / rows);

        for y in 0..rows {
            for x in 0..cols {
                frames.push([
                    (x * spr_width) as f64,
                    (y * spr_height) as f64,
                    spr_width as f64,
                    spr_height as f64,
                ]);
            }
        }
        Animation {
            texture: texture,
            frames: frames,
            pos: pos,
            frame_idx: 0,
            length: length,
            duration: 0.0,
            playing: false,
            zoom: zoom,
        }
    }

    pub fn draw(&self, c: piston_window::Context, g: &mut G2d) -> () {
        if self.playing == false {
            return;
        };
        let frame = self.frames[self.frame_idx];
        Image::new().src_rect(frame).draw(
            &self.texture,
            &c.draw_state,
            c.transform
                .trans(
                    -0.5 * self.zoom * frame[2] + self.pos.x,
                    -0.5 * self.zoom * frame[3] + self.pos.y,
                )
                .zoom(self.zoom),
            g,
        );
    }

    pub fn update(&mut self, dt: f64) -> () {
        if self.playing == false {
            return;
        };
        self.duration = self.duration + dt;
        let idx = ((self.duration / self.length) * (self.frames.len() as f64)).floor() as usize;
        if idx < self.frames.len() {
            self.frame_idx = idx;
        } else {
            self.stop();
        }
    }

    pub fn play(&mut self) -> () {
        self.duration = 0.0;
        self.playing = true;
    }

    pub fn stop(&mut self) -> () {
        self.playing = false;
        self.duration = 0.0;
    }

    pub fn set_pos(&mut self, pos: Point) -> () {
        self.pos = pos;
    }

    pub fn is_playing(&self) -> bool {
        self.playing
    }
}

pub fn load_sprite(
    window: &mut PistonWindow,
    folder: &::std::path::PathBuf,
    file: &str,
) -> Sprite<G2dTexture> {
    let texture = Texture::from_path(
        &mut window.factory,
        folder.join(file),
        Flip::None,
        &TextureSettings::new(),
    ).unwrap();
    Sprite::from_texture(::std::rc::Rc::new(texture))
}

pub fn load_texture(
    window: &mut PistonWindow,
    folder: &::std::path::PathBuf,
    file: &str,
) -> G2dTexture {
    Texture::from_path(
        &mut window.factory,
        folder.join(file),
        Flip::None,
        &TextureSettings::new(),
    ).unwrap()
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
