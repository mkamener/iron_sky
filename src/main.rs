extern crate find_folder;
extern crate piston_window;
extern crate rand;
extern crate sprite;

mod background;
mod game;
mod missile;
mod pickups;
mod player;
mod settings;
mod traits;

use background::*;
use game::*;
use missile::*;
use pickups::*;
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

    // Player
    let mut tex_explosion_player = AnimTexture::new(&mut window, &assets, "explosions/2.png", 8, 8);
    let mut spr_player = initialise_player_sprites(
        &mut window,
        &assets,
        ["playerLeft.png", "player.png", "playerRight.png"],
        0.8,
    );
    let mut player = Player::new(
        Collider::new(centre, settings::player::COLLIDER_RADIUS),
        Animation::new(
            settings::player::EXPLOSION_LENGTH,
            settings::player::EXPLOSION_ZOOM,
        ),
    );

    // Missiles
    let mut spr_missile = load_sprite(&mut window, &assets, "missile.png");
    let mut tex_explosion_missile =
        AnimTexture::new(&mut window, &assets, "explosions/4.png", 8, 8);

    let mut missiles = initialise_missiles();
    let mut missile_gen = missile::Generator::new();
    missile_gen.reset_missiles(&mut missiles);

    // Pickups
    let mut spr_pickup = load_sprite(&mut window, &assets, "star.png");
    spr_pickup.set_scale(0.3, 0.3);

    let mut pickups = initialise_pickups();
    let mut pickup_gen = pickups::Generator::new();
    pickup_gen.reset_pickups(&mut pickups);

    // Background
    let mut background = Background::new(&mut window, &assets, settings::background::FILES);

    // Key State
    let mut left_key = KeyState::NotPressed;
    let mut right_key = KeyState::NotPressed;

    while let Some(e) = window.next() {
        // Render loop
        window.draw_2d(&e, |c, g| {
            clear([1.0; 4], g); // Clear to white

            // Render objects in background first
            background.draw(height, width, c, g);

            for pickup in &mut pickups {
                pickup.draw(&mut spr_pickup, c, g);
            }
            for missile in &mut missiles {
                missile.draw(&mut spr_missile, &mut tex_explosion_missile, c, g);
            }
            player.draw(&mut spr_player, &mut tex_explosion_player, c, g);

            // Draw debug shapes if requested
            if settings::game::DRAW_DEBUG {
                player.collider.draw_debug(c, g);
                for missile in &mut missiles {
                    missile.collider.draw_debug(c, g);
                }
                for pickup in &mut pickups {
                    pickup.collider.draw_debug(c, g);
                }
            }
        });

        // Input loop
        if let Some(press_args) = e.press_args() {
            match press_args {
                Button::Keyboard(Key::Left) => left_key = KeyState::Pressed,
                Button::Keyboard(Key::Right) => right_key = KeyState::Pressed,
                Button::Keyboard(Key::R) => {
                    missile_gen.reset_missiles(&mut missiles);
                    pickup_gen.reset_pickups(&mut pickups);
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
            for pickup in &mut pickups {
                pickup.update(&player, u.dt);
            }

            background.update(&player, u.dt);
            missile_gen.update(&mut missiles, &player, u.dt);
            pickup_gen.update(&mut pickups, &player, u.dt);

            explosion_collisions(&mut player, &mut missiles);
            collect_collisions(&player, &mut pickups);
        }
    }
}
