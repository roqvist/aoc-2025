pub struct Dial<F, G>
where
    F: FnMut(u32),
    G: FnMut(u32),
{
    pub max_value: u32,
    pub position: u32,
    pub on_movement_done: F,
    pub on_movement: G,
}

impl<F, G> Dial<F, G>
where
    F: FnMut(u32),
    G: FnMut(u32),
{
    pub fn from_position(position: u32, max_value: u32, on_movement_done: F, on_movement: G) -> Self {
        Dial {
            position,
            max_value,
            on_movement_done,
            on_movement,
        }
    }
    pub fn turn_left(&mut self, times: u32) {
        (0..times).for_each(|_| {
            self.position = if self.position == 0 { self.max_value } else { self.position - 1 };
            (self.on_movement)(self.position);
        });

        (self.on_movement_done)(self.position);
    }
    pub fn turn_right(&mut self, times: u32) {
        (0..times).for_each(|_| {
            self.position = if self.position == self.max_value { 0 } else { self.position + 1 };
            (self.on_movement)(self.position);
        });

        (self.on_movement_done)(self.position);
    }

    pub fn turn_from_input(&mut self, input: &str) {
        let (direction, count_str) = input.split_at(1);

        if let Ok(times) = count_str.trim().parse::<u32>() {
            match direction {
                "L" => self.turn_left(times),
                "R" => self.turn_right(times),
                _ => {}
            }
        }
    }
}
