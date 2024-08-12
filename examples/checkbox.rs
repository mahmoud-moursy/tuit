//! A simple example of a checkbox.

use tuit::terminal::{ConstantSize, MouseButton, UpdateInfo};
use tuit::prelude::*;
use tuit::std::stdout_render::StdoutRenderer;
use tuit::style::{Ansi4, Colour};
use tuit::widgets::builtins::checkbox::Checkbox;

fn main() {
    let mut terminal: ConstantSize<50, 20> = ConstantSize::new();

    let checkbox = Checkbox::new("Check me!");
    let mut checkbox_stack = checkbox.on_top_of(checkbox).on_top_of(checkbox.checked()).on_top_of(checkbox).on_top_of(checkbox);

    checkbox_stack.use_backdrop(Colour::Ansi16(Ansi4::Yellow)).drawn(&mut terminal).expect("Should draw successfully :)");

    checkbox_stack.drawn(&mut terminal).expect("Should draw successfully :)");

    checkbox_stack.update(UpdateInfo::CellClicked(1, 1, MouseButton::Primary), &mut terminal).expect("Should update successfully :)");

    terminal.display(StdoutRenderer::default()).expect("Should render successfully :)");
}