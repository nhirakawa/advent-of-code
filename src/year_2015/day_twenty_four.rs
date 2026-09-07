use anyhow::{anyhow, bail};
use itertools::Itertools;

pub fn part_one(input: &str) -> anyhow::Result<impl ToString> {
    let weights: anyhow::Result<Vec<usize>> = input
        .lines()
        .map(|line| line.parse::<usize>().map_err(anyhow::Error::from))
        .collect();

    let weights = weights?;

    let sum = weights.iter().sum::<usize>();

    if sum % 3 != 0 {
        bail!("Cannot evenly divide {sum} into 3 equal-weight buckets");
    }

    Err::<usize, _>(anyhow!("Not implemented"))
}

pub fn part_two(_input: &str) -> anyhow::Result<impl ToString> {
    Err::<usize, _>(anyhow!("Not implemented"))
}

fn partition_into_buckets(
    weights: &[usize],
    number_of_buckets: usize,
) -> anyhow::Result<Vec<Vec<usize>>> {
    if weights.is_empty() {
        bail!("Must have at least one element");
    }

    if number_of_buckets == 0 {
        bail!("Must have at least one bucket");
    }

    if weights.len() < number_of_buckets {
        bail!("Number of buckets is too small");
    }

    let total_weight = weights.iter().sum::<usize>();

    if total_weight % number_of_buckets != 0 {
        bail!("Cannot even divide {total_weight} into {number_of_buckets} buckets");
    }

    let target_bucket_weight = total_weight / number_of_buckets;

    let sorted_weights = weights.iter().sorted().copied().collect_vec();

    let mut buckets = vec![Vec::new(); number_of_buckets];

    fn backtrack(
        index: usize,
        weights: &[usize],
        target_weight: usize,
        buckets: &mut [Vec<usize>],
    ) -> bool {
        let number_of_buckets = buckets.len();

        // base case
        if index == weights.len() {
            return buckets
                .iter()
                .all(|bucket| bucket.iter().sum::<usize>() == target_weight);
        }

        let value = weights[index];

        // try to place the weight
        for i in 0..number_of_buckets {
            let current_bucket_sum = buckets[i].iter().sum::<usize>();
            if current_bucket_sum + value > target_weight {
                // Adding the current weight would over-fill the bucket
                continue;
            }

            if i > 0 && buckets[i - 1].iter().sum::<usize>() == current_bucket_sum {
                continue;
            }

            buckets[i].push(value);

            if backtrack(index + 1, weights, target_weight, buckets) {
                return true;
            }

            buckets[i].pop();

            if current_bucket_sum == 0 {
                break;
            }
        }

        todo!()
    }

    if backtrack(0, &sorted_weights, target_bucket_weight, &mut buckets) {
        Ok(buckets)
    } else {
        bail!("No solution found")
    }
}
