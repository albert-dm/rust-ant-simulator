use crate::local_indicator;
use crate::vector_utils;
use crate::leaf;

use ggez::graphics;
use glam::*;
use std::collections::HashMap;
use std::f32::consts::PI;
use rand::Rng;

use local_indicator::LocalIndicator;
use vector_utils::get_rotation;
use leaf::Leaf;

const ANT_VIEW_RANGE_MAX: f32 = 80.0;
const ANT_VIEW_RANGE_MIN: f32 = 10.0;


#[derive(Eq, PartialEq, Clone, Copy)]
pub enum AntMode {
  FindFood = 1,
  StoreFood,
}

pub struct Ant {
  position: Vec2,
  velocity: Vec2,
  width: f32,
  height: f32,
  mode: AntMode,
}

impl Ant {
  pub fn new(inicial_position: Vec2, inicial_velocity: Vec2, width: f32, height: f32) -> Ant {
    Ant {
      position: inicial_position,
      velocity: inicial_velocity,
      width: width,
      height: height,
      mode: AntMode::FindFood,
    }
  }

  pub fn get_position(&mut self) -> Vec2 {
    self.position
  }
  pub fn set_position(&mut self, position: Vec2) {
    self.position = position;
  }

  pub fn get_mode(&mut self) -> AntMode {
    self.mode
  }

  pub fn set_mode(&mut self, mode: AntMode) {
    self.mode = mode;
  }

  pub fn set_rotation(&mut self, rotation: f32) {
    let velocity = self.velocity.length();
    self.velocity = Vec2::new(velocity * rotation.cos(), velocity * rotation.sin());
  }

  pub fn get_rotation(&mut self) -> f32 {
    get_rotation(self.velocity)
  }

  pub fn get_draw_params(&mut self) -> graphics::DrawParam {
    let drawparams = graphics::DrawParam::new()
      .dest(self.position)
      .rotation(get_rotation(self.velocity));
      // .offset(Vec2::new(self.width / 2.0, self.height / 2.0));
    drawparams
  }

  pub fn find_indicator(&mut self, mut indicators: Vec<LocalIndicator>) -> Option<LocalIndicator> {
    let mut found_distance = ANT_VIEW_RANGE_MAX;
    let mut found_indicator: Option<LocalIndicator> = None;
    for idx in 0..indicators.len() {
      let indicator_position = indicators[idx].get_position();

      let visibility_info = self.get_visibility_info(indicator_position);

      if visibility_info.is_some() {
        let distance = visibility_info.unwrap();
        if distance < found_distance {
          found_indicator = Some(indicators[idx].clone());
          found_distance = distance;
        }
      }
    }

    found_indicator
  }

  pub fn find_food(&mut self, food: HashMap<u16, Leaf>) -> Option<u16> {
    let mut found_distance = ANT_VIEW_RANGE_MAX;
    let mut found_food_key: Option<u16> = None;
    for (key, leaf) in food {
      let indicator_position = leaf.clone().get_position();

      let visibility_info = self.get_visibility_info(indicator_position);
      if visibility_info.is_some() {
        let distance = visibility_info.unwrap();
        if distance < found_distance {
          found_food_key = Some(key);
          found_distance = distance;
        }
      }
    }
    found_food_key
  }

  pub fn tick(&mut self, max_x: f32, max_y: f32) {
    self.position += self.velocity;
    if self.position.x > max_x || self.position.x < 0.0 {
      self.velocity *= Vec2::new(-1.0, 1.0);
    }
    if self.position.y > max_y || self.position.y < 0.0 {
      self.velocity *= Vec2::new(1.0, -1.0);
    }
  }

  fn get_visibility_info(&mut self, position: Vec2) -> Option<f32> {
    let ant_rotation = self.get_rotation();
    let distance_vector = self.get_position() - position;
    let distance = distance_vector.length();
    let distance_angle = get_rotation(distance_vector);
    if distance < ANT_VIEW_RANGE_MAX && distance > ANT_VIEW_RANGE_MIN && distance_angle < ant_rotation + 0.7 && distance_angle > ant_rotation - 0.7 {
        Some(distance)
    } else {
      None
    }
  }

  pub fn invert_direction(&mut self) {
    let current_rotation = self.get_rotation();
    self.set_rotation(current_rotation + PI);
  }
}
