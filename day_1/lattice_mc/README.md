# Day 1 — Lattice-gas Monte Carlo (Rust vs. Python)

A 2D lattice-gas Metropolis Monte Carlo simulation, implemented **twice** — once
in Rust (`src/main.rs`) and once in Python (`lattice_mc.py`) — so the two can be
timed head-to-head.

Each site is empty (`0`) or occupied (`1`). A sweep repeatedly picks a random
site and flips its occupancy with the Metropolis probability `min(1, exp(-ΔE/T))`,
where `ΔE = (2·s − 1)·(neighbor_sum + ACTIVITY)` on a periodic lattice.

## Run

```bash
cargo run --release          # Rust version (a release build is essential)
python lattice_mc.py         # Python version (same algorithm & parameters)
```

Both use identical parameters — `SITES_PER_SIDE = 10`, `N_SWEEPS = 1_000_000`,
`TEMPERATURE = 5.0`, `ACTIVITY = 1.0` — i.e. **100 million** trial moves, and both
print the final lattice as a grid of ⬜ / 🟦 squares.

## Timing

```bash
# bash / Git Bash
time cargo run --release
time python lattice_mc.py
```

```powershell
# Windows PowerShell
Measure-Command { cargo run --release }
Measure-Command { python lattice_mc.py }
```

## Example results

Measured on one Windows laptop (Python 3.13, `rustc` release build):

| Program | Wall time | Relative |
|---------|-----------|----------|
| Rust (`--release`) | ~2.9 s | 1× |
| Python 3.13        | ~68 s  | ~23× slower |

Your absolute numbers will vary by machine; the point is the order-of-magnitude
gap for the *same* algorithm doing the *same* work.

## Notes

- **Always build Rust with `--release`.** A debug build skips optimizations and is
  many times slower — not a fair comparison.
- The two programs use different RNGs (Rust `SFC64`, Python's `random`), so their
  final grids differ cell-by-cell. The amount of work is identical, which is what
  the benchmark measures.
- On Windows the Python script reconfigures stdout to UTF-8 so the emoji grid
  prints on the default `cp1252` console.
