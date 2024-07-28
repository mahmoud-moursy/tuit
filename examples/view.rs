//! An example that demonstrates mutable views.

use std::array;

use tuit::prelude::*;
use tuit::terminal::{Ansi4, ConstantSize};
use tuit::terminal::Colour::Ansi16;
use tuit::widgets::{CenteredPrompt, Sweeper};

#[cfg(not(feature = "ansi_terminal"))]
compile_error!("You must apply the ansi_terminal feature to view this example. Use `cargo --features ansi_terminal`");

#[cfg(feature = "ansi_terminal")]
fn main() {
    let mut terminal: ConstantSize<58, 14> = ConstantSize::new();
    //
    // let text = CenteredText::new("Hello world!");
    //
    // text.drawn(&mut terminal)
    //     .expect("This method CAN fail, but only if the prompt is too large. Here, it is not.");

    let sweeper = Sweeper::new(Ansi16(Ansi4::BrightCyan));

    sweeper.drawn(&mut terminal).ok();

    let test_buttons: [String; 25] = array::from_fn(|x| format!(" Test {x:>02} "));
    let test_buttons_str: [&str; 25] = array::from_fn(|x| test_buttons[x].as_str());

    let mut prompt = CenteredPrompt::new("Hello world!", &test_buttons_str).select(2);

    let prompt_ref = &mut prompt;

    *prompt_ref = prompt_ref.select(1);

    prompt_ref.centered_text.style = prompt_ref.centered_text.style.bg(Ansi16(Ansi4::Red));
    prompt_ref.selected_button_style = prompt_ref.selected_button_style.bg(Ansi16(Ansi4::Blue));

    prompt.drawn(&mut terminal).expect("Oops...");

    let stdio = std::io::stdout();

    // Make sure to enable the "ansi-terminal" feature!
    terminal.display(stdio).expect("Failed to display terminal");
}
