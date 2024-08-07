//! This example shows how to use the [`Buttons`] widget.

use tuit::draw::AnsiTerminal;
use tuit::terminal::ConstantSize;
use tuit::widgets::builtins::buttons::Buttons;
use tuit::prelude::*;
use tuit::std::stdout_terminal::StdoutTerminal;
use tuit::style::{Ansi4, Colour};
use tuit::widgets::BoundingBox;
use tuit::widgets::builtins::Sweeper;

fn main() {
    let mut terminal: ConstantSize<10, 20> = ConstantSize::new();

    let sweeper = Sweeper::of_colour(Colour::Ansi16(Ansi4::Cyan));

    sweeper.drawn(&mut terminal).unwrap();

    let mut buttons = Buttons::new(&["hellottttt", "World", "!@HG@!"]);

    let yellow = Sweeper::of_colour(Colour::Ansi16(Ansi4::Yellow));
    let sweeper_subsection = buttons.bounding_box_in(&terminal).unwrap();
    let terminal_subsection = terminal.view_mut(sweeper_subsection).unwrap();

    yellow.drawn(terminal_subsection).unwrap();

    buttons.unselected_button_style = buttons.unselected_button_style.bg(Colour::Ansi16(Ansi4::Red));
    buttons.selected_button_style = buttons.selected_button_style.bg(Colour::Ansi16(Ansi4::Green));

    buttons.drawn(&mut terminal).unwrap();

    terminal.display(StdoutTerminal::default()).unwrap();
}