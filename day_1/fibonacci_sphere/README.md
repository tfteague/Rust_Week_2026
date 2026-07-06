# Day 1 — Fibonacci-lattice mesh particles on a sphere

Zoe, Ben, and Jared are initializing **mesh particles on the unit sphere** using
a **Fibonacci lattice** — the quasi-uniform spiral you get by stepping around the
sphere by the golden angle. This project is a `for`-loop workout: the whole
algorithm is one loop that pushes a point per sample.

Built on the [hoomd-rs](https://hoomd-rs.readthedocs.io/en/1.1.0) crates
(`hoomd-vector` for `Cartesian<3>`, `hoomd-gsd` for GSD I/O).

## Run

```bash
cargo run                 # student template — you fill it in (writes student.gsd)
cargo run --bin solution  # worked solution (writes fibonacci_sphere.gsd)
```

## The algorithm

This is the classic Python `fibonacci_sphere`, translated to Rust:

```python
def fibonacci_sphere(samples=1000):
    points = []
    phi = math.pi * (math.sqrt(5.) - 1.)          # golden angle
    for i in range(samples):
        y = 1 - (i / float(samples - 1)) * 2       # y: +1 -> -1
        radius = math.sqrt(1 - y * y)              # radius of circle at height y
        theta = phi * i                            # golden-angle spiral
        x = math.cos(theta) * radius
        z = math.sin(theta) * radius
        points.append((x, y, z))
    return points
```

The idea: march `y` evenly from the north pole (`+1`) to the south pole (`-1`).
At each height the point lives on a circle of `radius = √(1 − y²)`, and we step
the angle `theta` around that circle by the **golden angle**
`φ = π(√5 − 1) ≈ 2.399963` radians each time. Because the golden angle is
irrational with respect to a full turn, consecutive points never line up, and
the samples spread almost uniformly over the sphere.

### Rust translation

`points = []` becomes a `Vec<Cartesian<3>>`; `points.append(...)` becomes
`points.push(...)`; `range(samples)` becomes `0..samples`; and the integer `i`
has to be cast with `i as f64` before it does floating-point math.

```rust
fn fibonacci_sphere(samples: usize) -> Vec<Cartesian<3>> {
    let mut points = Vec::with_capacity(samples);
    let phi = PI * (5.0_f64.sqrt() - 1.0);

    for i in 0..samples {
        let y = 1.0 - (i as f64 / (samples - 1) as f64) * 2.0;
        let radius = (1.0 - y * y).sqrt();
        let theta = phi * i as f64;
        let x = theta.cos() * radius;
        let z = theta.sin() * radius;
        points.push(Cartesian::from([x, y, z]));
    }
    points
}
```

## What you implement (`src/main.rs`)

The template (run with `cargo run`) hands over the GSD-writing helper and the
box, and leaves the loops:

- **TODO 1** — the golden angle `phi`.
- **TODO 2** — the main `for i in 0..samples` loop that computes each point and
  pushes it.
- **TODO 3** — a second `for` loop over the points that finds the min/max radius
  (they should both be ~1.0).
- **TODO 4** — a `for` loop that prints the first five points.

## Verification

The solution prints the radius range (both ends read `1.000000`) and the first
few points, then writes `fibonacci_sphere.gsd` (all particles are a **single type
`M`**; only position differs).

## Plotting the result

`plot.py` reads the GSD and draws the points as a 3D scatter on the sphere,
colored by height, so you can see whether the loop spread the mesh evenly:

```bash
pip install gsd numpy matplotlib
python plot.py                         # plots student.gsd
python plot.py fibonacci_sphere.gsd    # plots the worked solution
python plot.py student.gsd out.png     # save to a PNG instead of showing a window
```

You can also import `fibonacci_sphere.gsd` into OVITO to view the point cloud
directly.
