# Day 1 — Hexagonal-diamond unit cell of patchy tetrahedra

Yiyang, Kate, and Sajib are setting up the **unit cell** of a hexagonal-diamond
(lonsdaleite) assembly of patchy tetrahedra, oriented so that the four patches
of every tetrahedron point straight down its four bonds.

Built on the [hoomd-rs](https://hoomd-rs.readthedocs.io/en/1.1.0) crates
(`hoomd-vector` for `Cartesian<3>` / `Versor`, `hoomd-gsd` for GSD I/O).

## Run

```bash
cargo run                 # student template — you fill it in (writes student.gsd)
cargo run --bin solution  # worked solution (writes hex_diamond.gsd)
```

## The structure

Hexagonal-diamond has a 4-atom basis on a hexagonal lattice (γ = 120°). For the
ideal tetrahedral network the axial ratio is `c/a = √(8/3)` and the internal
parameter is `u = 3/8`. Lattice vectors:

```
a1 = a (1, 0, 0)        a2 = a (-1/2, √3/2, 0)        a3 = c (0, 0, 1)
```

We scale so the nearest-neighbor bond length is **1**, giving
`a = √(8/3) ≈ 1.632993` and `c = 8/3 ≈ 2.666667`.

This matches the [gisaxs hexagonal-diamond
reference](https://gisaxs.com/index.php/Lattice:Hexagonal_diamond): space group
P6₃/mmc, `c/a = √(8/3) ≈ 1.633`, four atoms per cell. Their bond relation
`a = (2√6/3)·l` is the same as ours since `(2√6/3)² = 8/3`.

### Basis positions (fractional → Cartesian, bond = 1)

The fractional basis below sits at a cell corner, so the Cartesian positions are
**recentered on the cell centroid** — otherwise the origin-centered HOOMD box
would leave particles hanging outside it.

| # | fractional (a1,a2,a3) | Cartesian `[x, y, z]` (centered)   |
|---|-----------------------|------------------------------------|
| 0 | (1/3, 2/3, 0)         | `[-0.408248,  0.235702, -1.166667]`|
| 1 | (1/3, 2/3, 3/8)       | `[-0.408248,  0.235702, -0.166667]`|
| 2 | (2/3, 1/3, 1/2)       | `[ 0.408248, -0.235702,  0.166667]`|
| 3 | (2/3, 1/3, 7/8)       | `[ 0.408248, -0.235702,  1.166667]`|

Box `[Lx, Ly, Lz, xy, xz, yz] = [1.632993, 1.414214, 2.666667, -0.577350, 0, 0]`.

### Basis orientations (Versor `q = [w, x, y, z] = (cos θ⁄2, sin θ⁄2 · n̂)`)

Particle 0 is the reference (identity). The others are the proper rotations that
carry the reference tetrahedron onto the required set of patch directions.

| # | quaternion `[w, x, y, z]`        | axis n̂ (from particle 0) | angle |
|---|----------------------------------|--------------------------|-------|
| 0 | `[1, 0, 0, 0]`                   | —                        | 0°    |
| 1 | `[0, 0.866025, 0.5, 0]`          | `(√3/2, 1/2, 0)`         | 180°  |
| 2 | `[0.866025, 0, 0, 0.5]`          | `(0, 0, 1)`              | 60°   |
| 3 | `[0, 0.5, 0.866025, 0]`          | `(1/2, √3/2, 0)`         | 180°  |

Particles 0 & 2 are "patch-up" tetrahedra; 1 & 3 are "patch-down". The extra 60°
twist between {0,1} and {2,3} is exactly what makes the stacking *hexagonal*
(eclipsed) diamond rather than *cubic* (staggered) diamond.

### Distances

Every nearest-neighbor (patch) bond has length **1.000000**:
`|P0−P1| = |P1−P2| = |P2−P3| = 1`. Second neighbors are at `a = 1.632993`.

### Relative orientation (axis & angle to rotate particle i → j)

`q_rel = q_j · q_i⁻¹`, then `θ = 2·atan2(|vector|, scalar)`, `n̂ = vector/|vector|`:

| pair | axis            | angle |
|------|-----------------|-------|
| 0→1  | `(√3/2, 1/2, 0)`| 180°  |
| 0→2  | `(0, 0, 1)`     | 60°   |
| 0→3  | `(1/2, √3/2, 0)`| 180°  |

## What you implement (`src/main.rs`)

The template (run with `cargo run`) hands over the box and the `axis_angle` /
GSD-writing helpers, and leaves four TODOs:

- **TODO 1** — the four basis positions as a `[Cartesian<3>; N]`.
- **TODO 2** — the four basis orientations as a `[Versor; N]`.
- **TODO 3** — the distance between the first two particles.
- **TODO 4** — the relative rotation `q_rel = q_1 · q_0⁻¹` (axis & angle printed
  by the provided `axis_angle` helper).

## Verification

The solution (`cargo run --bin solution`) writes `hex_diamond.gsd` (all particles
are a **single type `T`**; only position and orientation differ). The student
template (`cargo run`) writes `student.gsd` once its four TODOs are filled in.
`verify.py` independently re-derives every bond
distance (all `1.0`) and the relative-rotation axes/angles, and confirms that
each tetrahedron's patches point along its bonds.

Cross-check the GSD with the reference Python `gsd` package:

```bash
pip install gsd
python verify.py hex_diamond.gsd
```

## Viewing the result

Generate a patchy-**tetrahedron** shape mesh (built in the same base orientation
as particle 0) and load it as the particle shape in OVITO:

```bash
pip install pyvista
python create_patchy_tetrahedron.py     # writes tetrahedron.vtk
```

The body is blush pink with baby-blue patches. Bond length is 1, so patches on
neighboring particles overlap by `2*(tetra_radius + patch_length) - 1`; the
defaults `tetra_radius = 0.30`, `patch_length = 0.25` give a small (~0.1)
overlap. Lower them toward `0.5` total for a gap instead.

Import `hex_diamond.gsd` into OVITO, set the particle shape to `tetrahedron.vtk`,
and OVITO applies each particle's `particles/orientation` quaternion — the four
patch tips then point straight down the four bonds.
