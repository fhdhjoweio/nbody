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
    fn runge_kutta(&mut self, time_step: f64) {
        const NUM_STAGES: usize = 4;
        const COEFFECIENTS: [f64; 3] = [0.5, 0.5, 1.0];
        const WEIGHTS: [f64; 4] = [1.0 / 6.0, 2.0 / 6.0, 2.0 / 6.0, 1.0 / 6.0];
        for i in 0..self.particles.len() {
            let x_initial = self.particles[i].r;
            let v_initial = self.particles[i].v;

            let mut xk = [na::SVector::<f64, D>::zeros(); NUM_STAGES];
            let mut vk = [na::SVector::<f64, D>::zeros(); NUM_STAGES];

            xk[0] = v_initial;
            vk[0] = self.gravitational_accel(i);

            for stage in 1..NUM_STAGES {
                self.particles[i].r =
                    x_initial + time_step * COEFFECIENTS[stage - 1] * xk[stage - 1];
                let a = self.gravitational_accel(i);

                xk[stage] = v_initial + time_step * COEFFECIENTS[stage - 1] * vk[stage - 1];
                vk[stage] = a;
            }

            let mut x_delta = na::SVector::<f64, D>::zeros();
            let mut v_delta = na::SVector::<f64, D>::zeros();
            for stage in 0..NUM_STAGES {
                x_delta += WEIGHTS[stage] * xk[stage];
                v_delta += WEIGHTS[stage] * vk[stage];
            }
            self.particles[i].r = x_initial + time_step * x_delta;
            self.particles[i].v = v_initial + time_step * v_delta;
        }
    }
    #[allow(dead_code)]
    fn euler(&mut self, time_step: f64) {
        for i in 0..self.particles.len() {
            let ag = self.gravitational_accel(i);
            let p = &mut self.particles[i];
            for d in 0..p.r.len() {
                p.v[d] += ag[d] * time_step;
                p.r[d] += p.v[d] * time_step;
            }
        }
    }
    pub fn tick(&mut self, time_step: f64) {
        self.runge_kutta(time_step);
    }
    fn gravitational_accel(&self, i: usize) -> na::SVector<f64, D> {
        let mut a: na::SVector<f64, D> = na::SVector::zeros();
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
    pub fn total_energy(&self) -> f64 {
        let mut e = 0.0;
        for i in 0..self.particles.len() {
            e += 0.5 * self.particles[i].m * self.particles[i].v.norm_squared();
            for j in i + 1..self.particles.len() {
                e -= GRAVITATIONAL_CONSTANT * self.particles[i].m * self.particles[j].m
                    / (self.particles[i].r - self.particles[j].r).norm();
            }
        }
        e
    }
}
