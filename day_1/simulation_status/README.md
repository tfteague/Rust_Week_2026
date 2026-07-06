# Day 1 — Simulation status as an `enum`

A simulation doesn't have one state — it moves through a sequence of **stages**:
it initializes, compresses the box a few steps, equilibrates, samples, and
finishes. This project models "which stage are we in?" with a Rust `enum` and
then walks the stages with a random pause between each, so you can watch it
advance in real time.

It's an `enum` workout: define the variants (one of which, `Compress`, carries
data), and `match` on them to print a status line.

Uses [hoomd-rs](https://hoomd-rs.readthedocs.io/en/1.1.0)'s `hoomd-rand` (`SFC64`)
for the random pauses, matching the other Day 1 projects.

## Run

```bash
cargo run                 # student template — you fill it in
cargo run --bin solution  # worked solution (uses Display)
```

Each run prints the seven stages in order, pausing a random 200–1200 ms between
them:

```
Stage: Initializing simulation
Stage: Compressing (step 1)
Stage: Compressing (step 2)
Stage: Compressing (step 3)
Stage: Equilibrating system
Stage: Sampling data
Stage: Simulation complete
```

## The `enum`

```rust
enum Phase {
    Initialize,
    Compress(i32),   // carries the compression step number
    Equilibrate,
    Sampling,
    Complete,
}
```

`Compress(i32)` is the interesting one: unlike the other variants, it holds a
value. When you `match` it you bind that value (`Phase::Compress(step) => ...`)
and can use it in the output.

## Two ways to print a variant

**In the student template**, a named method does the printing:

```rust
impl Phase {
    fn print_status(&self) {
        match self {
            Phase::Initialize => println!("Stage: Initializing simulation"),
            Phase::Compress(step) => println!("Stage: Compressing (step {})", step),
            // ...
        }
    }
}
```

**In the worked solution**, we implement the standard-library `Display` trait
instead. This is the idiomatic Rust approach: once a type is `Display`, the `{}`
formatter, `.to_string()`, `println!`, `format!`, and friends all know how to
render it — no bespoke method required.

```rust
use std::fmt;

impl fmt::Display for Phase {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Phase::Initialize => write!(f, "Stage: Initializing simulation"),
            Phase::Compress(step) => write!(f, "Stage: Compressing (step {step})"),
            // ...
        }
    }
}
```

The difference: `print_status` hard-codes `println!` (it can *only* print, to
stdout), while `Display` describes *how the value formats* and lets the caller
decide where it goes — `println!("{phase}")`, into a `String`, into a log, etc.
`write!(f, ...)` writes into the formatter instead of straight to the terminal.

## The random pause

`run_simulation` seeds an `SFC64` RNG from the wall clock (so the pacing differs
every run), then loops over the stages, sleeping a random number of milliseconds
*between* them:

```rust
for (i, phase) in phases.iter().enumerate() {
    if i > 0 {
        let millis = rng.random_range(200..1200);
        sleep(Duration::from_millis(millis));
    }
    println!("{phase}");   // Display in the solution; phase.print_status() in the template
}
```

## What you implement (`src/main.rs`)

- **TODO 1** — define the `Phase` enum with its five variants (remember
  `Compress` carries an `i32`). The provided `print_status` method won't compile
  until this enum exists, so this is the first thing to write.
- **TODO 2** — build the ordered `phases` array the simulation walks through.
- **TODO 3** — the loop: wait a random time between stages, then
  `phase.print_status()`.

Once all three are filled in, `cargo run` prints the seven stages with a pause
between each.
