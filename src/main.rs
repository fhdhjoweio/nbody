use macroquad::prelude::*;
use nbody::sim::{Particle, System};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SerializedParticle<const D: usize> {
    // n-dimensional vector (units: kilometers)
    pub x: Vec<f64>,
    // n-dimensional vector (units: kilometers/seconds)
    pub v: Vec<f64>,
    // scalar (units: kg)
    pub m: f64,
}

impl<const D: usize> SerializedParticle<D> {
    pub fn into_particle(self) -> Particle<D> {
        if self.x.len() != D || self.v.len() != D {
            panic!();
        }
        Particle::new(self.x, self.v, self.m)
    }
    pub fn load_from_path(p: &Path) -> Vec<Self> {
        let file = std::fs::OpenOptions::new()
            .read(true)
            .write(false)
            .open(p)
            .unwrap();
        let mut o: Vec<SerializedParticle<D>> = serde_json::from_reader(file).unwrap();
        for p in &mut o {
            p.x = p.x.iter().map(|n| n * 1000.0).collect();
            p.v = p.v.iter().map(|n| n * 1000.0).collect();
        }
        o
    }
}

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
    let particles = SerializedParticle::<3>::load_from_path(&PathBuf::from(
        std::env::args()
            .nth(1)
            .unwrap_or("initial_conditions.json".to_string()),
    ));
    let mut system =
        System::<3>::from_particles(particles.into_iter().map(|p| p.into_particle()).collect());
    let mut speed_factor = 10.0;
    loop {
        let frame_time = get_frame_time();
        if is_key_down(KeyCode::Q) {
            speed_factor *= 1.1;
        } else if is_key_down(KeyCode::A) {
            speed_factor /= 1.1;
        }
        if is_key_down(KeyCode::W) {
            system.zoom *= 1.1;
        } else if is_key_down(KeyCode::S) {
            system.zoom /= 1.1;
        }
        clear_background(BLACK);
        draw_text(
            &format!("Speed: {speed_factor:.1}X"),
            10.0,
            10.0,
            20.0,
            WHITE,
        );
        draw_text(
            &format!("Frame Time: {:.2}", frame_time * 1000.0),
            10.0,
            24.0,
            20.0,
            WHITE,
        );
        draw_text(
            &format!("1px={:.1e}m", 1.0 / system.zoom),
            10.0,
            38.0,
            20.0,
            WHITE,
        );
        for _ in 0..5000 {
            system.runge_kutta(frame_time as f64 / 5000.0 * speed_factor);
        }
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
                    2.0,
                    WHITE,
                );
            }
        }
        next_frame().await;
    }
}
