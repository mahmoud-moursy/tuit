use tuit::prelude::*;
use tuit::terminal::{ConstantSizeTerminal, TerminalCell};

fn main() {
    let mut terminal: ConstantSizeTerminal<20, 20> = ConstantSizeTerminal::new();

    terminal.characters_mut()[0] = TerminalCell {
        character: 'h',
        style: Default::default(),
    };

    let mut my_vec: Vec<&TerminalCell> = Vec::with_capacity(1);

    terminal.view(0, 0, 1, 1, &mut my_vec).unwrap();
}