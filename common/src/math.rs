pub fn gcd<I: Into<i128>>(a: I, b: I) -> i128 {
    let a = a.into();
    let b = b.into();

    if b == 0 {
        a
    } else {
        gcd(b, a % b)
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
}
