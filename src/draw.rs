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
//! impl Renderer for MyGPU {
//!     fn render(&mut self, terminal: impl TerminalConst) -> tuit::Result<()> {
//!         for character in terminal.cells() {
//!             self.gpu_magic(character)
//!         }
//!
//!         Ok(())
//!     }
//! }
//! ```

#[cfg(feature = "ansi_renderer")]
use core::fmt::{Formatter, Write};
#[cfg(feature = "ansi_renderer")]
use anyhow::anyhow;
#[cfg(feature = "ansi_renderer")]
use crate::terminal::Cell;
use crate::terminal::TerminalConst;

/// This trait is written by the implementor and is responsible for rendering the terminal's data
/// to the screen.
///
/// It only has one method, which is [`Renderer::render`].
///
/// The method receives a reference to a type that implements the [`TerminalConst`] trait, and uses the data within to render the terminal.
///
/// ```feature,stdout_render
/// use tuit::draw::Target;
/// use tuit::terminal::ConstantSize;
///
/// let my_terminal = ConstantSize::<20, 20>::new();
/// let mut stdout = std::io::stdout();
///
/// stdout.render(&my_terminal).expect("Failed to draw to stdout");
/// ```
///
pub trait Renderer {
    /// This method gets called when the implementor calls [`TerminalConst::display`]
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
    /// impl Renderer for MyGPU {
    ///     fn render(&mut self, terminal: impl TerminalConst) -> tuit::Result<()> {
    ///         let cells = terminal.cells();
    ///
    ///         // Do some magic to render characters!
    ///         for cell in cells {
    ///             self.gpu_magic(cell)
    ///         }
    ///
    ///         return Ok(());
    ///     }
    /// }
    /// ```
    ///
    /// # Errors
    ///
    /// When you implement [`Renderer`], you can return an [`Err`] that will help you better cope
    /// with render failures and may help with debugging.
    fn render(&mut self, terminal: impl TerminalConst) -> crate::Result<()>;
}

/// Doesn't really do anything when [`Renderer::render`] is called. I mean... what would you
/// expect a struct called [`DummyTarget`] to accomplish? World peace?
pub struct DummyTarget;

impl Renderer for DummyTarget {
    fn render(&mut self, _terminal: impl TerminalConst) -> crate::Result<()> {
        Ok(())
    }
}

#[cfg(feature = "ansi_renderer")]
/// A [`Renderer`] that takes in a writer and outputs ANSI escape codes to it to use for formatting.
pub struct AnsiRenderer<T>(pub T);

#[cfg(feature = "ansi_renderer")]
impl<T: Write> Renderer for AnsiRenderer<T> {
    fn render(&mut self, terminal: impl TerminalConst) -> crate::Result<()> {
        let terminal_width = terminal.width();

        let characters = terminal.cells();

        for (idx, character_cell) in characters.enumerate() {
            if idx % terminal_width == 0 {
                let style: anstyle::Style = character_cell.style.into();
                write!(self.0, "{style:#}").map_err(|e| anyhow!(e))?;
                writeln!(self.0).map_err(|e| anyhow!(e))?;
                write!(self.0, "{style}").map_err(|e| anyhow!(e))?;
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

        Ok(())
    }
}

#[cfg(feature = "ansi_renderer")]
impl core::fmt::Display for Cell {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        let style: anstyle::Style = self.style.into();

        write!(f, "{style}{}", self.character)
    }
}
