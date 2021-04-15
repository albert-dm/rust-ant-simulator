use std::path;

use ggez::graphics::{Color, Image};
use ggez::Context;
use ggez::*;

use glam::*;

const WIDTH: u16 = 1280;
const HEIGHT: u16 = 600;
const INICIAL_ANTS: usize = 1;

struct Ant {
    position: Vec2,
    velocity: Vec2,
}

impl Ant {
    fn new(inicial_position: Vec2) -> Ant {
        Ant {
            position: inicial_position,
            velocity: Vec2::new(5.0, 5.0),
        }
    }
}

struct MyGame {
    max_x: f32,
    max_y: f32,
    ants: Vec<Ant>,
    ant_texture: Image,
}

impl MyGame {
    fn new(ctx: &mut Context) -> ggez::GameResult<MyGame> {
        let ant_texture = Image::new(ctx, "/player.png")?;
        let mut ants = Vec::with_capacity(INICIAL_ANTS);
        let max_x = (WIDTH - ant_texture.width()) as f32;
        let max_y = (HEIGHT - ant_texture.height()) as f32;

        println!("Max_x: {}, width: {}", max_x, WIDTH);

        for _ in 0..INICIAL_ANTS {
            ants.push(Ant::new(Vec2::new(400.0, 400.0)));
        }

        Ok(MyGame {
            max_x,
            max_y,
            ants,
            ant_texture,
        })
    }
}

impl event::EventHandler for MyGame {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        for ant in &mut self.ants {
            ant.position += ant.velocity;
            if ant.position.x > self.max_x || ant.position.x < 0.0 {
                ant.velocity *= Vec2::new(-1.0, 1.0);
            }
            if ant.position.y > self.max_y || ant.position.y < 0.0 {
                ant.velocity *= Vec2::new(1.0, -1.0);
            }
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, Color::from((0.392, 0.584, 0.929)));
        for ant in &self.ants {
            graphics::draw(ctx, &self.ant_texture, (ant.position,))?;
        }

        graphics::set_window_title(ctx, "Ant simulator");
        graphics::present(ctx)?;

        Ok(())
    }
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
