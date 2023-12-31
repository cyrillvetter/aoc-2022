use itertools::Itertools;
use regex::Regex;
use advent_of_code::Solution;

pub fn part_one(input: &str) -> Solution {
    let parts = input.split_once("\n\n").unwrap();
    let number_match = Regex::new("\\d+").unwrap();
    let mut crates = build_crates(parts.0);

    for l in parts.1.lines() {
        let instr = build_instructions(l, &number_match);

        let remove_index = crates[instr.from - 1].len() - instr.amount;
        let mut pulled_crates = crates[instr.from - 1].split_off(remove_index);

        pulled_crates.reverse();
        crates[instr.to - 1].append(&mut pulled_crates);
    }

    let top_crates: String = crates.iter().map(|s| s.last().unwrap()).collect();
    Solution::Str(top_crates)
}

pub fn part_two(input: &str) -> Solution {
    let parts = input.split_once("\n\n").unwrap();
    let number_match = Regex::new("\\d+").unwrap();
    let mut crates = build_crates(parts.0);

    for l in parts.1.lines() {
        let instr = build_instructions(l, &number_match);

        let remove_index = crates[instr.from - 1].len() - instr.amount;
        let mut pulled_crates = crates[instr.from - 1].split_off(remove_index);
        crates[instr.to - 1].append(&mut pulled_crates);
    }

    let top_crates: String = crates.iter().map(|s| s.last().unwrap()).collect();
    Solution::Str(top_crates)
}

struct Instruction {
    amount: usize,
    from: usize,
    to: usize,
}

fn build_crates(crate_part: &str) -> Vec<Vec<char>> {
    let input_stack = crate_part.lines().rev().collect_vec();
    let mut crates: Vec<Vec<char>> = Vec::new();

    for (i, c) in input_stack[0].chars().enumerate() {
        if c.is_whitespace() {
            continue;
        }

        let mut stack: Vec<char> = Vec::new();
        for l in input_stack.iter().skip(1) {
            if let Some(crate_char) = l.chars().nth(i) {
                if !crate_char.is_whitespace() {
                    stack.push(crate_char);
                }
            }
        }

        crates.push(stack);
    }

    crates
}

fn build_instructions(line: &str, pattern: &Regex) -> Instruction {
    let instr = pattern
        .find_iter(line)
        .map(|c| c.as_str().parse::<usize>().unwrap())
        .collect_vec();

    Instruction {
        amount: instr[0],
        from: instr[1],
        to: instr[2],
    }
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 5);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = advent_of_code::read_file("inputs", 5);
        assert_eq!(part_one(&input), Solution::Str(String::from("ZSQVCCJLL")));
        assert_eq!(part_two(&input), Solution::Str(String::from("QZFJRWHGS")));
    }
}
