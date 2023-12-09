#![feature(test)]

extern crate test;

use std::ops::Range;

fn main() {
    let input = include_str!("../../input1.txt");
    let output = part2(input);
    dbg!(output);
}

fn part2(input: &str) -> String {
    input
        .lines()
        .filter_map(|line| {
            let ranges = line
                .split(',')
                .map(|raw_range| {
                    raw_range.split('-').next().unwrap().parse::<u32>().unwrap()
                        ..raw_range.split('-').nth(1).unwrap().parse::<u32>().unwrap()
                })
                .collect::<Vec<Range<u32>>>();

            assert!(ranges.len() == 2);

            ((ranges[0].start <= ranges[1].end && ranges[0].end >= ranges[1].start)
                || (ranges[1].start <= ranges[0].end && ranges[1].end >= ranges[0].start))
                .then_some(ranges[0].clone())
        })
        .count()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part2_example1() {
        let result = part2(
            "\
            2-4,6-8\n\
            2-3,4-5\n\
            5-7,7-9\n\
            2-8,3-7\n\
            6-6,4-6\n\
            2-6,4-8",
        );
        assert_eq!(result, "4");
    }

    #[test]
    fn test_part2_real_data() {
        let input = include_str!("../../input1.txt");
        let result = part2(input);
        assert_eq!(result, "806");
    }
}

#[bench]
fn bench_part2(b: &mut test::Bencher) {
    b.iter(|| {
        let input = include_str!("../../input1.txt");
        let _ = part2(input);
    });
}
