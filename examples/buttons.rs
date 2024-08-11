//! This example shows how to use the [`Buttons`] widget.

use tuit::prelude::*;
use tuit::std::stdout_render::StdoutRenderer;
use tuit::style::Colour::Ansi16;
use tuit::style::{Ansi4, Colour};
use tuit::terminal::ConstantSize;
use tuit::widgets::builtins::buttons::Buttons;
use tuit::widgets::builtins::Sweeper;

//noinspection SpellCheckingInspection
fn main() {
    let mut terminal: ConstantSize<10, 20> = ConstantSize::new();

    let sweeper = Sweeper::of_colour(Ansi16(Ansi4::Cyan));

    sweeper.drawn(&mut terminal).expect("Infallible");

    // noinspection SpellCheckingInspection
    let mut buttons = Buttons::new(&["hellot", "World", "!@HG@!", "BBBBBBBBBB", "BBBBBBBBBB123"]);
    
    buttons.use_backdrop(Ansi16(Ansi4::Yellow)).drawn(terminal).expect("Infallible");

    buttons.unselected_button_style = buttons.unselected_button_style.bg(Ansi16(Ansi4::Red));
    buttons.selected_button_style = buttons.selected_button_style.bg(Ansi16(Ansi4::Green));

    buttons.drawn(&mut terminal).expect("Won't fail");

    terminal.display(StdoutRenderer::default()).expect("Infallible");
}