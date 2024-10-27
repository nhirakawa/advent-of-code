pub fn part_one(input: &str) -> anyhow::Result<String> {
    let digits = parse(input);
    Ok(sum_similar_digits(&digits, 1).to_string())
}

pub fn part_two(input: &str) -> anyhow::Result<String> {
    let digits = parse(input);
    Ok(sum_similar_digits(&digits, digits.len() / 2).to_string())
}

fn sum_similar_digits(digits: &[u32], step: usize) -> u32 {
    let mut sum = 0;

    for index in 0..digits.len() {
        let first = digits[index];
        let second = digits[(index + step) % digits.len()];

        if first == second {
            sum += first;
        }
    }

    sum
}

fn parse(i: &str) -> Vec<u32> {
    i.chars().filter_map(|c| c.to_digit(10)).collect()
}
