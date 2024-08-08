//! This example shows how to use the [`Buttons`] widget.

use tuit::prelude::*;
use tuit::std::stdout_render::StdoutRender;
use tuit::style::{Ansi4, Colour};
use tuit::terminal::ConstantSize;
use tuit::widgets::BoundingBox;
use tuit::widgets::builtins::buttons::Buttons;
use tuit::widgets::builtins::Sweeper;

fn main() {
    let mut terminal: ConstantSize<10, 20> = ConstantSize::new();

    let sweeper = Sweeper::of_colour(Colour::Ansi16(Ansi4::Cyan));

    sweeper.drawn(&mut terminal).unwrap();

    let mut buttons = Buttons::new(&["hellot", "World", "!@HG@!", "BBBBBBBBBB", "BBBBBBBBBB123"]);

    let yellow = Sweeper::of_colour(Colour::Ansi16(Ansi4::Yellow));
    let sweeper_subsection = buttons.bounding_box_in(&terminal).unwrap();
    let terminal_subsection = terminal.view_mut(sweeper_subsection).expect("Should be valid view");

    yellow.drawn(terminal_subsection).unwrap();

    buttons.unselected_button_style = buttons.unselected_button_style.bg(Colour::Ansi16(Ansi4::Red));
    buttons.selected_button_style = buttons.selected_button_style.bg(Colour::Ansi16(Ansi4::Green));

    buttons.drawn(&mut terminal).unwrap();

    terminal.display(StdoutRender::default()).unwrap();
}