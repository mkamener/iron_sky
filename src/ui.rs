extern crate piston_window;

use game::*;
use piston_window::*;
use settings::ui::*;

pub fn draw_text(
    text: &str,
    transform: [[f64; 3]; 2],
    font: &mut Glyphs,
    color: [f32; 4],
    font_size: u32,
    c: piston_window::Context,
    g: &mut G2d,
) -> () {
    text::Text::new_color(color, font_size)
        .draw(text, font, &c.draw_state, transform, g)
        .unwrap();
}

pub fn draw_score(score: Score, font: &mut Glyphs, c: piston_window::Context, g: &mut G2d) -> () {
    let text = &format!("Score: {}", score);

    // Draw shadow
    let transform = c.transform
        .trans(SHADOW_OFFSET + SCORE_OFFSET, SHADOW_OFFSET + SCORE_OFFSET);
    draw_text(text, transform, font, SHADOW_COLOR, SCORE_FONT_SIZE, c, g);

    // Draw score
    let transform = c.transform.trans(SCORE_OFFSET, SCORE_OFFSET);
    draw_text(text, transform, font, SCORE_COLOR, SCORE_FONT_SIZE, c, g);
}
