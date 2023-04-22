use std::fmt::Display;

#[derive(Debug, Clone)]
pub enum Value {
    Empty,
    Bomb,
    Number(u8),
}

#[derive(Debug, Clone)]
pub enum State {
    Opened,
    Closed,
    Marked,
}

#[derive(Debug, Clone)]
pub struct Cell {
    pub state: State,
    pub value: Value,
}

impl Default for Cell {
    fn default() -> Self {
        Self {
            state: State::Closed,
            value: Value::Empty,
        }
    }
}

impl Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            " {} ",
            match self.state {
                State::Closed => String::from("■"),
                State::Marked => String::from("⚑"),
                State::Opened => match self.value {
                    Value::Empty => String::from(" "),
                    Value::Bomb => String::from("⚠"),
                    Value::Number(a) => a.to_string(),
                },
            }
        )
    }
}

impl Cell {
    pub fn new(state: State, value: Value) -> Self {
        Self { state, value }
    }
}
