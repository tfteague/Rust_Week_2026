// Day 1 — Rust Week 2026 :: SOLUTION

use std::error::Error;
use std::f64::consts::PI;

use hoomd_gsd::hoomd::{Dimensions, HoomdGsdFile};
use hoomd_vector::{Cartesian, InnerProduct};

// How many mesh particles to lay down on the sphere.
const SAMPLES: usize = 1000;

// A cubic box big enough to hold the unit sphere with a little breathing room,
// as [Lx, Ly, Lz, xy, xz, yz].
const BOX: [f64; 6] = [2.5, 2.5, 2.5, 0.0, 0.0, 0.0];

fn main() -> Result<(), Box<dyn Error>> {
    let points = fibonacci_sphere(SAMPLES);

    // Sanity check: every point should sit on the unit sphere (radius ~ 1).
    let mut min_r = f64::INFINITY;
    let mut max_r = f64::NEG_INFINITY;
    for p in &points {
        let r = p.norm_squared().sqrt();
        min_r = min_r.min(r);
        max_r = max_r.max(r);
    }
    println!("placed {} points", points.len());
    println!("radius range: [{min_r:.6}, {max_r:.6}]  (should be ~1.0)");
    for (i, p) in points.iter().take(5).enumerate() {
        println!("  P{i}: [{:.6}, {:.6}, {:.6}]", p[0], p[1], p[2]);
    }

    // Write the frame to GSD so it can be plotted / checked in OVITO.
    save_to_gsd("fibonacci_sphere.gsd", BOX, &points)?;
    println!("Wrote fibonacci_sphere.gsd");
    Ok(())
}

/// Place `samples` points on the unit sphere using a Fibonacci lattice.
fn fibonacci_sphere(samples: usize) -> Vec<Cartesian<3>> {
    let mut points = Vec::with_capacity(samples);
    let phi = PI * (5.0_f64.sqrt() - 1.0); // golden angle in radians

    for i in 0..samples {
        // Spread y evenly from +1 down to -1 across the samples.
        let y = 1.0 - (i as f64 / (samples - 1) as f64) * 2.0;
        let radius = (1.0 - y * y).sqrt(); // radius of the circle at height y
        let theta = phi * i as f64; // golden-angle increment per point

        let x = theta.cos() * radius;
        let z = theta.sin() * radius;

        points.push(Cartesian::from([x, y, z]));
    }

    points
}

// All particles share a single type "M" (mesh); only position differs.
fn save_to_gsd(
    path: &str,
    box6: [f64; 6],
    positions: &[Cartesian<3>],
) -> Result<(), Box<dyn Error>> {
    let mut file = HoomdGsdFile::create(path)?;
    file.append_frame(0)?
        .configuration_dimensions(Dimensions::Three)?
        .configuration_box(box6)?
        .particles_position(positions.iter().copied())?
        .particles_types(["M"])?
        .end()?;
    Ok(())
}
