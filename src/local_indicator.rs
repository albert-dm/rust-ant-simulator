use glam::*;

const INDICATOR_LIFESPAN: u16 = 500;
pub struct LocalIndicator {
  position: Vec2,
  direction: f32,
  lifespan: u16,
  creation_tic: u16,
}

impl LocalIndicator {
  pub fn new(position: Vec2, direction: f32, creation_tic: u16) -> LocalIndicator {
    LocalIndicator{position, direction, lifespan: INDICATOR_LIFESPAN, creation_tic}
  }

  pub fn get_position(&mut self) -> Vec2 {
    self.position
  }

  pub fn get_direction(&mut self) -> f32 {
    self.direction
  }

  pub fn tick(&mut self) {
    self.lifespan += 1;
  }

  pub fn get_lifespan(&mut self) -> u16 {
    self.lifespan
  }

  pub fn set_lifespan(&mut self, lifespan: u16) {
    self.lifespan = lifespan;
  }

  pub fn get_creation_tic(&mut self) -> u16 {
    self.creation_tic
  }
}

impl Clone for LocalIndicator {
  fn clone(&self) -> Self {
    let mut new_indicator = LocalIndicator::new(self.position.clone(), self.direction.clone(), self.creation_tic.clone());
    new_indicator.set_lifespan(self.lifespan.clone());
    new_indicator
  }
}

impl Copy for LocalIndicator {
}