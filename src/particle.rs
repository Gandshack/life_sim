use raylib::{color::Color, math::Vector2};

pub struct Particle {
    pub particle_type_id: u32,
    pub position: Vector2,
    pub color: Color,
}
