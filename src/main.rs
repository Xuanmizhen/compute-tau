use compute_tau::compute_tau_str;
use std::env;

/// The main function of the program. It parses the command-line arguments
/// to determine the number of decimal places of tau to calculate and then
/// prints the calculated value to the standard output.
///
/// # Arguments
///
/// This function expects a single command-line argument:
///
/// * The first argument should be the number of decimal places of tau to calculate.
///
/// # Panics
///
/// This function will panic if the command-line arguments are not provided
/// as expected or if the provided argument cannot be parsed into a `usize`.
///
/// # Examples
///
/// Run the program from the command line:
///
/// ```shell
/// compute-tau 100
/// ```
///
/// This will calculate and print the value of tau to 100 decimal places.
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <digits>", args[0]);
        return;
    }
    let digits: usize = args[1].parse().expect("Please provide a valid number of digits.");
    if digits > 1_292_913_983 {
        eprintln!("Digits should not exceed 1,292,913,983.");
        return;
    }
    let tau = compute_tau_str(digits);
    println!("tau to {} decimal places: {}", digits, tau);
}
