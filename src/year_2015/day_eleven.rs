use anyhow::bail;
use model::Password;

pub fn part_one(input: &str) -> anyhow::Result<impl ToString> {
    let password = Password::from_str(input)?;

    next_password(password)
}

pub fn part_two(input: &str) -> anyhow::Result<impl ToString> {
    let password = Password::from_str(input)?;
    next_password(password).and_then(next_password)
}

fn next_password(password: Password) -> anyhow::Result<Password> {
    let mut password = password;

    while let Ok(next_password) = password.increment() {
        password = next_password;

        if password.has_increasing_straight()
            && password.has_no_forbidden_letters()
            && password.has_distinct_pairs()
        {
            return Ok(password);
        }
    }

    bail!("No valid password found")
}

mod model {
    use std::{
        collections::HashSet,
        fmt::{self, Display, Formatter},
    };

    use anyhow::bail;
    use itertools::Itertools;

    pub struct Password([u8; 8]);

    impl Password {
        pub fn from_str(s: &str) -> anyhow::Result<Password> {
            if s.len() != 8 {
                bail!("Password {} is not 8 characters long", s);
            }

            let mut password = [0; 8];
            for (i, c) in s.bytes().enumerate() {
                password[i] = c;
            }

            Ok(Password(password))
        }

        pub fn increment(&self) -> anyhow::Result<Password> {
            if self.0 == [b'z'; 8] {
                bail!("Password is already at maximum value");
            }

            let mut password = self.0;
            let mut i = 7;
            loop {
                if password[i] == b'z' {
                    password[i] = b'a';
                    i -= 1;
                } else {
                    password[i] += 1;
                    break;
                }
            }

            Ok(Password(password))
        }

        pub fn has_increasing_straight(&self) -> bool {
            self.0
                .iter()
                .tuple_windows()
                .any(|(a, b, c)| *b == a + 1 && *c == b + 1)
        }

        pub fn has_no_forbidden_letters(&self) -> bool {
            !self.0.iter().any(|&c| c == b'i' || c == b'o' || c == b'l')
        }

        pub fn has_distinct_pairs(&self) -> bool {
            let mut pair_chars = HashSet::new();

            for (a, b) in self.0.iter().tuple_windows() {
                if a == b {
                    pair_chars.insert(*a);
                }
            }

            pair_chars.len() >= 2
        }
    }

    impl Display for Password {
        fn fmt(&self, f: &mut Formatter) -> fmt::Result {
            for &c in &self.0 {
                write!(f, "{}", c as char)?;
            }
            Ok(())
        }
    }
}
