use std::collections::HashMap;

#[aoc_generator(day4)]
fn generate(input: &str) -> Vec<u32> {
    let mut input = input.split("-");
    let start: u32 = input.next().unwrap().parse().unwrap();
    let end: u32 = input.next().unwrap().parse().unwrap();

    (start..=end).collect()
}

#[aoc(day4, part1)]
fn part_one(input: &[u32]) -> usize {
    let valid_passwords: Vec<&u32> = input
        .iter()
        .filter(|entry| is_password_part_one(entry))
        .collect();

    valid_passwords.len()
}

fn is_password_part_one(input: &u32) -> bool {
    let mut has_double = false;
    let mut last_char = 0;
    for current_char in input.to_string().chars().map(|c| c.to_digit(10).unwrap()) {
        if current_char == last_char {
            has_double = true;
        }
        if current_char < last_char {
            return false;
        }
        last_char = current_char;
    }

    has_double
}

#[aoc(day4, part2)]
fn part_two(input: &[u32]) -> usize {
    let valid_passwords: Vec<&u32> = input
        .iter()
        .filter(|entry| is_password_part_one(entry))
        .filter(|entry| is_password_part_two(entry))
        .collect();

    valid_passwords.len()
}

fn is_password_part_two(input: &u32) -> bool {
    let duplicates: HashMap<char, usize> =
        input.to_string().chars().fold(HashMap::new(), |mut acc, i| {
            let entry = acc.entry(i).or_insert(0);
            *entry += 1;
            acc
        });

    duplicates.values().any(|&x| x == 2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day4_part1() {
        let result = part_one(&[111111,223450,123789]);

        assert_eq!(1, result);
    }

    #[test]
    fn day4_part2() {
        let result = part_two(&[112233,123444,111122,001234]);
        
        assert_eq!(true, is_password_part_two(&001234));
        assert_eq!(3, result);
    }    
}