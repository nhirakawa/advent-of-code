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
}
