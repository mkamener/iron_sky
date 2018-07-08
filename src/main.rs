extern crate piston_window;
extern crate sprite;
extern crate find_folder;

use std::rc::Rc;
use piston_window::*;
use sprite::*;

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

struct Player {
    rot: f64
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

    let mut spr_player = Sprite::from_texture(player.clone());
    spr_player.set_position(width as f64 / 2.0, height as f64 / 2.0);
    spr_player.set_scale(0.8 as f64, 0.8 as f64);

    let mut spr_player_left = Sprite::from_texture(player_left.clone());
    spr_player_left.set_position(width as f64 / 2.0, height as f64 / 2.0);
    spr_player_left.set_scale(0.8 as f64, 0.8 as f64);

    let mut spr_player_right = Sprite::from_texture(player_right.clone());
    spr_player_right.set_position(width as f64 / 2.0, height as f64 / 2.0);
    spr_player_right.set_scale(0.8 as f64, 0.8 as f64);

    let mut player_action = Actions::NoMove;
    let mut player = Player { rot: 0.0 };

    let mut left_key = KeyState::NotPressed;
    let mut right_key = KeyState::NotPressed;

    while let Some(e) = window.next() {
        // Render
        window.draw_2d(&e, |c, g| {
            clear([1.0; 4], g); // Clear to white
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

            let curr_rotation = player.rot;
            let added_rotation = 360.0 as f64;
            match player_action {
                Actions::Left => player.rot = curr_rotation - added_rotation*u.dt,
                Actions::Right => player.rot = curr_rotation + added_rotation*u.dt,
                Actions::NoMove => (),
            }
        }
    }
}

