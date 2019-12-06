#[aoc_generator(day1)]
fn generator_input(input: &str) -> Vec<i32> {
    input.lines().map(|a| a.parse::<i32>().unwrap()).collect()
}

#[aoc(day1, part1)]
fn part_one(input: &[i32]) -> i32 {
    let mut total_fuel = 0;

    for module_mass in input.iter() {
        total_fuel += calculate_fuel(module_mass);
    }

    total_fuel
}

#[aoc(day1, part2)]
fn part_two(input: &[i32]) -> i32 {
    let mut total_fuel = 0;

    for module_mass in input.iter() {
        let mut remaining_mass = *module_mass;
        loop {
            let fuel = calculate_fuel(&remaining_mass);
            if fuel <= 0 {
                break;
            }

            remaining_mass = fuel;
            total_fuel += fuel;

        }
    }

    total_fuel
}


fn calculate_fuel(mass: &i32) -> i32 {
    (mass / 3) -2
}