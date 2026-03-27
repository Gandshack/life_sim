use raylib::{color::Color, math::Vector2};

#[derive(Debug)]
pub struct Particle {
    pub particle_type_id: u32,
    pub position: Vector2,
    pub color: Color,
    pub range_of_influence: u32,
}

impl Particle {
    pub fn new(particle_type_id: u32, position: Vector2, color: Color) -> Self {
        Self {
            particle_type_id,
            position,
            color,
            range_of_influence: 50,
        }
    }
}
