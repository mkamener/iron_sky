extern crate piston_window;

use game::*;
use piston_window::*;
use settings::{offscreen_pointer, window};
use sprite::Sprite;

pub fn draw_offscreen(
    obj_spr: &mut Sprite<G2dTexture>,
    pointer_spr: &mut Sprite<G2dTexture>,
    obj_pos: Point,
    obj_rot: f64,
    c: piston_window::Context,
    g: &mut G2d,
) -> () {
    if let Some((pos, rot)) = place_pointer(obj_pos) {
        draw_pointer(pointer_spr, pos, rot, c, g);
        draw_overlay(obj_spr, pos, obj_rot, c, g)
    }
}

pub fn draw_anim_offscreen(
    obj_anim: &mut Animation,
    obj_anim_tex: &mut AnimTexture,
    pointer_spr: &mut Sprite<G2dTexture>,
    obj_pos: Point,
    c: piston_window::Context,
    g: &mut G2d,
) -> () {
    if let Some((pos, rot)) = place_pointer(obj_pos) {
        draw_pointer(pointer_spr, pos, rot, c, g);
        obj_anim.draw_at_pos(obj_anim_tex, pos, 0.4, c, g);
    }
}

fn draw_pointer(
    sprite: &mut Sprite<G2dTexture>,
    pos: Point,
    rot: f64,
    c: piston_window::Context,
    g: &mut G2d,
) -> () {
    sprite.set_position(pos.x, pos.y);
    sprite.set_rotation(rot);
    sprite.draw(c.transform, g);
}

fn draw_overlay(
    sprite: &mut Sprite<G2dTexture>,
    pos: Point,
    rot: f64,
    c: piston_window::Context,
    g: &mut G2d,
) -> () {
    let (x_scale, y_scale) = sprite.get_scale();
    sprite.set_scale(x_scale * 0.4, y_scale * 0.4);
    sprite.set_position(pos.x, pos.y);
    sprite.set_rotation(rot);
    sprite.draw(c.transform, g);

    // Set scale back
    sprite.set_scale(x_scale, y_scale);
}

fn is_offscreen(pos: Point) -> bool {
    let (screen_x, screen_y) = window::SIZE;
    let (screen_x, screen_y) = (screen_x as f64, screen_y as f64);

    pos.x <= 0.0 || pos.y <= 0.0 || pos.x >= screen_x || pos.y >= screen_y
}

fn place_pointer(obj_pos: Point) -> Option<(Point, f64)> {
    if !is_offscreen(obj_pos) {
        return None;
    }

    let offset = offscreen_pointer::OFFSET;
    let (screen_x, screen_y) = window::SIZE;
    let (screen_x, screen_y) = (screen_x as f64, screen_y as f64);

    // Shift coordinates so that player is at (0,0)
    let pos = obj_pos - Point::new(screen_x / 2.0, screen_y / 2.0);

    let (min_x, min_y) = (-screen_x / 2.0 + offset, -screen_y / 2.0 + offset);
    let (max_x, max_y) = (screen_x / 2.0 - offset, screen_y / 2.0 - offset);

    // Early exit if object point is directly above or below, to avoid zero division
    if pos.x == 0.0 {
        if obj_pos.y <= 0.0 {
            // Directly above
            return Some((Point::new(0.0, offset), 0.0));
        }
        if obj_pos.y >= screen_y {
            // Directly below
            return Some((Point::new(0.0, screen_y - offset), 180.0));
        }
        return None;
    }

    // Find gradient of line between player and object point
    let m = pos.y / pos.x;

    // First check what coords would be if you only considered going out of bounds on left or right side
    let mut x1 = pos.x;
    let mut y1 = pos.y;
    if obj_pos.x <= 0.0 {
        x1 = min_x;
        y1 = x1 * m;
    } else if obj_pos.x >= screen_x {
        x1 = max_x;
        y1 = x1 * m;
    }

    // Then check what coors would be if you only considered going out of bounds on top or bottom side
    let mut x2 = pos.x;
    let mut y2 = pos.y;
    if obj_pos.y <= 0.0 {
        y2 = min_y;
        x2 = y2 / m;
    } else if obj_pos.y >= screen_y {
        y2 = max_y;
        x2 = y2 / m;
    }

    // Finally check which version of coords gives the correct representation within the screen
    let final_pos: Point;
    if x1 >= min_x && x1 <= max_x && y1 >= min_y && y1 <= max_y {
        final_pos = Point::new(x1 + screen_x / 2.0, y1 + screen_y / 2.0);
    } else {
        final_pos = Point::new(x2 + screen_x / 2.0, y2 + screen_y / 2.0);
    }

    Some((
        final_pos,
        pos.y.atan2(pos.x) * 180.0 / ::std::f64::consts::PI + 90.0, // Add 90 so that 0 rotation is upwards
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_be_offscreen() {
        let (x, y) = window::SIZE;
        let (x, y) = (x as f64, y as f64);
        let pos1 = Point::new(x + 1.0, y / 2.0);
        let pos2 = Point::new(x / 2.0, y + 1.0);
        let pos3 = Point::new(-1.0, y / 2.0);
        let pos4 = Point::new(x / 2.0, -1.0);

        assert_eq!(is_offscreen(pos1), true);
        assert_eq!(is_offscreen(pos2), true);
        assert_eq!(is_offscreen(pos3), true);
        assert_eq!(is_offscreen(pos4), true);
    }

    #[test]
    fn it_should_not_be_offscreen() {
        let (x, y) = window::SIZE;
        let (x, y) = (x as f64, y as f64);
        let pos1 = Point::new(x / 2.0, y / 2.0);

        assert_eq!(is_offscreen(pos1), false);
    }
}
