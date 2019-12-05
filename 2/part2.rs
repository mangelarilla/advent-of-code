const INPUT: [i32; 145] = [1,0,0,3,1,1,2,3,1,3,4,3,1,5,0,3,2,1,13,19,2,9,19,23,1,23,6,27,1,13,27,31,1,31,10,35,1,9,35,39,1,39,9,43,2,6,43,47,1,47,5,51,2,10,51,55,1,6,55,59,2,13,59,63,2,13,63,67,1,6,67,71,1,71,5,75,2,75,6,79,1,5,79,83,1,83,6,87,2,10,87,91,1,9,91,95,1,6,95,99,1,99,6,103,2,103,9,107,2,107,10,111,1,5,111,115,1,115,6,119,2,6,119,123,1,10,123,127,1,127,5,131,1,131,2,135,1,135,5,0,99,2,0,14,0];
const EXPECTED_OUTPUT: i32 = 19690720;

fn main() {
	for pos1 in 0..99 {
		for pos2 in 0..99 {
			if try_pos(pos1, pos2) {
				println!("result = {}", 100 * pos1 + pos2);
				break;
			}
		}
	}
}

fn try_pos(pos1: i32, pos2: i32) -> bool {
	let mut input = INPUT.clone();
	input[1] = pos1;
	input[2] = pos2;

	let mut current_index = 0;

	loop {
		match process_index(current_index, &input) {
			Opcode::Add(first, second, output) => input[output] = input[first] + input[second],
			Opcode::Multiply(first, second, output) => input[output] = input[first] * input[second],
			Opcode::Halt => break,
			Opcode::Unspecified => println!("Unspecified")
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