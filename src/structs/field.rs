use std::fmt::Display;

use rand::Rng;

use super::{
    cell::{Cell, State, Value},
    coords::Coords,
};

pub struct Field {
    size: Coords,
    pub bombc: usize,
    pub field: Vec<Vec<Cell>>,
}

impl Field {
    pub fn new(size: Coords, bombc: usize) -> Self {
        Self {
            size,
            bombc,
            field: vec![
                vec![Cell::default(); size.x.try_into().unwrap_or_default()];
                size.y.try_into().unwrap_or_default()
            ],
        }
    }

    pub fn generate(&mut self, start_pos: Coords) {
        let mut rng = rand::thread_rng();
        let mut i = 0;
        // generate all bombs
        while i < self.bombc {
            // random coords
            let (rand_x, rand_y) = (rng.gen_range(0..self.size.x), rng.gen_range(0..self.size.y));
            // if at start pos try again
            if rand_x == start_pos.x && rand_y == start_pos.y {
                continue;
            }
            // if already bomb try again
            match self.field[rand_y as usize][rand_x as usize].value {
                Value::Bomb => continue,
                Value::Empty | Value::Number(_) => {
                    self.field[rand_y as usize][rand_x as usize].value = Value::Bomb;
                    i += 1;
                }
            }
        }
        self.calc_numbers();
    }

    pub fn set_cell(&mut self, coords: Coords, cell: Cell) {
        if self.get_cell(coords).is_some() {
            self.field[coords.y as usize][coords.x as usize] = cell;
        }
    }

    pub fn get_cell(&mut self, coords: Coords) -> Option<Cell> {
        if (coords.x >= 0 && coords.y >= 0) && (coords.x < self.size.x && coords.y < self.size.y) {
            Some(self.field[coords.y as usize][coords.x as usize].clone())
        } else {
            None
        }
    }

    fn get_num(&mut self, coords: Coords) -> u8 {
        let mut num = 0;
        for y in -1..=1 {
            for x in -1..=1 {
                if self.check_bomb(Coords::new(coords.x + x, coords.y + y)) {
                    num += 1;
                } else {
                }
            }
        }
        num
    }

    fn check_bomb(&mut self, coords: Coords) -> bool {
        match self.get_cell(coords) {
            Some(cell) => match cell.value {
                Value::Bomb => true,
                _ => false,
            },
            _ => false,
        }
    }

    fn calc_numbers(&mut self) {
        for x in 0..self.size.x {
            for y in 0..self.size.y {
                let coords = Coords::new(x, y);
                match self.field[y as usize][x as usize].value {
                    Value::Empty | Value::Number(_) => {
                        let value = match self.get_num(coords) {
                            0 => Value::Empty,
                            num => Value::Number(num),
                        };
                        self.set_cell(coords, Cell::new(State::Closed, value));
                    }
                    Value::Bomb => continue,
                }
            }
        }
    }
}

impl Display for Field {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let sep = format!("+{}+\n", vec!["---"; self.size.x as usize].join("+"));
        write!(
            f,
            "{sep}{}{sep}",
            self.field
                .clone()
                .into_iter()
                .map(|line| {
                    format!(
                        "|{}|\n",
                        line.into_iter()
                            .map(|cell| format!("{}", cell))
                            .collect::<Vec<String>>()
                            .join("|")
                    )
                })
                .collect::<Vec<String>>()
                .join(sep.as_str())
        )
    }
}
