mod particle;
mod particle_world;
mod random_color;
use raylib::prelude::*;

use log::*;

use crate::particle_world::ParticleWorld;

fn main() {
    env_logger::init();
    let (mut rl, thread) = raylib::init().size(1280, 720).resizable().build();

    let particle_count_per_type = 200;
    let particle_types = 6;
    let mut particle_world = ParticleWorld::new(particle_count_per_type, particle_types);

    let width = rl.get_screen_width();
    let height = rl.get_screen_height();
    particle_world.spawn_particles(width, height);
    particle_world.build_force_matrix();
    for particle in &particle_world.particles {
        println!("{:?}", particle.position);
    }

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);
        particle_world.draw(&mut d);
        particle_world.apply_forces();
    }
}
