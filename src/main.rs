mod particle;
mod particle_world;
mod random_color;
use raylib::prelude::*;

use crate::particle_world::ParticleWorld;

fn main() {
    env_logger::init();
    let (mut rl, thread) = raylib::init().size(1280, 720).resizable().build();

    let particle_count_per_type = 10_000;
    let particle_types = 3;
    let mut particle_world = ParticleWorld::new(particle_count_per_type, particle_types);

    let width = rl.get_screen_width();
    let height = rl.get_screen_height();
    particle_world.spawn_particles(width, height);

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);
        particle_world.draw(&mut d);
    }
}
