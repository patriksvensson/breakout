mod ball;
mod math;
mod paddle;

use ball::Ball;
use ggez::{self, Context, conf::{WindowMode, WindowSetup}, graphics::{self, Color, DrawMode, DrawParam, Rect}};

use graphics::{Drawable, StrokeOptions};
use math::Vector2;
use paddle::Paddle;

pub struct Brick {
    pub bounds: Rect,
    pub color: Color,
}

struct MainState {
    pub paddle: Paddle,
    pub ball: Ball,
    pub bricks: Vec<Brick>,
}

pub const BRICK_WIDTH : f32 = 80.0;
pub const BRICK_HEIGHT: f32 = 25.0;

impl MainState {
    fn new(ctx: &Context) -> ggez::GameResult<MainState> {
        let mut bricks = Vec::new();
        for x in 0..10 {
            for y in 0..10 {
                bricks.push(Brick {
                    bounds: Rect:: new(x as f32 * BRICK_WIDTH, y as f32 * BRICK_HEIGHT, BRICK_WIDTH, BRICK_HEIGHT),
                    color: ggez::graphics::Color::from_rgb(255, 0,0 ),
                })
            }
        }

        let s = MainState {
            paddle: Paddle::new(ctx),
            ball: Ball::new(ctx),
            bricks,
        };
        Ok(s)
    }
}

impl ggez::event::EventHandler for MainState {
    fn update(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        let elapsed = ggez::timer::delta(ctx);
        self.paddle.update(ctx, &elapsed)?;
        self.ball.update(ctx, &self.paddle, &elapsed)?;

        Ok(())
    }

    fn draw(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        ggez::graphics::clear(ctx, [0.0, 0.0, 0.0, 1.0].into());

        for brick in self.bricks.iter() {
            let rect = ggez::graphics::Mesh::new_rectangle(ctx, DrawMode::Stroke(StrokeOptions::DEFAULT), brick.bounds, brick.color)?;
            rect.draw(ctx, DrawParam::default())?;
        }

        self.paddle.draw(ctx)?;
        self.ball.draw(ctx)?;

        ggez::graphics::present(ctx)?;

        Ok(())
    }
}

pub fn main() -> ggez::GameResult {
    let cb = ggez::ContextBuilder::new("breakout", "Patrik Svensson")
        .window_setup(WindowSetup::default().title("Breakout").vsync(false))
        .window_mode(WindowMode::default());
    let (ctx, event_loop) = cb.build()?;
    let state = MainState::new(&ctx)?;
    ggez::event::run(ctx, event_loop, state)
}
