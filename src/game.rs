extern crate piston_window;
extern crate sprite;

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
        if !self.explosion.playing {
            self.velocity = self.velocity
                + (player.collider.pos - self.collider.pos).normalized()
                    * Missile::ACCELERATION
                    * dt;
            if self.velocity.magnitude() >= Missile::MAX_SPEED {
                self.velocity = self.velocity.normalized() * Missile::MAX_SPEED;
            }
        }

        // Update position based off player movement
        let player_dir = Point::new(player.rot.to_radians().cos(), player.rot.to_radians().sin());
        self.collider.pos = self.collider.pos - player_dir * Player::SPEED * dt;

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
        match self.explosion.playing {
            true => {
                self.explosion.draw(c, g);
            }
            false => {
                sprite.set_position(self.collider.pos.x, self.collider.pos.y);
                sprite.set_rotation(self.velocity.y.atan2(self.velocity.x).to_degrees());
                sprite.draw(c.transform, g);
            }
        }
        // Draw missile sprite
    }
}

pub struct BGLayer {
    sprite: Sprite<G2dTexture>,
    pos: Point,
    clamp: Point,
    factor: f64,
}

impl BGLayer {
    fn new(sprite: Sprite<G2dTexture>, factor: f64) -> BGLayer {
        let clamp = sprite.bounding_box();
        let clamp = Point::new(clamp[2], clamp[3]);
        BGLayer {
            sprite: sprite,
            pos: Point::new(0.0, 0.0),
            clamp: clamp,
            factor: factor,
        }
    }

    fn update(&mut self, player: &Player, dt: f64) {
        // Update position based off player movement
        let player_dir = Point::new(player.rot.to_radians().cos(), player.rot.to_radians().sin());
        self.pos = self.pos - player_dir * Player::SPEED * dt * self.factor;

        // Clamp position to bounding box
        let new_x = ((self.pos.x % self.clamp.x) + self.clamp.x) % self.clamp.x;
        let new_y = ((self.pos.y % self.clamp.y) + self.clamp.y) % self.clamp.y;
        self.pos = Point::new(new_x, new_y);
    }

    fn draw(
        &mut self,
        height: u32,
        width: u32,
        context: piston_window::Context,
        g: &mut G2d,
    ) -> () {
        let max_x = ((width as f64) / (self.clamp.x)) as i32 + 1;
        let max_y = ((height as f64) / (self.clamp.y)) as i32 + 1;

        for x in -1..=max_x {
            for y in -1..=max_y {
                let x_pos = self.pos.x + (x as f64) * self.clamp.x;
                let y_pos = self.pos.y + (y as f64) * self.clamp.y;

                self.sprite.set_position(x_pos, y_pos);
                self.sprite.draw(context.transform, g);
            }
        }
    }
}

pub struct Background(Vec<BGLayer>);

impl Background {
    pub fn new(
        window: &mut PistonWindow,
        folder: &::std::path::PathBuf,
        names_and_factors: Vec<(&str, f64)>,
    ) -> Background {
        let mut all_bg: Vec<BGLayer> = vec![];

        for (file, factor) in names_and_factors {
            let bg = load_sprite(window, folder, file);
            all_bg.push(BGLayer::new(bg, factor));
        }

        Background(all_bg)
    }

    pub fn update(&mut self, player: &Player, dt: f64) -> () {
        let Background(ref mut backgrounds) = *self;
        for bg in backgrounds.iter_mut() {
            bg.update(player, dt);
        }
    }

    pub fn draw(&mut self, height: u32, width: u32, context: piston_window::Context, g: &mut G2d) {
        let Background(ref mut backgrounds) = *self;
        for bg in &mut backgrounds.iter_mut() {
            bg.draw(height, width, context, g);
        }
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
