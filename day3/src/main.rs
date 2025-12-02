#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let puzzle_input = include_str!("../puzzle_input.txt").trim();
    let sum: usize = puzzle_input.lines().map(find_largest_pair).sum();
    println!("Combined largest pairs: {}", sum);
    let sum: usize = puzzle_input.lines().map(find_largest_twelve).sum();
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

fn find_largest_twelve(input: &str) -> usize {
    if input.len() < 12 {
        return 0;
    }

    let mut chunk = 12;
    let mut max = 0;
    let mut multiplier: usize = 100000000000;
    let mut skip = 0;

    while chunk > 0 {
        let working_set = &input[skip..];
        let search_times = working_set.len().abs_diff(chunk);
        let mut local_max = 0;
        let mut local_skip = 0;
        for i in 0..=search_times {
            let val = working_set.chars().nth(i).unwrap().to_digit(10).unwrap();
            if val > local_max {
                local_max = val;
                local_skip = i + 1;
            }
        }
        max += local_max as usize * multiplier;
        multiplier /= 10;
        skip += local_skip;
        chunk -= 1;
    }

    max
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

        let sum: usize = puzzle_input.lines().map(find_largest_twelve).sum();

        assert_eq!(sum, 3121910778619);
    }

    #[test]
    fn test_known_sequence() {
        assert_eq!(find_largest_twelve("111119999999999999"), 999999999999);
    }
}
