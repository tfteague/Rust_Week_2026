"""Cross-check the Rust output with the reference `gsd` Python package.

    pip install gsd numpy
    python verify.py hex_diamond.gsd

Prints the positions/orientations stored in the file and independently
re-derives the bond distances and the relative-rotation axes/angles, so the
students' Rust results can be compared against a second implementation.
"""

import sys
import numpy as np
import gsd.hoomd

# The four bonds each particle emits: (neighbor_index, image) with image in
# units of the lattice vectors. Same topology used by the Rust program.
NEIGHBORS = [
    [(1, (0, 0, 0)), (3, (0, 0, -1)), (3, (0, 1, -1)), (3, (-1, 0, -1))],
    [(0, (0, 0, 0)), (2, (0, 0, 0)), (2, (0, 1, 0)), (2, (-1, 0, 0))],
    [(3, (0, 0, 0)), (1, (0, 0, 0)), (1, (0, -1, 0)), (1, (1, 0, 0))],
    [(2, (0, 0, 0)), (0, (0, 0, 1)), (0, (0, -1, 1)), (0, (1, 0, 1))],
]


def axis_angle(qj, qi):
    """Axis and angle of q_rel = q_j * q_i^{-1} (Hamilton product)."""
    def mul(a, b):
        w0, x0, y0, z0 = a
        w1, x1, y1, z1 = b
        return np.array([
            w0 * w1 - x0 * x1 - y0 * y1 - z0 * z1,
            w0 * x1 + x0 * w1 + y0 * z1 - z0 * y1,
            w0 * y1 - x0 * z1 + y0 * w1 + z0 * x1,
            w0 * z1 + x0 * y1 - y0 * x1 + z0 * w1,
        ])
    inv = np.array([qi[0], -qi[1], -qi[2], -qi[3]])
    q = mul(qj, inv)
    vnorm = np.linalg.norm(q[1:])
    angle = 2.0 * np.arctan2(vnorm, q[0])
    axis = q[1:] / vnorm if vnorm > 1e-12 else np.array([0.0, 0.0, 1.0])
    return axis, angle


def box_matrix(box):
    lx, ly, lz, xy, xz, yz = box
    return np.array([
        [lx, xy * ly, xz * lz],
        [0.0, ly, yz * lz],
        [0.0, 0.0, lz],
    ])


def main(path):
    with gsd.hoomd.open(path, mode="r") as traj:
        frame = traj[0]

    pos = np.asarray(frame.particles.position)
    ori = np.asarray(frame.particles.orientation)
    box = np.asarray(frame.configuration.box)
    L = box_matrix(box)

    print(f"N = {frame.particles.N}")
    print(f"box = {box}")
    print("\npositions:")
    for i, p in enumerate(pos):
        print(f"  P{i}: {np.round(p, 6)}")
    print("\norientations [w, x, y, z]:")
    for i, q in enumerate(ori):
        print(f"  P{i}: {np.round(q, 6)}")

    print("\nbond distances (should all be 1.0):")
    for i, bonds in enumerate(NEIGHBORS):
        for j, image in bonds:
            shift = L @ np.array(image, dtype=float)
            d = np.linalg.norm(pos[j] + shift - pos[i])
            print(f"  P{i}->P{j} {image}: {d:.6f}")

    print("\nrelative rotations from particle 0:")
    for j in range(1, len(ori)):
        axis, angle = axis_angle(ori[j], ori[0])
        print(f"  0->{j}: axis={np.round(axis, 6)}, angle={np.degrees(angle):.3f} deg")


if __name__ == "__main__":
    main(sys.argv[1] if len(sys.argv) > 1 else "hex_diamond.gsd")
