use std::usize;

use nalgebra::SVector;

extern crate nalgebra as na;

const GRAVITATIONAL_CONSTANT: f64 = 6.6743e-11;
#[derive(Debug, Clone)]
pub struct Particle<const D: usize> {
    // n-dimensional vector (units: meters)
    pub r: na::SVector<f64, D>,
    // n-dimensional vector (units: meters/seconds)
    pub v: na::SVector<f64, D>,
    // scalar (units: kg)
    m: f64,
}

impl<const D: usize> Particle<D> {
    pub fn new(r: Vec<f64>, v: Vec<f64>, m: f64) -> Particle<D> {
        assert!(r.len() == v.len());
        Particle {
            r: na::SVector::from_vec(r),
            v: na::SVector::from_vec(v),
            m,
        }
    }
}
#[derive(Debug, Clone)]

pub struct System<const D: usize> {
    pub particles: Vec<Particle<D>>,
    // units: pixels/meter
    pub zoom: f64,
}

impl<const D: usize> System<D> {
    pub fn new(particles: Vec<Particle<D>>) -> Self {
        Self {
            particles,
            zoom: 1.0,
        }
    }
    pub fn tick(&mut self, time_step: f64) {
        for i in 0..self.particles.len() {
            let ag = self.gravitational_accel(i);
            let p = &mut self.particles[i];
            for d in 0..p.r.len() {
                p.r[d] += p.v[d] * time_step;
                p.v[d] += ag[d] * time_step;
            }
        }
    }
    pub fn gravitational_accel(&self, i: usize) -> SVector<f64, D> {
        let mut a: SVector<f64, D> = SVector::zeros();
        for (c, p) in self.particles.iter().enumerate() {
            if c == i {
                continue;
            }
            // (G*M*m)/r^2
            let dist = p.r - self.particles[i].r;
            if dist.norm() < 0.001 {
                continue;
            }
            a += (dist * GRAVITATIONAL_CONSTANT * p.m) / (dist.norm().powi(3))
        }
        a
    }
}
