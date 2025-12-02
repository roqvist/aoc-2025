use rayon::prelude::*;
use std::ops::RangeInclusive;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let puzzle_input = include_str!("../puzzle_input.txt").trim();

    let count = puzzle_input
        .par_split(',')
        .fold(
            || 0,
            |acc, id_pair| {
                let (first_str, second_str) = id_pair.split_once('-').unwrap();
                let first = first_str.parse::<usize>().unwrap();
                let second = second_str.parse::<usize>().unwrap();
                acc + sequences(first..=second)
            },
        )
        .reduce(|| 0, |a, b| a + b);

    println!("Sum of Invalid IDs: {}", count);

    let extended_count = puzzle_input
        .par_split(',')
        .fold(
            || 0,
            |acc, id_pair| {
                let (first_str, second_str) = id_pair.split_once('-').unwrap();
                let first = first_str.parse::<usize>().unwrap();
                let second = second_str.parse::<usize>().unwrap();
                acc + extended_sequences(first..=second)
            },
        )
        .reduce(|| 0, |a, b| a + b);

    println!("Sum of Invalid IDs with extended method is: {}", extended_count);
    Ok(())
}

pub fn sequences(r: RangeInclusive<usize>) -> usize {
    r.into_par_iter().map(get_sequence).sum()
}

pub fn extended_sequences(r: RangeInclusive<usize>) -> usize {
    r.into_par_iter().map(get_extended_sequence).sum()
}

pub fn get_sequence(input: usize) -> usize {
    let input_string = input.to_string();
    let length = input_string.len();
    if length < 2 || !length.is_multiple_of(2) {
        return 0;
    }

    let mid = length / 2;
    let (first, second) = input_string.split_at(mid);

    if first == second { input } else { 0 }
}

pub fn get_extended_sequence(input: usize) -> usize {
    let input_string = input.to_string();
    let length = input_string.len();
    if length < 2 {
        return 0;
    }

    for chunk_size in 1..length {
        if !length.is_multiple_of(chunk_size) {
            continue;
        }

        let mut chunks = input_string.as_bytes().chunks(chunk_size);
        if let Some(first_chunk) = chunks.next()
            && chunks.all(|chunk| chunk == first_chunk)
        {
            return input;
        }
    }

    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sequence_test() {
        assert_eq!(get_sequence(4545), 4545);
        assert_eq!(get_sequence(454), 0);
        assert_eq!(get_sequence(99), 99);
        assert_eq!(get_sequence(1188511885), 1188511885);
    }

    #[test]
    fn extended_sequence_test() {
        assert_eq!(get_extended_sequence(4545), 4545);
        assert_eq!(get_extended_sequence(454), 0);
        assert_eq!(get_extended_sequence(99), 99);
        assert_eq!(get_extended_sequence(1188511885), 1188511885);
        assert_eq!(get_extended_sequence(999), 999);
        assert_eq!(get_extended_sequence(824824824), 824824824);
        assert_eq!(get_extended_sequence(111), 111);
        assert_eq!(get_extended_sequence(565656), 565656);
        assert_eq!(get_extended_sequence(2121212121), 2121212121);
        assert_eq!(get_extended_sequence(12341234123412341234), 12341234123412341234);
    }

    #[test]
    fn test_known_puzzle() {
        let puzzle_input = include_str!("../puzzle_input_test.txt").trim();

        let count = puzzle_input
            .par_split(',')
            .fold(
                || 0,
                |acc, id_pair| {
                    let (first_str, second_str) = id_pair.split_once('-').unwrap();
                    let first = first_str.parse::<usize>().unwrap();
                    let second = second_str.parse::<usize>().unwrap();
                    acc + sequences(first..=second)
                },
            )
            .reduce(|| 0, |a, b| a + b);

        assert_eq!(count, 1227775554);
    }

    #[test]
    fn test_known_puzzle_part_2() {
        let puzzle_input = include_str!("../puzzle_input_test.txt").trim();

        let count = puzzle_input
            .par_split(',')
            .fold(
                || 0,
                |acc, id_pair| {
                    let (first_str, second_str) = id_pair.split_once('-').unwrap();
                    let first = first_str.parse::<usize>().unwrap();
                    let second = second_str.parse::<usize>().unwrap();
                    acc + extended_sequences(first..=second)
                },
            )
            .reduce(|| 0, |a, b| a + b);

        assert_eq!(count, 4174379265);
    }
}
