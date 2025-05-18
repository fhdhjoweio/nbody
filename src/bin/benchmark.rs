use nbody::sim::{Particle, System};

const TRIALS: u32 = 30;
const D: usize = 2;
fn bench(n: usize) -> u128 {
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
    let start = std::time::Instant::now();
    for _ in 0..TRIALS {
        sys.tick(0.1);
    }
    let end = std::time::Instant::now();
    let avg_time = (end - start) / TRIALS;
    avg_time.as_nanos()
}
fn main() {
    println!("|bodies    |tick (ns) |");
    for n in [1, 5, 100, 1000, 10_000] {
        println!("|{: <10}|{: <10}|", n, bench(n));
    }
}
