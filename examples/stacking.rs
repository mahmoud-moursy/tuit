//! This example shows how to use the [`Stacked`] widget.

use std::io::stdout;
use tuit::terminal::ConstantSize;
use tuit::widgets::builtins::{Sweeper, Text, WithLayout};
use tuit::prelude::*;
use tuit::std::stdout_terminal::StdoutTerminal;
use tuit::style::{Ansi4, Colour, Style};

fn main() {
    let sweeper = Sweeper::of_colour(Colour::Ansi16(Ansi4::Cyan));

    let top_text    = Text::new("Top widget");
    let middle_text = Text::new("Middle widget");
    let bottom_text = Text::new("Bottom widget").styled(Style::new().bg(Colour::Ansi16(Ansi4::Red)));

    let stacked = top_text
        .on_top_of(middle_text)
        .on_top_of(bottom_text)
        .centered();

    let mut terminal: ConstantSize<30, 9> = ConstantSize::new();

    sweeper.drawn(&mut terminal).unwrap();
    stacked.drawn(&mut terminal).unwrap();

    terminal.display(StdoutTerminal::default()).unwrap();
}