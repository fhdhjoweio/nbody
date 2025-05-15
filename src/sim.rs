const GRAVITATIONAL_CONSTANT: f64 = 6.6743e-11;
#[derive(Debug, Clone)]
pub struct Particle {
    // n-dimensional vector (units: meters)
    pub r: Vec<f64>,
    // n-dimensional vector (units: meters/seconds)
    pub v: Vec<f64>,
    // scalar (units: kg)
    m: f64,
}

impl Particle {
    pub fn new(r: Vec<f64>, v: Vec<f64>, m: f64) -> Particle {
        assert!(r.len() == v.len());
        Particle { r, v, m }
    }
}
#[derive(Debug, Clone)]

pub struct System {
    pub particles: Vec<Particle>,
    // units: pixels/meter
    pub zoom: f64,
}

impl System {
    pub fn new(particles: Vec<Particle>) -> Self {
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
    pub fn gravitational_accel(&self, i: usize) -> Vec<f64> {
        let mut a: Vec<f64> = vec![0.0; 2];
        for (c, p) in self.particles.iter().enumerate() {
            if c == i {
                continue;
            }
            for d in 0..(p.r.len()) {
                // (G*M*m)/r^2
                let dist = p.r[d] - self.particles[i].r[d];
                if dist.abs() < 0.001 {
                    continue;
                }
                a[d] += dist.signum() * (GRAVITATIONAL_CONSTANT * p.m) / (dist.powi(2));
            }
        }
        a
    }
}
