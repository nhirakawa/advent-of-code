use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
};

use itertools::Itertools;
use log::debug;

pub fn part_one(input: &str) -> anyhow::Result<impl ToString> {
    debug!("--- part one ---");

    let (start, graph) = parse::parse(input)?;

    let mut queue = StepQueue::new(start, graph);

    while let Some(current) = queue.pop() {
        debug!("Visiting {current}");
        queue.complete_step(current);
    }

    Ok(queue.visited.into_iter().collect::<String>())
}

pub fn part_two(input: &str) -> anyhow::Result<impl ToString> {
    debug!("---- part two ----");

    let (start, graph) = parse::parse(input)?;

    let mut queue = StepQueue::new(start, graph);

    let mut workers = vec![
        Worker::new(1),
        Worker::new(2),
        Worker::new(3),
        Worker::new(4),
        Worker::new(5),
    ];

    for now in 0u16.. {
        for worker in workers.iter_mut() {
            if worker.finished_at <= now {
                if worker.current_step != '-' {
                    debug!(
                        "Worker {} finished {} at {}",
                        worker.id, worker.current_step, now
                    );
                    queue.complete_step(worker.current_step);
                }

                if let Some(next_step) = queue.pop() {
                    worker.current_step = next_step;
                    worker.finished_at = now + next_step as u8 as u16 + 60 - 65 + 1;
                    debug!(
                        "Worker {} will complete {} at {}",
                        worker.id, worker.current_step, worker.finished_at
                    );
                } else {
                    worker.current_step = '-';
                }
            }
        }

        if queue.is_empty() && !workers.iter().any(Worker::is_working) {
            return Ok((now - 1).to_string());
        }
    }

    unreachable!()
}

struct Worker {
    id: u8,
    current_step: char,
    finished_at: u16,
}

impl Worker {
    fn new(id: u8) -> Worker {
        let current_step = '-';
        let finished_at = 0;

        Worker {
            id,
            current_step,
            finished_at,
        }
    }

    fn is_working(&self) -> bool {
        self.current_step != '-'
    }
}

struct StepQueue {
    queue: BinaryHeap<Reverse<char>>,
    graph: HashMap<char, Vec<char>>,
    visited: Vec<char>,
}

impl StepQueue {
    fn new(initial: Vec<char>, graph: HashMap<char, Vec<char>>) -> StepQueue {
        let mut queue = BinaryHeap::new();

        for c in initial {
            queue.push(Reverse(c));
        }

        let visited = Vec::new();

        StepQueue {
            queue,
            graph,
            visited,
        }
    }

    fn pop(&mut self) -> Option<char> {
        self.queue.pop().map(|rev| rev.0)
    }

    fn is_empty(&self) -> bool {
        self.queue.is_empty()
    }

    fn complete_step(&mut self, step: char) {
        self.visited.push(step);

        if let Some(successors) = self.graph.get(&step) {
            for successor in successors {
                let predecessors = self
                    .graph
                    .iter()
                    .filter_map(|(predecessor, successors)| {
                        if successors.contains(successor) {
                            Some(predecessor)
                        } else {
                            None
                        }
                    })
                    .collect_vec();

                if !self.visited.contains(successor)
                    && predecessors
                        .iter()
                        .all(|predecessor| self.visited.contains(&predecessor))
                {
                    debug!("  Will visit {successor}");
                    self.queue.push(Reverse(*successor));
                }
            }
        }
    }
}

mod parse {
    use std::collections::HashMap;

    use nom::Parser;
    use nom::{bytes::complete::tag, multi::separated_list1, sequence::preceded};
    use nom::{character::complete::one_of, IResult};

    use crate::common::parse::finish;

    pub fn parse(i: &str) -> anyhow::Result<(Vec<char>, HashMap<char, Vec<char>>)> {
        let steps = finish(steps, i)?;

        let mut graph = HashMap::new();

        for (dependency, dependent) in &steps {
            if !graph.contains_key(dependency) {
                graph.insert(*dependency, Vec::new());
            }

            graph.get_mut(dependency).unwrap().push(*dependent);
        }

        let mut start = Vec::new();

        for (dependency, _) in &steps {
            if !start.contains(dependency) && !graph.values().any(|v| v.contains(dependency)) {
                start.push(*dependency);
            }
        }

        Ok((start, graph))
    }

    fn steps(i: &str) -> IResult<&str, Vec<(char, char)>> {
        separated_list1(tag("\n"), step).parse(i)
    }

    fn step(i: &str) -> IResult<&str, (char, char)> {
        let (i, (dependency, dependent)) = (
            preceded(tag("Step "), step_name),
            preceded(tag(" must be finished before step "), step_name),
        )
            .parse(i)?;

        let (i, _) = tag(" can begin.")(i)?;

        Ok((i, (dependency, dependent)))
    }

    fn step_name(i: &str) -> IResult<&str, char> {
        one_of("ABCDEFGHIJKLMNOPQRSTUVWXYZ")(i)
    }
}
