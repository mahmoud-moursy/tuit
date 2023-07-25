use tuit::prelude::*;
use tuit::terminal::{Ansi4, ConstantSizeTerminal, TerminalCell, TerminalColour, UpdateInfo};
use tuit::widgets::Sweeper;

fn main() {
    let mut terminal: ConstantSizeTerminal<20, 20> = ConstantSizeTerminal::new();

    let mut background: Sweeper = Sweeper {
        colour: TerminalColour::Ansi16(Ansi4::BrightCyan)
    };

    background.draw(UpdateInfo::NoInfo, &mut terminal).expect("I won't fail ;)");

    terminal.characters_mut()[0] = TerminalCell {
        character: 'h',
        style: Default::default(),
    };

    terminal.characters_mut()[1] = TerminalCell {
        character: 'h',
        style: Default::default(),
    };

    let stdio = std::io::stdout();

    // Make sure to enable the "ansi-terminal" feature!
    terminal.display(stdio).unwrap();
}