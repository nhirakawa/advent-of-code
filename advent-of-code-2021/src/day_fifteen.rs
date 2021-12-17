use std::{
    collections::{BinaryHeap, HashMap},
    fmt::Display,
};

use common::{parse::unsigned_number, prelude::*};
use log::{debug, info, trace};
use nom::{
    bytes::complete::{tag, take},
    character::complete::multispace0,
    combinator::{all_consuming, into, map_parser},
    multi::{many1, separated_list1},
    sequence::terminated,
    IResult,
};

pub fn run() -> AdventOfCodeResult {
    let input = include_str!("../input/day-15.txt");
    let grid = parse_grid(input);

    let part_one = part_one(&grid);
    let part_two = part_two(&grid);

    Ok((part_one, part_two))
}

fn part_one(grid: &Grid) -> PartAnswer {
    let start = SystemTime::now();

    let distance = min_distance(grid);

    PartAnswer::new(distance, start.elapsed().unwrap())
}

fn part_two(grid: &Grid) -> PartAnswer {
    let start = SystemTime::now();

    let grid = scale(grid, 5);

    let distance = min_distance(&grid);

    PartAnswer::new(distance, start.elapsed().unwrap())
}

fn scale(grid: &Grid, scalar: usize) -> Grid {
    let mut new_grid = HashMap::new();

    for (vertex, weight) in grid.grid.iter() {
        for x_multiplier in 0..scalar {
            let x_step = vertex.x + (grid.x_size * x_multiplier);

            for y_multiplier in 0..scalar {
                let y_step = vertex.y + (grid.y_size * y_multiplier);

                debug!(
                    "scaling {} with x_multiplier {}, y_multiplier: {}, max_x: {}, max_y: {}, x_step: {}, y_step: {}",
                    vertex, x_multiplier, y_multiplier, grid.x_size, grid.y_size, x_step, y_step
                );

                let new_vertex = Vertex::new(x_step, y_step);
                let new_weight = *weight + x_multiplier + y_multiplier;
                let new_weight = if new_weight >= 10 {
                    (new_weight + 1) % 10
                } else {
                    new_weight
                };

                debug!("new vertex {}, new weight {}", new_vertex, new_weight);

                new_grid.insert(new_vertex, new_weight);
            }
        }
    }

    Grid::new(new_grid)
}

fn min_distance(grid: &Grid) -> usize {
    let distances = dijkstra(grid);

    let lower_right = Vertex {
        x: grid.x_size - 1,
        y: grid.y_size - 1,
    };

    info!("lower right is {}", lower_right);

    distances.get(&lower_right)
}

fn dijkstra(grid: &Grid) -> Distances {
    let mut priority_queue = BinaryHeap::new();

    let mut distances = HashMap::new();

    for weighted_vertex in grid.vertices() {
        let initial_distance = if weighted_vertex.is_start() {
            0
        } else {
            usize::MAX
        };

        distances.insert(weighted_vertex.vertex, initial_distance);

        if weighted_vertex.is_start() {
            priority_queue.push(weighted_vertex);
        }
    }

    while let Some(WeightedVertex { vertex, weight }) = priority_queue.pop() {
        debug!("min-distance-vertex is {}, weight is {}", vertex, weight);

        let neighbors = grid.neighbors(&vertex);

        trace!("neighbors are {:?}", neighbors);

        for (neighbor, neighbor_weight) in grid.neighbors(&vertex) {
            let alternate_distance = weight + neighbor_weight;

            let current_neighbor_distance = distances[&neighbor];

            trace!(
                "{} currently has distance {}",
                neighbor,
                current_neighbor_distance
            );

            if alternate_distance < current_neighbor_distance {
                debug!(
                    "updating distance for {} to {}",
                    neighbor, alternate_distance
                );

                priority_queue.push(WeightedVertex {
                    vertex: neighbor,
                    weight: alternate_distance,
                });
                distances.insert(neighbor, alternate_distance);
            }
        }

        debug!("{:?}", priority_queue);
    }

    Distances { d: distances }
}

#[derive(Debug, PartialEq)]
struct Grid {
    grid: HashMap<Vertex, usize>,
    x_size: usize,
    y_size: usize,
}

impl Grid {
    fn new(grid: HashMap<Vertex, usize>) -> Grid {
        let mut max_x = 0;
        let mut max_y = 0;

        for vertex in grid.keys() {
            max_x = max_x.max(vertex.x);
            max_y = max_y.max(vertex.y);
        }

        let x_size = max_x + 1;
        let y_size = max_y + 1;

        Grid {
            grid,
            x_size,
            y_size,
        }
    }

    fn vertices(&self) -> Vec<WeightedVertex> {
        self.grid
            .iter()
            .map(|(vertex, weight)| {
                let weight = if vertex.is_start() { 0 } else { *weight };

                WeightedVertex {
                    vertex: *vertex,
                    weight,
                }
            })
            .collect()
    }

    fn neighbors(&self, source: &Vertex) -> Vec<(Vertex, usize)> {
        let mut neighbors = Vec::new();

        for potential_neighbor in source.neighbors() {
            if let Some(neighbor_weight) = self.grid.get(&potential_neighbor) {
                neighbors.push((potential_neighbor, *neighbor_weight));
            }
        }

        neighbors
    }
}

impl From<Vec<Vec<usize>>> for Grid {
    fn from(raw: Vec<Vec<usize>>) -> Grid {
        let mut grid = HashMap::new();

        for (y, row) in raw.into_iter().enumerate() {
            for (x, risk_level) in row.into_iter().enumerate() {
                grid.insert((x, y).into(), risk_level);
            }
        }

        Grid::new(grid)
    }
}

#[derive(Debug, PartialEq)]
struct Distances {
    d: HashMap<Vertex, usize>,
}

impl Distances {
    fn get(&self, vertex: &Vertex) -> usize {
        self.d.get(vertex).copied().unwrap_or(usize::MAX)
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Vertex {
    x: usize,
    y: usize,
}

impl Vertex {
    fn new(x: usize, y: usize) -> Vertex {
        Vertex { x, y }
    }

    fn is_start(&self) -> bool {
        self.x == 0 && self.y == 0
    }

    fn neighbors(&self) -> Vec<Vertex> {
        let mut neighbors = vec![self.down(), self.right()];

        if let Some(up) = self.up() {
            neighbors.push(up);
        }

        if let Some(left) = self.left() {
            neighbors.push(left);
        }

        neighbors
    }

    fn up(&self) -> Option<Vertex> {
        if self.y == 0 {
            None
        } else {
            Some(Vertex {
                x: self.x,
                y: self.y - 1,
            })
        }
    }

    fn down(&self) -> Vertex {
        Vertex {
            x: self.x,
            y: self.y + 1,
        }
    }

    fn left(&self) -> Option<Vertex> {
        if self.x == 0 {
            None
        } else {
            Some(Vertex {
                x: self.x - 1,
                y: self.y,
            })
        }
    }

    fn right(&self) -> Vertex {
        Vertex {
            x: self.x + 1,
            y: self.y,
        }
    }
}

impl From<(usize, usize)> for Vertex {
    fn from(tuple: (usize, usize)) -> Vertex {
        let (x, y) = tuple;
        Vertex::new(x, y)
    }
}

impl PartialOrd for Vertex {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Vertex {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.x.cmp(&other.x).then(self.y.cmp(&other.y))
    }
}

impl Display for Vertex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct WeightedVertex {
    vertex: Vertex,
    weight: usize,
}

impl WeightedVertex {
    fn new(vertex: Vertex, weight: usize) -> WeightedVertex {
        WeightedVertex { vertex, weight }
    }

    fn is_start(&self) -> bool {
        self.vertex.is_start()
    }
}

impl From<(Vertex, usize)> for WeightedVertex {
    fn from(tuple: (Vertex, usize)) -> WeightedVertex {
        let (vertex, weight) = tuple;
        WeightedVertex::new(vertex, weight)
    }
}

impl PartialOrd for WeightedVertex {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for WeightedVertex {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.weight
            .cmp(&other.weight)
            .then(self.vertex.cmp(&other.vertex))
            .reverse()
    }
}

fn parse_grid(i: &str) -> Grid {
    all_consuming(terminated(grid, multispace0))(i).unwrap().1
}

fn grid(i: &str) -> IResult<&str, Grid> {
    into(separated_list1(tag("\n"), row))(i)
}

fn row(i: &str) -> IResult<&str, Vec<usize>> {
    many1(risk_level)(i)
}

fn risk_level(i: &str) -> IResult<&str, usize> {
    map_parser(take(1_usize), unsigned_number)(i)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_small_example() {
        let grid = parse_grid("1163751742\n1381373672\n2136511328\n3694931569\n7463417111\n1319128137\n1359912421\n3125421639\n1293138521\n2311944581\n");

        assert_eq!(min_distance(&grid), 40);
    }

    #[test]
    fn test_small_dijkstra() {
        let grid = Grid::from(vec![vec![1, 3], vec![2, 4]]);
        let distances = dijkstra(&grid);

        debug!("{:#?}", distances);

        assert_eq!(distances.get(&Vertex::new(1, 0)), 3);
        assert_eq!(distances.get(&Vertex::new(0, 1)), 2);
        assert_eq!(distances.get(&Vertex::new(1, 1)), 6);
    }

    #[test]
    fn test_larger_dijkstra() {
        let grid = parse_grid("1163751742\n1381373672\n2136511328\n3694931569\n7463417111\n1319128137\n1359912421\n3125421639\n1293138521\n2311944581\n");
        let grid = scale(&grid, 5);

        assert_eq!(min_distance(&grid), 315);
    }

    #[test]
    fn test_scale_grid() {
        let grid = Grid::from(vec![vec![1, 3], vec![7, 9]]);

        let scaled = scale(&grid, 2);

        assert_eq!(scaled.grid.get(&Vertex::new(0, 0)).copied(), Some(1));
        assert_eq!(scaled.grid.get(&Vertex::new(0, 2)).copied(), Some(2));
        assert_eq!(scaled.grid.get(&Vertex::new(3, 1)).copied(), Some(1));
        assert_eq!(scaled.grid.get(&Vertex::new(2, 3)).copied(), Some(9));
    }

    #[test]
    fn test_scale_grid_example() {
        let expected_grid = parse_grid("11637517422274862853338597396444961841755517295286\n13813736722492484783351359589446246169155735727126\n21365113283247622439435873354154698446526571955763\n36949315694715142671582625378269373648937148475914\n74634171118574528222968563933317967414442817852555\n13191281372421239248353234135946434524615754563572\n13599124212461123532357223464346833457545794456865\n31254216394236532741534764385264587549637569865174\n12931385212314249632342535174345364628545647573965\n23119445813422155692453326671356443778246755488935\n22748628533385973964449618417555172952866628316397\n24924847833513595894462461691557357271266846838237\n32476224394358733541546984465265719557637682166874\n47151426715826253782693736489371484759148259586125\n85745282229685639333179674144428178525553928963666\n24212392483532341359464345246157545635726865674683\n24611235323572234643468334575457944568656815567976\n42365327415347643852645875496375698651748671976285\n23142496323425351743453646285456475739656758684176\n34221556924533266713564437782467554889357866599146\n33859739644496184175551729528666283163977739427418\n35135958944624616915573572712668468382377957949348\n43587335415469844652657195576376821668748793277985\n58262537826937364893714847591482595861259361697236\n96856393331796741444281785255539289636664139174777\n35323413594643452461575456357268656746837976785794\n35722346434683345754579445686568155679767926678187\n53476438526458754963756986517486719762859782187396\n34253517434536462854564757396567586841767869795287\n45332667135644377824675548893578665991468977611257\n44961841755517295286662831639777394274188841538529\n46246169155735727126684683823779579493488168151459\n54698446526571955763768216687487932779859814388196\n69373648937148475914825958612593616972361472718347\n17967414442817852555392896366641391747775241285888\n46434524615754563572686567468379767857948187896815\n46833457545794456865681556797679266781878137789298\n64587549637569865174867197628597821873961893298417\n45364628545647573965675868417678697952878971816398\n56443778246755488935786659914689776112579188722368\n55172952866628316397773942741888415385299952649631\n57357271266846838237795794934881681514599279262561\n65719557637682166874879327798598143881961925499217\n71484759148259586125936169723614727183472583829458\n28178525553928963666413917477752412858886352396999\n57545635726865674683797678579481878968159298917926\n57944568656815567976792667818781377892989248891319\n75698651748671976285978218739618932984172914319528\n56475739656758684176786979528789718163989182927419\n67554889357866599146897761125791887223681299833479");

        let grid = parse_grid("1163751742\n1381373672\n2136511328\n3694931569\n7463417111\n1319128137\n1359912421\n3125421639\n1293138521\n2311944581\n");

        assert_eq!(scale(&grid, 5), expected_grid);
    }

    #[test]
    fn test_binary_heap() {
        let mut heap = BinaryHeap::new();

        let first = WeightedVertex::new(Vertex::new(1, 3), 1);
        let second = WeightedVertex::new(Vertex::new(2, 4), 10);
        let third = WeightedVertex::new(Vertex::new(1, 3), 3);
        let fourth = WeightedVertex::new(Vertex::new(2, 5), 5);

        let ordered = vec![first, third, fourth, second];

        heap.push(first.clone());
        heap.push(second.clone());
        heap.push(third.clone());

        let mut index = 0;

        while let Some(popped) = heap.pop() {
            assert_eq!(popped, ordered[index]);

            if index == 0 {
                heap.push(fourth);
            }

            index += 1;
        }
    }
}
