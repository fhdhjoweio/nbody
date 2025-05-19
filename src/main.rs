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
    let mut system = System::<2>::from_particles(vec![
        // Particle::new(vec![-6.3781e6, 100.0], vec![0.0, 0.0], 5.9722e24),
        Particle::new(vec![0.0, 0.0], vec![0.0, 0.0], 2.0e16),
        Particle::new(vec![0.0, 100.0], vec![50.0, -100.0], 10.0),
        Particle::new(vec![200.0, 0.0], vec![0.0, 81.70], 10.0),
        Particle::new(vec![0.0, 75.0], vec![133.41, 0.0], 10.0),
    ]);
    loop {
        clear_background(BLACK);
        let frame_time = get_frame_time();
        for _ in 0..100 * 10 {
            system.runge_kutta(frame_time as f64 / 100.0);
        }
        draw_fps();
        for i in 0..system.x.nrows() {
            let x = (system.x.row(i)[0] * system.zoom) as f32;
            let y = if system.x.ncols() >= 2 {
                (system.x.row(i)[1] * system.zoom) as f32
            } else {
                screen_height() / 2.0
            };
            if x.abs() < screen_width() / 2.0 && y.abs() < screen_height() / 2.0 {
                draw_circle(
                    screen_width() / 2.0 + x,
                    screen_height() / 2.0 - y,
                    5.0,
                    WHITE,
                );
            }
        }
        next_frame().await;
    }
}
