// Day 1 — Rust Week 2026  ::  STUDENT TEMPLATE
// ----------------------------------------------------------------------------
//
// Run with:   cargo run            (the solution is `cargo run --bin solution`)
//
// Useful bits:
//   for i in 0..samples { ... }           loop i = 0, 1, ..., samples-1
//   i as f64                              turn a usize into an f64
//   x.sqrt()  x.cos()  x.sin()            f64 math methods
//   Vec::new()   v.push(item)             a growable list (like Python's [])
//   Cartesian::from([x, y, z])            a 3D point
//   p[0], p[1], p[2]                      read a point's x, y, z
//   p.norm_squared()                      squared length of a point (InnerProduct)
//   PI                                    std::f64::consts::PI
// ----------------------------------------------------------------------------

use std::error::Error;
use std::f64::consts::PI;

use hoomd_gsd::hoomd::{Dimensions, HoomdGsdFile};
use hoomd_vector::{Cartesian, InnerProduct};

const SAMPLES: usize = 1000;
const BOX: [f64; 6] = [2.5, 2.5, 2.5, 0.0, 0.0, 0.0];

fn main() -> Result<(), Box<dyn Error>> {
    let points = fibonacci_sphere(SAMPLES);
    // ----- TODO 3: print the first 5 points

    save_to_gsd("student.gsd", BOX, &points)?;
    println!("Wrote student.gsd");
    Ok(())
}

/// Place `samples` points on the unit sphere using a Fibonacci lattice.
fn fibonacci_sphere(samples: usize) -> Vec<Cartesian<3>> {
    // ----- TODO 1: the golden angle
    // phi = pi * (sqrt(5) - 1)
    let phi: f64 = todo!("compute the golden angle");

    // Start with an empty list of points (this is Python's `points = []`).
    let mut points: Vec<Cartesian<3>> = Vec::with_capacity(samples);

    // ----- TODO 2: the main loop
    // Write a `for` loop over `i` in `0..samples` that reproduces the Python
    // body above: compute `y`, `radius`, `theta`, then `x` and `z`, and
    // `points.push(Cartesian::from([x, y, z]))`.
    //
    todo!("fill the loop that pushes each Fibonacci point");

    points
}

// Helper Functions

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
