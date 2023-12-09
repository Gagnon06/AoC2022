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
    let output = part2(input);
    dbg!(output);
}

fn part2(input: &str) -> String {
    let lines: Vec<&str> = input.lines().collect();

    let priorities: Vec<u32> = (0..lines.len())
        .step_by(3)
        .filter_map(|line_idx| {
            let elf1 = lines[line_idx];
            let elf2 = lines[line_idx + 1];
            let elf3 = lines[line_idx + 2];

            let matching: Vec<char> = elf1
                .chars()
                .filter(|&a| elf2.find(a).is_some() && elf3.find(a).is_some())
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
    fn test_part2_example1() {
        let result = part2(
            "\
            vJrwpWtwJgWrhcsFMMfFFhFp\n\
            jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL\n\
            PmmdzqPrVvPwwTWBwg\n\
            wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn\n\
            ttgJtRGJQctTZtZT\n\
            CrZsJsPPZsGzwwsLwLmpwMDw",
        );
        assert_eq!(result, "70");
    }

    #[test]
    fn test_part2_real_data() {
        let input = include_str!("../../input1.txt");
        let result = part2(input);
        assert_eq!(result, "2525");
    }
}

#[bench]
fn bench_part2(b: &mut test::Bencher) {
    b.iter(|| {
        let input = include_str!("../../input1.txt");
        let _ = part2(input);
    });
}
