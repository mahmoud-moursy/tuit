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
//! impl Target for MyGPU {
//!     fn render(&mut self, terminal: impl Terminal) -> tuit::Result<()> {
//!         for character in terminal.characters_slice() {
//!             self.gpu_magic(character)
//!         }
//!
//!         Ok(())
//!     }
//! }
//! ```

use crate::terminal::TerminalConst;

/// This trait is written by the implementor and is responsible for rendering the terminal's data
/// to the screen.
///
/// It only has one method, which is [`Renderer::render`].
///
/// The method receives a reference to a type that implements the [`TerminalConst`] trait, and uses the data within to render the terminal.
///
/// ```feature,ansi_terminal
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
