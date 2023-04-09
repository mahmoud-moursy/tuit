//! # The draw module is responsible for the traits that let you draw to the screen
//!
//! Tuit itself does not handle I/O by itself. This means that Tuit users are left responsible
//! for rendering the terminal as well as handling input and output.
//!
//! ## Example
//! ```
//! use tuit::errors::Error;
//! use tuit::prelude::*;
//!
//! #[derive(GpuMagic)]
//! struct MyGPU {}
//!
//! impl TerminalDrawTarget for MyGPU {
//!     fn render(&mut self, terminal: impl Terminal) -> Result<(), Error> {
//!         for character in terminal.characters() {
//!             self.gpu_magic(x)
//!         }
//!
//!         Ok(())
//!     }
//! }
//! ```

use crate::terminal::Terminal;

/// This trait is written by the implementor and is responsible for rendering the terminal's data
/// to the screen.
pub trait TerminalDrawTarget {
    /// This method gets called when the implementor calls [`super::terminal::Terminal::draw`]
    ///
    /// Inversely, this method can be directly called on a Terminal
    ///
    ///
    fn render(&mut self, terminal: impl Terminal) -> crate::Result<()>;
}

#[cfg(feature = "ansi_terminal")]
impl TerminalDrawTarget for std::io::Stdout {
    fn render(&mut self, terminal: impl Terminal) -> crate::Result<()> {
        use std::prelude::rust_2021::*;
        use std::io::Write;

        let terminal_width = terminal.width();

        let characters = terminal.characters().iter();

        for (idx, character_cell) in characters.enumerate() {
            if idx % terminal_width == 0 {
                writeln!(self)?
            }
            write!(self, "{character_cell}")?
        }

        Ok(())
    }
}