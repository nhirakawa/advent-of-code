use anyhow::anyhow;
use itertools::Itertools;
use std::collections::HashMap;
use std::str::FromStr;

pub fn part_one(input: &str) -> anyhow::Result<impl ToString> {
    let graph = Graph::from_str(input)?;
    Ok(graph.count_paths("you", "out"))
}

pub fn part_two(input: &str) -> anyhow::Result<impl ToString> {
    let graph = Graph::from_str(input)?;

    let svr_dac_fft_out = graph.count_paths("svr", "dac")
        * graph.count_paths("dac", "fft")
        * graph.count_paths("fft", "out");

    let svr_fft_dac_out = graph.count_paths("svr", "fft")
        * graph.count_paths("fft", "dac")
        * graph.count_paths("dac", "out");

    Ok(svr_dac_fft_out + svr_fft_dac_out)
}

#[derive(Debug, Clone, Default)]
struct Graph {
    graph: HashMap<String, Vec<String>>,
}

impl Graph {
    fn new() -> Self {
        Default::default()
    }

    fn add_edge(&mut self, a: &str, b: &str) {
        self.graph
            .entry(a.to_string())
            .or_insert_with(Vec::new)
            .push(b.to_string());
    }

    fn count_paths<S: ToString>(&self, source: S, target: &str) -> usize {
        let mut cache = HashMap::new();
        self.count_paths_memoized(&source.to_string(), target, &mut cache)
    }

    fn count_paths_memoized(
        &self,
        current: &str,
        target: &str,
        cache: &mut HashMap<String, usize>,
    ) -> usize {
        // Check cache first
        if let Some(&count) = cache.get(current) {
            return count;
        }

        // Base case: we've reached the target
        if current == target {
            return 1;
        }

        // Recursive case: sum paths through all neighbors
        let count = if let Some(outputs) = self.graph.get(current) {
            outputs
                .iter()
                .map(|neighbor| self.count_paths_memoized(neighbor, target, cache))
                .sum()
        } else {
            0
        };

        // Cache the result
        cache.insert(current.to_string(), count);
        count
    }
}

impl FromStr for Graph {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut graph = Graph::new();

        for line in s.lines() {
            if line.is_empty() {
                continue;
            }

            let (input, outputs) = line
                .split_once(": ")
                .ok_or_else(|| anyhow!("Could not split {s} on input/outputs"))?;

            let outputs = outputs.split(' ').collect_vec();

            for output in outputs {
                graph.add_edge(input, output);
            }
        }

        Ok(graph)
    }
}
