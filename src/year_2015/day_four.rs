use anyhow::bail;
use rayon::iter::{IntoParallelIterator, ParallelIterator};

pub fn part_one(input: &str) -> anyhow::Result<impl ToString> {
    find_hash_with_leading_zeros_parallel(input, &LeadingZeros::Five)
}

pub fn part_two(input: &str) -> anyhow::Result<impl ToString> {
    find_hash_with_leading_zeros_parallel(input, &LeadingZeros::Six)
}

fn find_hash_with_leading_zeros_parallel(
    input: &str,
    leading_zeros: &LeadingZeros,
) -> anyhow::Result<String> {
    let number_of_threads = 8;
    let single_thread_batch_size = 100_000;

    for i in (0..).step_by(number_of_threads) {
        let results = (0..number_of_threads)
            .into_par_iter()
            .map(|j| {
                let start = (i + j) * single_thread_batch_size;
                let end = start + single_thread_batch_size;
                start..end
            })
            .map(|numbers| find_hash_with_leading_zeros(input, leading_zeros, numbers))
            .collect::<Vec<_>>();

        if let Some(i) = results.into_iter().flatten().next() {
            return Ok(i.to_string());
        }
    }

    bail!("No solution found")
}

fn find_hash_with_leading_zeros<I: IntoIterator<Item = usize>>(
    input: &str,
    leading_zeros: &LeadingZeros,
    numbers: I,
) -> anyhow::Result<usize> {
    for i in numbers {
        let digest = md5::compute(format!("{}{}", input, i)).0;
        if leading_zeros.matches(&digest) {
            return Ok(i);
        }
    }

    bail!("No solution found")
}

#[derive(Debug, Copy, Clone)]
enum LeadingZeros {
    Five,
    Six,
}

impl LeadingZeros {
    fn matches(&self, digest: &[u8; 16]) -> bool {
        match self {
            LeadingZeros::Five => digest.starts_with(&[0, 0]) && digest[2] < 16,
            LeadingZeros::Six => digest.starts_with(&[0, 0, 0]),
        }
    }
}
