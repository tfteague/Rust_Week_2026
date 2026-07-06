// Day 1 — Rust Week 2026 :: SOLUTION

use std::error::Error;
use std::f64::consts::PI;

use hoomd_gsd::hoomd::{Dimensions, HoomdGsdFile};
use hoomd_vector::{Cartesian, InnerProduct, Metric, Rotation, Versor};

const N: usize = 4;

// The simulation box for the ideal hexagonal-diamond cell (bond length = 1),
// as [Lx, Ly, Lz, xy, xz, yz].
const BOX: [f64; 6] = [1.632993, 1.414214, 2.666667, -0.577350, 0.0, 0.0];

fn main() -> Result<(), Box<dyn Error>> {
    // Basis positions of the hexagonal-diamond cell
    let x = 6.0_f64.sqrt() / 6.0;
    let y = 2.0_f64.sqrt() / 6.0;
    let positions: [Cartesian<3>; N] = [
        Cartesian::from([-x, y, -7.0 / 6.0]),
        Cartesian::from([-x, y, -1.0 / 6.0]),
        Cartesian::from([x, -y, 1.0 / 6.0]),
        Cartesian::from([x, -y, 7.0 / 6.0]),
    ];

    // Basis orientations.
    let orientations: [Versor; N] = [
        Versor::identity(),
        Versor::from_axis_angle([3.0_f64.sqrt(), 1.0, 0.0].try_into()?, PI),
        Versor::from_axis_angle([0.0, 0.0, 1.0].try_into()?, PI / 3.0),
        Versor::from_axis_angle([1.0, 3.0_f64.sqrt(), 0.0].try_into()?, PI),
    ];

    // Distance between two particles.
    let d01 = positions[0].distance(&positions[1]);
    println!("|P0 - P1| = {d01:.6}");

    // Axis & angle that rotate particle 0 onto particle 1 (q_rel = q_1 * q_0^-1).
    let q_rel = orientations[1].combine(&orientations[0].inverted());
    let (axis, angle) = axis_angle(&q_rel);
    println!(
        "P0 -> P1: axis = {axis}, angle = {:.3}°",
        angle.to_degrees()
    );

    // Write the frame to GSD so it can be checked in OVITO / gsd.
    save_to_gsd("hex_diamond.gsd", BOX, &positions, &orientations)?;
    println!("Wrote hex_diamond.gsd");
    Ok(())
}

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
