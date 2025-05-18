use macroquad::prelude::*;
use nbody::sim::{Particle, System};

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
    let mut system = System::<2>::new(vec![
        // earth
        Particle::new(vec![-6.3781e6, 100.0], vec![0.0, 0.0], 5.9722e24),
        // particles at surface
        Particle::new(vec![200.0, 50.0], vec![0.0, 0.0], 10.0),
        Particle::new(vec![200.0, 100.0], vec![0.0, 10.0], 10.0),
        Particle::new(vec![200.0, 200.0], vec![0.0, 0.0], 10.0),
        //Particle::new(vec![400.0], vec![0.0], 1.0e10),
        //Particle::new(vec![800.0], vec![0.0], 1.0e10),
    ]);
    loop {
        clear_background(BLACK);
        let frame_time = get_frame_time();
        for _ in 0..1000 {
            system.tick(frame_time as f64 / 1000.0);
        }
        draw_fps();
        for particle in &system.particles {
            let x = (particle.r.x * system.zoom) as f32;
            let y = if particle.r.len() > 1 {
                (particle.r.y * system.zoom) as f32
            } else {
                screen_height() / 2.0
            };
            if x > 0.0 && x < screen_width() && y > 0.0 && y < screen_height() {
                draw_circle(x, y, 5.0, WHITE);
            }
        }
        println!("{:?}", system.particles);
        next_frame().await;
    }
}
