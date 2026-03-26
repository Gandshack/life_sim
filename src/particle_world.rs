use log::*;

use rand::random_range;

use raylib::{
    math::Vector2,
    prelude::{RaylibDraw, RaylibDrawHandle},
};

use crate::{particle::Particle, random_color::ColorPicker};

pub struct ParticleWorld {
    pub particle_count_per_type: u32,
    pub particle_types: u32,
    pub particles: Vec<Particle>,
}

impl ParticleWorld {
    pub fn new(particle_count: u32, particle_types: u32) -> Self {
        Self {
            particle_count_per_type: particle_count,
            particle_types,
            particles: Vec::new(),
        }
    }

    pub fn spawn_particles(&mut self, width: i32, height: i32) {
        let mut color_picker = ColorPicker::new();
        for _particle_type in 0..self.particle_types {
            let type_color = color_picker.random_color();
            for _particle in 0..self.particle_count_per_type {
                info!("SPAWNING");
                let position_x = random_range(0..width);
                let position_y = random_range(0..height);
                let position = Vector2::new(position_x as f32, position_y as f32);

                self.particles.push(Particle {
                    particle_type_id: 0,
                    position: position,
                    color: type_color,
                });
            }
        }
    }

    pub fn draw(&mut self, d: &mut RaylibDrawHandle<'_>) {
        for particle in &self.particles {
            d.draw_circle(
                particle.position.x as i32,
                particle.position.y as i32,
                1.0,
                particle.color,
            );
        }
    }
}
