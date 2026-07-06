// Day 1 — Rust Week 2026  ::  WORKED SOLUTION
// ----------------------------------------------------------------------------
// A simulation moves through a fixed sequence of stages. We model the current
// stage with an `enum Phase`, and let the program walk the stages with a random
// pause between each so you can watch it advance in real time.
//
// This solution implements the standard-library `Display` trait instead of a
// named `print_status` method, so `println!("{phase}")` just works.
//
// Run with:   cargo run --bin solution
// ----------------------------------------------------------------------------

use std::fmt;
use std::thread::sleep;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use hoomd_rand::SFC64;
use rand::{RngExt, SeedableRng};

/// The stage a simulation is currently in. `Compress` carries the compression
/// step number, so it prints differently each step.
enum Phase {
    Initialize,
    Compress(i32),
    Equilibrate,
    Sampling,
    Complete,
}

// The idiomatic Rust way to make a type printable: implement `Display`. Now the
// `{}` formatter (and `.to_string()`, `println!`, etc.) all know how to render a
// `Phase`, with no bespoke `print_status` method needed.
impl fmt::Display for Phase {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Phase::Initialize => write!(f, "Stage: Initializing simulation"),
            Phase::Compress(step) => write!(f, "Stage: Compressing (step {step})"),
            Phase::Equilibrate => write!(f, "Stage: Equilibrating system"),
            Phase::Sampling => write!(f, "Stage: Sampling data"),
            Phase::Complete => write!(f, "Stage: Simulation complete"),
        }
    }
}

fn main() {
    run_simulation();
}

fn run_simulation() {
    // Seed the RNG from the wall clock so the pauses (and thus the pacing)
    // differ on every run.
    let seed = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("clock is before the Unix epoch")
        .as_nanos() as u64;
    let mut rng = SFC64::seed_from_u64(seed);

    // The stages, in the order the simulation moves through them.
    let phases = [
        Phase::Initialize,
        Phase::Compress(1),
        Phase::Compress(2),
        Phase::Compress(3),
        Phase::Equilibrate,
        Phase::Sampling,
        Phase::Complete,
    ];

    for (i, phase) in phases.iter().enumerate() {
        // Wait a random amount of time *between* stages, so each stage lingers
        // on screen before the next one prints.
        if i > 0 {
            let millis = rng.random_range(200..1200);
            sleep(Duration::from_millis(millis));
        }
        println!("{phase}");
    }
}
