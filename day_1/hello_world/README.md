# Day 1 — Hello, world (functions & bindings)

The smallest possible Rust program: a `main`, a `let` binding, a function that
takes an argument and returns a value, and `println!` string interpolation.

## Run

```bash
cargo run
```

## The code

`plus_one` takes an `i32` and returns an `i32`. The last expression in a Rust
function is its return value — no `return` keyword and no trailing semicolon:

```rust
fn plus_one(x: i32) -> i32 {
    x + 1        // no semicolon: this *is* the return value
}
```

`main` binds the result to `y` and prints it with `{y}` interpolation:

```rust
let n = 5;
let y = plus_one(n);
println!("y is: {y}");
```

## Expected output

```
y is: 6
```
