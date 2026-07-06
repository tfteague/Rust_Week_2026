// Day 1 — Rust Week 2026  ::  STUDENT TEMPLATE
// ----------------------------------------------------------------------------
// Model a simulation's current stage with an `enum`, then walk the stages with
// a random pause between each so the program prints a different stage over time.
//
// Run with:   cargo run            (the solution is `cargo run --bin solution`)
//
// Useful bits:
//   enum Name { A, B(i32), C }        an enum; `B` carries an i32 payload
//   match self { Name::A => ... }     match every variant of an enum
//   Phase::Compress(3)                build a variant that carries data
//   std::thread::sleep(dur)           pause the current thread
//   Duration::from_millis(n)          a duration of n milliseconds
//   rng.random_range(200..1200)       a random integer in [200, 1200)
// ----------------------------------------------------------------------------

use std::thread::sleep;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use hoomd_rand::SFC64;
use rand::{RngExt, SeedableRng};

// ----- TODO 1: define the `Phase` enum.
// It needs five variants:
//   Initialize, Compress(i32), Equilibrate, Sampling, Complete
// `Compress` carries the compression step number (an i32); the rest carry no
// data. Uncomment and fill this in:
//
// enum Phase {
//     // ...
// }

// This method is already written for you — it prints a human-readable line for
// each variant. (In the worked solution this becomes a real `Display` impl.)
impl Phase {
    fn print_status(&self) {
        match self {
            Phase::Initialize => println!("Stage: Initializing simulation"),
            Phase::Compress(step) => println!("Stage: Compressing (step {})", step),
            Phase::Equilibrate => println!("Stage: Equilibrating system"),
            Phase::Sampling => println!("Stage: Sampling data"),
            Phase::Complete => println!("Stage: Simulation complete"),
        }
    }
}

fn main() {
    run_simulation();
}

fn run_simulation() {
    // Seed the RNG from the wall clock so the pauses differ every run.
    let seed = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("clock is before the Unix epoch")
        .as_nanos() as u64;
    let mut rng = SFC64::seed_from_u64(seed);
}
