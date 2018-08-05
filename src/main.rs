extern crate find_folder;
extern crate piston_window;
extern crate sprite;

mod background;
mod game;
mod missile;
mod player;
mod settings;

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

    let mut spr_missile1 = load_sprite(&mut window, &assets, "missile.png");
    let mut spr_missile2 = load_sprite(&mut window, &assets, "missile.png");

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
        Collider::new(Point::new(0.0, 0.0), settings::missile::COLLIDER_RADIUS),
        Point::new(0.0, 1000.0),
        missile_explosion1,
    );

    let mut missile2 = Missile::new(
        Collider::new(
            Point::new(width as f64 / 2.0, height as f64),
            settings::missile::COLLIDER_RADIUS,
        ),
        Point::new(0.0, -100.0),
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
            missile1.draw(&mut spr_missile1, c, g);
            missile2.draw(&mut spr_missile2, c, g);

            // Draw debug shapes if requested
            if settings::game::DRAW_DEBUG {
                player.collider.draw_debug(c, g);
                missile1.collider.draw_debug(c, g);
                missile2.collider.draw_debug(c, g);
            }
        });

        // Input loop
        let (prev_left_key, prev_right_key) = (left_key, right_key);
        if let Some(press_args) = e.press_args() {
            match press_args {
                Button::Keyboard(Key::Left) => left_key = KeyState::Pressed,
                Button::Keyboard(Key::Right) => right_key = KeyState::Pressed,
                Button::Keyboard(Key::R) => {
                    player.reset();
                    missile1.reset(Point::new(width as f64 / 2.0, 0.0), Point::new(1000.0, 0.0));
                    missile2.reset(Point::new(0.0, 0.0), Point::new(0.0, 0.0));
                }
                _ => (),
            }
        }

        if let Some(release_args) = e.release_args() {
            match release_args {
                Button::Keyboard(Key::Left) => left_key = KeyState::NotPressed,
                Button::Keyboard(Key::Right) => right_key = KeyState::NotPressed,
                _ => (),
            }
        }

        // Set player action based on key presses
        if prev_left_key != left_key || prev_right_key != right_key {
            player.input(left_key, right_key);
        }

        // Update loop
        if let Some(u) = e.update_args() {
            player.update(u.dt);
            missile1.update(&player, u.dt);
            missile2.update(&player, u.dt);
            background.update(&player, u.dt);

            // Collisions
            let mut coll_player = false;
            let mut coll_missile1 = false;
            let mut coll_missile2 = false;

            if player.collider.collides_with(&missile1.collider) {
                coll_player = true;
                coll_missile1 = true;
            }
            if player.collider.collides_with(&missile2.collider) {
                coll_player = true;
                coll_missile2 = true;
            }
            if missile1.collider.collides_with(&missile2.collider) {
                coll_missile1 = true;
                coll_missile2 = true;
            }

            if coll_player {
                player.explode();
            }
            if coll_missile1 {
                missile1.explode();
            }
            if coll_missile2 {
                missile2.explode();
            }
        }
    }
}
