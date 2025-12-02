use std::ops::RangeInclusive;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let puzzle_input = include_str!("../puzzle_input.txt").trim();

    let count = puzzle_input.split(',').fold(0, |acc, id_pair| {
        let ids: Vec<usize> = id_pair.split('-').map(|s| s.parse::<usize>().unwrap()).collect();
        let first = ids.first().unwrap().to_owned();
        let second = ids.last().unwrap().to_owned();
        acc + sequences(first..=second)
    });

    println!("Sum of Invalid IDs: {}", count);

    let extended_count = puzzle_input.split(',').fold(0, |acc, id_pair| {
        let ids: Vec<usize> = id_pair.split('-').map(|s| s.parse::<usize>().unwrap()).collect();
        let first = ids.first().unwrap().to_owned();
        let second = ids.last().unwrap().to_owned();
        acc + extended_sequences(first..=second)
    });

    println!("Sum of Invalid IDs with extended method is: {}", extended_count);
    Ok(())
}

pub fn sequences(r: RangeInclusive<usize>) -> usize {
    r.fold(0, |acc, x| acc + get_sequence(x))
}

pub fn extended_sequences(r: RangeInclusive<usize>) -> usize {
    r.fold(0, |acc, x| acc + get_extended_sequence(x))
}

pub fn get_sequence(input: usize) -> usize {
    let input_string = input.to_string();
    let length = input_string.len();
    if length < 2 {
        return 0;
    }

    let first: String = input_string.chars().take(length / 2).collect();
    let second: String = input_string.chars().skip(length / 2).collect();

    if first == second { input } else { 0 }
}

pub fn get_extended_sequence(input: usize) -> usize {
    let input_string = input.to_string();
    let length = input_string.len();
    if length < 2 {
        return 0;
    }

    for x in 1..length {
        let mut parts = input_string.as_bytes().chunks(x);
        let first = String::from_utf8_lossy(parts.next().unwrap());
        if parts.all(|part| String::from_utf8_lossy(part) == first) {
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

        let count = puzzle_input.split(',').fold(0, |acc, id_pair| {
            let ids: Vec<usize> = id_pair.split('-').map(|s| s.parse::<usize>().unwrap()).collect();
            let first = ids.first().unwrap().to_owned();
            let second = ids.last().unwrap().to_owned();
            acc + sequences(first..=second)
        });

        assert_eq!(count, 1227775554);
    }

    #[test]
    fn test_known_puzzle_part_2() {
        let puzzle_input = include_str!("../puzzle_input_test.txt").trim();

        let count = puzzle_input.split(',').fold(0, |acc, id_pair| {
            let ids: Vec<usize> = id_pair.split('-').map(|s| s.parse::<usize>().unwrap()).collect();
            let first = ids.first().unwrap().to_owned();
            let second = ids.last().unwrap().to_owned();
            acc + extended_sequences(first..=second)
        });

        assert_eq!(count, 4174379265);
    }
}
