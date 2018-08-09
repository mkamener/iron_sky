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

    let player_explosion = Animation::new(
        &mut window,
        &assets,
        "explosions/2.png",
        centre,
        8,
        8,
        settings::player::EXPLOSION_LENGTH,
        settings::player::EXPLOSION_ZOOM,
    );

    let mut player = Player::new(
        Collider::new(centre, settings::player::COLLIDER_RADIUS),
        [spr_player_left, spr_player, spr_player_right],
        player_explosion,
    );

    let missile1_pos = Point::new(width as f64 / 2.0, 0.0);
    let missile1_vel = Point::new(-500.0, 1000.0);

    let missile2_pos = Point::new(0.0, height as f64 / 2.0);
    let missile2_vel = Point::new(1000.0, 0.0);

    let missile_explosion1 = Animation::new(
        &mut window,
        &assets,
        "explosions/3.png",
        centre,
        8,
        8,
        settings::missile::EXPLOSION_LENGTH,
        settings::missile::EXPLOSION_ZOOM,
    );

    let missile_explosion2 = Animation::new(
        &mut window,
        &assets,
        "explosions/4.png",
        centre,
        8,
        8,
        settings::missile::EXPLOSION_LENGTH,
        settings::missile::EXPLOSION_ZOOM,
    );

    let mut missile1 = Missile::new(
        Collider::new(missile1_pos, settings::missile::COLLIDER_RADIUS),
        missile1_vel,
        load_sprite(&mut window, &assets, "missile.png"),
        missile_explosion1,
    );

    let mut missile2 = Missile::new(
        Collider::new(missile2_pos, settings::missile::COLLIDER_RADIUS),
        missile2_vel,
        load_sprite(&mut window, &assets, "missile.png"),
        missile_explosion2,
    );

    let mut background = Background::new(&mut window, &assets, settings::background::FILES);

    let mut left_key = KeyState::NotPressed;
    let mut right_key = KeyState::NotPressed;

    while let Some(e) = window.next() {
        // Render loop
        window.draw_2d(&e, |c, g| {
            clear([1.0; 4], g); // Clear to white

            background.draw(height, width, c, g);
            player.draw(c, g);
            missile1.draw(c, g);
            missile2.draw(c, g);

            // Draw debug shapes if requested
            if settings::game::DRAW_DEBUG {
                player.collider.draw_debug(c, g);
                missile1.collider.draw_debug(c, g);
                missile2.collider.draw_debug(c, g);
            }
        });

        // Input loop
        if let Some(press_args) = e.press_args() {
            match press_args {
                Button::Keyboard(Key::Left) => left_key = KeyState::Pressed,
                Button::Keyboard(Key::Right) => right_key = KeyState::Pressed,
                Button::Keyboard(Key::R) => {
                    player.reset();
                    missile1.reset(missile1_pos, missile1_vel);
                    missile2.reset(missile2_pos, missile2_vel);
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
            missile1.update(&player, u.dt);
            missile2.update(&player, u.dt);
            background.update(&player, u.dt);

            explosion_collisions(&mut player, vec![&mut missile1, &mut missile2]);
        }
    }
}
