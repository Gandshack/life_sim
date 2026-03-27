use std::{any::Any, ops::Index};

use log::*;

use rand::random_range;

use raylib::{
    math::Vector2,
    prelude::{RaylibDraw, RaylibDrawHandle},
};

use crate::{
    particle::{self, Particle},
    random_color::ColorPicker,
};

pub struct ParticleWorld {
    pub width: i32,
    pub height: i32,
    pub particle_count_per_type: u32,
    pub particle_types: u32,
    pub particles: Vec<Particle>,
    pub force_matrix: Vec<Vec<f32>>,
}

impl ParticleWorld {
    pub fn new(particle_count: u32, particle_types: u32, width: i32, height: i32) -> Self {
        Self {
            width,
            height,
            particle_count_per_type: particle_count,
            particle_types,
            particles: Vec::new(),
            force_matrix: Vec::new(),
        }
    }

    pub fn spawn_particles(&mut self, width: i32, height: i32) {
        let mut color_picker = ColorPicker::new();

        for particle_type in 0..self.particle_types {
            let type_color = color_picker.random_color();
            let type_id = particle_type;

            for _particle in 0..self.particle_count_per_type {
                let position_x = random_range(0..width);
                let position_y = random_range(0..height);
                let position = Vector2::new(position_x as f32, position_y as f32);

                // debug!(
                //     "Spawning particle w/ type_id: {}, position: {:?}, color: {:?}",
                //     type_id, position, type_color
                // );
                self.particles
                    .push(Particle::new(type_id, position, type_color));
            }
        }
    }

    pub fn build_force_matrix(&mut self) {
        self.force_matrix
            .resize(self.particle_types as usize, Vec::new());

        for i in 0..self.force_matrix.len() {
            let mut vec_col = Vec::new(); // Random -1..1
            vec_col.resize(self.particle_types as usize, 0.0);

            for i in 0..vec_col.len() {
                vec_col[i] = random_range(-1.0..1.0);
            }
            self.force_matrix[i] = vec_col;
        }
        debug!("{:?}", self.force_matrix);
    }

    pub fn draw(&mut self, d: &mut RaylibDrawHandle<'_>) {
        for particle in &self.particles {
            d.draw_circle(
                particle.position.x as i32,
                particle.position.y as i32,
                3.0,
                particle.color,
            );
        }
    }

    pub fn apply_forces(&mut self) {
        let friction = 0.5;
        let mut velocities = Vec::<Vector2>::new();
        velocities.resize(self.particles.len(), Vector2::zero());
        for (i, partice_a) in self.particles.iter().enumerate() {
            for particle_b in &self.particles {
                let distance = partice_a.position.distance_to(particle_b.position);
                if distance <= partice_a.range_of_influence as f32 {
                    let vec = particle_b.position - partice_a.position;
                    let force = self.force_matrix[partice_a.particle_type_id as usize]
                        [particle_b.particle_type_id as usize];
                    let vel = vec * force;
                    velocities[i] += vel.normalized();
                }
            }
        }

        for (particle, velocity) in self.particles.iter_mut().zip(velocities.iter()) {
            let mut velocity = *velocity;
            if particle.position.x >= self.width as f32 || particle.position.x <= 0.0 {
                velocity = velocity * -1.0 as f32;
            }

            if particle.position.y >= self.height as f32 || particle.position.y <= 0.0 {
                velocity = velocity * -1.0 as f32;
            }

            particle.position += velocity * friction;
        }
    }
}
