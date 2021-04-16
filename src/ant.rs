use ggez::graphics;
use glam::*;

#[derive(Eq, PartialEq, Clone, Copy)]
pub enum AntMode {
  FindFood = 1,
  StoreFood,
}

pub struct Ant {
  position: Vec2,
  velocity: Vec2,
  mode: AntMode,
}

impl Ant {
  pub fn new(inicial_position: Vec2, inicial_velocity: Vec2) -> Ant {
    Ant {
      position: inicial_position,
      velocity: inicial_velocity,
      mode: AntMode::FindFood
    }
  }

  pub fn get_position(&mut self) -> Vec2 {
    self.position
  }

  pub fn get_mode(&mut self) -> AntMode {
    self.mode
  }

  pub fn set_mode(&mut self, mode: AntMode) {
    self.mode = mode;
  }

  fn get_rotation(&mut self) -> f32 {
    // println!("velocidade: x - {}, y -{}, norma- {}", self.velocity.x, self.velocity.y, self.velocity.length());
    let cos = self.velocity.x / self.velocity.length();
    let rotation = if self.velocity.y > 0.0 { cos.acos() } else { -cos.acos() };
    rotation
  }

  pub fn get_draw_params(&mut self) -> graphics::DrawParam {
    let drawparams = graphics::DrawParam::new()
      .dest(self.position)
      .rotation(self.get_rotation());
    drawparams
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
}
