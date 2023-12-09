#![feature(test)]

extern crate test;

fn item_to_priority(item: char) -> u32 {
    if item as u32 >= 'a' as u32 {
        return (item as u32) - ('a' as u32) + 1;
    }

    (item as u32) - ('A' as u32) + 27
}

fn main() {
    let input = include_str!("../../input1.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> String {
    let priorities: Vec<u32> = input
        .lines()
        .filter_map(|line| {
            let compartments = line.split_at(line.len() / 2);
            let matching: Vec<char> = compartments
                .0
                .chars()
                .filter(|&a| compartments.1.find(a).is_some())
                .collect();
            (!matching.is_empty()).then_some(item_to_priority(matching[0]))
        })
        .collect();

    priorities.iter().sum::<u32>().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_example1() {
        let result = part1(
            "\
            vJrwpWtwJgWrhcsFMMfFFhFp\n\
            jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL\n\
            PmmdzqPrVvPwwTWBwg\n\
            wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn\n\
            ttgJtRGJQctTZtZT\n\
            CrZsJsPPZsGzwwsLwLmpwMDw",
        );
        assert_eq!(result, "157");
    }

    #[test]
    fn test_part1_real_data() {
        let input = include_str!("../../input1.txt");
        let result = part1(input);
        assert_eq!(result, "7581");
    }
}

#[bench]
fn bench_part1(b: &mut test::Bencher) {
    b.iter(|| {
        let input = include_str!("../../input1.txt");
        let _ = part1(input);
    });
}
