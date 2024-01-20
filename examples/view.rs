//! An example that demonstrates mutable views.
#![feature(slice_flatten)]
#![allow(clippy::cast_possible_truncation)]

use std::array;
use tuit::prelude::*;
use tuit::terminal::{Ansi4, Character, Colour, ConstantSize, Style};
use tuit::widgets::{CenteredPrompt, Direction, Ruler, Sweeper};

#[cfg(not(feature = "ansi_terminal"))]
compile_error!("You must apply the ansi_terminal feature to view this example. Use `cargo --features ansi_terminal`");


fn main() {
    let mut terminal: ConstantSize<290, 70> = ConstantSize::new();

    let background: Sweeper = Sweeper {
        colour: Colour::Ansi16(Ansi4::BrightCyan),
    };

    let ruler = Ruler::new(10, Direction::Right).expect("Radix is too small to fail.");

    let text_size = 290;

    let text: String = (0..text_size)
            .map(|x| char::from_digit(x%10, 10).expect("Never overflows."))
            .collect();

    let prompt = CenteredPrompt::new(&text);

    background.drawn(&mut terminal).expect("Never fails.");

    terminal.characters_slice_mut()[0] = Character {
        character: 'h',
        style: Style::default(),
    };
    terminal.characters_slice_mut()[1] = Character {
        character: 'h',
        style: Style::default(),
    };

    let mut terminal_view: [[&mut Character; 5]; 5] = terminal.view_mut(9, 7).expect("This will not fail.");

    for character in terminal_view.flatten_mut() {
        character.character = 'h';
    }

    ruler.drawn(&mut terminal).expect("This method does not fail.");

    prompt.drawn(&mut terminal).expect("This method CAN fail, but only if the prompt is too large. Here, it is not.");

    let stdio = std::io::stdout();

    // Make sure to enable the "ansi-terminal" feature!
    terminal.display(stdio).expect("Failed to display terminal");
}
