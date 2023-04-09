//! # Errors
//!
//! This module contains errors that may occur during the execution of the program.
//!
//! ```
//! use tuit::draw::TerminalDrawTarget;
//! use tuit::prelude::Terminal;
//! use tuit::terminal::ConstantSizeTerminal;
//!
//! use tuit::errors::Error;
//!
//! # struct MagicGPU;
//!
//! # impl TerminalDrawTarget for MagicGPU {fn render(&mut self, terminal: impl Terminal) -> tuit::Result<()> {
//! #        todo!()
//! #    }
//! #
//! # }
//!
//! let mut my_gpu = MagicGPU;
//!
//! let my_terminal: ConstantSizeTerminal<20, 20> = ConstantSizeTerminal::new();
//!
//! let my_render = my_gpu.render(my_terminal);
//!
//! match my_render {
//!    Ok(_) => { /* Happy path! */ }
//!    Err(Error::Generic(anyhow_error)) => { /* Depressing path :( */ }
//!    Err(_) => { /* Miserable path :( */ }
//! }
//! ```

use thiserror::Error;

/// This enum contains errors that may occur at runtime
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum Error {
    /// I/O related errors.
    #[error("Encountered an I/O error.")]
    Io,
    /// This error is for when an implementor of TerminalDrawTarget fails for whatever reason
    #[error("Failed to render terminal screen.")]
    RenderError,
    /// This error is for when an out-of-bounds index occurs within any Terminal/TerminalObject method.
    /// It does not include x,y co-ordinates, just the index into the character
    /// buffer slice.
    #[error("Attempted to access a character that was out of bounds at index {0}")]
    OutOfBoundsCharacter(usize),
    /// This error is for when an out-of-bounds index occurs within any Terminal/TerminalObject method.
    /// It includes the x,y coordinates used to index into the slice.
    #[error("Attempted to access a character co-ordinate that was out of bounds at: (x: {0}, y: {0})")]
    OutOfBoundsCoordinate(usize, usize),
    /// For when an error case is not covered by Tuit. If you are forced to use this, and think
    /// that your use-case for the error is in fact general enough, please feel free to submit a PR!
    #[error(transparent)]
    Generic(#[from] anyhow::Error),
    /// A slightly more specific version of [`Error::Generic`] that specifies
    /// the error occurred within an object that was drawing to the terminal.
    #[error("{0}")]
    GenericDrawError(#[source] anyhow::Error),
    /// A slightly more specific version of [`Error::Generic`] that specifies
    /// the error occurred within an object that was updating.
    #[error("{0}")]
    GenericUpdateError(#[source] anyhow::Error)
}