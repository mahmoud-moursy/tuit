//! Demonstrates mutable views.

use std::io::stdout;

use tuit::prelude::*;
use tuit::style::{Ansi4, Colour};
use tuit::terminal::{Cell, ConstantSize};
use tuit::widgets::builtins::sweeper::Sweeper;

#[cfg(not(feature = "ansi_terminal"))]
fn main() {
    println!("You must apply the ansi_terminal feature to view this example. Use `cargo --features ansi_terminal`");
}

#[cfg(feature = "ansi_terminal")]
fn main() {
    let mut terminal: ConstantSize<20, 20> = ConstantSize::new();

    let sweeper = Sweeper::new(Colour::Ansi16(Ansi4::Cyan));

    sweeper.drawn(&mut terminal).expect("Should not fail");

    let view: [[&mut Cell; 5]; 5] = terminal.view_mut(5, 5).expect("Should not fail");

    view[4][4].character = 'h';

    terminal.display(stdout()).expect("Should not fail");
}