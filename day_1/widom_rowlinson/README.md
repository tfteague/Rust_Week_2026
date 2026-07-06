# Day 1 — Widom-Rowlinson lattice as colored squares

Tash is simulating the **Widom-Rowlinson model** with four species on a lattice.
Each of the `N_SITES × N_SITES` sites is either empty or occupied by one of four
species, stored as a flat `[u32; N_SITES * N_SITES]` array. To eyeball a
configuration, we render it as a grid of colored-square emojis.

This is a **closures + `match`** workout: write one closure that turns the raw
state array into a `String`, mapping each site value to a square.

Uses [hoomd-rs](https://hoomd-rs.readthedocs.io/en/1.1.0)'s `hoomd-rand` (`SFC64`)
to generate a random sample state, matching the other Day 1 projects.

## Run

```bash
cargo run                 # student template — you fill it in
cargo run --bin solution  # worked solution
```

## The palette

| value | square | meaning   |
|-------|--------|-----------|
| `0`   | ⬜     | empty     |
| `1`   | 🟨     | species 1 |
| `2`   | 🟦     | species 2 |
| `3`   | 🟩     | species 3 |
| `4`   | 🟥     | species 4 |

## The closure

Iterate the array, `.map()` each value to a `char` with a `match` (the `_` arm is
the default, catching species 4), and `.collect()` the chars into a `String`:

```rust
let to_emoji_string = |state: [u32; N_SITES * N_SITES]| -> String {
    state
        .into_iter()
        .map(|x| match x {
            0 => '⬜',
            1 => '🟨',
            2 => '🟦',
            3 => '🟩',
            _ => '🟥',
        })
        .collect()
};
```

A `char` in Rust is a full Unicode scalar value, so each colored square is a
single `char` — mapping to `char` and collecting into a `String` just works.

## What you implement (`src/main.rs`)

The template generates the random state and prints the grid for you; the single
**TODO** is the `to_emoji_string` closure above.

## Expected output

A random `N_SITES × N_SITES` grid (12 × 12 by default), e.g.:

```
🟨🟦🟩🟦🟨🟨🟥🟩⬜🟨⬜🟦
🟥🟥🟦⬜🟨🟩🟥🟥🟩🟩🟥⬜
🟦⬜🟦🟥🟥🟦🟩🟦🟦🟥🟩⬜
...
```
