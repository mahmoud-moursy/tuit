//! This example shows how to use the [`Stacked`] widget.

use std::io::stdout;
use tuit::terminal::ConstantSize;
use tuit::widgets::builtins::{Stacked, Sweeper, Text, WithLayout};
use tuit::prelude::*;
use tuit::style::{Ansi4, Colour};

fn main() {
    let sweeper = Sweeper::of_colour(Colour::Ansi16(Ansi4::Cyan));

    let top_text = Text::new("Top widget");
    let bottom_text = Text::new("Bottom widget");

    let stacked = Stacked::new(top_text, bottom_text).centered();

    let mut terminal: ConstantSize<50, 20> = ConstantSize::new();

    sweeper.drawn(&mut terminal).unwrap();
    stacked.drawn(&mut terminal).unwrap();

    terminal.display(stdout()).unwrap();
}