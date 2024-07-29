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

use crate::terminal::TerminalConst;

/// This trait is written by the implementor and is responsible for rendering the terminal's data
/// to the screen.
///
/// It only has one method, which is [`Target::render`].
///
/// The method receives a reference to a type that implements the [`TerminalConst`] trait, and uses the data within to render the terminal.
///
/// ```
/// #![feature(ansi_terminal)]
/// use tuit::draw::Target;
/// use tuit::terminal::ConstantSize;
///
/// let my_terminal = ConstantSize::<20, 20>::new();
/// let mut stdout = std::io::stdout();
///
/// stdout.render(&my_terminal).expect("Failed to draw to stdout");
/// ```
///
pub trait Target {
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
    /// impl Target for MyGPU {
    ///     fn render(&mut self, terminal: &impl TerminalConst) -> tuit::Result<()> {
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
    ///
    /// # Errors
    ///
    /// When you implement [`Target`], you can return an [`Err`] that will help you better cope
    /// with render failures and may help with debugging.
    fn render(&mut self, terminal: &impl TerminalConst) -> crate::Result<()>;
}

/// Doesn't really do anything when [`Target::render`] is called. I mean... what would you
/// expect a struct called [`DummyTarget`] to accomplish? World peace?
pub struct DummyTarget;

impl Target for DummyTarget {
    fn render(&mut self, _terminal: &impl TerminalConst) -> crate::Result<()> {
        Ok(())
    }
}
