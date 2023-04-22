use structs::coords::Coords;
use util::funcs::get_input;

use crate::structs::field::Field;

mod structs;
mod util;
fn main() -> std::io::Result<()> {
    let sizex = get_input(Some("Enter width: ".to_owned()))?
        .parse::<i32>()
        .unwrap_or_default();
    let sizey = get_input(Some("Enter height: ".to_owned()))?
        .parse::<i32>()
        .unwrap_or_default();
    let mut bombc;
    loop {
        bombc = get_input(Some(format!(
            "Enter bomb count (must be < {}): ",
            (sizex * sizey) - 1
        )))?
        .parse::<usize>()
        .unwrap_or_default();
        if bombc <= ((sizex * sizey) - 1).try_into().unwrap_or_default() {
            break;
        }
        println!("Invalid input, try again");
    }

    let mut field = Field::new(Coords::new(sizex, sizey), bombc);

    field.generate(Coords::new(3, 2));
    println!("{field}");
    loop {
        let x = get_input(Some("x: ".to_owned()))?
            .parse::<i32>()
            .unwrap_or_default();
        let y = get_input(Some("y: ".to_owned()))?
            .parse::<i32>()
            .unwrap_or_default();
        field.open_cell(Coords::new(x, y));
        println!("{field}");
    }
}
