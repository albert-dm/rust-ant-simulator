use glam::*;

pub fn get_rotation(vector: Vec2) -> f32 {
  let cos = vector.x / vector.length();
  let rotation = if vector.y > 0.0 {
    cos.acos()
  } else {
    -cos.acos()
  };
  rotation
}