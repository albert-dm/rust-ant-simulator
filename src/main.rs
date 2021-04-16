mod ant;
mod anthill;
mod local_indicator;

use ggez::graphics::{Color, Image};
use ggez::Context;
use ggez::*;
use glam::*;
use rand::Rng;
use std::f32::consts::PI;
use std::path;

use ant::{AntMode, Ant};
use anthill::{Anthill};
use local_indicator::{LocalIndicator};


const WIDTH: u16 = 1280;
const HEIGHT: u16 = 600;
const INICIAL_ANTS: usize = 53;

struct MyGame {
    max_x: f32,
    max_y: f32,
    ants: Vec<Ant>,
    anthill: Anthill,
    ant_texture: Image,
    anthill_texture: Image,

    home_indicators: Vec<LocalIndicator>,
    food_indicators: Vec<LocalIndicator>,
}

impl MyGame {
    fn new(ctx: &mut Context) -> ggez::GameResult<MyGame> {
        let ant_texture = Image::new(ctx, "/ant.png")?;
        let anthill_texture = Image::new(ctx, "/anthill.png")?;
        let mut ants = Vec::with_capacity(INICIAL_ANTS);
        let home_indicators = Vec::new();
        let food_indicators = Vec::new();
        let max_x = (WIDTH - ant_texture.width()) as f32;
        let max_y = (HEIGHT - ant_texture.height()) as f32;
        let mut rng = rand::thread_rng();

        let inicial_position = Vec2::new(400.0, 400.0);
        let inicial_velocity_length = 3.0;

        let anthill = Anthill::new(Vec2::new(
            inicial_position.x - anthill_texture.width() as f32,
            inicial_position.y - ant_texture.height() as f32,
        ));
        for _ in 0..INICIAL_ANTS {
            let random_angle = rng.gen_range(0.0..(2.0 * PI));
            let random_direction_versor = Vec2::new(random_angle.sin(), random_angle.cos());
            let ant_velocity = inicial_velocity_length * random_direction_versor;
            ants.push(Ant::new(inicial_position, ant_velocity));
        }

        Ok(MyGame {
            max_x,
            max_y,
            ants,
            anthill,
            ant_texture,
            anthill_texture,
            home_indicators,
            food_indicators,
        })
    }
}

impl event::EventHandler for MyGame {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        for ant in &mut self.ants {
            // check if ant sees a LocalIndicator and change its direction
            // check if ant sees a Leaf, take it, change mode and invert direction
            // check if ant sees home, leave Leaf, change mode and invert direction
            ant.tick(self.max_x, self.max_y);
            let indicator = LocalIndicator::new(ant.get_position());
            let mode = ant.get_mode();
            if mode == AntMode::FindFood {
                self.food_indicators.push(indicator);
            } else if mode == AntMode::StoreFood {
                self.home_indicators.push(indicator);
            }
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(
            ctx,
            Color::from((183.0 / 255.0, 156.0 / 255.0, 115.0 / 255.0)),
        );
        graphics::draw(ctx, &self.anthill_texture, (self.anthill.get_position(),))?;
        for ant in &mut self.ants {
            graphics::draw(ctx, &self.ant_texture, ant.get_draw_params())?;
        }

        graphics::set_window_title(ctx, "Ant simulator");
        graphics::present(ctx)?;

        Ok(())
    }

    // events come here
}

fn main() -> GameResult {
    let resource_dir = path::PathBuf::from("./resources");

    let cb = ggez::ContextBuilder::new("ant_simulator", "ggez")
        .window_mode(conf::WindowMode::default().dimensions(WIDTH as f32, HEIGHT as f32))
        .add_resource_path(resource_dir);
    let (mut ctx, mut event_loop) = cb.build()?;

    let mut state = MyGame::new(&mut ctx)?;
    event::run(&mut ctx, &mut event_loop, &mut state)
}
