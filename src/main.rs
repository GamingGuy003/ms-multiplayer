use std::rc::Rc;

use cursive::{
    menu::{self, Tree},
    views::MenuPopup,
    Cursive, CursiveExt,
};

mod structs;
mod util;
fn main() -> std::io::Result<()> {
    let mut crs = Cursive::new();
    let options = vec![("Open", |_| println!("Opened"))];
    crs.add_layer(build_menu(options));
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

fn build_menu(options: Vec<(&str, fn(&mut Cursive) -> ())>) -> MenuPopup {
    let mut tree = Tree::new();
    options.iter().for_each(|option| {
        tree.add_item(menu::Item::leaf(option.0, option.1));
    });
    MenuPopup::new(Rc::new(tree))
}
