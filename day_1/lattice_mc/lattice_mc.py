import math
import random
import sys

# Simulation parameters (kept identical to the Rust version in
# lattice_mc/src/main.rs so the two can be benchmarked head-to-head).
SITES_PER_SIDE = 10
N_SWEEPS = 1_000_000
N_SITES = SITES_PER_SIDE * SITES_PER_SIDE
TEMPERATURE = 5.0  # T, in units of epsilon/k_B
ACTIVITY = 1.0


def delta_energy(lattice_state, site_id):
    """ΔE for flipping the current spin: (2*current - 1) * (neighbor_sum + C)."""
    row = site_id // SITES_PER_SIDE

    right_id = row * SITES_PER_SIDE + (site_id + 1) % SITES_PER_SIDE
    left_id = row * SITES_PER_SIDE + (site_id + SITES_PER_SIDE - 1) % SITES_PER_SIDE
    above_id = (site_id + N_SITES - SITES_PER_SIDE) % N_SITES
    below_id = (site_id + SITES_PER_SIDE) % N_SITES

    neighbor_sum = (
        lattice_state[right_id]
        + lattice_state[left_id]
        + lattice_state[above_id]
        + lattice_state[below_id]
    )

    sign = 2.0 * lattice_state[site_id] - 1.0  # -1 if empty, +1 if filled
    return sign * (neighbor_sum + ACTIVITY)


def log_state(lattice_state):
    """Print the lattice as a grid of filled/empty cells."""
    if hasattr(sys.stdout, "reconfigure"):
        sys.stdout.reconfigure(encoding="utf-8")
    for i, x in enumerate(lattice_state):
        if i > 0 and i % SITES_PER_SIDE == 0:
            print()
        print('\N{WHITE LARGE SQUARE}' if x == 0 else '\N{LARGE BLUE SQUARE}', end='')
    print()


def main():
    random.seed(42)
    lattice_state = [0] * N_SITES

    for _ in range(N_SWEEPS):
        for _ in range(N_SITES):
            site = random.randrange(N_SITES)
            delta = delta_energy(lattice_state, site)
            accept = delta <= 0.0 or random.random() < math.exp(-delta / TEMPERATURE)

            if accept:
                lattice_state[site] = (lattice_state[site] + 1) % 2

    log_state(lattice_state)


if __name__ == "__main__":
    main()
