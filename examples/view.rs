//! Demonstrates mutable views.

use std::io::stdout;

use tuit::prelude::*;
use tuit::style::{Ansi4, Colour};
use tuit::terminal::ConstantSize;
use tuit::widgets::builtins::sweeper::Sweeper;
use tuit::widgets::builtins::Uv;
use tuit::widgets::Rectangle;

#[cfg(not(feature = "ansi_terminal"))]
fn main() {
    println!("You must apply the ansi_terminal feature to view this example. Use `cargo --features ansi_terminal`");
}

// a regression causes this
// lint to return a false-positive,
// so I disabled it.
#[allow(clippy::needless_borrows_for_generic_args)]
#[cfg(feature = "ansi_terminal")]
fn main() {
    let mut terminal: ConstantSize<100, 20> = ConstantSize::new();

    let sweeper = Sweeper::of_colour(Colour::Ansi16(Ansi4::Cyan));

    sweeper.drawn(&mut terminal).expect("Should not fail");

    let uv = Uv;

    uv.drawn(&mut terminal).ok();

    let mut view = terminal.view_mut(Rectangle::of_size(30, 12).to((5, 5)));
    let mut x: usize = 0;

    #[allow(clippy::cast_possible_truncation)]
    for character in view.cells_mut() {
        print!("{character}");
        character.style.bg_colour = Some(Colour::Rgb24(0, 255-(x as u8), x as u8));
        x += 1;
    }

    println!("\nI counted {x} cells");

    terminal.display(stdout()).unwrap();
}