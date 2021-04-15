use glam::*;


pub struct Anthill {
  position: Vec2,
}

impl Anthill {
  pub fn new(position: Vec2) -> Anthill {
    Anthill {
      position: position,
    }
  }

  pub fn get_position(&mut self) -> Vec2 {
    let position = self.position;
    position
  }
}

