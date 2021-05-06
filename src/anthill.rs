use glam::*;

const HILL_WIDTH: f32 = 150.0;
const HILL_HEIGHT: f32 = 150.0;

pub struct Anthill {
  position: Vec2,
}

impl Anthill {
  pub fn new(position: Vec2) -> Anthill {
    Anthill {
      position: position,
    }
  }

  pub fn is_inside(&mut self, position: Vec2) -> bool {
    position.x > self.position.x && position.y > self.position.y && position.x < (self.position.x + HILL_WIDTH) && position.y < self.position.y + HILL_HEIGHT
  }

  pub fn get_position(&mut self) -> Vec2 {
    let position = self.position;
    position
  }
}

