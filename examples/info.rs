// notcurses::examples::info

use notcurses::*;

#[rustfmt::skip]
fn main() {
    println!("{:#?}", Notcurses::new_cli().unwrap().capabilities());
}
