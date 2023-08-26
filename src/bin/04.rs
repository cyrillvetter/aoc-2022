use itertools::Itertools;
use advent_of_code::Solution;

pub fn part_one(input: &str) -> Solution {
    let fully_contains_count = input
        .lines()
        .map(|l| l.split(','))
        .map(|s| s.map(|i| i.split('-').map(|j| j.parse::<u16>().unwrap()).collect_tuple::<(_, _)>().unwrap()).collect_vec())
        .filter(|r| fully_contains(r[0], r[1]))
        .count();

    Solution::USize(fully_contains_count)
}

pub fn part_two(input: &str) -> Solution {
    let overlaps_count = input
        .lines()
        .map(|l| l.split(','))
        .map(|s| s.map(|i| i.split('-').map(|j| j.parse::<u16>().unwrap()).collect_tuple::<(_, _)>().unwrap()).collect_vec())
        .filter(|r| overlaps(r[0], r[1]))
        .count();

    Solution::USize(overlaps_count)
}

fn fully_contains(a1: (u16, u16), a2: (u16, u16)) -> bool {
    a2.0 >= a1.0 && a2.1 <= a1.1 || a1.0 >= a2.0 && a1.1 <= a2.1
}

fn overlaps(a1: (u16, u16), a2: (u16, u16)) -> bool {
    a1.0 <= a2.1 && a2.0 <= a1.1
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 4);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = advent_of_code::read_file("inputs", 4);
        assert_eq!(part_one(&input), Solution::USize(530));
    }

    #[test]
    fn test_part2() {
        let input = advent_of_code::read_file("inputs", 4);
        assert_eq!(part_two(&input), Solution::USize(903));
    }
}
