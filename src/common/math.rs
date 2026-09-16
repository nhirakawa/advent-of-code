#![allow(unused)]

use std::{
    iter::Sum,
    ops::{Add, Div, Mul, Sub},
};

use log::trace;

pub fn gcd<I: Into<i128>>(a: I, b: I) -> i128 {
    let a = a.into();
    let b = b.into();

    if b == 0 { a } else { gcd(b, a % b) }
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

    if sorted.len().is_multiple_of(2) {
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

pub fn triangular_number<U>(nth: U) -> u64
where
    U: Into<usize> + From<usize>,
{
    let nth = nth.into();

    ((nth * (nth + 1)) / 2) as u64
}

pub fn manhattan_distance<U: Ord + Eq + Sub<Output = U> + Sum + Copy, const N: usize>(
    first: [U; N],
    second: [U; N],
) -> U {
    first
        .iter()
        .copied()
        .zip(second.iter().copied())
        .map(|(a, b)| std::cmp::max(a, b) - std::cmp::min(a, b))
        .sum()
}

pub fn squared_euclidean_distance<
    U: Ord + Eq + Sub<Output = U> + Mul<Output = U> + Sum + Copy,
    const N: usize,
>(
    first: [U; N],
    second: [U; N],
) -> U {
    std::iter::zip(first, second)
        .map(|(a, b)| {
            let diff = std::cmp::max(a, b) - std::cmp::min(a, b);
            diff * diff
        })
        .sum()
}

/// Taken from https://rustp.org/number-theory/modular-exponentiation/#program-for-modular-exponentiation-in-rust
/// Computes n^x % p
pub fn mod_pow(n: i128, x: i128, p: i128) -> i128 {
    let mut n = n;
    let mut x = x;

    // Initialize ans = 1
    let mut ans = 1;

    // x is 0, return 1
    if x <= 0 {
        return 1;
    }

    // use loop statement in rust for infinite loop
    loop {
        // Step 2. If x is 1, return (answer * n) % p
        if x == 1 {
            return (ans * n) % p;
        }

        // Step 3. If x > 1 and even, change n to n^2, change x to x/2, and go to step 2

        // for checking if x is even, we check the LSB. is 0 or 1
        // Alternatively, we can also check x%2, but this is more efficient
        if x & 1 == 0 {
            n = (n * n) % p;
            x >>= 1; // or x = x/2
            continue;
        }
        // Step 4. If X > 1 and odd, multiply answer by n and store answer modulo p,
        // and reduce x to x-1 and go to step 2.
        else {
            ans = (ans * n) % p;
            x -= 1;
        }
    }
}

fn extended_gcd(a: i128, b: i128) -> (i128, i128, i128) {
    if a == 0 {
        (b, 0, 1)
    } else {
        let (g, x, y) = extended_gcd(b % a, a);
        (g, y - (b / a) * x, x)
    }
}

pub fn mod_inverse(a: i128, m: i128) -> Option<i128> {
    let (g, x, _) = extended_gcd(a, m);
    if g != 1 {
        None // Inverse does not exist
    } else {
        Some((x % m + m) % m)
    }
}

pub mod geom {
    use anyhow;
    use num_traits::Bounded;

    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
    pub struct Coord<T, const N: usize> {
        coords: [T; N],
    }

    impl<T, const N: usize> AsRef<[T; N]> for Coord<T, N> {
        fn as_ref(&self) -> &[T; N] {
            &self.coords
        }
    }

    impl<T, const N: usize> std::fmt::Display for Coord<T, N>
    where
        T: std::fmt::Display,
    {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let mut iter = self.coords.iter();

            write!(f, "(")?;
            if let Some(first) = iter.next() {
                write!(f, "{}", first)?;
                for coord in iter {
                    write!(f, ", {}", coord)?;
                }
            }
            write!(f, ")")
        }
    }

    impl<T> Coord<T, 2>
    where
        T: Copy,
    {
        #[inline(always)]
        pub const fn new(x: T, y: T) -> Self {
            Self { coords: [x, y] }
        }

        #[inline(always)]
        pub const fn x(&self) -> T {
            self.coords[0]
        }

        #[inline(always)]
        pub const fn y(&self) -> T {
            self.coords[1]
        }
    }

    impl<T> Coord<T, 3>
    where
        T: Copy,
    {
        #[inline(always)]
        pub const fn new(x: T, y: T, z: T) -> Self {
            Self { coords: [x, y, z] }
        }

        #[inline(always)]
        pub const fn x(&self) -> T {
            self.coords[0]
        }

        #[inline(always)]
        pub const fn y(&self) -> T {
            self.coords[1]
        }

        #[inline(always)]
        pub const fn z(&self) -> T {
            self.coords[2]
        }
    }

    impl<T, const N: usize> From<[T; N]> for Coord<T, N>
    where
        T: Copy,
    {
        fn from(coords: [T; N]) -> Self {
            Self { coords }
        }
    }

    impl<T, const N: usize> From<Coord<T, N>> for [T; N]
    where
        T: Copy,
    {
        fn from(coord: Coord<T, N>) -> Self {
            coord.coords
        }
    }

    impl<T> From<(T, T)> for Coord<T, 2>
    where
        T: Copy,
    {
        fn from((x, y): (T, T)) -> Self {
            Self::new(x, y)
        }
    }

    impl<T> From<(T, T, T)> for Coord<T, 3>
    where
        T: Copy,
    {
        fn from((x, y, z): (T, T, T)) -> Self {
            Self::new(x, y, z)
        }
    }

    impl<T> std::str::FromStr for Coord<T, 2>
    where
        T: std::str::FromStr + Copy,
        T::Err: std::error::Error + Send + Sync + 'static,
    {
        type Err = anyhow::Error;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let (x, y) = s
                .split_once(',')
                .ok_or_else(|| anyhow::anyhow!("Invalid coordinate: {}", s))?;
            let x = x
                .trim()
                .parse()
                .map_err(|_| anyhow::anyhow!("Failed to parse x coordinate"))?;
            let y = y
                .trim()
                .parse()
                .map_err(|_| anyhow::anyhow!("Failed to parse y coordinate"))?;
            Ok(Self::new(x, y))
        }
    }

    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub struct BoundingBox<T, const N: usize> {
        min: Coord<T, N>,
        max: Coord<T, N>,
    }

    impl<T, const N: usize> BoundingBox<T, N>
    where
        T: Ord + Copy + Bounded,
    {
        pub fn from_points<I>(points: I) -> Self
        where
            I: IntoIterator<Item = Coord<T, N>>,
            T: num_traits::One + std::ops::Add<Output = T> + std::ops::Sub<Output = T>,
        {
            // Initialize with inverted bounds (empty box where min > max)
            let mut min = [T::max_value(); N];
            let mut max = [T::min_value(); N];

            for point in points {
                for i in 0..N {
                    min[i] = std::cmp::min(min[i], point.coords[i]);
                    max[i] = std::cmp::max(max[i], point.coords[i]);
                }
            }

            // Apply padding of 1 in all directions (only if box is not inverted/empty)
            let is_valid = (0..N).all(|i| min[i] <= max[i]);
            if is_valid {
                let padding = T::one();
                for i in 0..N {
                    min[i] = min[i] - padding;
                    max[i] = max[i] + padding;
                }
            }

            Self {
                min: Coord { coords: min },
                max: Coord { coords: max },
            }
        }

        pub const fn new(min: Coord<T, N>, max: Coord<T, N>) -> Self {
            Self { min, max }
        }

        pub fn contains(&self, point: Coord<T, N>) -> bool {
            for i in 0..N {
                if point.coords[i] < self.min.coords[i] || point.coords[i] > self.max.coords[i] {
                    return false;
                }
            }
            true
        }

        pub const fn min(&self) -> Coord<T, N> {
            self.min
        }

        pub const fn max(&self) -> Coord<T, N> {
            self.max
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_coord_2d_construction() {
            let c = Coord::<i32, 2>::new(5, 10);
            assert_eq!(c.x(), 5);
            assert_eq!(c.y(), 10);
        }

        #[test]
        fn test_coord_from_array() {
            let c: Coord<i32, 2> = [3, 7].into();
            assert_eq!(c.x(), 3);
            assert_eq!(c.y(), 7);
        }

        #[test]
        fn test_coord_3d() {
            let c = Coord::<i32, 3>::new(1, 2, 3);
            assert_eq!(c.x(), 1);
            assert_eq!(c.y(), 2);
            assert_eq!(c.z(), 3);
        }

        #[test]
        fn test_bbox_from_points_2d() {
            let points = vec![
                Coord::<i32, 2>::new(5, 10),
                Coord::<i32, 2>::new(1, 3),
                Coord::<i32, 2>::new(8, 7),
            ];
            let bbox = BoundingBox::from_points(points);
            // With padding of 1: min (1-1, 3-1) = (0, 2), max (8+1, 10+1) = (9, 11)
            assert_eq!(bbox.min(), Coord::<i32, 2>::new(0, 2));
            assert_eq!(bbox.max(), Coord::<i32, 2>::new(9, 11));
        }

        #[test]
        fn test_bbox_contains_inclusive() {
            let bbox = BoundingBox::new(Coord::<i32, 2>::new(0, 0), Coord::<i32, 2>::new(4, 4));
            // Corners (inclusive)
            assert!(bbox.contains(Coord::<i32, 2>::new(0, 0)));
            assert!(bbox.contains(Coord::<i32, 2>::new(4, 4)));
            // Interior
            assert!(bbox.contains(Coord::<i32, 2>::new(2, 2)));
            // Exterior
            assert!(!bbox.contains(Coord::<i32, 2>::new(-1, 2)));
            assert!(!bbox.contains(Coord::<i32, 2>::new(5, 2)));
        }

        #[test]
        fn test_bbox_from_empty() {
            let bbox = BoundingBox::<i32, 2>::from_points(vec![]);
            // Empty box has inverted bounds (min > max)
            assert_eq!(bbox.min(), Coord::<i32, 2>::new(i32::MAX, i32::MAX));
            assert_eq!(bbox.max(), Coord::<i32, 2>::new(i32::MIN, i32::MIN));
            // Should not contain any points
            assert!(!bbox.contains(Coord::<i32, 2>::new(0, 0)));
        }

        #[test]
        fn test_coord_parse() {
            use std::str::FromStr;
            let c = Coord::<isize, 2>::from_str("7,1").unwrap();
            assert_eq!(c.x(), 7);
            assert_eq!(c.y(), 1);
        }
    }
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

    #[test]
    fn test_triangular_number() {
        assert_eq!(triangular_number(0), 0);
        assert_eq!(triangular_number(1), 1);
        assert_eq!(triangular_number(2), 3);
        assert_eq!(triangular_number(3), 6);
        assert_eq!(triangular_number(4), 10);
        assert_eq!(triangular_number(5), 15);
    }

    #[test]
    fn test_modular_exponent() {
        assert_eq!(mod_pow(2, 100000, 1000000007), 607723520);
    }
}
