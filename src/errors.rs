//! # Errors
//!
//! This module contains errors that may occur during the execution of the program.
//!
//! ```
//! use tuit::draw::Renderer;
//! use tuit::prelude::TerminalConst;
//! use tuit::terminal::ConstantSize;
//!
//! use tuit::errors::Error;
//!
//! # struct MagicGPU;
//! # impl Renderer for MagicGPU {
//! #   fn render(&mut self, terminal: impl TerminalConst) -> tuit::Result<()> {
//! #        Ok(())
//! #    }
//! # }
//!
//! let mut my_gpu = MagicGPU;
//!
//! let my_terminal: ConstantSize<20, 20> = ConstantSize::new();
//!
//! let my_render = my_gpu.render(my_terminal);
//!
//! match my_render {
//!    Ok(_) => { /* Happy path! :D */ }
//!    Err(Error::Generic(anyhow_error)) => { /* Depressing path :( */ }
//!    Err(_) => { /* Miserable path ;( */ }
//! }
//! ```

pub use anyhow::Error as AnyhowError;
use thiserror::Error;

// Unused in code, used in docs.
#[allow(unused_imports)]
use crate::{
    draw,
    terminal::TerminalConst,
    widgets::Widget,
};

/// This enum contains errors that may occur at runtime
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum Error {
    /// For generic I/O related errors.
    #[error("Encountered an I/O error.")]
    Io,
    /// This error is for when an implementor of [`draw::Renderer`] fails to
    /// render the screen for any reason.
    #[error("Failed to render terminal screen.")]
    RenderError,
    /// This error is for when an out-of-bounds index occurs within any [`TerminalConst`]/[`Widget`] method.
    /// It does not include x,y co-ordinates, just the index into the character
    /// buffer slice.
    #[error("Attempted to access a character that was out of bounds at index {0}")]
    OutOfBoundsIndex(usize),
    /// This error is for when an out-of-bounds index occurs within any [`TerminalConst`]/[`Widget`] method.
    /// It includes the x,y coordinates used to index into the slice.
    #[error(
    "Attempted to access a character co-ordinate that was out of bounds at: (x: {:?}, y: {:?})",
        x,
        y
    )]
    OutOfBoundsCoordinate {
        /// The X out-of-bounds coordinate. This may be `None` in cases where a coordinate cannot be provided.
        x: Option<usize>,
        /// The Y out-of-bounds coordinate. This may be `None` in cases where a coordinate cannot be provided.
        y: Option<usize>,
    },
    /// If a terminal or a widget encounters an error due to running out of space,
    /// it should return an [`Error::RequestRescale`] to a suitable new size.
    #[error("There was not enough space in the terminal, so a rescale to a width of {new_width} and a height of {new_height}")]
    RequestRescale {
        /// The requested new width.
        new_width: usize,
        /// The requested new height.
        new_height: usize
    },
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
    GenericUpdateError(#[source] anyhow::Error),
    /// This error does not exist for use by [`crate`], it exists for usage by the users of the library.
    ///
    /// It is better to return an [`Error::Todo`] than to panic using the `todo!()` macro when you use a widget
    /// that is not fully implemented.
    #[error("This area has not been implemented!")]
    Todo,
}

impl Error {
    /// Returns an [`Error::OutOfBoundsCoordinate`] with both x and y coordinates set to `None`.
    #[must_use]
    pub const fn oob() -> Self {
        Self::OutOfBoundsCoordinate { x: None, y: None }
    }
    /// Returns an [`Error::OutOfBoundsCoordinate`] with x and y coordinates set to `Some` using the
    /// provided values.
    #[must_use]
    pub const fn oob_with((x, y): (usize, usize)) -> Self {
        Self::OutOfBoundsCoordinate { x: Some(x), y: Some(y) }
    }

    //noinspection SpellCheckingInspection
    /// Returns an [`Error::OutOfBoundsIndex`] with the provided index.
    #[must_use]
    pub const fn oobi(index: usize) -> Self {
        Self::OutOfBoundsIndex(index)
    }

    /// Returns an [`Error::RequestRescale`] with the provided width and height.
    #[must_use]
    pub const fn rescale((x, y): (usize, usize)) -> Self {
        Self::RequestRescale { new_width: x, new_height: y }
    }
}
