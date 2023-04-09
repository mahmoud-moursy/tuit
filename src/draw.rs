//! # Draw module
//!
//! Twinkle itself does not handle I/O by itself. This means that Twinkle users are left responsible
//! for rendering the terminal as well as handling input and output.
//!
//! ## Example
//! ```
//! use twinkle::errors::Error;
//! use twinkle::prelude::*;
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

/// This trait is implemented by
pub trait TerminalDrawTarget {
    /// This method gets called when the implementor calls `Terminal::draw`
    fn render(&mut self, terminal: impl Terminal) -> crate::Result<()>;
}