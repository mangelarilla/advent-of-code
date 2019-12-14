mod computer;

use computer::Wire;

#[aoc_generator(day3)]
fn generator(input: &str) -> Vec<Wire> {
    input
        .lines()
        .map(|instructions| Wire::from_str(instructions))
        .collect()
}

#[aoc(day3, part1)]
fn part_one(input: &[Wire]) -> i32 {
    let first_wire = input[0].clone();
    let second_wire = input[1].clone();
    let intersections = first_wire.find_intersections(&second_wire);
    intersections
        .into_iter()
        .map(|intersection| intersection.0.abs() + intersection.1.abs())
        .min()
        .unwrap()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_one() {
        let result = part_one(&generator("\
U7,R6,D4,L4
R8,U5,L5,D3"));

        assert_eq!(6, result);
    }
}