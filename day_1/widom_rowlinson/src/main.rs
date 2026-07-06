// Day 1 — Rust Week 2026  ::  STUDENT TEMPLATE
// ----------------------------------------------------------------------------
// Tash is simulating the Widom-Rowlinson model with four species on a lattice.
// Each site is either empty or occupied by one of four species. Given the raw
// state array, render it as a grid of colored-square emojis so it can be eyeballed.
//
// Run with:   cargo run            (the solution is `cargo run --bin solution`)
//
// Palette:  0 ⬜ empty  ·  1 🟨  ·  2 🟦  ·  3 🟩  ·  4 🟥
//
// Useful bits:
//   arr.into_iter()                iterate an array by value
//   .map(|x| ...)                  transform each item with a closure
//   match x { 0 => 'a', _ => 'b' } pick a value per case; `_` is the default
//   .collect::<String>()           gather an iterator of `char` into a String
// ----------------------------------------------------------------------------

use hoomd_rand::SFC64;
use rand::{RngExt, SeedableRng};

const N_SITES: usize = 12; // the lattice is N_SITES x N_SITES

fn main() {
    // A random lattice state: 0 = empty, 1..=4 = the four species.
    let mut rng = SFC64::seed_from_u64(42);
    let mut lattice_state = [0u32; N_SITES * N_SITES];
    for site in lattice_state.iter_mut() {
        *site = rng.random_range(0..5);
    }

    // ----- TODO: write a closure that transforms the lattice state into a
    // String of colored squares, one emoji per site. Iterate the array, `.map()`
    // each value to a square with a `match`, and `.collect()` a `String`.
    let to_emoji_string = todo!("map each site value to a colored square and .collect() a String");

    let output = to_emoji_string(lattice_state);
    print_grid(&output);
}

/// Print the emoji string as an N_SITES x N_SITES grid.
fn print_grid(output: &str) {
    for (i, c) in output.chars().enumerate() {
        if i > 0 && i % N_SITES == 0 {
            println!();
        }
        print!("{c}");
    }
    println!();
}
