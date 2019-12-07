use line_intersection::{LineInterval, LineRelation};
use geo::Line;

#[aoc_generator(day3)]
fn generator(input: &str) -> (Vec<String>, Vec<String>) {
    input
        .lines()
        .map(|instructions| to_lines(line.split(",").collect()))
    

}

#[aoc(day3, part1)]
fn part_one(input: &[String]) -> i32 {
    0
}

#[derive(Debug)]
struct Wire {
    lines: Vec<geo::Line<f64>>
}

impl Wire {
    fn from_directions() -> Self {
        Wire { 
            
        }
    }
}