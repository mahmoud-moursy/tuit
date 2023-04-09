//! # Twinkle is a no_std TUI library intended for use in bare-metal applications
//!
//! Twinkle is meant to be the successor to my (very infant) library made to fill the same gap called Tooey.
//!
//! Tooey relied upon `alloc` and was generally not very easy-to-use, well-documented or extensible. Twinkle
//! is made to change that, using the lessons learnt from Tooey.
//!
//!

#![feature(slice_flatten)]
#![feature(iterator_try_collect)]
#![feature(iter_collect_into)]
#![feature(anonymous_lifetime_in_impl_trait)]

#![deny(clippy::undocumented_unsafe_blocks)]
#![deny(rustdoc::broken_intra_doc_links)]

#![warn(missing_docs)]
#![warn(rustdoc::missing_doc_code_examples)]

#![warn(clippy::todo)]

#![warn(clippy::perf)]

// Never include the standard library unless the "std" feature is specified.
#![cfg_attr(not(feature = "std"), no_std)]

pub use errors::Error;

/// This is a type alias used by `twinkle` for its errors.
pub type Result<T> = core::result::Result<T, Error>;

pub mod terminal;
pub mod draw;
pub mod errors;
#[cfg(feature = "widgets")]
pub mod widgets;

pub mod prelude {
    //! The crate's prelude includes items that you'd usually want imported in a project that uses
    //! Twinkle.
    //!
    //! This module is intended to be glob-imported.
    pub use crate::{
        terminal::{
            Terminal,
            TerminalExtended,
            TerminalObject,
        },
        draw::TerminalDrawTarget,
    };
}

#[doc(hidden)]
#[cfg(test)]
mod test {
    use crate::terminal::TerminalCell;
    use super::terminal::ConstantSizeTerminal;
    use crate::prelude::*;

    extern crate std;
    extern crate alloc;

    use std::prelude::rust_2021::*;

    #[test]
    fn views() {
        let mut terminal: ConstantSizeTerminal<20, 20> = ConstantSizeTerminal::new();

        terminal.characters_mut()[22] = TerminalCell {
            character: 'h',
            style: Default::default(),
        };

        let mut my_array: Vec<&TerminalCell> = Default::default();

        terminal.view(1, 1, 8, 5, &mut my_array).unwrap();

        assert_eq!(my_array[0], &TerminalCell {
            character: 'h',
            style: Default::default(),
        })
    }
}