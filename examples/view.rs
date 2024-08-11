//! Demonstrates mutable views.

use tuit::prelude::*;
use tuit::std::stdout_render::StdoutRenderer;
use tuit::style::Colour;
use tuit::terminal::ConstantSize;
use tuit::terminal::Rectangle;
use tuit::widgets::builtins::{Ruler, Uv};
use tuit::widgets::Direction;

#[cfg(not(feature = "ansi_renderer"))]
fn main() {
    println!("You must apply the stdout_render feature to view this example. Use `cargo --features stdout_render`");
}

// a regression causes this
// lint to return a false-positive,
// so I disabled it.
#[allow(clippy::needless_borrows_for_generic_args)]
#[cfg(feature = "ansi_renderer")]
fn main() -> anyhow::Result<()> {
    let mut terminal: ConstantSize<100, 20> = ConstantSize::new();

    let uv = Uv;

    uv.drawn(&mut terminal)?;

    let ruler = Ruler::new(10, Direction::Left).expect("Can't fail");
    ruler.drawn(&mut terminal)?;
    
    let ruler = Ruler::new(10, Direction::Down).expect("Can't fail");
    ruler.drawn(&mut terminal)?;

    let mut view_text = "V I E W T E X T ".chars().cycle();
    let mut view = terminal
        .view_mut(Rectangle::of_size((95, 15)).at((3, 1)))
        .expect("Should always create a view successfully");

    for cell in view.cells_mut() {
        cell.style.bg_colour = Some(Colour::Rgb24(0, 127, 0));
        cell.character = view_text.next().expect("Won't fail yo. like... ever... trust me bro.");
    }

    terminal.display(StdoutRenderer::default())?;

    Ok(())
}