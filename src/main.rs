use macroquad::prelude::*;
use sim::{Particle, System};

mod sim;

fn window_conf() -> Conf {
    Conf {
        window_title: "nbody".to_owned(),
        window_height: 500,
        window_width: 500,
        platform: miniquad::conf::Platform {
            linux_backend: miniquad::conf::LinuxBackend::WaylandOnly,
            ..Default::default()
        },
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut system = System::new(vec![
        // mass of earth
        //Particle::new(vec![-6.3781e6], vec![0.0], 5.9722e24),
        // particle at surface
        //Particle::new(vec![700.0], vec![0.0], 10.0),
        Particle::new(vec![200.0], vec![0.0], 1.0e15),
        Particle::new(vec![300.0], vec![0.0], 1.0e15),
    ]);
    loop {
        clear_background(BLACK);
        system.tick(get_frame_time() as f64);
        draw_fps();
        for particle in &system.particles {
            let x = (particle.r[0] * system.zoom).round();
            if x > 0.0 && x < screen_width().into() {
                draw_circle(x as f32, screen_height() / 2.0, 15.0, WHITE);
            }
        }
        next_frame().await;
    }
}
