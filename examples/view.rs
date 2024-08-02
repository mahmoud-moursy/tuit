//! Demonstrates mutable views.

use std::io::stdout;

use tuit::prelude::*;
use tuit::style::Colour;
use tuit::terminal::ConstantSize;
use tuit::widgets::builtins::Uv;
use tuit::widgets::Rectangle;

#[cfg(not(feature = "ansi_terminal"))]
fn main() {
    println!("You must apply the ansi_terminal feature to view this example. Use `cargo --features ansi_terminal`");
}

// a regression causes this
// lint to return a false-positive,
// so I disabled it.
#[allow(clippy::needless_borrows_for_generic_args)]
#[cfg(feature = "ansi_terminal")]
fn main() -> anyhow::Result<()> {
    let mut terminal: ConstantSize<100, 20> = ConstantSize::new();

    let uv = Uv;

    uv.drawn(&mut terminal)?;

    let mut view_text = "V I E W T E X T ".chars().cycle();
    let mut view = terminal
        .view_mut(Rectangle::of_size(10, 10).to((5, 5)))
        .expect("Should always create a view successfully");
    
    for cell in view.cells_mut() {
        cell.style.bg_colour = Some(Colour::Rgb24(0, 127, 0));
        cell.character = view_text.next().expect("Won't fail yo. like... ever... trust me bro.");
    }

    terminal.display(stdout())?;

    Ok(())
}