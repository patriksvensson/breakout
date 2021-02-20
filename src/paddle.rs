use std::time::Duration;

use ggez::{
    event::KeyCode,
    graphics::{Color, DrawMode, DrawParam, Drawable, FillOptions, Rect},
    Context,
};

use crate::math::Vector2;

const PADDLE_WIDTH: f32 = 100.0;
const PADDLE_HEIGHT: f32 = 10.0;
const PADDLE_SPEED: f32 = 350.0;

pub struct Paddle {
    pub position: Vector2,
}

impl Paddle {
    pub fn new(ctx: &Context) -> Self {
        let (_, h) = ggez::graphics::drawable_size(ctx);
        let position = Vector2::new(100.0, h - PADDLE_HEIGHT - 20.0);

        Self { position }
    }

    pub fn get_bounds(&self) -> Rect {
        Rect::new(
            self.position.x,
            self.position.y,
            PADDLE_WIDTH,
            PADDLE_HEIGHT,
        )
    }

    pub fn update(&mut self, ctx: &mut ggez::Context, elapsed: &Duration) -> ggez::GameResult {
        if ggez::input::keyboard::is_key_pressed(ctx, KeyCode::Left) {
            // Move left
            self.position.x -= PADDLE_SPEED * elapsed.as_secs_f32();
        } else if ggez::input::keyboard::is_key_pressed(ctx, KeyCode::Right) {
            // Move right
            self.position.x += PADDLE_SPEED * elapsed.as_secs_f32();
        }

        if self.position.x < 0.0 {
            self.position.x = 0.0;
        }

        let (window_width, _) = ggez::graphics::drawable_size(ctx);
        if self.position.x > window_width - PADDLE_WIDTH {
            self.position.x = window_width - PADDLE_WIDTH;
        }

        Ok(())
    }

    pub fn draw(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        let paddle = ggez::graphics::Mesh::new_rectangle(
            ctx,
            DrawMode::Fill(FillOptions::DEFAULT),
            Rect::new(
                self.position.x,
                self.position.y,
                PADDLE_WIDTH,
                PADDLE_HEIGHT,
            ),
            Color::from_rgb(255, 255, 255),
        )?;

        paddle.draw(ctx, DrawParam::default())?;

        Ok(())
    }
}
