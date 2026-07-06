use hoomd_rand::SFC64;
use rand::{RngExt, SeedableRng};

const SITES_PER_SIDE: usize = 10;
const N_SWEEPS: u32 = 1e6 as u32;
const N_SITES: usize = SITES_PER_SIDE * SITES_PER_SIDE;
const TEMPERATURE: f64 = 5.0; // T, in units of epsilon/k_B
const ACTIVITY: f64 = 1.0;

fn main() {
    let mut lattice_state: Vec<u8> = vec![0; N_SITES];
    let mut rng = SFC64::seed_from_u64(42);

    for _ in 0..N_SWEEPS {
        for _ in 0..N_SITES {
            let site = rng.random_range(0..N_SITES);
            let delta = delta_energy(&lattice_state, site);
            let accept = delta <= 0.0 || (rng.random::<f64>() < (-delta / TEMPERATURE).exp());

            if accept {
                lattice_state[site] = (lattice_state[site] + 1) % 2;
            }
        }
    }
    log_state(&lattice_state);
}

/// ΔE for flipping the current spin: (2*current - 1) * (neighbor_sum + C)
fn delta_energy(lattice_state: &[u8], site_id: usize) -> f64 {
    let row = site_id / SITES_PER_SIDE;

    let right_id = row * SITES_PER_SIDE + (site_id + 1) % SITES_PER_SIDE;
    let left_id = row * SITES_PER_SIDE + (site_id + SITES_PER_SIDE - 1) % SITES_PER_SIDE;
    let above_id = (site_id + N_SITES - SITES_PER_SIDE) % N_SITES;
    let below_id = (site_id + SITES_PER_SIDE) % N_SITES;

    let neighbor_sum = (lattice_state[right_id]
        + lattice_state[left_id]
        + lattice_state[above_id]
        + lattice_state[below_id]) as f64;

    let sign = 2.0 * lattice_state[site_id] as f64 - 1.0; // -1 if empty, +1 if filled
    sign * (neighbor_sum + ACTIVITY)
}

fn log_state(lattice_state: &[u8]) {
    let output: String = lattice_state
        .into_iter()
        .map(|x| match x {
            0 => '⬜',
            _ => '🟦',
        })
        .collect();

    for (i, c) in output.chars().enumerate() {
        if i > 0 && i % SITES_PER_SIDE == 0 {
            println!(); // Newline
        }
        print!("{c}");
    }
}
