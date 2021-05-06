mod ant;
mod anthill;
mod leaf;
mod local_indicator;
mod vector_utils;

use ggez::graphics::{spritebatch::SpriteBatch, Color, Image};
use ggez::Context;
use ggez::*;
use glam::*;
use rand::Rng;
use std::collections::HashMap;
use std::f32::consts::PI;
use std::path;
use vector_utils::get_rotation;

use ant::{Ant, AntMode};
use anthill::Anthill;
use leaf::Leaf;
use local_indicator::LocalIndicator;

const WIDTH: u16 = 2400;
const HEIGHT: u16 = 1800;
const INICIAL_ANTS: usize = 500;
const INICIAL_FOOD: usize = 2000;
const INICIAL_ANT_VELOCITY: f32 = 10.0;

struct MyGame {
    max_x: f32,
    max_y: f32,
    ants: Vec<Ant>,
    anthill: Anthill,
    ant_texture: Image,
    anthill_texture: Image,
    leaf_texture: Image,

    ant_texture_batch: SpriteBatch,
    leaf_texture_batch: SpriteBatch,

    home_indicators: Vec<LocalIndicator>,
    food_indicators: Vec<LocalIndicator>,
    food: HashMap<u16, Leaf>,

    current_tic: u16,
}

impl MyGame {
    fn new(ctx: &mut Context) -> ggez::GameResult<MyGame> {
        let ant_texture = Image::new(ctx, "/ant.png")?;
        let anthill_texture = Image::new(ctx, "/anthill.png")?;
        let leaf_texture = Image::new(ctx, "/leaf.png")?;

        let ant_texture_batch = SpriteBatch::new(ant_texture.clone());
        let leaf_texture_batch = SpriteBatch::new(leaf_texture.clone());

        let mut ants = Vec::with_capacity(INICIAL_ANTS);
        let mut home_indicators = Vec::new();
        let mut food_indicators = Vec::new();
        let mut food = HashMap::new();
        let max_x = (WIDTH - ant_texture.width()) as f32;
        let max_y = (HEIGHT - ant_texture.height()) as f32;
        let mut rng = rand::thread_rng();

        let inicial_position = Vec2::new(700.0, 400.0);

        let anthill = Anthill::new(Vec2::new(
            inicial_position.x - anthill_texture.width() as f32,
            inicial_position.y - ant_texture.height() as f32,
        ));
        for _ in 0..INICIAL_ANTS {
            let random_angle = rng.gen_range(0.0..(2.0 * PI));
            let random_direction_versor = Vec2::new(random_angle.sin(), random_angle.cos());
            let ant_velocity = INICIAL_ANT_VELOCITY * random_direction_versor;
            ants.push(Ant::new(
                inicial_position,
                ant_velocity,
                ant_texture.width() as f32,
                ant_texture.height() as f32,
            ));
        }

        for food_idx in 0..INICIAL_FOOD {
            let random_x = rng.gen_range(2200.0..2300.0);
            let random_y = rng.gen_range(50.0..150.0);
            let food_position = Vec2::new(random_x, random_y);
            food.insert(food_idx as u16, Leaf::new(food_position));
        }

        for food_idx in INICIAL_FOOD..2 * INICIAL_FOOD {
            let random_x = rng.gen_range(50.0..250.0);
            let random_y = rng.gen_range(800.0..900.0);
            let food_position = Vec2::new(random_x, random_y);
            food.insert(food_idx as u16, Leaf::new(food_position));
        }

        for food_idx in 2 * INICIAL_FOOD..3 * INICIAL_FOOD {
            let random_x = rng.gen_range(1550.0..1702.0);
            let random_y = rng.gen_range(1500.0..1600.0);
            let food_position = Vec2::new(random_x, random_y);
            food.insert(food_idx as u16, Leaf::new(food_position));
        }

        Ok(MyGame {
            max_x,
            max_y,
            ants,
            anthill,
            ant_texture,
            anthill_texture,
            leaf_texture,

            ant_texture_batch,
            leaf_texture_batch,

            home_indicators,
            food_indicators,
            food,
            current_tic: 1,
        })
    }
}

impl event::EventHandler for MyGame {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        for ant in &mut self.ants {
            let mode = ant.get_mode();
            let mut food_indicators = self.food_indicators.clone();
            let mut home_indicators = self.home_indicators.clone();

            let found_food_indicator: Option<LocalIndicator> = ant.find_indicator(food_indicators);
            let found_home_indicator: Option<LocalIndicator> = ant.find_indicator(home_indicators);

            if mode == AntMode::FindFood {
                let found_food: Option<u16> = ant.find_food(self.food.clone());

                if found_food.is_some() {
                    let found_index = found_food.unwrap();
                    self.food.remove(&found_index);

                    ant.set_mode(AntMode::StoreFood);

                    ant.invert_direction();
                } else if found_food_indicator.is_some() {
                    ant.set_rotation(found_food_indicator.unwrap().get_direction());
                    // ant.set_position(found_food_indicator.unwrap().get_position());
                }

                // println!("{}",self.home_indicators.len());
                if !found_home_indicator.is_some() {
                    let home_vec = self.anthill.get_position() - ant.get_position();
                    let indicator_direction = get_rotation(home_vec);
                    self.home_indicators.push(LocalIndicator::new(
                        ant.get_position(),
                        indicator_direction,
                        self.current_tic,
                    ))
                }
            } else if mode == AntMode::StoreFood {
                if self.anthill.is_inside(ant.get_position()) {
                    ant.set_mode(AntMode::FindFood);

                    ant.invert_direction();
                } else if found_home_indicator.is_some() {
                    ant.set_rotation(found_home_indicator.unwrap().get_direction());
                    // ant.set_position(found_home_indicator.unwrap().get_position());
                }

                if !found_food_indicator.is_some() {
                    self.food_indicators.push(LocalIndicator::new(
                        ant.get_position(),
                        ant.get_rotation() + PI,
                        self.current_tic,
                    ))
                }
            }

            let mut rng = rand::thread_rng();
            let current_rotation = ant.get_rotation();
            let adjust_angle = rng.gen_range(-(PI / 15.0)..(PI / 15.0));
            ant.set_rotation(current_rotation + adjust_angle);

            ant.tick(self.max_x, self.max_y);
        }

        self.current_tic += 1;

        self.food_indicators = self
            .food_indicators
            .clone()
            .into_iter()
            .filter(|indicator| {
                self.current_tic - indicator.clone().get_creation_tic()
                    <= indicator.clone().get_lifespan()
            })
            .collect();

        self.home_indicators = self
            .home_indicators
            .clone()
            .into_iter()
            .filter(|indicator| {
                self.current_tic - indicator.clone().get_creation_tic()
                    <= indicator.clone().get_lifespan()
            })
            .collect();

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(
            ctx,
            Color::from((183.0 / 255.0, 156.0 / 255.0, 115.0 / 255.0)),
        );

        graphics::set_drawable_size(ctx, 800.0, 600.0);

        self.ant_texture_batch.clear();
        self.leaf_texture_batch.clear();

        graphics::draw(ctx, &self.anthill_texture, (self.anthill.get_position(),))?;

        for ant in &mut self.ants {
            self.ant_texture_batch.add(ant.get_draw_params());
        }
        graphics::draw(ctx, &self.ant_texture_batch, (Vec2::new(0.0, 0.0),))?;

        for (_idx, leaf) in &mut self.food {
            self.leaf_texture_batch.add((leaf.get_position(),));
        }
        graphics::draw(ctx, &self.leaf_texture_batch, (Vec2::new(0.0, 0.0),))?;

        // let food_circle = graphics::Mesh::new_circle(
        //     ctx,
        //     graphics::DrawMode::fill(),
        //     Vec2::new(0.0, 0.0),
        //     5.0,
        //     2.0,
        //     Color::from_rgb(100, 0, 0),
        // )?;

        // let home_circle = graphics::Mesh::new_circle(
        //     ctx,
        //     graphics::DrawMode::fill(),
        //     Vec2::new(0.0, 0.0),
        //     5.0,
        //     2.0,
        //     Color::from_rgb(0, 0, 100),
        // )?;

        // for home_indicators in &mut self.home_indicators {
        //     graphics::draw(ctx, &home_circle, (home_indicators.get_position(),))?;
        // }
        // for food_indicator in &mut self.food_indicators {
        //     graphics::draw(ctx, &food_circle, (food_indicator.get_position(),))?;
        // }

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
