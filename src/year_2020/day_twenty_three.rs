pub fn part_one(input: &str) -> anyhow::Result<impl ToString> {
    let labels = parse_labels(input)?;
    let mut cups = Cups::new(labels);

    for _ in 0..100 {
        cups.shuffle();
    }

    Ok(cups.get_label_string())
}

pub fn part_two(input: &str) -> anyhow::Result<impl ToString> {
    let labels = parse_labels(input)?;
    let mut cups = Vec::with_capacity(1_000_000);
    cups.extend(labels);
    cups.extend(10..=1_000_000);

    let mut cups = Cups::new(cups);

    for _i in 0..10_000_000 {
        cups.shuffle();
    }

    let first = cups.labels[1];
    let second = cups.labels[first];

    Ok(first * second)
}

fn parse_labels(input: &str) -> anyhow::Result<Vec<Label>> {
    let labels = input
        .chars()
        .map(|c| c.to_digit(10).map(|d| d as usize))
        .collect::<Option<Vec<Label>>>()
        .ok_or(anyhow::anyhow!("Failed to parse labels"))?;

    Ok(labels)
}

type Label = usize;
struct Cups {
    labels: Vec<Label>,
    current: Label,
}

impl Cups {
    fn new(labels: Vec<Label>) -> Cups {
        let current = labels[0];

        let mut linked_list = vec![0; labels.len() + 1];

        for i in 0..labels.len() {
            let current = labels[i];
            let next = if i + 1 == labels.len() { 0 } else { i + 1 };
            let next = labels[next];

            linked_list[current] = next;
        }

        Cups {
            labels: linked_list,
            current,
        }
    }

    fn shuffle(&mut self) {
        let first = self.labels[self.current];
        let second = self.labels[first];
        let third = self.labels[second];

        let mut destination = if self.current == 1 {
            self.labels.len() - 1
        } else {
            self.current - 1
        };

        while destination == first || destination == second || destination == third {
            destination = if destination == 1 {
                self.labels.len() - 1
            } else {
                destination - 1
            }
        }

        let destination_next = self.labels[destination];
        self.labels[self.current] = self.labels[third];
        self.labels[destination] = first;
        self.labels[third] = destination_next;

        self.current = self.labels[self.current];
    }

    fn get_label_string(&self) -> String {
        let mut output: Vec<String> = Vec::new();
        let mut current = self.labels[1];

        while current != 1 {
            output.push(current.to_string());
            current = self.labels[current];
        }

        output.join("")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shuffle() {
        let mut cups = Cups::new(vec![3, 8, 9, 1, 2, 5, 4, 6, 7]); // initial

        cups.shuffle(); // move 1

        assert_eq!(cups.get_label_string(), "54673289".to_string()); // after move 1

        cups.shuffle(); // move 2

        assert_eq!(cups.get_label_string(), "32546789".to_string()); // after move 2

        cups.shuffle(); // move 3

        assert_eq!(cups.get_label_string(), "34672589".to_string()); // after move 3
    }

    #[test]
    fn test_label_string() {
        let cups = Cups::new(vec![3, 8, 9, 1, 2, 5, 4, 6, 7]);

        assert_eq!(cups.get_label_string(), "25467389".to_string());
    }

    #[test]
    fn test_shuffle_ten_times() {
        let mut cups = Cups::new(vec![3, 8, 9, 1, 2, 5, 4, 6, 7]);

        for _ in 0..10 {
            cups.shuffle();
        }

        assert_eq!(cups.get_label_string(), "92658374".to_string());
    }

    #[test]
    fn test_shuffle_one_hundred_times() {
        let mut cups = Cups::new(vec![3, 8, 9, 1, 2, 5, 4, 6, 7]);

        for _ in 0..100 {
            cups.shuffle();
        }

        assert_eq!(cups.get_label_string(), "67384529");
    }

    #[test]
    fn test_ten_million_shuffles_slow() {
        let input = vec![3, 8, 9, 1, 2, 5, 4, 6, 7];

        let mut labels = Vec::with_capacity(1_000_000);
        labels.extend(input);
        labels.extend(10..=1_000_000);

        let mut cups = Cups::new(labels);

        for _ in 0..10_000_000 {
            cups.shuffle();
        }

        let first = cups.labels[1];
        let second = cups.labels[first];

        assert_eq!(first, 934001);
        assert_eq!(second, 159792);
    }
}
