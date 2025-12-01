mod dial;
use dial::Dial;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let puzzle_input = include_str!("../puzzle_input.txt").trim();

    let mut times_on_stopped_zero: u32 = 0;
    let mut times_passed_zero: u32 = 0;

    let on_movement_done = |position: u32| {
        if position == 0 {
            times_on_stopped_zero += 1;
        }
    };

    let on_movement = |position: u32| {
        if position == 0 {
            times_passed_zero += 1;
        }
    };

    let mut dial = Dial::from_position(50, 99, on_movement_done, on_movement);

    for line in puzzle_input.lines() {
        dial.turn_from_input(line);
    }

    println!("Landed on zero: {}", times_on_stopped_zero);
    println!("Passed zero: {}", times_passed_zero);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_known_puzzle() {
        let puzzle_input = include_str!("../puzzle_input_test.txt").trim();
        let mut times_on_stopped_zero: u32 = 0;

        let on_movement = |_| {};
        let on_movement_done = |position: u32| {
            if position == 0 {
                times_on_stopped_zero += 1;
            }
        };

        let mut dial = Dial::from_position(50, 99, on_movement_done, on_movement);

        for line in puzzle_input.lines() {
            dial.turn_from_input(line);
        }

        assert_eq!(times_on_stopped_zero, 3);
    }

    #[test]
    fn test_known_puzzle_2() {
        let puzzle_input = include_str!("../puzzle_input_test.txt").trim();
        let mut times_passed_zero: u32 = 0;

        let on_movement_done = |_| {};
        let on_movement = |position: u32| {
            if position == 0 {
                times_passed_zero += 1;
            }
        };

        let mut dial = Dial::from_position(50, 99, on_movement_done, on_movement);

        for line in puzzle_input.lines() {
            dial.turn_from_input(line);
        }

        assert_eq!(times_passed_zero, 6);
    }
}
