//! # The draw module is responsible for the traits that let you draw to the screen
//!
//! Tuit itself does not handle I/O by itself. This means that Tuit users are left responsible
//! for rendering the terminal as well as handling input and output.
//!
//! ## Example
//! ```compile_fail
//! use tuit::errors::Error;
//! use tuit::prelude::*;
//!
//! #[derive(GpuMagic)]
//! struct MyGPU {}
//!
//! impl TerminalDrawTarget for MyGPU {
//!     fn render(&mut self, terminal: impl Terminal) -> Result<(), Error> {
//!         for character in terminal.characters_slice() {
//!             self.gpu_magic(character)
//!         }
//!
//!         Ok(())
//!     }
//! }
//! ```

use crate::terminal::Terminal;

/// This trait is written by the implementor and is responsible for rendering the terminal's data
/// to the screen.
///
/// It only has one method, which is [`TerminalDrawTarget::render`].
///
/// The method receives a reference to a type that implements the [`Terminal`] trait, and uses the data within to render the terminal.
///
/// ```
/// #![feature(ansi_terminal)]
/// use tuit::draw::TerminalDrawTarget;
/// use tuit::terminal::ConstantSize;
///
/// let my_terminal = ConstantSize::<20, 20>::new();
/// let mut stdout = std::io::stdout();
///
/// stdout.render(&my_terminal).expect("Failed to draw to stdout");
/// ```
///
pub trait TerminalDrawTarget {
    /// This method gets called when the implementor calls [`Terminal::display`]
    ///
    /// ## Dummy render implementation:
    ///
    /// ```
    /// use tuit::prelude::*;
    /// use tuit::terminal::Cell;
    /// struct MyGPU;
    ///
    /// impl MyGPU {
    ///     pub fn gpu_magic(&mut self, character: &Cell) {
    ///         // Huzzah!
    ///     }
    /// }
    ///
    /// impl TerminalDrawTarget for MyGPU {
    ///     fn render(&mut self, terminal: &impl Terminal) -> tuit::Result<()> {
    ///         let characters = terminal.characters_slice();
    ///
    ///         // Do some magic to render characters!
    ///         for character in characters {
    ///             self.gpu_magic(character)
    ///         }
    ///
    ///         return Ok(());
    ///     }
    /// }
    /// ```
    fn render(&mut self, terminal: &impl Terminal) -> crate::Result<()>;
}

#[cfg(feature = "ansi_terminal")]
impl TerminalDrawTarget for std::io::Stdout {
    fn render(&mut self, terminal: &impl Terminal) -> crate::Result<()> {
        use std::io::Write;
        use std::prelude::rust_2021::*;

        let terminal_width = terminal.width();

        let characters = terminal.characters_slice().iter();

        for (idx, character_cell) in characters.enumerate() {
            if idx % terminal_width == 0 {
                writeln!(self)?;
            }

            let mut character_cell = *character_cell;

            // Protect against alignment issues that can arise from characters
            // like `\0` or `\t` by replacing them with a space.
            //
            // FIXME: Wide characters not handled.
            if character_cell.character.is_whitespace() || character_cell.character.is_control() {
                character_cell.character = ' ';
            }

            write!(self, "{character_cell}")?;
        }

        Ok(())
    }
}
