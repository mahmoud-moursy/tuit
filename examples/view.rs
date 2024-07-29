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

    let test_buttons: [String; 25] = array::from_fn(|x| format!(" Test {x:>02} ")); // A format! String can have a dynamic length, thus requires dynamic allocation...
    let test_buttons: [&str; 25] = array::from_fn(|x| test_buttons[x].as_str()); // So we allocate each String, and then borrow it from where it is already owned...
                                                                                 // So no more errors about a value being dropped!

    let mut prompt = CenteredPrompt::new("Hello\0\tworld!", &test_buttons).select(2);

    prompt.centered_text.style = prompt
        .centered_text
        .style
        .bg(Ansi16(Ansi4::Red))
        .fg(Ansi16(Ansi4::BrightWhite));
    prompt.selected_button_style = prompt.selected_button_style.inverted();

    prompt.drawn(&mut terminal).expect("Oops...");

    let stdio = std::io::stdout();

    // Make sure to enable the "ansi-terminal" feature!
    terminal.display(stdio).expect("Failed to display terminal");
}
