use nbody::sim::{Particle, System};

const TOTAL_TIME: f64 = 100_000_000.0;
const TRIALS: u32 = 500;
const D: usize = 2;
fn bench(n: usize) -> (u128, f64) {
    let mut parts = Vec::new();
    for i in 0..n {
        let mut v = Vec::new();
        let mut r = Vec::new();
        for _ in 0..D {
            v.push(0.0);
            r.push(i as f64 * 100.0);
        }
        parts.push(Particle::new(r, v, 1.0e10));
    }
    let mut sys = System::<D>::new(parts);
    let initial_energy = sys.total_energy();
    let start = std::time::Instant::now();
    for _ in 0..TRIALS {
        sys.tick(TOTAL_TIME / TRIALS as f64);
    }
    let end = std::time::Instant::now();
    let avg_time = (end - start) / TRIALS;
    let final_energy = sys.total_energy();
    let energy_error = (initial_energy - final_energy) / initial_energy;
    (avg_time.as_nanos(), energy_error)
}
fn main() {
    println!("|bodies    |tick (ns) |energy err|");
    for n in [1, 5, 100, 1000] {
        let (average_tick_time, energy_error) = bench(n);
        println!(
            "|{: <10}|{: <10}|{: <10.4e}|",
            n, average_tick_time, energy_error
        );
    }
}
