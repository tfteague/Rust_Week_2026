// Day 1 — Rust Week 2026  ::  STUDENT TEMPLATE
// ----------------------------------------------------------------------------
//
// Run with:   cargo run            (the solution is `cargo run --bin solution`)
//
// Useful methods:
//   Cartesian<3>::from([x, y, z])         a 3D vector
//   a.distance(&b)                        distance between two points (Metric)
//   Versor::identity()                    the identity rotation
//   Versor::from_axis_angle(axis, angle)  axis is `[x, y, z].try_into()?`
//   q.combine(&r)                         quaternion product  q * r
//   q.inverted()                          inverse rotation (conjugate)
//   q.get().scalar , q.get().vector       the (w, xyz) parts of the versor
// ----------------------------------------------------------------------------

use std::error::Error;
use std::f64::consts::PI;

use hoomd_gsd::hoomd::{Dimensions, HoomdGsdFile};
use hoomd_vector::{Cartesian, InnerProduct, Metric, Rotation, Versor};

const N: usize = 4;
const BOX: [f64; 6] = [1.632993, 1.414214, 2.666667, -0.577350, 0.0, 0.0];

fn main() -> Result<(), Box<dyn Error>> {
    // ----- TODO 1: basis positions
    // Build the four basis positions as an array of `Cartesian<3>`.
    let positions: [Cartesian<3>; N] = todo!("build the 4 basis positions");

    // ----- TODO 2: basis orientations
    // Create the four orientations as an array of `Versor`.
    // Give each tetrahedron the orientation whose four patches point along its
    // four bonds.
    let orientations: [Versor; N] = todo!("build the 4 basis orientations");

    // ----- TODO 3: distance
    // Calculate and print the distance between the first two particles.
    // (See the axis_angle / distance helpers below.)
    let d01: f64 = todo!("distance between positions[0] and positions[1]");
    println!("|P0 - P1| = {d01:.6}");

    // ----- TODO 4: relative rotation axis & angle
    // Print the axis and angle that rotates particle 0 onto particle 1.
    let relative_angle: Versor = todo!("form q_rel = q_1 * q_0^-1");
    let (axis, angle) = axis_angle(&relative_angle);
    println!(
        "P0 -> P1: axis = {axis}, angle = {:.3}°",
        angle.to_degrees()
    );

    // --------- Save the result (box is pre-written above) -------------------
    save_to_gsd("student.gsd", BOX, &positions, &orientations)?;
    Ok(())
}

// Helper Functions

fn axis_angle(q: &Versor) -> (Cartesian<3>, f64) {
    let quat = q.get();
    let v_norm = quat.vector.norm_squared().sqrt();
    let angle = 2.0 * v_norm.atan2(quat.scalar);
    let axis = if v_norm > 1e-12 {
        quat.vector / v_norm
    } else {
        Cartesian::from([0.0, 0.0, 1.0]) // no rotation: axis is arbitrary
    };
    (axis, angle)
}

// All particles share a single type "T"; only position and orientation differ.
fn save_to_gsd(
    path: &str,
    box6: [f64; 6],
    positions: &[Cartesian<3>],
    orientations: &[Versor],
) -> Result<(), Box<dyn Error>> {
    let mut file = HoomdGsdFile::create(path)?;
    file.append_frame(0)?
        .configuration_dimensions(Dimensions::Three)?
        .configuration_box(box6)?
        .particles_position(positions.iter().copied())?
        .particles_orientation(orientations.iter().copied())?
        .particles_types(["T"])?
        .end()?;
    Ok(())
}
