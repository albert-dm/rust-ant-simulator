use glam::*;


pub struct Ant {
  position: Vec2,
  velocity: Vec2,
}

impl Ant {
  pub fn new(inicial_position: Vec2) -> Ant {
    Ant {
      position: inicial_position,
      velocity: Vec2::new(5.0, 5.0),
    }
  }

  pub fn get_position(&mut self) -> Vec2 {
    let position = self.position;
    position
  }
  
  pub fn tick(&mut self, max_x: f32, max_y: f32 ){
    self.position += self.velocity;
    if self.position.x > max_x || self.position.x < 0.0 {
      self.velocity *= Vec2::new(-1.0, 1.0);
    }
    if self.position.y > max_y || self.position.y < 0.0 {
      self.velocity *= Vec2::new(1.0, -1.0);
    }
  }
}

