use array2d::Array2D;
use std::{fmt, num::ParseIntError, str::FromStr};
#[derive(Debug)]
pub struct Point {
    x: usize,
    y: usize,
}

impl FromStr for Point {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let coords: Vec<&str> = s.split(',').collect();

        let x_fromstr = coords[0].parse::<usize>()?;
        let y_fromstr = coords[1].parse::<usize>()?;

        Ok(Point {
            x: x_fromstr,
            y: y_fromstr,
        })
    }
}
#[derive(Debug)]
pub struct LineSegment {
    start: Point,
    end: Point,
}

impl LineSegment {
    fn find_points(&self, include_diagonals: bool) -> Vec<Point> {
        let x_range: Vec<_> = if self.start.x < self.end.x {
            (self.start.x..=self.end.x).collect()
        } else {
            (self.end.x..=self.start.x).rev().collect()
        };

        let y_range: Vec<_> = if self.start.y < self.end.y {
            (self.start.y..=self.end.y).collect()
        } else {
            (self.end.y..=self.start.y).rev().collect()
        };

        if self.start.x == self.end.x {
            return y_range
                .into_iter()
                .map(|y| Point {
                    x: self.start.x,
                    y: y,
                })
                .collect();
        }
        if self.start.y == self.end.y {
            return x_range
                .into_iter()
                .map(|x| Point {
                    x: x,
                    y: self.start.y,
                })
                .collect();
        }
        if include_diagonals {
            return x_range
                .into_iter()
                .enumerate()
                .map(|(i, x)| Point {
                    x: x,
                    y: y_range[i],
                })
                .collect();
        } else {
            return vec![];
        }
    }
}

pub struct ParseLineSegmentError;

impl fmt::Display for ParseLineSegmentError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error occured parsing line") // user-facing output
    }
}

impl fmt::Debug for ParseLineSegmentError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{ file: {}, line: {} }}", file!(), line!()) // programmer-facing output
    }
}

impl FromStr for LineSegment {
    type Err = ParseLineSegmentError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split(" -> ");
        let start = match split.next() {
            Some(point_string) => match point_string.parse() {
                Ok(point) => point,
                Err(_) => return Err(ParseLineSegmentError),
            },
            None => return Err(ParseLineSegmentError),
        };
        let end = match split.next() {
            Some(point_string) => match point_string.parse() {
                Ok(point) => point,
                Err(_) => return Err(ParseLineSegmentError),
            },
            None => return Err(ParseLineSegmentError),
        };
        Ok(LineSegment { start, end })
    }
}

#[aoc_generator(day5)]
pub fn input_generator(input: &str) -> Vec<LineSegment> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

#[aoc(day5, part1)]
fn part1(input: &Vec<LineSegment>) -> usize {
    let mut map: Array2D<usize> = Array2D::filled_with(0, 1000, 1000);

    for line_segement in input {
        for point in line_segement.find_points(false) {
            map.set(point.x, point.y, map.get(point.x, point.y).unwrap() + 1)
                .unwrap();
        }
    }

    map.as_rows().into_iter().fold(0, |acc, row| {
        acc + row.into_iter().filter(|num| *num > 1).count()
    })
}

#[aoc(day5, part2)]
fn part2(input: &Vec<LineSegment>) -> usize {
    let mut map: Array2D<usize> = Array2D::filled_with(0, 1000, 1000);

    for line_segement in input {
        for point in line_segement.find_points(true) {
            map.set(point.x, point.y, map.get(point.x, point.y).unwrap() + 1)
                .unwrap();
        }
    }

    map.as_rows().into_iter().fold(0, |acc, row| {
        acc + row.into_iter().filter(|num| *num > 1).count()
    })
}
