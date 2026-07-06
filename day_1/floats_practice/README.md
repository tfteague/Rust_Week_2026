# Day 1 — Integer vs. float arithmetic

Two warm-up questions on Rust's numeric types: integer division/remainder vs.
floating-point math, casting, powers, and formatted printing.

## Run

```bash
cargo run
```

## The questions

**Question 1 — break a duration into weeks/days/hours.** With unsigned integers,
`/` truncates toward zero and `%` gives the remainder, so a repeated
divide-and-remainder peels off each unit in turn. `question1_float` (marked
`#[allow(dead_code)]`) does the same thing with `f32` and `.trunc()`.

**Question 2 — evaluate `x² + y² − z²`.** Computed twice: once with integer
`.pow(2)`, once with float `.powi(2)` and `{:.2}` formatting. The two answers
**disagree** — the squares (~1.8 × 10⁷) are larger than the largest integer an
`f32` can hold exactly (2²⁴ ≈ 1.6 × 10⁷), so the float version rounds and lands
on `2.00` instead of the exact `1`. A tidy demonstration of floating-point
precision loss.

## Expected output

```
Philipp has waited 2 weeks, 5 days, and 7 hours
Int: 1
Float: 2.00
```
