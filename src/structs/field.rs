use std::{fmt::Display, process::exit};

use console::Style;
use rand::Rng;

use super::{
    cell::{Cell, State, Value},
    coords::Coords,
};

pub struct Field {
    size: Coords,
    generated: bool,
    bombc: usize,
    pub field: Vec<Vec<Cell>>,
}

impl Field {
    pub fn new(size: Coords, bombc: usize) -> Self {
        Self {
            size,
            generated: false,
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
        self.generated = true;
    }

    pub fn open_cell(&mut self, coords: Coords) {
        match self.get_cell(coords) {
            Some(cell) => {
                if !self.generated {
                    self.generate(coords);
                }
                match cell.value {
                    Value::Number(_) => match cell.state {
                        State::Closed => self.set_state(coords, State::Opened),
                        State::Opened => self.open_adjacened_safe(coords),
                        _ => {}
                    },
                    Value::Bomb => self.trigger_loss(),
                    Value::Empty => self.open_adjacened(coords),
                }
            }
            None => {}
        }
    }

    // sets state of cell as marked
    pub fn mark_cell(&mut self, coords: Coords) {
        if self.get_cell(coords).is_some() {
            self.set_state(coords, State::Marked);
            self.check_win();
        }
    }

    // sets given cell for coord
    fn set_cell(&mut self, coords: Coords, cell: Cell) {
        if self.get_cell(coords).is_some() {
            self.field[coords.y as usize][coords.x as usize] = cell;
        }
    }

    // returns cell or non if coord points nowhere
    fn get_cell(&mut self, coords: Coords) -> Option<Cell> {
        if (coords.x >= 0 && coords.y >= 0) && (coords.x < self.size.x && coords.y < self.size.y) {
            Some(self.field[coords.y as usize][coords.x as usize].clone())
        } else {
            None
        }
    }

    // opens adjacened empty fields
    fn open_adjacened(&mut self, coords: Coords) {
        if self.get_cell(coords).is_some() {
            self.set_state(coords, State::Opened);
            for y in -1..=1 {
                for x in -1..=1 {
                    if (x == 0 || y == 0) && !(x == 0 && y == 0) {
                        let new_coords = Coords::new(coords.x + x, coords.y + y);
                        match self.get_cell(new_coords) {
                            Some(cell) => match cell.value {
                                Value::Empty | Value::Number(_) => match cell.state {
                                    State::Closed => self.open_cell(new_coords),
                                    _ => {}
                                },
                                _ => {}
                            },
                            _ => {}
                        }
                    }
                }
            }
        }
    }

    // opens all fields around coord where all bombs have been marked
    fn open_adjacened_safe(&mut self, coords: Coords) {
        match self.get_value(coords) {
            Some(value) => match value {
                Value::Number(_) => {
                    for x in -1..=1 {
                        for y in -1..=1 {
                            let new_coords = Coords::new(coords.x + x, coords.y + y);
                            match self.get_cell(new_coords) {
                                Some(cell) => match cell.state {
                                    State::Closed => self.open_cell(new_coords),
                                    _ => {}
                                },
                                None => {}
                            }
                        }
                    }
                }
                _ => {}
            },
            None => {}
        }
    }

    // calculates surrounding bombs for coord
    fn get_num(&mut self, coords: Coords) -> u8 {
        let mut num = 0;
        for y in -1..=1 {
            for x in -1..=1 {
                match self.get_value(Coords::new(coords.x + x, coords.y + y)) {
                    Some(value) => match value {
                        Value::Bomb => num += 1,
                        _ => {}
                    },
                    _ => {}
                }
            }
        }
        num
    }

    // returns value of coord
    fn get_value(&mut self, coords: Coords) -> Option<Value> {
        match self.get_cell(coords) {
            Some(cell) => Some(cell.value),
            _ => None,
        }
    }

    // calculates all numbers in the field
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

    // sets the state for coord
    fn set_state(&mut self, coords: Coords, state: State) {
        if self.get_cell(coords).is_some() {
            self.field[coords.y as usize][coords.x as usize].state = state;
        }
    }

    // checks if all bombs have been marked
    fn check_win(&mut self) {
        let mut num_marked = 0;
        for x in 0..self.size.x {
            for y in 0..self.size.y {
                match self.get_cell(Coords::new(x, y)) {
                    Some(cell) => match cell.value {
                        Value::Bomb => match cell.state {
                            State::Marked => num_marked += 1,
                            _ => {}
                        },
                        _ => {}
                    },
                    _ => {}
                }
            }
        }
        if num_marked == self.bombc {
            self.trigger_win()
        }
    }

    // win function
    pub fn trigger_win(&mut self) {
        println!("{}", self);
        println!("You won!");
        exit(0)
    }

    // fail function
    pub fn trigger_loss(&mut self) {
        println!("{}", self);
        println!("You lost!");
        exit(0)
    }
}

impl Display for Field {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let italic = Style::new().italic();
        let colc = format!(
            "   |{}|",
            (0..self.size.x)
                .into_iter()
                .map(|num| format!(" {} ", italic.apply_to(num + 1)))
                .collect::<Vec<String>>()
                .join("|")
        );
        let sep = format!("{}+\n", vec!["---"; (self.size.x + 1) as usize].join("+"));
        write!(
            f,
            "{colc}\n{sep}{}{sep}",
            self.field
                .clone()
                .into_iter()
                .enumerate()
                .map(|(idx, line)| {
                    format!(
                        " {} |{}|\n",
                        italic.apply_to(idx + 1),
                        line.into_iter()
                            .map(|cell| format!("{}", cell))
                            .collect::<Vec<String>>()
                            .join("|"),
                    )
                })
                .collect::<Vec<String>>()
                .join(sep.as_str())
        )
    }
}
