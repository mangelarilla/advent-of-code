#[aoc_generator(day2)]
fn generator_input(input: &str) -> Vec<i32> {
    input
        .split(",")
        .map(|a| a.parse::<i32>().unwrap())
        .collect()
}

#[aoc(day2, part1)]
fn part_one(input: &[i32]) -> i32 {

	let mut input = input.to_vec();
	input[1] = 12;
	input[2] = 2;

	let mut current_index = 0;

	loop {
		match process_index(current_index, &input) {
			Opcode::Add(first, second, output) => input[output] = input[first] + input[second],
			Opcode::Multiply(first, second, output) => input[output] = input[first] * input[second],
			Opcode::Halt => break,
			Opcode::Unspecified => panic!("Unspecified")
		};
		current_index += 4;
	}

	input[0]
}

const EXPECTED_OUTPUT: i32 = 19690720;

#[aoc(day2, part2)]
fn part_two(input: &[i32]) -> i32 {
	for pos1 in 0..99 {
		for pos2 in 0..99 {
			if try_pos(pos1, pos2, input) {
				return 100 * pos1 + pos2;
			}
		}
	}

	panic!("Not found!");
}

fn try_pos(pos1: i32, pos2: i32, input: &[i32]) -> bool {
	let mut input = input.to_vec();
	input[1] = pos1;
	input[2] = pos2;

	let mut current_index = 0;

	loop {
		match process_index(current_index, &input) {
			Opcode::Add(first, second, output) => input[output] = input[first] + input[second],
			Opcode::Multiply(first, second, output) => input[output] = input[first] * input[second],
			Opcode::Halt => break,
			Opcode::Unspecified => panic!("Unspecified")
		};
		current_index += 4;
	}

	input[0] == EXPECTED_OUTPUT
}


fn process_index(index: usize, collection: &[i32]) -> Opcode {
	let opcode = collection[index];

	let first_operand_index = index + 1;
	let second_operand_index = index + 2;
	let output_index = index + 3;

	if opcode == 1 {
		return Opcode::Add(collection[first_operand_index] as usize, collection[second_operand_index] as usize, collection[output_index] as usize);
	}

	if opcode == 2 {
		return Opcode::Multiply(collection[first_operand_index] as usize, collection[second_operand_index] as usize, collection[output_index] as usize);
	}

	if opcode == 99 {
		return Opcode::Halt;
	}

	Opcode::Unspecified
}

enum Opcode {
	Add(usize,usize,usize),
	Multiply(usize,usize,usize),
	Halt,
	Unspecified
}