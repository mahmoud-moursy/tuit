use tuit::prelude::*;
use tuit::terminal::{ConstantSizeTerminal, TerminalCell};

fn main() {
    let mut terminal: ConstantSizeTerminal<20, 20> = ConstantSizeTerminal::new();

    terminal.characters_mut()[0] = TerminalCell {
        character: 'h',
        style: Default::default(),
    };

    terminal.characters_mut()[1] = TerminalCell {
        character: 'h',
        style: Default::default(),
    };
}