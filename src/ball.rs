use std::time::Duration;

use ggez::{
    graphics::{Color, DrawMode, DrawParam, Drawable, FillOptions, Rect},
    Context,
};

use crate::{math::Vector2, paddle::Paddle};

const BALL_RADIUS: f32 = 10.0;
const BALL_SPEED: f32 = 250.0;

pub struct Ball {
    pub position: Vector2,
    pub velocity: Vector2,
}

impl Ball {
    pub fn new(_ctx: &Context) -> Self {
        Self {
            position: Vector2::new(0.0, 0.0),
            velocity: Vector2::new(1.0, 1.0),
        }
    }

    pub fn get_bounds(&self) -> Rect {
        Rect::new(
            self.position.x - BALL_RADIUS,
            self.position.y - BALL_RADIUS,
            BALL_RADIUS * 2.0,
            BALL_RADIUS * 2.0,
        )
    }

    pub fn update(
        &mut self,
        ctx: &mut Context,
        paddle: &Paddle,
        elapsed: &Duration,
    ) -> ggez::GameResult {
        self.position += (self.velocity * BALL_SPEED) * elapsed.as_secs_f32();

        let (window_width, window_height) = ggez::graphics::drawable_size(ctx);

        // Bounce?
        if self.position.x <= BALL_RADIUS || self.position.x + (BALL_RADIUS) >= window_width {
            self.velocity.x = -self.velocity.x;
        }
        if self.position.y <= BALL_RADIUS {
            self.velocity.y = -self.velocity.y;
        }

        // Does the ball hit the paddle?
        if intersects(&paddle.get_bounds(), &self.get_bounds()) {
            self.velocity.y = -self.velocity.y;
        }

        // Did we miss the paddle?
        if self.position.y + BALL_RADIUS >= window_height {
            // BAD!
        }

        Ok(())
    }

    pub fn draw(&mut self, ctx: &mut Context) -> ggez::GameResult {
        let circle = ggez::graphics::Mesh::new_circle(
            ctx,
            DrawMode::Fill(FillOptions::DEFAULT),
            self.position,
            BALL_RADIUS,
            0.1,
            Color::from_rgb(255, 255, 0),
        )?;

        circle.draw(ctx, DrawParam::default())?;

        Ok(())
    }
}

pub fn intersects(first: &Rect, second: &Rect) -> bool {
    first.contains(Vector2::new(second.left(), second.top()))
        || first.contains(Vector2::new(second.right(), second.top()))
        || first.contains(Vector2::new(second.right(), second.bottom()))
        || first.contains(Vector2::new(second.left(), second.bottom()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn should_contain_rectangle_1() {
        // Given
        let first = Rect::new(0.0, 0.0, 100.0, 100.0);
        let second = Rect::new(50.0, 50.0, 25.0, 25.0);
        // When
        let result = intersects(&first, &second);
        // Then
        assert!(result);
    }

    #[test]
    pub fn should_contain_rectangle_2() {
        // Given
        let first = Rect::new(50.0, 0.0, 100.0, 100.0);
        let second = Rect::new(0.0, 10.0, 100.0, 100.0);
        // When
        let result = intersects(&first, &second);
        // Then
        assert!(result);
    }

    #[test]
    pub fn should_contain_rectangle_3() {
        // Given
        let first = Rect::new(50.0, 0.0, 100.0, 100.0);
        let second = Rect::new(0.0, -10.0, 100.0, 80.0);
        // When
        let result = intersects(&first, &second);
        // Then
        assert!(result);
    }
}
