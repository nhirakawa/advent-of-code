use anyhow::anyhow;

pub fn part_one(input: &str) -> anyhow::Result<String> {
    let container_sizes = parse(input)?;

    let count = count_containers(0, vec![], &container_sizes, 150).len();

    Ok(count.to_string())
}

pub fn part_two(input: &str) -> anyhow::Result<String> {
    let container_sizes = parse(input)?;

    let count = count_containers(0, vec![], &container_sizes, 150);

    let min_containers = count.iter().map(|c| c.len()).min().ok_or(anyhow!(
        "No containers found that can fill the target amount of liters"
    ))?;

    let count = count.iter().filter(|c| c.len() == min_containers).count();

    Ok(count.to_string())
}

/// Returns the number of ways to fill the target amount of containers with the given container sizes.
/// The index is the index of the current container size being considered.
/// The chosen_containers is the number of containers that have been chosen so far.
/// The container_sizes is the list of container sizes as a usize - each bit represents whether the container at that index has been chosen.
/// The target is the amount of liters that need to be filled.
fn count_containers(
    index: usize,
    chosen_containers: Vec<usize>,
    container_sizes: &[u8],
    target: u8,
) -> Vec<Vec<usize>> {
    if target == 0 {
        return vec![chosen_containers];
    }

    if index >= container_sizes.len() {
        return vec![];
    }

    let mut filled_containers = vec![];

    filled_containers.extend(count_containers(
        index + 1,
        chosen_containers.clone(),
        container_sizes,
        target,
    ));

    if container_sizes[index] <= target {
        let mut chosen_containers = chosen_containers.clone();
        chosen_containers.push(index);

        filled_containers.extend(count_containers(
            index + 1,
            chosen_containers,
            container_sizes,
            target - container_sizes[index],
        ));
    }

    filled_containers
}

fn parse(input: &str) -> anyhow::Result<Vec<u8>> {
    input
        .lines()
        .map(|line| line.parse::<u8>().map_err(Into::into))
        .collect::<anyhow::Result<Vec<_>>>()
}
