extern crate piston_window;

use game::*;
use piston_window::*;
use player::*;
use sprite::*;

pub struct Background(Vec<BGLayer>);

impl Background {
    pub fn new(
        window: &mut PistonWindow,
        folder: &::std::path::PathBuf,
        names_and_factors: &'static [(&str, f64)],
    ) -> Background {
        let mut all_bg: Vec<BGLayer> = vec![];

        for (file, factor) in names_and_factors.iter() {
            let bg = load_sprite(window, folder, file, 1.0);
            all_bg.push(BGLayer::new(bg, *factor));
        }

        Background(all_bg)
    }

    pub fn update(&mut self, player: &Player, dt: f64) -> () {
        let Background(ref mut backgrounds) = *self;
        for bg in backgrounds.iter_mut() {
            bg.update(player, dt);
        }
    }

    pub fn draw(&mut self, height: u32, width: u32, context: piston_window::Context, g: &mut G2d) {
        let Background(ref mut backgrounds) = *self;
        for bg in &mut backgrounds.iter_mut() {
            bg.draw(height, width, context, g);
        }
    }
}

struct BGLayer {
    sprite: Sprite<G2dTexture>,
    pos: Point,
    clamp: Point,
    factor: f64,
}

impl BGLayer {
    fn new(sprite: Sprite<G2dTexture>, factor: f64) -> BGLayer {
        let clamp = sprite.bounding_box();
        let clamp = Point::new(clamp[2], clamp[3]);
        BGLayer {
            sprite: sprite,
            pos: Point::new(0.0, 0.0),
            clamp: clamp,
            factor: factor,
        }
    }

    fn update(&mut self, player: &Player, dt: f64) {
        // Update position based off player movement
        self.pos = self.pos - player.velocity() * dt * self.factor;

        // Clamp position to bounding box
        let new_x = ((self.pos.x % self.clamp.x) + self.clamp.x) % self.clamp.x;
        let new_y = ((self.pos.y % self.clamp.y) + self.clamp.y) % self.clamp.y;
        self.pos = Point::new(new_x, new_y);
    }

    fn draw(
        &mut self,
        height: u32,
        width: u32,
        context: piston_window::Context,
        g: &mut G2d,
    ) -> () {
        let max_x = ((width as f64) / (self.clamp.x)) as i32 + 1;
        let max_y = ((height as f64) / (self.clamp.y)) as i32 + 1;

        for x in -1..=max_x {
            for y in -1..=max_y {
                let x_pos = self.pos.x + (x as f64) * self.clamp.x;
                let y_pos = self.pos.y + (y as f64) * self.clamp.y;

                self.sprite.set_position(x_pos, y_pos);
                self.sprite.draw(context.transform, g);
            }
        }
    }
}
