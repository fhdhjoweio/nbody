use nbody::sim::{Particle, System};

const TRIALS: u32 = 10000;
fn main() {
    let parts = vec![
        Particle::new(vec![100.0], vec![0.0], 1.0e10),
        Particle::new(vec![200.0], vec![0.0], 1.0e10),
        Particle::new(vec![300.0], vec![0.0], 1.0e10),
        Particle::new(vec![400.0], vec![0.0], 1.0e10),
        Particle::new(vec![500.0], vec![0.0], 1.0e10),
    ];
    let mut sys = System::new(parts.clone());
    let start = std::time::Instant::now();
    for _ in 0..TRIALS {
        sys.tick(0.1);
    }
    let end = std::time::Instant::now();
    let avg_time = (end - start) / TRIALS;
    println!(
        "Average tick for {} bodies: {} ns",
        parts.len(),
        avg_time.as_nanos()
    );
}
