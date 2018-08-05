extern crate find_folder;
extern crate piston_window;
extern crate sprite;

mod background;
mod game;
mod missile;
mod player;

use background::*;
use game::*;
use missile::*;
use piston_window::*;
use player::*;

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

    let draw_debug = false;

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

    let player_explosion = Animation::new(
        &mut window,
        &assets,
        "explosions/2.png",
        centre,
        8,
        8,
        1.0,
        1.5,
    );

    let mut player = Player::new(
        Collider::new(centre, 35.0),
        [spr_player_left, spr_player, spr_player_right],
        player_explosion,
    );

    let missile_explosion = Animation::new(
        &mut window,
        &assets,
        "explosions/4.png",
        centre,
        8,
        8,
        0.5,
        1.0,
    );

    let mut missile = Missile::new(
        Collider::new(Point::new(width as f64 / 2.0, height as f64), 15.0),
        Point::new(0.0, -100.0),
        missile_explosion,
    );

    let bg_files = vec![
        ("bkgd_0.png", 0.0),
        ("bkgd_1.png", 0.01),
        ("bkgd_2.png", 0.02),
        ("bkgd_3.png", 0.03),
        ("bkgd_4.png", 0.04),
        ("bkgd_5.png", 0.05),
        ("bkgd_6.png", 0.5),
        ("bkgd_7.png", 1.0),
    ];
    let mut background = Background::new(&mut window, &assets, bg_files);

    let mut left_key = KeyState::NotPressed;
    let mut right_key = KeyState::NotPressed;

    while let Some(e) = window.next() {
        // Render
        window.draw_2d(&e, |c, g| {
            clear([1.0; 4], g); // Clear to white

            background.draw(height, width, c, g);
            player.draw(c, g);
            missile.draw(&mut spr_missile, c, g);

            // Draw debug shapes if requested
            if draw_debug {
                player.collider.draw_debug(c, g);
                missile.collider.draw_debug(c, g);
            }
        });

        // Check for keyboard input
        match e.press_args() {
            Some(Button::Keyboard(Key::Left)) => left_key = KeyState::Pressed,
            Some(Button::Keyboard(Key::Right)) => right_key = KeyState::Pressed,
            Some(Button::Keyboard(Key::E)) => player.explode(),
            Some(Button::Keyboard(Key::W)) => missile.explode(),
            Some(Button::Keyboard(Key::Q)) => {
                missile.reset(Point::new(0.0, 0.0), Point::new(0.0, 0.0))
            }
            Some(Button::Keyboard(Key::R)) => player.reset(),
            _ => (),
        }

        match e.release_args() {
            Some(Button::Keyboard(Key::Left)) => left_key = KeyState::NotPressed,
            Some(Button::Keyboard(Key::Right)) => right_key = KeyState::NotPressed,
            _ => (),
        }

        // Set player action based on key presses
        player.input(left_key, right_key);

        if let Some(u) = e.update_args() {
            // Update player
            player.update(u.dt);

            // Update missile
            missile.update(&player, u.dt);

            // Update background position
            background.update(&player, u.dt);
        }
    }
}
