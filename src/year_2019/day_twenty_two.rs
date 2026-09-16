use anyhow::{anyhow, bail};

pub fn part_one(input: &str) -> anyhow::Result<impl ToString> {
    let shuffle_fn = compose_shuffle_fns(input, 10_007)?;
    Ok(shuffle_fn.call(2019))
}

pub fn part_two(_input: &str) -> anyhow::Result<impl ToString> {
    Err::<usize, _>(anyhow!("Not implemented"))
}

fn compose_shuffle_fns(s: &str, deck_size: i128) -> anyhow::Result<ShuffleFn> {
    let fns = s
        .lines()
        .map(|line| ShuffleFn::parse(line, deck_size))
        .collect::<anyhow::Result<Vec<_>>>()?;
    let mut iter = fns.into_iter();
    let first = iter.next().ok_or(anyhow!("No composed functions"))?;
    iter.try_fold(first, |accumulator, element| accumulator.compose(element))
}

struct ShuffleFn {
    a: i128,
    b: i128,
    deck_size: i128,
}

impl ShuffleFn {
    fn new(a: i128, b: i128, deck_size: i128) -> Self {
        Self { a, b, deck_size }
    }

    fn parse(s: &str, deck_size: i128) -> anyhow::Result<Self> {
        if let Some(cut) = s.strip_prefix("cut ") {
            let cut = cut.parse::<i128>()?;
            Ok(Self::new(1, -cut, deck_size))
        } else if let Some(increment) = s.strip_prefix("deal with increment ") {
            let increment = increment.parse()?;
            Ok(Self::new(increment, 0, deck_size))
        } else if s == "deal into new stack" {
            Ok(Self::new(-1, -1, deck_size))
        } else {
            bail!("Invalid shuffle: {s}")
        }
    }

    fn call(&self, n: i128) -> i128 {
        (self.a * n + self.b).rem_euclid(self.deck_size)
    }

    /// Returns a ShuffleFn of the form other(self(x))
    fn compose(self, other: Self) -> anyhow::Result<Self> {
        let Self {
            a: old_a,
            b: old_b,
            deck_size: old_deck_size,
        } = self;
        let Self {
            a: other_a,
            b: other_b,
            deck_size: other_deck_size,
        } = other;

        if old_deck_size != other_deck_size {
            bail!(
                "self.deck_size ({old_deck_size}) is not compatible with other.deck_size ({other_deck_size})"
            );
        }

        // old_deck_size and other_deck_size are guaranteed to be equal, so pick either one
        let new_a = (old_a * other_a).rem_euclid(old_deck_size);
        let new_b = (other_a * old_b + other_b).rem_euclid(old_deck_size);

        Ok(Self::new(new_a, new_b, old_deck_size))
    }
}
