use cursive::{menu, views, Cursive, CursiveExt};
use structs::coords::Coords;
use util::funcs::get_input;

use crate::structs::field::Field;

mod structs;
mod util;
fn main() -> std::io::Result<()> {
    let mut crs = Cursive::new();
    let mut options = menu::Tree::new();
    options.add_item(menu::Item::leaf("Option1", |dings| {
        let dialog = views::Dialog::new()
            .content(views::TextView::new("Hello!"))
            .button("Quit", |s| s.quit());
        dings.add_layer(dialog);
        println!("Option1 selected")
    }));
    options.add_item(menu::Item::leaf("Option2", |_| {
        println!("Option2 selected")
    }));
    let menu = views::MenuPopup::new(options.into());
    crs.add_layer(menu);
    crs.run();
    /*
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

    println!("{field}");
    loop {
        let x = get_input(Some("x: ".to_owned()))?
            .parse::<i32>()
            .unwrap_or_default();
        let y = get_input(Some("y: ".to_owned()))?
            .parse::<i32>()
            .unwrap_or_default();
        let action = get_input(Some("action: ".to_owned()))?;
        let coords = Coords::new(x - 1, y - 1);
        match action.as_str() {
            "m" => field.mark_cell(coords),
            "o" => field.open_cell(coords),
            other => println!("{other}"),
        }

        print!("{field}");
    }
    */
    Ok(())
}
