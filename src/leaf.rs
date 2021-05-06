use glam::*;


pub struct Leaf {
  position: Vec2,
}

impl Leaf {
  pub fn new(position: Vec2) -> Leaf {
    Leaf {
      position: position,
    }
  }

  pub fn get_position(&mut self) -> Vec2 {
    let position = self.position;
    position
  }
}

impl Clone for Leaf {
  fn clone(&self) -> Self {
    Leaf::new(self.position.clone())
  }
}

