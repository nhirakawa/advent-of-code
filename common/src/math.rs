use std::ops::{Add, Div};

use log::trace;

pub fn gcd<I: Into<i128>>(a: I, b: I) -> i128 {
    let a = a.into();
    let b = b.into();

    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

pub fn lcm<I: Into<i128>>(a: I, b: I) -> i128 {
    let a = a.into();
    let b = b.into();

    if a == 0 && b == 0 {
        return 0;
    }

    let gcd = gcd(a, b);

    (a * b) / gcd
}

pub fn median<I>(numbers: &[I]) -> f64
where
    I: Add<I> + Div<I> + Ord + Clone + Into<f64>,
{
    let mut sorted: Vec<I> = numbers.to_vec();
    sorted.sort();

    let midpoint = sorted.len() / 2;
    trace!("len {}, midpoint {}", sorted.len(), midpoint);

    if sorted.len() % 2 == 0 {
        (sorted[midpoint].clone().into() + sorted[midpoint - 1].clone().into()) / 2.0
    } else {
        sorted[midpoint].clone().into()
    }
}

pub fn average<I>(numbers: &[I]) -> f64
where
    I: Add<I, Output = I> + Div<I, Output = I> + Default + Into<f64> + Copy,
{
    let mut sum = I::default();
    let mut count = 0;
    for number in numbers {
        sum = sum + *number;
        count += 1;
    }

    sum.into() / count as f64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gcd() {
        assert_eq!(gcd(12, 8), 4);
        assert_eq!(gcd(1071, 462), 21);
        assert_eq!(gcd(7, 4), 1);
    }

    #[test]
    fn test_lcm() {
        assert_eq!(lcm(21, 6), 42);
        assert_eq!(lcm(lcm(8, 9), 21), 504);
    }

    #[test]
    fn test_median() {
        let error_margin = f64::EPSILON;
        assert!((median(&[1, 2, 3]) - 2.0) < error_margin);
        assert!((median(&[1, 2, 3, 4]) - 2.5) < error_margin);
    }

    #[test]
    fn test_average() {
        let error_margin = f64::EPSILON;
        assert!((average(&[4, 5, 6]) - 5.0) < error_margin);
        assert!((average(&[16, 1, 2, 0, 4, 2, 7, 1, 2, 14]) - 4.9) < error_margin);
    }
}
