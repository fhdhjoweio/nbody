extern crate nalgebra as na;
use rayon::prelude::*;

const GRAVITATIONAL_CONSTANT: f64 = 6.6743e-11;

#[derive(Debug, Clone)]
pub struct Particle<const D: usize> {
    // n-dimensional vector (units: meters)
    pub x: na::RowSVector<f64, D>,
    // n-dimensional vector (units: meters/seconds)
    pub v: na::RowSVector<f64, D>,
    // scalar (units: kg)
    pub m: f64,
}

impl<const D: usize> Particle<D> {
    pub fn new(r: Vec<f64>, v: Vec<f64>, m: f64) -> Particle<D> {
        assert!(r.len() == v.len());
        Particle {
            x: na::RowSVector::from_vec(r),
            v: na::RowSVector::from_vec(v),
            m,
        }
    }
}

#[derive(Debug, Clone)]
pub struct System<const D: usize> {
    pub x: na::OMatrix<f64, na::Dyn, na::Const<D>>,
    pub v: na::OMatrix<f64, na::Dyn, na::Const<D>>,
    pub m: Vec<f64>,
    // units: pixels/meter
    pub zoom: f64,
}

impl<const D: usize> System<D> {
    pub fn from_particles(particles: Vec<Particle<D>>) -> Self {
        let mut x = na::OMatrix::<f64, na::Dyn, na::Const<D>>::zeros(particles.len());
        let mut v = na::OMatrix::<f64, na::Dyn, na::Const<D>>::zeros(particles.len());
        let mut m = Vec::new();
        // for i in 0..particles.len() {
        for (i, p) in particles.iter().enumerate() {
            x.set_row(i, &p.x);
            v.set_row(i, &p.v);
            m.push(p.m);
        }
        Self { x, v, m, zoom: 1.0 }
    }
    pub fn new(
        x: na::OMatrix<f64, na::Dyn, na::Const<D>>,
        v: nalgebra::OMatrix<f64, na::Dyn, na::Const<D>>,
        m: Vec<f64>,
    ) -> Self {
        Self { x, v, m, zoom: 1.0 }
    }
    pub fn runge_kutta(&mut self, time_step: f64) {
        const NUM_STAGES: usize = 4;
        const COEFFECIENTS: [f64; 3] = [0.5, 0.5, 1.0];
        const WEIGHTS: [f64; 4] = [1.0 / 6.0, 2.0 / 6.0, 2.0 / 6.0, 1.0 / 6.0];
        let x_initial = self.x.clone();
        let v_initial = self.v.clone();
        let mut xk =
            vec![
                unsafe { na::OMatrix::<f64, na::Dyn, na::Const<D>>::assume_init(self.x.len()) };
                NUM_STAGES
            ];
        let mut vk =
            vec![na::OMatrix::<f64, na::Dyn, na::Const<D>>::zeros(self.x.len()); NUM_STAGES];

        xk[0] = v_initial.clone();
        vk[0] = self.gravitational_accel();

        for stage in 1..NUM_STAGES {
            self.x = &x_initial + time_step * COEFFECIENTS[stage - 1] * &xk[stage - 1];
            xk[stage] = &v_initial + time_step * COEFFECIENTS[stage - 1] * &vk[stage - 1];
            vk[stage] = self.gravitational_accel();
        }

        let mut x_delta = na::OMatrix::<f64, na::Dyn, na::Const<D>>::zeros(x_initial.nrows());
        let mut v_delta = na::OMatrix::<f64, na::Dyn, na::Const<D>>::zeros(x_initial.nrows());
        for stage in 0..NUM_STAGES {
            x_delta += WEIGHTS[stage] * &xk[stage];
            v_delta += WEIGHTS[stage] * &vk[stage];
        }
        self.x = x_initial + time_step * x_delta;
        self.v = v_initial + time_step * v_delta;
    }
    pub fn euler(&mut self, time_step: f64) {
        let ag = self.gravitational_accel();
        self.v += ag * time_step;
        self.x += self.v.clone() * time_step;
    }
    fn gravitational_accel(&self) -> na::OMatrix<f64, na::Dyn, na::Const<D>> {
        let mut a = na::OMatrix::<f64, na::Dyn, na::Const<D>>::zeros(self.x.nrows());
        let backing = (0..self.x.nrows())
            .into_par_iter()
            .map(|current_index| {
                self.x
                    .row_iter()
                    .zip(&self.m)
                    .map(|(other_row_x, other_m)| {
                        // The distance is 0 when both are the same object, so
                        // return 0 early instead of causing NaN
                        let current_row_x = self.x.row(current_index);
                        if other_row_x == current_row_x {
                            return na::SMatrix::<f64, 1, D>::zeros();
                        }
                        // There is a distance term on the top and bottom
                        // because a naive norm_squared does not preserve
                        // the direction of the vector
                        // (G*M*m)/r^2
                        let distance = other_row_x - current_row_x;
                        distance * GRAVITATIONAL_CONSTANT * *other_m / distance.norm().powi(3)
                    })
                    .sum()
            })
            .collect::<Vec<na::SMatrix<f64, 1, D>>>();
        for (i, r) in backing.iter().enumerate() {
            a.set_row(i, r);
        }
        // print!("\r accel: {:.4e}", a.row(1).norm());
        a
    }
    pub fn total_energy(&self) -> f64 {
        let mut e = 0.0;
        //   println!("{}", self.x);
        for i in 0..self.x.nrows() {
            let m = self.m[i];
            e += 0.5 * m * self.v.row(i).norm_squared();
            for j in i + 1..self.x.nrows() {
                e -=
                    GRAVITATIONAL_CONSTANT * m * self.m[j] / (self.x.row(i) - self.x.row(j)).norm();
            }
        }
        e
    }
}
