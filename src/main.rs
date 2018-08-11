extern crate find_folder;
extern crate piston_window;
extern crate sprite;

mod background;
mod game;
mod missile;
mod player;
mod settings;
mod traits;

use background::*;
use game::*;
use missile::*;
use piston_window::*;
use player::*;

fn main() {
    let (width, height) = settings::window::SIZE;
    let centre = Point::new(width as f64 / 2.0, height as f64 / 2.0);
    let opengl = OpenGL::V3_2;
    let mut window: PistonWindow = WindowSettings::new("Iron Sky", (width, height))
        .exit_on_esc(true)
        .opengl(opengl)
        .build()
        .unwrap();

    // window.set_bench_mode(true);

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

    let mut tex_explosion_player = AnimTexture::new(&mut window, &assets, "explosions/2.png", 8, 8);
    let mut spr_player = [spr_player_left, spr_player, spr_player_right];
    let mut player = Player::new(
        Collider::new(centre, settings::player::COLLIDER_RADIUS),
        Animation::new(
            settings::player::EXPLOSION_LENGTH,
            settings::player::EXPLOSION_ZOOM,
        ),
    );

    let mut background = Background::new(&mut window, &assets, settings::background::FILES);

    let mut spr_missile = load_sprite(&mut window, &assets, "missile.png");
    let mut tex_explosion_missile =
        AnimTexture::new(&mut window, &assets, "explosions/4.png", 8, 8);

    let mut missiles = initialise_missiles();

    let mut missile_gen = Generator::new();
    missile_gen.reset_missiles(&mut missiles);

    let mut left_key = KeyState::NotPressed;
    let mut right_key = KeyState::NotPressed;

    while let Some(e) = window.next() {
        // Render loop
        window.draw_2d(&e, |c, g| {
            clear([1.0; 4], g); // Clear to white

            background.draw(height, width, c, g);
            player.draw(&mut spr_player, &mut tex_explosion_player, c, g);
            for missile in &mut missiles {
                missile.draw(&mut spr_missile, &mut tex_explosion_missile, c, g);
            }

            // Draw debug shapes if requested
            if settings::game::DRAW_DEBUG {
                player.collider.draw_debug(c, g);
                missiles[0].collider.draw_debug(c, g);
                missiles[1].collider.draw_debug(c, g);
            }
        });

        // Input loop
        if let Some(press_args) = e.press_args() {
            match press_args {
                Button::Keyboard(Key::Left) => left_key = KeyState::Pressed,
                Button::Keyboard(Key::Right) => right_key = KeyState::Pressed,
                Button::Keyboard(Key::R) => {
                    missile_gen.reset_missiles(&mut missiles);
                    player.reset();
                }
                _ => (),
            }
            player.input(left_key, right_key);
        }

        if let Some(release_args) = e.release_args() {
            match release_args {
                Button::Keyboard(Key::Left) => left_key = KeyState::NotPressed,
                Button::Keyboard(Key::Right) => right_key = KeyState::NotPressed,
                _ => (),
            }
            player.input(left_key, right_key);
        }

        // Update loop
        if let Some(u) = e.update_args() {
            player.update(u.dt);
            for missile in &mut missiles {
                missile.update(&player, u.dt);
            }
            background.update(&player, u.dt);
            missile_gen.update(&mut missiles, &player, u.dt);

            explosion_collisions(&mut player, &mut missiles);
        }
    }
}
