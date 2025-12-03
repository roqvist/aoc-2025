#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let puzzle_input = include_str!("../puzzle_input.txt").trim();
    let sum: usize = puzzle_input.lines().map(find_largest_pair).sum();
    println!("Combined largest pairs: {}", sum);
    let sum: usize = puzzle_input.lines().map(|l| find_largest_sequence(l, 12)).sum();
    println!("Combined largest twelves: {}", sum);
    Ok(())
}

fn find_largest_pair(input: &str) -> usize {
    if input.len() < 2 {
        return 0;
    }

    let mut max = 0;

    let primary = input.chars();
    for (i, c) in primary.enumerate() {
        let base = c.to_digit(10).unwrap() as usize * 10;
        let secondary = input.chars().skip(i + 1);
        for j in secondary {
            let curr = base + j.to_digit(10).unwrap() as usize;
            if curr > max {
                max = curr;
            }
        }
    }

    max
}

fn find_largest_sequence(input: &str, sequence_size: usize) -> usize {
    let mut result = 0;
    if input.len() < sequence_size {
        return result;
    }

    let mut skip = 0;

    for i in (0..sequence_size).rev() {
        let slice = &input[skip..];
        let searches = slice.len().abs_diff(i + 1);

        let (max, idx) = (0..=searches).fold((0, 0), |(curr_max, curr_idx), k| {
            let val = slice.chars().nth(k).unwrap().to_digit(10).unwrap();
            if val > curr_max { (val, k) } else { (curr_max, curr_idx) }
        });

        let multiplier = 10_usize.pow(i as u32);
        result += max as usize * multiplier;
        skip += idx + 1;
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_known_puzzle() {
        let puzzle_input = include_str!("../puzzle_input_test.txt").trim();

        let sum: usize = puzzle_input.lines().map(find_largest_pair).sum();

        assert_eq!(sum, 357);
    }

    #[test]
    fn test_known_puzzle_part_2() {
        let puzzle_input = include_str!("../puzzle_input_test.txt").trim();

        let sum: usize = puzzle_input.lines().map(|l| find_largest_sequence(l, 12)).sum();

        assert_eq!(sum, 3121910778619);
    }

    #[test]
    fn test_known_sequence() {
        assert_eq!(find_largest_sequence("111119999999999999", 12), 999999999999);
    }
}
