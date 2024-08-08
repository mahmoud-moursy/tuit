//! Demonstrates the Text widget.

use tuit::prelude::*;
use tuit::std::stdout_render::StdoutRender;
use tuit::style::{Ansi4, Colour};
use tuit::terminal::ConstantSize;
use tuit::widgets::builtins::{Sweeper, Text};

fn main() {
    let mut terminal: ConstantSize<20, 20> = ConstantSize::new();
    let sweeper = Sweeper::of_colour(Colour::Ansi16(Ansi4::Cyan));
    let text = Text::new("Hello world!");

    sweeper.drawn(&mut terminal).ok();
    text.drawn(&mut terminal).ok();

    terminal.display(StdoutRender::default()).expect("Infallible.");
}