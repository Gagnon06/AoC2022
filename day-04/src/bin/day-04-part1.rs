#![feature(test)]

extern crate test;

use std::ops::Range;

fn main() {
    let input = include_str!("../../input1.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> String {
    input.lines()
        .filter_map(|line| {
            let ranges = line.split(",")
                .map(|raw_range| {
                    raw_range.split("-").nth(0).unwrap().parse::<u32>().unwrap()..
                    raw_range.split("-").nth(1).unwrap().parse::<u32>().unwrap()
                })
                .collect::<Vec<Range<u32>>>();

            assert!(ranges.len() == 2);

            ((ranges[0].start >= ranges[1].start && ranges[0].end <= ranges[1].end) ||
             (ranges[1].start >= ranges[0].start && ranges[1].end <= ranges[0].end)).then_some(ranges[0].clone())
        })
        .count()
        .to_string()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_example1() {
        let result = part1(
            "\
            2-4,6-8\n\
            2-3,4-5\n\
            5-7,7-9\n\
            2-8,3-7\n\
            6-6,4-6\n\
            2-6,4-8");
        assert_eq!(result, "2");
    }

    #[test]
    fn test_part1_real_data() {
        let input = include_str!("../../input1.txt");
        let result = part1(input);
        assert_eq!(result, "413");
    }

}

#[bench]
fn bench_part1(b: &mut test::Bencher) {
    b.iter(|| {
        let input = include_str!("../../input1.txt");
        let _ = part1(input);
    });
}
