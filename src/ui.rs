extern crate piston_window;

use game::*;
use piston_window::*;
use player;
use settings::ui::*;

#[derive(Copy, Clone, PartialEq)]
enum State {
    GameActive,
    GameOver,
}

pub struct UI {
    state: State,
}

impl UI {
    pub fn new() -> Self {
        UI {
            state: State::GameActive,
        }
    }

    pub fn update(&mut self, player: &player::Player) -> () {
        match player.state {
            player::State::Active(_) => self.state = State::GameActive,
            player::State::Exploding => self.state = State::GameActive,
            player::State::Inactive => self.state = State::GameOver,
        }
    }

    pub fn draw(
        &self,
        score: Score,
        font: &mut Glyphs,
        c: piston_window::Context,
        g: &mut G2d,
    ) -> () {
        match self.state {
            State::GameActive => {
                draw_active_score(score, font, c, g);
            }
            State::GameOver => {
                draw_game_over_text(font, c, g);
                draw_game_over_score(score, font, c, g);
            }
        }
    }
}

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
        .round()
        .draw(text, font, &c.draw_state, transform, g)
        .unwrap();
}

pub fn draw_active_score(
    score: Score,
    font: &mut Glyphs,
    c: piston_window::Context,
    g: &mut G2d,
) -> () {
    use settings::ui::game_active::*;

    let text = &format!("Score: {}", score);

    // Draw shadow
    let transform = c.transform
        .trans(SHADOW_OFFSET + SCORE_OFFSET, SHADOW_OFFSET + SCORE_OFFSET);
    draw_text(text, transform, font, SHADOW_COLOR, SCORE_FONT_SIZE, c, g);

    // Draw score
    let transform = c.transform.trans(SCORE_OFFSET, SCORE_OFFSET);
    draw_text(text, transform, font, SCORE_COLOR, SCORE_FONT_SIZE, c, g);
}

pub fn draw_game_over_score(
    score: Score,
    font: &mut Glyphs,
    c: piston_window::Context,
    g: &mut G2d,
) -> () {
    use settings::ui::game_over::*;

    let text = &format!("Final Score: {}", score);

    // Draw shadow
    let transform = c.transform.trans(
        SHADOW_OFFSET + SCORE_H_OFFSET,
        SHADOW_OFFSET + SCORE_V_OFFSET,
    );
    draw_text(text, transform, font, SHADOW_COLOR, SCORE_FONT_SIZE, c, g);

    // Draw score
    let transform = c.transform.trans(SCORE_H_OFFSET, SCORE_V_OFFSET);
    draw_text(text, transform, font, SCORE_COLOR, SCORE_FONT_SIZE, c, g);
}

pub fn draw_game_over_text(font: &mut Glyphs, c: piston_window::Context, g: &mut G2d) -> () {
    use settings::ui::game_over::*;

    let text = "Game Over";

    // Draw shadow
    let transform = c.transform.trans(
        SHADOW_OFFSET + GAME_OVER_H_OFFSET,
        SHADOW_OFFSET + GAME_OVER_V_OFFSET,
    );
    draw_text(
        text,
        transform,
        font,
        SHADOW_COLOR,
        GAME_OVER_FONT_SIZE,
        c,
        g,
    );

    // Draw game over
    let transform = c.transform.trans(GAME_OVER_H_OFFSET, GAME_OVER_V_OFFSET);
    draw_text(
        text,
        transform,
        font,
        GAME_OVER_COLOR,
        GAME_OVER_FONT_SIZE,
        c,
        g,
    );
}
