// Day 1 — Rust Week 2026  ::  WORKED SOLUTION
// ----------------------------------------------------------------------------
// Tash is simulating the Widom-Rowlinson model with four species on a lattice.
// Given the raw state array, a closure maps each site to a colored-square emoji
// and collects the result into one String, which we print as a grid.
//
// Run with:   cargo run --bin solution
//
// Palette:  0 ⬜ empty  ·  1 🟨  ·  2 🟦  ·  3 🟩  ·  4 🟥
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

    // The closure: transform the state array into a String of colored squares.
    // `into_iter()` yields each `u32`; `match` picks the emoji; `collect()`
    // concatenates the resulting `char`s into a `String`.
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
