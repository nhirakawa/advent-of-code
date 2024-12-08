use std::collections::HashMap;

pub fn part_one(input: &str) -> anyhow::Result<String> {
    let password = (0..)
        .map(|u| format!("{input}{u}"))
        .map(|content| md5::compute(content).0)
        .filter(|hash| hash_starts_with_zeroes(hash))
        .take(8)
        .map(|hash| extract_sixth_hex_digit(&hash))
        .fold(String::new(), |acc, digit| acc + &format!("{digit:x}"));

    Ok(password)
}

pub fn part_two(input: &str) -> anyhow::Result<String> {
    let mut password = HashMap::new();

    for index in 0.. {
        let content = format!("{input}{index}");
        let hash = md5::compute(content).0;

        if hash_starts_with_zeroes(&hash) {
            let position = extract_sixth_hex_digit(&hash) as usize;

            if position < 8 && !password.contains_key(&position) {
                let digit = format!("{:x}", hash[3] >> 4);
                password.insert(position, digit);

                if password.len() == 8 {
                    break;
                }
            }
        }
    }

    let password = (0..8)
        .map(|index| password[&index].clone())
        .collect::<String>();

    Ok(password)
}

fn hash_starts_with_zeroes(hash: &[u8]) -> bool {
    hash.starts_with(&[0, 0]) && hash[2] < 16
}

fn extract_sixth_hex_digit(hash: &[u8; 16]) -> u8 {
    hash[2] & 0x0F
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_hash_starts_with_zeroes() {
        assert!(hash_starts_with_zeroes(&*md5::compute("abc3231929")));
        assert!(hash_starts_with_zeroes(&*md5::compute("abc5017308")));
        assert!(hash_starts_with_zeroes(&*md5::compute("abc5278568")));
    }

    #[test]
    fn test_extract_sixth_hex_digit() {
        assert_eq!(extract_sixth_hex_digit(&*md5::compute("abc3231929")), 1);
        assert_eq!(extract_sixth_hex_digit(&*md5::compute("abc5017308")), 8);
        assert_eq!(extract_sixth_hex_digit(&*md5::compute("abc5278568")), 0xf);
    }
}
