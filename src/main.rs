use std::rc::Rc;

use cursive::{
    menu::{self, Tree},
    views::MenuPopup,
    Cursive, CursiveExt,
};
use structs::{coords::Coords, field::Field};

mod structs;
mod util;
fn main() -> std::io::Result<()> {
    let mut crs = Cursive::new();
    //crs.add_layer(action_menu());
    crs.add_layer(draw_window(Field::new(Coords::new(9, 9), 10)));
    crs.run();
    Ok(())
}

fn get_size(screen: &mut Cursive, x: &str) {
    screen.add_layer(
        cursive::views::Dialog::around(cursive::views::TextView::new(x)).button("OK", |screen| {
            screen.pop_layer();
        }),
    );
}

fn action_menu() -> MenuPopup {
    let mut tree = Tree::new();
    tree.add_item(menu::Item::leaf("Open", |scr| {
        let mut dialog = cursive::views::Dialog::around(cursive::views::TextView::new("opened"));
        dialog.add_button("OK", |scr| {
            scr.pop_layer();
        });
        scr.add_layer(dialog);
    }));
    tree.add_item(menu::Item::leaf("Marked", |scr| {
        let mut dialog = cursive::views::Dialog::around(cursive::views::TextView::new("marked"));
        dialog.add_button("OK", |scr| {
            scr.pop_layer();
        });
        scr.add_layer(dialog);
    }));
    tree.add_item(menu::Item::leaf("Cancel", |scr| scr.quit()));
    MenuPopup::new(Rc::new(tree))
}

fn draw_window(field: Field) -> cursive::views::Canvas<String> {
    cursive::views::Canvas::new(String::new())
        .with_draw(move |text: &String, printer| {
            printer.print((0, 0), format!("{}", field).as_str());
        })
        .with_required_size(|text, _constraints| (text.len(), 10).into())
}
