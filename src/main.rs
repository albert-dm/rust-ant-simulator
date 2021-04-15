use std::path;

use ggez::graphics::{Color, Image};
use ggez::Context;
use ggez::*;

use glam::*;

mod ant;
mod anthill;

const WIDTH: u16 = 1280;
const HEIGHT: u16 = 600;
const INICIAL_ANTS: usize = 1;


struct MyGame {
    max_x: f32,
    max_y: f32,
    ants: Vec<ant::Ant>,
    anthill: anthill::Anthill,
    ant_texture: Image,
    anthill_texture: Image,
}

impl MyGame {
    fn new(ctx: &mut Context) -> ggez::GameResult<MyGame> {
        let ant_texture = Image::new(ctx, "/ant.png")?;
        let anthill_texture = Image::new(ctx, "/anthill.png")?;
        let mut ants = Vec::with_capacity(INICIAL_ANTS);
        let max_x = (WIDTH - ant_texture.width()) as f32;
        let max_y = (HEIGHT - ant_texture.height()) as f32;

        let inicial_position = Vec2::new(400.0, 400.0);

        let anthill = anthill::Anthill::new(Vec2::new(inicial_position.x - anthill_texture.width() as f32, inicial_position.y - ant_texture.height() as f32));

        for _ in 0..INICIAL_ANTS {
            ants.push(ant::Ant::new(inicial_position));
        }

        Ok(MyGame {
            max_x,
            max_y,
            ants,
            anthill,
            ant_texture,
            anthill_texture
        })
    }
}

impl event::EventHandler for MyGame {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {

        for ant in &mut self.ants {
            ant.tick(self.max_x, self.max_y);
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, Color::from((183.0/255.0, 156.0/255.0, 115.0/255.0)));
        graphics::draw(ctx, &self.anthill_texture, (self.anthill.get_position(),))?;
        for ant in &mut self.ants {
            graphics::draw(ctx, &self.ant_texture, (ant.get_position(),))?;
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
