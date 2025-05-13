const TIME_STEP: f64 = 0.001;
const GRAVITATIONAL_CONSTANT: f64 = 6.6743e-11;
#[derive(Debug, Clone)]
struct Particle {
    // n-dimensional vector (units: meters)
    r: Vec<f64>,
    // n-dimensional vector (units: meters/seconds)
    v: Vec<f64>,
    // scalar (units: kg)
    m: f64,
}

impl Particle {
    fn new(r: Vec<f64>, v: Vec<f64>, m: f64) -> Particle {
        assert!(r.len() == v.len());
        Particle { r, v, m }
    }
}
#[derive(Debug, Clone)]

struct System {
    particles: Vec<Particle>,
}

impl System {
    fn new(particles: Vec<Particle>) -> Self {
        Self { particles }
    }
    fn tick(&mut self, time_step: f64) {
        for i in 0..self.particles.len() {
            let ag = self.gravitational_accel(i);
            let p = &mut self.particles[i];
            for d in 0..p.r.len() {
                p.r[d] += p.v[d] * time_step;
                p.v[d] += ag[d] * time_step;
            }
        }
    }
    fn gravitational_accel(&self, i: usize) -> Vec<f64> {
        let mut a: Vec<f64> = vec![0.0; 2];
        for (c, p) in self.particles.iter().enumerate() {
            if c == i {
                continue;
            }
            for d in 0..(p.r.len()) {
                // (G*M*m)/r^2
                let dist = p.r[d] - self.particles[i].r[d];
                a[d] += dist.signum() * (GRAVITATIONAL_CONSTANT * p.m) / (dist.powi(2));
            }
        }
        a
    }
}

fn main() {
    let mut system = System::new(vec![
        // mass of earth
        Particle::new(vec![0.0], vec![0.0], 5.9722e24),
        // particle at surface
        Particle::new(vec![6.3781e6], vec![0.0], 10.0),
    ]);
    let mut t = 1.0;
    while t > 0.0 {
        system.tick(TIME_STEP);
        t -= TIME_STEP;
    }
    println!("{:?}", system);
}
