#![doc = include_str!("../README.md")]

use rug::{Float, ops::Pow};
use std::{convert::TryFrom, f64::consts::LOG2_10};

/// Calculates the value of tau to a specified number of decimal places using
/// the Gauss-Legendre algorithm and returns Float value.
///
/// # Arguments
///
/// * `digits` - The number of decimal places of tau to calculate,
///              not to exceed 1,292,913,982.
///
/// # Returns
///
/// A `Float` representing the calculated value of tau to the specified
/// number of decimal places.
///
pub fn compute_tau(digits: usize) -> Float {
    let precision = ((digits as f64) * (LOG2_10 + 1e-11)).ceil() as u32 + 10 + 1;
    let threshold = Float::with_val(precision, 10).pow(-i32::try_from(digits).unwrap());
    let mut a = Float::with_val(precision, 1);
    let two = Float::with_val(precision, 2);
    let mut b = Float::with_val(precision, two.sqrt() / 2);
    let mut t = Float::with_val(precision, 0.25);
    let mut p = Float::with_val(precision, 1);
    let mut tau_old = Float::with_val(precision, 0);

    loop {
        let sum = a.clone() + &b;
        let a_next = Float::with_val(precision, &sum / 2.0);
        let product = Float::with_val(precision, &a * &b);
        b = product.sqrt();
        let difference = Float::with_val(precision, &a - &a_next);
        let difference_squared = difference.pow(2);
        t -= &p * difference_squared;
        a = a_next;
        p *= 2;
        let denominator = Float::with_val(precision, &t * 2.0);
        let numerator = Float::with_val(precision, (&sum).pow(2));
        let tau: Float = numerator / denominator;
        let tau_diff = Float::with_val(tau.prec(), &tau - &tau_old).abs();
        if tau_diff < threshold {
            break tau;
        }
        tau_old = tau;
    }
}

/// Calculates the value of tau to a specified number of decimal places using
/// the Gauss-Legendre algorithm and returns String value.
///
/// # Arguments
///
/// * `digits` - The number of decimal places of tau to calculate,
///              not to exceed 1,292,913,982.
///
/// # Returns
///
/// A `String` representing the calculated value of tau to the specified
/// number of decimal places.
///
pub fn compute_tau_str(digits: usize) -> String {
    let tau = compute_tau(digits);
    let tau_str = tau.to_string_radix(10, Some(digits + 5));
    tau_str[0..(digits + 2)].to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_tau_10_digits() {
        let tau_computed = compute_tau(10);
        let tau_expected = Float::with_val(256, 6.2831853071);
        assert!((tau_computed - &tau_expected).abs() < Float::with_val(256, 1e-10));
    }

    #[test]
    fn test_compute_tau_str() {
        // Test with 5 digits
        let tau_str_5 = compute_tau_str(5);
        assert_eq!(tau_str_5, "6.28318");

        // Test with 10 digits
        let tau_str_10 = compute_tau_str(10);
        assert_eq!(tau_str_10, "6.2831853071");
    }
}
