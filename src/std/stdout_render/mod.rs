extern crate std;

use std::io::Write;

use anyhow::anyhow;

use crate::draw::Renderer;
use crate::terminal::TerminalConst;

/// Use [`StdoutRender::default`] to create a new [`StdoutRender`].
/// 
/// # Example
/// ```
/// use tuit::std::stdout_render::StdoutRender;
/// use tuit::prelude::*;
/// use tuit::terminal::ConstantSize;
///
/// let mut stdout = StdoutRender::default();
/// let mut terminal = ConstantSize::<20, 20>::new();
///
/// stdout.render(&terminal).expect("Failed to draw to stdout");
/// ```
pub struct StdoutRender(pub std::io::Stdout);

impl Default for StdoutRender {
    fn default() -> Self {
        Self(std::io::stdout())
    }
}

// Why does this duplicate exist, you ask?
// Well, it's because the standrad library's `std::io::Stdout` does not implement `Write`.
// That is... `core::fmt::Write`. It DOES, however, implement `std::io::Write`, which is an entirely
// separate trait that does almost exactly the same thing (except for change the error type).
// This is why we have to do... this. :(
impl Renderer for StdoutRender {
    fn render(&mut self, terminal: impl TerminalConst) -> crate::Result<()> {
        let terminal_width = terminal.width();

        let characters = terminal.cells();

        for (idx, character_cell) in characters.enumerate() {
            if idx % terminal_width == 0 {
                writeln!(self.0).map_err(|e| anyhow!(e))?;
            }

            let mut character_cell = *character_cell;

            // Protect against alignment issues that can arise from characters
            // like `\0` or `\t` by replacing them with a space.
            //
            // FIXME: Wide characters not handled.
            if character_cell.character.is_whitespace() || character_cell.character.is_control() {
                character_cell.character = ' ';
            }

            write!(self.0, "{character_cell}").map_err(|e| anyhow!(e))?;
        }

        self.0.flush()?;

        Ok(())
    }
}