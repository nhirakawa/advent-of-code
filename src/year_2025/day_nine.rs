use anyhow::anyhow;
use itertools::Itertools;
use std::collections::HashSet;
use std::fmt::{Display, Formatter};
use std::ops::RangeInclusive;
use std::str::FromStr;

use crate::common::math::geom::{BoundingBox, Coord};

pub fn part_one(input: &str) -> anyhow::Result<impl ToString> {
    let coords = Coords::from_str(input)?;

    coords
        .rectangles()
        .map(|r| r.area())
        .max()
        .ok_or(anyhow!("No solution found"))
}

pub fn part_two(input: &str) -> anyhow::Result<impl ToString> {
    let coords = Coords::from_str(input)?;

    let exterior_border = coords.exterior_border_coords();

    // sorted by area descending
    let rectangles = coords
        .rectangles()
        .sorted_by_key(|rectangle| rectangle.area())
        .rev()
        .collect_vec();

    for rectangle in rectangles {
        if is_rectangle_within_polygon(&rectangle, &coords, &exterior_border) {
            return Ok(rectangle.area());
        }
    }

    Err(anyhow!("Not implemented"))
}

fn is_rectangle_within_polygon(
    rectangle: &Rectangle,
    polygon: &Coords,
    exterior_border: &HashSet<Coord2D>,
) -> bool {
    let corners = rectangle.corners();
    for corner in &corners {
        if !polygon.is_in_polygon(*corner) {
            return false;
        }
    }

    for segment in &polygon.segments {
        if rectangle.intersects(segment) {
            return false;
        }
    }

    // Check if any exterior border points are inside the rectangle
    // This catches cases where corners are on the boundary but the rectangle encloses exterior area
    for &exterior_point in exterior_border {
        if rectangle.contains(exterior_point) {
            return false;
        }
    }

    true
}

type Num = isize;
type Coord2D = Coord<Num, 2>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Orientation {
    Horizontal,
    Vertical,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Segment {
    c0: Coord2D,
    c1: Coord2D,
}

impl Segment {
    #[allow(unused)]
    fn new(c0: Coord2D, c1: Coord2D) -> Self {
        if c0 == c1 {
            panic!("Cannot construct Segment from 2 identical points ({c0})");
        }

        if !(c0.x() == c1.x() || c0.y() == c1.y()) {
            panic!("Segment is not horizontal or vertical")
        }

        Self { c0, c1 }
    }

    fn orientation(&self) -> Orientation {
        if self.c0.x() == self.c1.x() {
            Orientation::Vertical
        } else if self.c0.y() == self.c1.y() {
            Orientation::Horizontal
        } else {
            panic!("Segment is not horizontal or vertical");
        }
    }

    fn range(&self) -> RangeInclusive<Num> {
        match self.orientation() {
            Orientation::Horizontal => {
                std::cmp::min(self.c0.x(), self.c1.x())..=std::cmp::max(self.c0.x(), self.c1.x())
            }
            Orientation::Vertical => {
                std::cmp::min(self.c0.y(), self.c1.y())..=std::cmp::max(self.c0.y(), self.c1.y())
            }
        }
    }

    /// Returns true if this segment contains coord, including either of the endpoints
    fn contains(&self, coord: Coord2D) -> bool {
        match self.orientation() {
            Orientation::Horizontal => {
                coord.y() == self.c0.y() && self.range().contains(&coord.x())
            }
            Orientation::Vertical => coord.x() == self.c0.x() && self.range().contains(&coord.y()),
        }
    }

    #[allow(unused)]
    fn len(&self) -> usize {
        match self.orientation() {
            Orientation::Horizontal => self.c0.x().abs_diff(self.c1.x()),
            Orientation::Vertical => self.c0.y().abs_diff(self.c1.y()),
        }
    }
}

impl From<(Coord2D, Coord2D)> for Segment {
    fn from((c0, c1): (Coord2D, Coord2D)) -> Self {
        Self { c0, c1 }
    }
}

impl From<Segment> for Vec<Coord2D> {
    fn from(val: Segment) -> Vec<Coord2D> {
        match val.orientation() {
            Orientation::Horizontal => {
                let min = std::cmp::min(val.c0.x(), val.c1.x());
                let max = std::cmp::max(val.c0.x(), val.c1.x());

                (min..=max).map(|x| Coord2D::new(x, val.c0.y())).collect()
            }
            Orientation::Vertical => {
                let min = std::cmp::min(val.c0.y(), val.c1.y());
                let max = std::cmp::max(val.c0.y(), val.c1.y());

                (min..=max).map(|y| Coord2D::new(val.c0.x(), y)).collect()
            }
        }
    }
}

impl Display for Segment {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{{} - {}}}", self.c0, self.c1)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Coords {
    coords: Vec<Coord2D>,
    segments: Vec<Segment>,
    bounding_box: BoundingBox<Num, 2>,
}

impl Coords {
    fn new<I: IntoIterator<Item = Coord2D>>(coords: I) -> Self {
        let coords = coords.into_iter().collect_vec();

        let segments: Vec<Segment> = coords
            .iter()
            .cycle()
            .take(coords.len() + 1)
            .copied()
            .tuple_windows()
            .map(|(c0, c1)| Segment::from((c0, c1)))
            .collect();

        // Validate that no two consecutive segments have the same orientation
        for window in segments.windows(2) {
            if window[0].orientation() == window[1].orientation() {
                panic!(
                    "Invalid polygon: consecutive segments have same orientation: {} and {}",
                    window[0], window[1]
                );
            }
        }

        let bounding_box = BoundingBox::from_points(coords.clone());

        Self {
            coords,
            segments,
            bounding_box,
        }
    }

    fn perimeter_coords(&self) -> HashSet<Coord2D> {
        self.segments
            .iter()
            .flat_map(|segment| Into::<Vec<Coord2D>>::into(*segment))
            .collect()
    }

    fn exterior_border_coords(&self) -> HashSet<Coord2D> {
        let perimeter = self.perimeter_coords();
        let mut exterior = HashSet::new();

        for &point in &perimeter {
            // Check all 4 neighbors
            let neighbors = [
                Coord2D::new(point.x() + 1, point.y()),
                Coord2D::new(point.x() - 1, point.y()),
                Coord2D::new(point.x(), point.y() + 1),
                Coord2D::new(point.x(), point.y() - 1),
            ];

            for neighbor in neighbors {
                // If neighbor is not in polygon, it's on the exterior border
                if !self.is_in_polygon(neighbor) {
                    exterior.insert(neighbor);
                }
            }
        }

        exterior
    }

    fn is_in_polygon(&self, point: Coord2D) -> bool {
        // Check if point is on the perimeter
        for segment in &self.segments {
            if segment.contains(point) {
                return true;
            }
        }

        // Ray casting: count how many vertical segments the ray crosses
        // Cast a ray to the right and count crossings
        let crossings = self
            .segments
            .iter()
            .filter(|segment| {
                // Only count vertical segments (horizontal segments are parallel to the ray)
                if segment.orientation() != Orientation::Vertical {
                    return false;
                }

                // The segment must be to the right of the point
                let segment_x = segment.c0.x();
                if segment_x <= point.x() {
                    return false;
                }

                // The point's y-coordinate must be within the segment's y-range
                segment.range().contains(&point.y())
            })
            .count();

        crossings % 2 == 1
    }

    fn rectangles(&self) -> impl Iterator<Item = Rectangle> + '_ {
        self.coords.iter().copied().permutations(2).map(|v| {
            let first = v[0];
            let second = v[1];
            Rectangle::new(first, second)
        })
    }
}

impl<'a> IntoIterator for &'a Coords {
    type Item = Coord2D;
    type IntoIter = std::iter::Copied<std::slice::Iter<'a, Coord2D>>;

    fn into_iter(self) -> Self::IntoIter {
        self.coords.iter().copied()
    }
}

impl FromStr for Coords {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut coords = Vec::new();
        for line in s.lines() {
            let coord = line.trim().parse()?;
            coords.push(coord);
        }
        Ok(Self::new(coords))
    }
}

#[derive(Debug, Copy, Clone)]
struct Rectangle {
    top_left: Coord2D,
    bottom_right: Coord2D,
}

impl Rectangle {
    fn new(c0: Coord2D, c1: Coord2D) -> Self {
        let top_left = Coord2D::new(std::cmp::min(c0.x(), c1.x()), std::cmp::min(c0.y(), c1.y()));
        let bottom_right =
            Coord2D::new(std::cmp::max(c0.x(), c1.x()), std::cmp::max(c0.y(), c1.y()));

        Self {
            top_left,
            bottom_right,
        }
    }

    fn is_right_of_left_edge(&self, coord: Coord2D) -> bool {
        coord.x() > self.top_left.x()
    }

    fn is_left_of_right_edge(&self, coord: Coord2D) -> bool {
        coord.x() < self.bottom_right.x()
    }

    fn is_above_bottom_edge(&self, coord: Coord2D) -> bool {
        coord.y() < self.bottom_right.y()
    }

    fn is_below_top_edge(&self, coord: Coord2D) -> bool {
        coord.y() > self.top_left.y()
    }

    fn intersects(&self, segment: &Segment) -> bool {
        self.contains(segment.c0) || self.contains(segment.c1)
    }

    /// Returns true if the Rectangle contains the coordinate
    /// Note: returns false if the point is on the perimeter of the Rectangle
    fn contains(&self, coord: Coord2D) -> bool {
        self.is_right_of_left_edge(coord)
            && self.is_left_of_right_edge(coord)
            && self.is_above_bottom_edge(coord)
            && self.is_below_top_edge(coord)
    }

    fn corners(&self) -> [Coord2D; 4] {
        let top_left = self.top_left;
        let bottom_right = self.bottom_right;

        let top_right = Coord2D::new(self.bottom_right.x(), top_left.y());
        let bottom_left = Coord2D::new(top_left.x(), bottom_right.y());

        [top_left, top_right, bottom_right, bottom_left]
    }

    fn area(&self) -> Num {
        let min_x = self.top_left.x();
        let min_y = self.top_left.y();
        let max_x = self.bottom_right.x();
        let max_y = self.bottom_right.y();

        let delta_x = max_x - min_x + 1;
        let delta_y = max_y - min_y + 1;

        delta_x * delta_y
    }

    #[allow(unused)]
    fn perimeter_coords(&self) -> HashSet<Coord2D> {
        self.corners()
            .iter()
            .circular_tuple_windows()
            .filter_map(|(c0, c1)| {
                if c0 == c1 {
                    None
                } else {
                    Some(Segment::new(*c0, *c1))
                }
            })
            .flat_map(Into::<Vec<Coord2D>>::into)
            .collect()
    }

    #[allow(unused)]
    fn perimeter(&self) -> usize {
        self.corners()
            .iter()
            .circular_tuple_windows()
            .filter_map(|(c0, c1)| {
                if c0 == c1 {
                    None
                } else {
                    Some(Segment::new(*c0, *c1))
                }
            })
            .map(|segment| segment.len())
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_rectangle_within_polygon() {
        let polygon = Coords::new([
            Coord2D::new(0, 0),
            Coord2D::new(0, 4),
            Coord2D::new(4, 4),
            Coord2D::new(4, 6),
            Coord2D::new(2, 6),
            Coord2D::new(2, 8),
            Coord2D::new(6, 8),
            Coord2D::new(6, 0),
        ]);

        let exterior_border = polygon.exterior_border_coords();

        let rectangle = Rectangle::new(Coord2D::new(0, 0), Coord2D::new(4, 4));
        assert!(is_rectangle_within_polygon(
            &rectangle,
            &polygon,
            &exterior_border
        ));

        let rectangle = Rectangle::new(Coord2D::new(6, 8), Coord2D::new(4, 4));
        assert!(is_rectangle_within_polygon(
            &rectangle,
            &polygon,
            &exterior_border
        ));

        let rectangle = Rectangle::new(Coord2D::new(0, 4), Coord2D::new(2, 8));
        assert!(!is_rectangle_within_polygon(
            &rectangle,
            &polygon,
            &exterior_border
        ));

        let rectangle = Rectangle::new(Coord2D::new(0, 0), Coord2D::new(6, 8));
        assert!(!is_rectangle_within_polygon(
            &rectangle,
            &polygon,
            &exterior_border
        ));

        let rectangle = Rectangle::new(Coord2D::new(0, 0), Coord2D::new(4, 6));
        assert!(!is_rectangle_within_polygon(
            &rectangle,
            &polygon,
            &exterior_border
        ));
    }

    // [2, 7, 9, 11]
    // [1, 3, 5, 7]
    const COORDS: [Coord2D; 8] = [
        Coord2D::new(7, 1),
        Coord2D::new(11, 1),
        Coord2D::new(11, 7),
        Coord2D::new(9, 7),
        Coord2D::new(9, 5),
        Coord2D::new(2, 5),
        Coord2D::new(2, 3),
        Coord2D::new(7, 3),
    ];

    #[test]
    fn test_coord_parse() {
        assert_eq!(Coord2D::from_str("7,1").unwrap(), Coord2D::new(7, 1));
        assert_eq!(Coord2D::from_str("11,1").unwrap(), Coord2D::new(11, 1));
        assert_eq!(
            Coord2D::from_str("98471,50022").unwrap(),
            Coord2D::new(98471, 50022)
        );
    }

    #[test]
    fn test_coords_ctor() {
        let coords = Coords::new(COORDS);
        assert_eq!(coords.coords, COORDS.into_iter().collect_vec());

        // assert_eq!(
        //     coords.compressed_coord_by_original,
        //     [
        //         (Coord::new(7, 1), Coord::new(1, 0)),
        //         (Coord::new(11, 1), Coord::new(3, 0)),
        //         (Coord::new(11, 7), Coord::new(3, 3)),
        //         (Coord::new(9, 7), Coord::new(2, 3)),
        //         (Coord::new(9, 5), Coord::new(2, 2)),
        //         (Coord::new(2, 5), Coord::new(0, 2)),
        //         (Coord::new(2, 3), Coord::new(0, 1)),
        //         (Coord::new(7, 3), Coord::new(1, 1)),
        //     ]
        //     .into_iter()
        //     .collect()
        // );
        // assert_eq!(
        //     coords.original_coord_by_compressed,
        //     [
        //         (Coord::new(1, 0), Coord::new(7, 1)),
        //         (Coord::new(3, 0), Coord::new(11, 1)),
        //         (Coord::new(3, 3), Coord::new(11, 7)),
        //         (Coord::new(2, 3), Coord::new(9, 7)),
        //         (Coord::new(2, 2), Coord::new(9, 5)),
        //         (Coord::new(0, 2), Coord::new(2, 5)),
        //         (Coord::new(0, 1), Coord::new(2, 3)),
        //         (Coord::new(1, 1), Coord::new(7, 3)),
        //     ]
        //     .into_iter()
        //     .collect()
        // );
    }

    #[test]
    fn test_coords_is_in_polygon() {
        let coords = Coords::new([
            Coord2D::new(0, 0),
            Coord2D::new(0, 4),
            Coord2D::new(4, 4),
            Coord2D::new(4, 6),
            Coord2D::new(2, 6),
            Coord2D::new(2, 8),
            Coord2D::new(6, 8),
            Coord2D::new(6, 0),
        ]);

        // Points on polygon perimeter
        assert!(coords.is_in_polygon(Coord2D::new(0, 0)));
        assert!(coords.is_in_polygon(Coord2D::new(4, 4)));
        assert!(coords.is_in_polygon(Coord2D::new(4, 6)));
        assert!(coords.is_in_polygon(Coord2D::new(6, 8)));

        // // Points within polygon
        assert!(coords.is_in_polygon(Coord2D::new(1, 1)));
        assert!(coords.is_in_polygon(Coord2D::new(3, 1)));
        assert!(coords.is_in_polygon(Coord2D::new(5, 5)));

        // // Points outside polygon
        assert!(!coords.is_in_polygon(Coord2D::new(-1, -1)));
        assert!(!coords.is_in_polygon(Coord2D::new(3, 5)));
        assert!(!coords.is_in_polygon(Coord2D::new(0, 10)));
        assert!(!coords.is_in_polygon(Coord2D::new(8, 0)));
    }

    #[test]
    fn test_rectangle_ctor() {
        let rectangle = Rectangle::new(Coord2D::new(5, 0), Coord2D::new(0, 5));
        assert_eq!(rectangle.top_left, Coord2D::new(0, 0));
        assert_eq!(rectangle.bottom_right, Coord2D::new(5, 5));
    }

    #[test]
    fn test_rectangle_is_right_of_left_edge() {
        // left edge is (0, 0) - (0, 4)
        let rectangle = Rectangle::new(Coord2D::new(0, 0), Coord2D::new(4, 4));
        assert!(rectangle.is_right_of_left_edge(Coord2D::new(1, 1)));
        assert!(rectangle.is_right_of_left_edge(Coord2D::new(5, 5)));
        assert!(!rectangle.is_right_of_left_edge(Coord2D::new(-1, -1)));
        assert!(!rectangle.is_right_of_left_edge(Coord2D::new(0, 3)));
    }

    #[test]
    fn test_rectangle_is_left_of_right_edge() {
        // right edge is (4, 0) - (4, 4)
        let rectangle = Rectangle::new(Coord2D::new(0, 0), Coord2D::new(4, 4));
        assert!(rectangle.is_left_of_right_edge(Coord2D::new(1, 1)));
        assert!(rectangle.is_left_of_right_edge(Coord2D::new(-1, 5)));
        assert!(!rectangle.is_left_of_right_edge(Coord2D::new(4, 3)));
        assert!(!rectangle.is_left_of_right_edge(Coord2D::new(15, 3)));
    }

    #[test]
    fn test_rectangle_is_above_bottom_edge() {
        // bottom edge is (4, 0) - (4, 4)
        let rectangle = Rectangle::new(Coord2D::new(0, 0), Coord2D::new(4, 4));
        assert!(rectangle.is_above_bottom_edge(Coord2D::new(1, 1)));
        assert!(rectangle.is_above_bottom_edge(Coord2D::new(3, 3)));
        assert!(rectangle.is_above_bottom_edge(Coord2D::new(-1, -1)));
        assert!(!rectangle.is_above_bottom_edge(Coord2D::new(2, 4)));
        assert!(!rectangle.is_above_bottom_edge(Coord2D::new(1, 15)));
    }

    #[test]
    fn test_rectangle_is_below_top_edge() {
        // top edge is (0, 0) - (4, 0)
        let rectangle = Rectangle::new(Coord2D::new(0, 0), Coord2D::new(4, 4));
        assert!(rectangle.is_below_top_edge(Coord2D::new(1, 1)));
        assert!(rectangle.is_below_top_edge(Coord2D::new(5, 5)));
        assert!(!rectangle.is_below_top_edge(Coord2D::new(2, -4)));
        assert!(!rectangle.is_below_top_edge(Coord2D::new(-15, -15)));
    }

    #[test]
    fn test_rectangle_intersects() {
        let rectangle = Rectangle::new(Coord2D::new(0, 0), Coord2D::new(4, 4));
        assert!(rectangle.intersects(&Segment::new(Coord2D::new(-1, 2), Coord2D::new(2, 2))));
        assert!(!rectangle.intersects(&Segment::new(Coord2D::new(6, 6), Coord2D::new(10, 6))));
    }

    #[test]
    fn test_rectangle_enclosing_exterior_area() {
        let input = "1,0\n3,0\n3,6\n16,6\n16,0\n18,0\n18,9\n13,9\n13,7\n6,7\n6,9\n1,9";
        let coords = Coords::from_str(input).unwrap();

        // Rectangle from (3,0) to (18,6) has all corners ON the polygon boundary
        // but encloses a huge gap from x=4 to x=15 at y=1 to y=5
        let rectangle = Rectangle::new(Coord2D::new(3, 0), Coord2D::new(18, 6));

        let exterior_border = coords.exterior_border_coords();

        // This should return false because the rectangle encloses exterior area
        assert!(
            !is_rectangle_within_polygon(&rectangle, &coords, &exterior_border),
            "Rectangle (3,0)-(18,6) encloses exterior area and should be rejected"
        );
    }

    #[test]
    fn test_rectangle_contains() {
        let rectangle = Rectangle::new(Coord2D::new(0, 0), Coord2D::new(4, 4));
        assert!(rectangle.contains(Coord2D::new(1, 1)));
        assert!(rectangle.contains(Coord2D::new(2, 2)));
        assert!(rectangle.contains(Coord2D::new(3, 3)));
        assert!(!rectangle.contains(Coord2D::new(4, 4)));
        assert!(!rectangle.contains(Coord2D::new(5, 5)));
        assert!(!rectangle.contains(Coord2D::new(-1, -1)));
        assert!(!rectangle.contains(Coord2D::new(0, 4)));
        assert!(!rectangle.contains(Coord2D::new(4, 0)));
    }

    #[test]
    fn test_rectangle_area() {
        assert_eq!(
            Rectangle::new(Coord2D::new(2, 5), Coord2D::new(9, 7)).area(),
            24
        );
        assert_eq!(
            Rectangle::new(Coord2D::new(7, 1), Coord2D::new(11, 7)).area(),
            35
        );
        assert_eq!(
            Rectangle::new(Coord2D::new(7, 3), Coord2D::new(2, 3)).area(),
            6
        );
    }

    #[test]
    fn test_rectangles_max_area() {
        let coords = Coords::new(COORDS);
        let max_area = coords
            .rectangles()
            .map(|rectangle| rectangle.area())
            .max()
            .unwrap();
        assert_eq!(max_area, 50);
    }

    #[test]
    fn test_rectangle_corners() {
        let rectangle = Rectangle::new(Coord2D::new(0, 0), Coord2D::new(4, 4));
        assert_eq!(
            rectangle.corners(),
            [
                Coord2D::new(0, 0),
                Coord2D::new(4, 0),
                Coord2D::new(4, 4),
                Coord2D::new(0, 4),
            ]
        );
    }

    #[test]
    fn test_segment_orientation() {
        assert_eq!(
            Segment::new(Coord2D::new(0, 0), Coord2D::new(2, 0)).orientation(),
            Orientation::Horizontal
        );
        assert_eq!(
            Segment::new(Coord2D::new(0, 0), Coord2D::new(0, 2)).orientation(),
            Orientation::Vertical
        );
    }

    #[test]
    fn test_segment_range() {
        assert_eq!(
            Segment::new(Coord2D::new(0, 0), Coord2D::new(0, 2)).range(),
            0..=2
        );
        assert_eq!(
            Segment::new(Coord2D::new(10, 1), Coord2D::new(5, 1)).range(),
            5..=10
        );
    }

    #[test]
    fn test_segment_contains() {
        // Horizontal segment from (2, 5) to (9, 5)
        let horizontal = Segment::new(Coord2D::new(2, 5), Coord2D::new(9, 5));

        // Points on horizontal segment (including endpoints)
        assert!(horizontal.contains(Coord2D::new(2, 5))); // left endpoint
        assert!(horizontal.contains(Coord2D::new(9, 5))); // right endpoint
        assert!(horizontal.contains(Coord2D::new(5, 5))); // middle point
        assert!(horizontal.contains(Coord2D::new(7, 5))); // another middle point

        // Points not on horizontal segment
        assert!(!horizontal.contains(Coord2D::new(1, 5))); // left of segment
        assert!(!horizontal.contains(Coord2D::new(10, 5))); // right of segment
        assert!(!horizontal.contains(Coord2D::new(5, 4))); // below segment
        assert!(!horizontal.contains(Coord2D::new(5, 6))); // above segment
        assert!(!horizontal.contains(Coord2D::new(0, 0))); // far away

        // Vertical segment from (3, 1) to (3, 8)
        let vertical = Segment::new(Coord2D::new(3, 1), Coord2D::new(3, 8));

        // Points on vertical segment (including endpoints)
        assert!(vertical.contains(Coord2D::new(3, 1))); // bottom endpoint
        assert!(vertical.contains(Coord2D::new(3, 8))); // top endpoint
        assert!(vertical.contains(Coord2D::new(3, 4))); // middle point
        assert!(vertical.contains(Coord2D::new(3, 5))); // another middle point

        // Points not on vertical segment
        assert!(!vertical.contains(Coord2D::new(3, 0))); // below segment
        assert!(!vertical.contains(Coord2D::new(3, 9))); // above segment
        assert!(!vertical.contains(Coord2D::new(2, 5))); // left of segment
        assert!(!vertical.contains(Coord2D::new(4, 5))); // right of segment
        assert!(!vertical.contains(Coord2D::new(0, 0))); // far away

        // Test that segment order doesn't matter (range is normalized)
        let horizontal_reversed = Segment::new(Coord2D::new(9, 5), Coord2D::new(2, 5));
        assert!(horizontal_reversed.contains(Coord2D::new(5, 5)));

        let vertical_reversed = Segment::new(Coord2D::new(3, 8), Coord2D::new(3, 1));
        assert!(vertical_reversed.contains(Coord2D::new(3, 5)));
    }
}
