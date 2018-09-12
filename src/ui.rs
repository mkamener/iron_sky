extern crate piston_window;

use game::*;
use piston_window::*;
use player;
use settings::ui::*;
use tween::*;

#[derive(Copy, Clone, PartialEq)]
enum State {
    GameActive,
    GameOver,
}

pub struct UI {
    state: State,
    game_over_tween: Tween,
}

impl UI {
    pub fn new() -> Self {
        UI {
            state: State::GameActive,
            game_over_tween: Tween::new(
                vec![(0.0, 0.0), (1.0, 1.0)],
                game_over::FADE_IN_LENGTH,
                false,
                false,
            ),
        }
    }

    pub fn update(&mut self, player: &player::Player, dt: f64) -> () {
        match player.state {
            player::State::Active(_) => self.go_to_game_active(),
            player::State::Exploding => self.go_to_game_active(),
            player::State::Inactive => {
                self.go_to_game_over();
                self.game_over_tween.update(dt);
            }
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
                draw_game_over_text(font, self.game_over_tween.get_val(), c, g);
                draw_game_over_score(score, font, self.game_over_tween.get_val(), c, g);
                draw_restart_text(font, self.game_over_tween.get_val(), c, g);
            }
        }
    }

    fn go_to_game_over(&mut self) -> () {
        match self.state {
            State::GameOver => (),
            State::GameActive => {
                self.game_over_tween.reset();
                self.state = State::GameOver;
            }
        }
    }

    fn go_to_game_active(&mut self) -> () {
        match self.state {
            State::GameActive => (),
            State::GameOver => {
                self.game_over_tween.stop();
                self.state = State::GameActive;
            }
        }
    }
}

fn draw_text(
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

fn draw_active_score(
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

fn draw_game_over_score(
    score: Score,
    font: &mut Glyphs,
    opacity: f64,
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
    draw_text(
        text,
        transform,
        font,
        set_opacity(SHADOW_COLOR, opacity as f32),
        SCORE_FONT_SIZE,
        c,
        g,
    );

    // Draw score
    let transform = c.transform.trans(SCORE_H_OFFSET, SCORE_V_OFFSET);
    draw_text(
        text,
        transform,
        font,
        set_opacity(SCORE_COLOR, opacity as f32),
        SCORE_FONT_SIZE,
        c,
        g,
    );
}

fn draw_game_over_text(
    font: &mut Glyphs,
    opacity: f64,
    c: piston_window::Context,
    g: &mut G2d,
) -> () {
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
        set_opacity(SHADOW_COLOR, opacity as f32),
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
        set_opacity(GAME_OVER_COLOR, opacity as f32),
        GAME_OVER_FONT_SIZE,
        c,
        g,
    );
}

fn draw_restart_text(
    font: &mut Glyphs,
    opacity: f64,
    c: piston_window::Context,
    g: &mut G2d,
) -> () {
    use settings::ui::game_over::*;

    let text = "Press SPACE to play again";

    // Draw shadow
    let transform = c.transform.trans(
        SHADOW_OFFSET + RESTART_H_OFFSET,
        SHADOW_OFFSET + RESTART_V_OFFSET,
    );
    draw_text(
        text,
        transform,
        font,
        set_opacity(SHADOW_COLOR, opacity as f32),
        RESTART_FONT_SIZE,
        c,
        g,
    );

    // Draw game over
    let transform = c.transform.trans(RESTART_H_OFFSET, RESTART_V_OFFSET);
    draw_text(
        text,
        transform,
        font,
        set_opacity(RESTART_COLOR, opacity as f32),
        RESTART_FONT_SIZE,
        c,
        g,
    );
}

fn set_opacity(color: [f32; 4], opacity: f32) -> [f32; 4] {
    let mut new_color = color;
    new_color[3] = opacity as f32;
    new_color
}
