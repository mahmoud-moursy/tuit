//! # Errors
//!
//! This module contains errors that may occur during the execution of the program.
//!

use thiserror::Error;

/// This enum contains errors that
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum Error {
    /// I/O related errors.
    #[error("Encountered an I/O error.")]
    Io,
    /// This error is for when an implementor of TerminalDrawTarget fails for whatever reason
    #[error("Failed to render terminal screen.")]
    RenderError,
    #[error("Attempted to access a character that was out of bounds at index {0}")]
    OutOfBoundsCharacter(usize),
    #[error("Attempted to access a character co-ordinate that was out of bounds at: (x: {0}, y: {0})")]
    OutOfBoundsCoordinate(usize, usize),
    /// For when an error case is not covered by Tuit. If you are forced to use this, and think
    /// that your use-case for the error is in fact general enough, please feel free to submit a PR!
    #[error(transparent)]
    Generic(#[from] anyhow::Error),
    /// A slightly more specific version of `Error::Generic` that specifies
    /// the error occurred within an object that was drawing to the terminal.
    #[error("{0}")]
    GenericDrawError(#[source] anyhow::Error),
    /// A slightly more specific version of `Error::Generic` that specifies
    /// the error occurred within an object that was updating.
    #[error("{0}")]
    GenericUpdateError(#[source] anyhow::Error)
}