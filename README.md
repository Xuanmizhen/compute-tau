# compute-tau

Compute-tau calculates the value of tau to an arbitrary number of digits using the [Gauss–Legendre algorithm](https://en.wikipedia.org/wiki/Gauss%E2%80%93Legendre_algorithm). It computes 1 million digits within a couple of seconds on your PC.

## Installation

You can include this crate in your `Cargo.toml` file as follows:

```toml
[dependencies]
compute-tau = "0.1"
```

## Usage

To use the `compute_tau_str` function in your Rust code, add the following to your crate root:

```rust
use compute_tau::compute_tau_str;

fn main() {
    // Specify the number of digits of Tau you want to compute
    let digits = 100;

    // Compute Tau
    let tau = compute_tau_str(digits);

    // Print calculated decimal
    println!("Tau to {} decimal places: {}", digits, tau);
}
```

## Command Line Usage

You can also use the `compute-tau` command from the command line. After installing the crate with
```bash
cargo install compute-tau
```
run the following command:

```bash
compute-tau <digits>
```

Replace `<digits>` with the number of digits of Tau you want to compute. For example:

```bash
compute-tau 100
```

This will print the value of Tau to 100 decimal places.

## Performance

On a MacBook Air (Apple M1, 16 GB), tau to 1 million digits was computed in 1.5 seconds, and to 320 million digits in 24 minutes, but the calculation did not complete within 10 hours for 330 million digits. Similarly, on a Mac mini (Apple M1, 16 GB), tau to 320 million digits was computed, but the calculation did not complete within 10 hours for 330 million digits. It is presumed that the calculation is taking a long time due to memory swapping, as it does not end in a panic due to memory allocation failure but instead continues indefinitely. Since both machines yielded the same result, it is considered that 320 million digits is the maximum number of digits that can be computed using compute-tau with 16 GB of memory. Another limitation is that the digit cannot exceed 1,292,913,983 due to the precision of [rug::Float](https://docs.rs/rug/latest/rug/struct.Float.html) being defined as u32. The computed result of tau to 320 million digits using compute-tau was confirmed to match exactly with the results obtained using [y-cruncher](http://www.numberworld.org/y-cruncher/) and the Chudnovsky algorithm.

## License

This crate is licensed under the [MIT license](https://en.wikipedia.org/wiki/MIT_License).
