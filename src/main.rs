extern crate find_folder;
extern crate piston_window;
extern crate sprite;

use piston_window::*;
use sprite::*;
use std::rc::Rc;

use std::ops::{Add, Div, Mul, Sub};

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
    fn new(x: f64, y: f64) -> Point {
        Point { x: x, y: y }
    }

    fn normalized(self) -> Point {
        let magnitude = self.magnitude();
        if magnitude > 0.0 {
            Point::new(self.x / magnitude, self.y / magnitude)
        } else {
            Point::new(0.0, 0.0)
        }
    }

    fn magnitude(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

struct Collider {
    pos: Point,
    r: f64,
}

impl Collider {
    fn new(pos: Point, r: f64) -> Collider {
        if r <= 0.0 {
            panic!("Radius of collider must be greater than 0");
        }
        Collider { pos: pos, r: r }
    }

    fn collides_with(&self, other: &Collider) -> bool {
        let min_distance = self.r + other.r;
        let distance = (self.pos - other.pos).magnitude();
        distance < min_distance
    }
}

struct Player {
    collider: Collider,
    rot: f64,
}

impl Player {
    const SPEED: f64 = 800.0;

    fn new(collider: Collider) -> Player {
        Player {
            collider: collider,
            rot: 0.0,
        }
    }

    fn update(&mut self, action: Actions, dt: f64) {
        let added_rotation = 270.0 as f64;
        match action {
            Actions::Left => self.rot = self.rot - added_rotation * dt,
            Actions::Right => self.rot = self.rot + added_rotation * dt,
            Actions::NoMove => (),
        }
    }
}

struct Missile {
    collider: Collider,
    velocity: Point,
}

impl Missile {
    const MAX_SPEED: f64 = 1200.0;
    const ACCELERATION: f64 = 3000.0;

    fn new(collider: Collider, velocity: Point) -> Missile {
        Missile {
            collider: collider,
            velocity: velocity,
        }
    }

    fn update(&mut self, player: &Player, dt: f64) {
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

struct ScrollingBG {
    pos: Point,
    clamp: Point,
    factor: f64,
}

impl ScrollingBG {
    fn new(clamp: Point, factor: f64) -> ScrollingBG {
        ScrollingBG {
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
}

fn draw_tiled_backgound(
    height: u32,
    width: u32,
    sprite: &mut Sprite<G2dTexture>,
    scroller: &ScrollingBG,
    context: piston_window::Context,
    g: &mut G2d,
) -> () {
    let max_x = ((width as f64) / (scroller.clamp.x)) as i32 + 1;
    let max_y = ((height as f64) / (scroller.clamp.y)) as i32 + 1;

    for x in -1..=max_x {
        for y in -1..=max_y {
            let x_pos = scroller.pos.x + (x as f64) * scroller.clamp.x;
            let y_pos = scroller.pos.y + (y as f64) * scroller.clamp.y;

            sprite.set_position(x_pos, y_pos);
            sprite.draw(context.transform, g);
        }
    }
}

fn load_sprite(
    window: &mut PistonWindow,
    folder: &std::path::PathBuf,
    file: &str,
) -> Sprite<G2dTexture> {
    let texture = Texture::from_path(
        &mut window.factory,
        folder.join(file),
        Flip::None,
        &TextureSettings::new(),
    ).unwrap();
    Sprite::from_texture(Rc::new(texture))
}

fn main() {
    let (width, height) = (1280, 720);
    let centre = Point::new(width as f64 / 2.0, height as f64 / 2.0);
    let opengl = OpenGL::V3_2;
    let mut window: PistonWindow = WindowSettings::new("Iron Sky", (width, height))
        .exit_on_esc(true)
        .opengl(opengl)
        .build()
        .unwrap();

    let assets = find_folder::Search::ParentsThenKids(3, 3)
        .for_folder("assets")
        .unwrap();

    let mut spr_player = load_sprite(&mut window, &assets, "player.png");
    spr_player.set_position(centre.x, centre.y);
    spr_player.set_scale(0.8 as f64, 0.8 as f64);

    let mut spr_player_left = load_sprite(&mut window, &assets, "playerLeft.png");
    spr_player_left.set_position(centre.x, centre.y);
    spr_player_left.set_scale(0.8 as f64, 0.8 as f64);

    let mut spr_player_right = load_sprite(&mut window, &assets, "playerRight.png");
    spr_player_right.set_position(centre.x, centre.y);
    spr_player_right.set_scale(0.8 as f64, 0.8 as f64);

    let mut spr_missile = load_sprite(&mut window, &assets, "missile.png");

    let mut player_action = Actions::NoMove;
    let mut player = Player::new(Collider::new(centre, 50.0));

    let mut left_key = KeyState::NotPressed;
    let mut right_key = KeyState::NotPressed;

    let mut missile = Missile::new(
        Collider::new(Point::new(width as f64 / 2.0, height as f64), 10.0),
        Point::new(0.0, -100.0),
    );

    // Backgrounds
    let mut bg_0 = load_sprite(&mut window, &assets, "bkgd_0.png");
    // bg_0.set_scale(2.0, 2.0);
    let clamp_0 = bg_0.bounding_box();
    let clamp_0 = Point::new(clamp_0[2], clamp_0[3]);
    let mut scroll_bg_0 = ScrollingBG::new(clamp_0, 0.0);

    let mut bg_1 = load_sprite(&mut window, &assets, "bkgd_1.png");
    // bg_1.set_scale(2.0, 2.0);
    let clamp_1 = bg_1.bounding_box();
    let clamp_1 = Point::new(clamp_1[2], clamp_1[3]);
    let mut scroll_bg_1 = ScrollingBG::new(clamp_1, 0.01);

    let mut bg_2 = load_sprite(&mut window, &assets, "bkgd_2.png");
    // bg_2.set_scale(2.0, 2.0);
    let clamp_2 = bg_2.bounding_box();
    let clamp_2 = Point::new(clamp_2[2], clamp_2[3]);
    let mut scroll_bg_2 = ScrollingBG::new(clamp_2, 0.05);

    let mut bg_3 = load_sprite(&mut window, &assets, "bkgd_3.png");
    // bg_3.set_scale(2.0, 2.0);
    let clamp_3 = bg_3.bounding_box();
    let clamp_3 = Point::new(clamp_3[2], clamp_3[3]);
    let mut scroll_bg_3 = ScrollingBG::new(clamp_3, 0.07);

    let mut bg_4 = load_sprite(&mut window, &assets, "bkgd_4.png");
    // bg_4.set_scale(2.0, 2.0);
    let clamp_4 = bg_4.bounding_box();
    let clamp_4 = Point::new(clamp_4[2], clamp_4[3]);
    let mut scroll_bg_4 = ScrollingBG::new(clamp_4, 0.1);

    let mut bg_7 = load_sprite(&mut window, &assets, "bkgd_7.png");
    // bg_7.set_scale(2.0, 2.0);
    let clamp_7 = bg_7.bounding_box();
    let clamp_7 = Point::new(clamp_7[2], clamp_7[3]);
    let mut scroll_bg_7 = ScrollingBG::new(clamp_7, 1.0);

    while let Some(e) = window.next() {
        // Render
        window.draw_2d(&e, |c, g| {
            clear([1.0; 4], g); // Clear to white

            // Draw background
            draw_tiled_backgound(height, width, &mut bg_0, &scroll_bg_0, c, g);
            draw_tiled_backgound(height, width, &mut bg_1, &scroll_bg_1, c, g);
            draw_tiled_backgound(height, width, &mut bg_2, &scroll_bg_2, c, g);
            draw_tiled_backgound(height, width, &mut bg_3, &scroll_bg_3, c, g);
            draw_tiled_backgound(height, width, &mut bg_4, &scroll_bg_4, c, g);
            draw_tiled_backgound(height, width, &mut bg_7, &scroll_bg_7, c, g);

            // Draw player sprite
            match player_action {
                Actions::Left => {
                    spr_player_left.set_rotation(player.rot);
                    spr_player_left.draw(c.transform, g);
                }
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
            spr_missile.set_position(missile.collider.pos.x, missile.collider.pos.y);
            spr_missile.set_rotation(missile.velocity.y.atan2(missile.velocity.x).to_degrees());
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
            missile.update(&player, u.dt);

            // Update background position
            scroll_bg_0.update(&player, u.dt);
            scroll_bg_1.update(&player, u.dt);
            scroll_bg_2.update(&player, u.dt);
            scroll_bg_3.update(&player, u.dt);
            scroll_bg_4.update(&player, u.dt);
            scroll_bg_7.update(&player, u.dt);

            // Display on command line if missile and player are colliding
            if player.collider.collides_with(&missile.collider) {
                println!("The player is colliding!!!")
            }
        }
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
