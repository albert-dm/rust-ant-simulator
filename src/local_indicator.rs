use glam::*;

pub struct LocalIndicator {
  direction: Vec2
}

impl LocalIndicator {
  pub fn new(direction: Vec2) -> LocalIndicator {
    LocalIndicator{direction}
  }

  pub fn get_direction(&mut self) -> Vec2 {
    self.direction
  }
}