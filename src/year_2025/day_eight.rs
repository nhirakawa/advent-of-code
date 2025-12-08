use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
    str::FromStr,
};

use anyhow::bail;
use itertools::Itertools;

use crate::common::math;

pub fn part_one(input: &str) -> anyhow::Result<impl ToString> {
    let junction_boxes = JunctionBoxes::from_str(input)?;
    let mut circuits = Circuits::new(junction_boxes.0.clone());
    let connections = collect_junction_connections(junction_boxes.clone());

    let mut ordered_connections = OrderedConnections::from(connections);

    let mut count = 0;

    while let Some(connection) = ordered_connections.pop() {
        circuits.connect(connection);
        count += 1;

        if count == 1000 {
            break;
        }
    }

    Ok(circuits.get_circuit_product())
}

pub fn part_two(input: &str) -> anyhow::Result<impl ToString> {
    let junction_boxes = JunctionBoxes::from_str(input)?;
    let mut circuits = Circuits::new(junction_boxes.0.clone());
    let connections = collect_junction_connections(junction_boxes.clone());

    let mut ordered_connections = OrderedConnections::from(connections);

    while let Some(connection) = ordered_connections.pop() {
        circuits.connect(connection);

        if circuits.count_circuits() == 1 {
            return Ok(connection.x_product());
        }
    }

    Err(anyhow::anyhow!("No solution found"))
}

type Num = usize;

#[derive(Debug)]
struct Circuits {
    next_id: usize,
    junction_boxes: HashMap<JunctionBox, usize>,
    aliases: HashMap<usize, usize>,
}

impl Circuits {
    fn new(junction_boxes: Vec<JunctionBox>) -> Circuits {
        let next_id = junction_boxes.len();
        let mut junction_box_map = HashMap::new();

        for (index, junction_box) in junction_boxes.into_iter().enumerate() {
            junction_box_map.insert(junction_box, index);
        }

        Circuits {
            next_id,
            junction_boxes: junction_box_map,
            aliases: HashMap::default(),
        }
    }

    fn connect(&mut self, connection: JunctionConnection) {
        let (first, second) = connection.into();

        let first_id = self.get_or_insert_id(&first);
        let second_id = self.get_or_insert_id(&second);

        let first_id = self.resolve_alias(first_id);
        let second_id = self.resolve_alias(second_id);

        if first_id == second_id {
            // Refer to same circuit - nothing to do
            return;
        }

        let id = self.next_id();
        self.aliases.insert(first_id, id);
        self.aliases.insert(second_id, id);
    }

    fn get_or_insert_id(&mut self, junction_box: &JunctionBox) -> usize {
        if let Some(id) = self.junction_boxes.get(junction_box) {
            *id
        } else {
            let id = self.next_id();
            self.junction_boxes.insert(*junction_box, id);
            id
        }
    }

    fn next_id(&mut self) -> usize {
        let id = self.next_id;
        self.next_id += 1;
        id
    }

    fn resolve_alias(&self, alias: usize) -> usize {
        let mut resolved = alias;
        while let Some(alias) = self.aliases.get(&resolved) {
            resolved = *alias;
        }
        resolved
    }

    fn get_circuit_sizes(&self) -> Vec<usize> {
        let mut circuit_sizes: HashMap<usize, usize> = HashMap::new();

        for &id in self.junction_boxes.values() {
            let root = self.resolve_alias(id);
            *circuit_sizes.entry(root).or_insert(0) += 1;
        }

        let mut sizes: Vec<usize> = circuit_sizes.into_values().collect();
        sizes.sort_unstable_by_key(|&size| Reverse(size));
        sizes
    }

    fn get_circuit_product(&self) -> usize {
        self.get_circuit_sizes().iter().take(3).copied().product()
    }

    fn count_circuits(&self) -> usize {
        self.get_circuit_sizes().len()
    }
}

#[derive(Debug)]
struct OrderedConnections(BinaryHeap<Reverse<JunctionConnection>>);

impl OrderedConnections {
    fn pop(&mut self) -> Option<JunctionConnection> {
        self.0.pop().map(|rev| rev.0)
    }
}

impl From<Vec<JunctionConnection>> for OrderedConnections {
    fn from(value: Vec<JunctionConnection>) -> Self {
        let value = value.into_iter().map(|item| Reverse(item)).collect_vec();
        Self(BinaryHeap::from(value))
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
struct JunctionBox([Num; 3]);

impl FromStr for JunctionBox {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut comma_indexes = Vec::new();

        for (index, b) in s.trim().as_bytes().iter().enumerate() {
            if *b == b',' {
                comma_indexes.push(index);
            }
        }

        if comma_indexes.len() != 2 {
            bail!(
                "Expected 2 commas, found {} -{:?}",
                comma_indexes.len(),
                comma_indexes
            );
        }

        let x = &s[..comma_indexes[0]];
        let x = x.parse()?;
        let y = &s[comma_indexes[0] + 1..comma_indexes[1]];
        let y = y.parse()?;
        let z = &s[comma_indexes[1] + 1..];
        let z = z.parse()?;

        Ok(JunctionBox([x, y, z]))
    }
}

#[derive(Debug, Copy, Clone, Eq)]
struct JunctionConnection {
    connection: (JunctionBox, JunctionBox),
    distance: Num,
}

impl JunctionConnection {
    fn new(first: JunctionBox, second: JunctionBox) -> JunctionConnection {
        JunctionConnection::from((first, second))
    }

    fn x_product(&self) -> Num {
        self.connection.0.0[0] * self.connection.1.0[0]
    }
}

impl From<(JunctionBox, JunctionBox)> for JunctionConnection {
    fn from((first, second): (JunctionBox, JunctionBox)) -> Self {
        let distance = math::squared_euclidean_distance(first.0, second.0);
        JunctionConnection {
            connection: (first, second),
            distance,
        }
    }
}

impl Into<(JunctionBox, JunctionBox)> for JunctionConnection {
    fn into(self) -> (JunctionBox, JunctionBox) {
        self.connection
    }
}

impl PartialEq for JunctionConnection {
    fn eq(&self, other: &Self) -> bool {
        (self.connection.0 == other.connection.0 && self.connection.1 == other.connection.1)
            || (self.connection.0 == other.connection.1 && self.connection.1 == other.connection.0)
    }
}

impl PartialOrd for JunctionConnection {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for JunctionConnection {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.distance.cmp(&other.distance)
    }
}

fn collect_junction_connections<J: Into<Vec<JunctionBox>>>(boxes: J) -> Vec<JunctionConnection> {
    boxes
        .into()
        .into_iter()
        .combinations(2)
        .map(|v| JunctionConnection::new(v[0], v[1]))
        .collect()
}

#[derive(Debug, Clone)]
struct JunctionBoxes(Vec<JunctionBox>);

impl From<Vec<JunctionBox>> for JunctionBoxes {
    fn from(value: Vec<JunctionBox>) -> Self {
        JunctionBoxes(value)
    }
}

impl Into<Vec<JunctionBox>> for JunctionBoxes {
    fn into(self) -> Vec<JunctionBox> {
        self.0
    }
}

impl FromStr for JunctionBoxes {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut junction_boxes = Vec::new();

        for line in s.lines() {
            let junction_box = JunctionBox::from_str(line)?;
            junction_boxes.push(junction_box);
        }

        Ok(JunctionBoxes(junction_boxes))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_junction_box_from_str() {
        assert_eq!(
            JunctionBox::from_str("162,817,812").unwrap(),
            JunctionBox([162, 817, 812])
        );
        assert!(JunctionBox::from_str("162,817, 812").is_err());
        assert!(JunctionBox::from_str("162,817,asdf").is_err());
    }

    #[test]
    fn test_ordered_connections_pop() {
        let junction_boxes = vec![
            JunctionBox([162, 817, 812]),
            JunctionBox([57, 618, 57]),
            JunctionBox([906, 360, 560]),
            JunctionBox([592, 479, 940]),
            JunctionBox([352, 342, 300]),
            JunctionBox([466, 668, 158]),
            JunctionBox([542, 29, 236]),
            JunctionBox([431, 825, 988]),
            JunctionBox([739, 650, 466]),
            JunctionBox([52, 470, 668]),
            JunctionBox([216, 146, 977]),
            JunctionBox([819, 987, 18]),
            JunctionBox([117, 168, 530]),
            JunctionBox([805, 96, 715]),
            JunctionBox([346, 949, 466]),
            JunctionBox([970, 615, 88]),
            JunctionBox([941, 993, 340]),
            JunctionBox([862, 61, 35]),
            JunctionBox([984, 92, 344]),
            JunctionBox([425, 690, 689]),
        ];

        let junction_connections = collect_junction_connections(junction_boxes);

        let mut ordered_connections = OrderedConnections::from(junction_connections);

        assert_eq!(
            ordered_connections
                .pop()
                .map(|connection| connection.connection),
            Some((JunctionBox([162, 817, 812]), JunctionBox([425, 690, 689])))
        );
        assert_eq!(
            ordered_connections
                .pop()
                .map(|connection| connection.connection),
            Some((JunctionBox([162, 817, 812]), JunctionBox([431, 825, 988])))
        );
        assert_eq!(
            ordered_connections
                .pop()
                .map(|connection| connection.connection),
            Some((JunctionBox([906, 360, 560]), JunctionBox([805, 96, 715])))
        );
        assert_eq!(
            ordered_connections
                .pop()
                .map(|connection| connection.connection),
            Some((JunctionBox([431, 825, 988]), JunctionBox([425, 690, 689])))
        );
    }

    #[test]
    fn test_circuits_connect() {
        let first = JunctionBox([162, 817, 812]);
        let second = JunctionBox([425, 690, 689]);

        let mut circuits = Circuits::new(vec![first, second]);

        let connection = JunctionConnection::from((first, second));

        circuits.connect(connection);

        assert_eq!(circuits.aliases.len(), 2);
        assert_eq!(circuits.aliases[&0], 2);
        assert_eq!(circuits.aliases[&1], 2);
    }

    #[test]
    fn test_circuits_connect_example() {
        let junction_boxes = vec![
            JunctionBox([162, 817, 812]),
            JunctionBox([57, 618, 57]),
            JunctionBox([906, 360, 560]),
            JunctionBox([592, 479, 940]),
            JunctionBox([352, 342, 300]),
            JunctionBox([466, 668, 158]),
            JunctionBox([542, 29, 236]),
            JunctionBox([431, 825, 988]),
            JunctionBox([739, 650, 466]),
            JunctionBox([52, 470, 668]),
            JunctionBox([216, 146, 977]),
            JunctionBox([819, 987, 18]),
            JunctionBox([117, 168, 530]),
            JunctionBox([805, 96, 715]),
            JunctionBox([346, 949, 466]),
            JunctionBox([970, 615, 88]),
            JunctionBox([941, 993, 340]),
            JunctionBox([862, 61, 35]),
            JunctionBox([984, 92, 344]),
            JunctionBox([425, 690, 689]),
        ];

        let mut circuits = Circuits::new(junction_boxes.clone());

        let connections = collect_junction_connections(junction_boxes.clone());

        let mut ordered_connections = OrderedConnections::from(connections);

        let mut count = 0;

        while let Some(connection) = ordered_connections.pop() {
            circuits.connect(connection);
            count += 1;
            if count == 10 {
                break;
            }
        }

        assert_eq!(
            circuits.get_circuit_sizes(),
            vec![5, 4, 2, 2, 1, 1, 1, 1, 1, 1, 1]
        );
        assert_eq!(circuits.get_circuit_product(), 40);
    }
}
