use std::collections::HashSet;

#[derive(Debug, PartialEq)]
pub enum Instruction {
    Up(i32),
    Right(i32),
    Down(i32),
    Left(i32)
}

#[derive(Debug, Clone)]
pub struct Wire {
    points: HashSet<(i32, i32)>,
    points_in_order: Vec<(i32, i32)>
}

impl Wire {
    pub fn from_str(instructions: &str) -> Self {
        let instructions = parse_instructions(instructions);

        let mut points = HashSet::new();
        let mut points_in_order = Vec::new();
        let mut last_vertex = (0,0);
        for instruction in instructions.into_iter() {
            let next_points = calculate_next_points(last_vertex, instruction);
            last_vertex = next_points.last().unwrap().clone();
            for point in next_points.into_iter() {
                points.insert(point);
                points_in_order.push(point);
            }
        }

        Wire { points: points, points_in_order: points_in_order}
    }

    pub fn find_intersections<'a>(&'a self, wire: &'a Wire) -> Vec<&'a (i32,i32)> {
        self.points
            .intersection(&wire.points)
            .collect()
    }

    pub fn steps_to(&self, point: &(i32,i32)) -> usize {
        self.points_in_order
            .iter()
            .position(|v| v == point)
            .unwrap() + 1
    }
}

fn calculate_next_points(starting_point: (i32, i32), instruction: Instruction) -> Vec<(i32,i32)> {
    let mut points = Vec::new();

    match instruction {
        Instruction::Up(distance) => {
            for counter in 1..=distance {
                points.push((starting_point.0, starting_point.1 + counter));
            }
        },
        Instruction::Down(distance) => {
            for counter in 1..=distance {
                points.push((starting_point.0, starting_point.1 - counter));
            }
        },
        Instruction::Right(distance) => {
            for counter in 1..=distance {
                points.push((starting_point.0 + counter, starting_point.1));
            }
        },
        Instruction::Left(distance) => {
            for counter in 1..=distance {
                points.push((starting_point.0 - counter, starting_point.1));
            }
        }
    }
    
    points
}

fn parse_instructions(instructions: &str) -> Vec<Instruction> {
    instructions
        .split(",")
        .map(|instruction| Instruction::from_str(instruction))
        .collect()
}

impl Instruction {
    pub fn from_str(instruction: &str) -> Self {
        let distance = instruction[1..].parse().unwrap();
        match &instruction[..1] {
            "R" => Instruction::Right(distance),
            "L" => Instruction::Left(distance),
            "D" => Instruction::Down(distance),
            "U" => Instruction::Up(distance),
            _ => panic!("bad instruction")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn instruction_from_str() {
        let simple_instruction = Instruction::from_str("U7");
        let complex_instruction = Instruction::from_str("R75");

        assert_eq!(Instruction::Up(7), simple_instruction);
        assert_eq!(Instruction::Right(75), complex_instruction);    
    }
}