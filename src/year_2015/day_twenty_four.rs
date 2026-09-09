use anyhow::bail;
use itertools::Itertools;

pub fn part_one(input: &str) -> anyhow::Result<impl ToString> {
    let weights: anyhow::Result<Vec<usize>> = input
        .lines()
        .map(|line| line.parse::<usize>().map_err(anyhow::Error::from))
        .collect();

    let weights = weights?;

    let buckets = partition_into_buckets(&weights, 3)?;

    if buckets.is_empty() {
        bail!("empty buckets");
    }

    let quantum_entanglement = buckets.into_iter().map(|bucket| quantum_entanglement(&bucket)).min().unwrap();

    Ok(quantum_entanglement)
}

pub fn part_two(input: &str) -> anyhow::Result<impl ToString> {
    let weights: anyhow::Result<Vec<usize>> = input
        .lines()
        .map(|line| line.parse::<usize>().map_err(anyhow::Error::from))
        .collect();

    let weights = weights?;

    let buckets = partition_into_buckets(&weights, 4)?;

    if buckets.is_empty() {
        bail!("empty buckets");
    }

    let quantum_entanglement = buckets.into_iter().map(|bucket| quantum_entanglement(&bucket)).min().unwrap();

    Ok(quantum_entanglement)
}

fn quantum_entanglement(weights: &[usize]) -> usize {
    weights.iter().product()
}

/// Finds every possible "first group" (the smallest one) that can be split off
/// from `weights` such that the rest can be divided into `number_of_buckets - 1`
/// remaining groups of equal weight. Returns all such minimal-size groups.
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
    let indices = (0..sorted_weights.len()).collect_vec();

    let max_first_group_size = sorted_weights.len() - (number_of_buckets - 1);

    for size in 1..=max_first_group_size {
        let groups: Vec<Vec<usize>> = indices
            .iter()
            .copied()
            .combinations(size)
            .filter(|group_indices| {
                group_indices.iter().map(|&i| sorted_weights[i]).sum::<usize>()
                    == target_bucket_weight
            })
            .filter(|group_indices| {
                let remaining = indices
                    .iter()
                    .filter(|i| !group_indices.contains(i))
                    .map(|&i| sorted_weights[i])
                    .collect_vec();

                can_partition_into_equal_groups(
                    &remaining,
                    number_of_buckets - 1,
                    target_bucket_weight,
                )
            })
            .map(|group_indices| group_indices.iter().map(|&i| sorted_weights[i]).collect())
            .collect();

        if !groups.is_empty() {
            return Ok(groups);
        }
    }

    bail!("No solution found")
}

/// Checks whether `weights` can be divided into `number_of_buckets` groups
/// that each sum to `target_weight`.
fn can_partition_into_equal_groups(
    weights: &[usize],
    number_of_buckets: usize,
    target_weight: usize,
) -> bool {
    if number_of_buckets == 0 {
        return weights.is_empty();
    }

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

        false
    }

    backtrack(0, weights, target_weight, &mut buckets)
}
