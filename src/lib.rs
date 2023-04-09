//! # Tuit is a no_std TUI library intended for use in bare-metal applications
//!
//! Tuit is meant to be the successor to my (very infant) library made to fill the same gap called Tooey.
//!
//! Tooey relied upon `alloc` and was generally not very easy-to-use, well-documented or extensible. Tuit
//! is made to change that, using the lessons learnt from Tooey.
//!
//!

#![feature(rustdoc_missing_doc_code_examples)]
#![feature(slice_flatten, int_roundings, associated_type_bounds)]

#![deny(unused_features)]

#![deny(clippy::undocumented_unsafe_blocks)]
#![warn(rustdoc::broken_intra_doc_links)]

#![warn(missing_docs)]
#![warn(rustdoc::missing_doc_code_examples)]

#![warn(clippy::todo)]

#![warn(clippy::perf)]

// Never include the standard library unless the "std" feature is specified.
#![cfg_attr(not(feature = "std"), no_std)]

#[doc(hidden)]
pub use errors::Error;

/// This is a type alias used by `tuit` for its errors.
pub type Result<T> = core::result::Result<T, Error>;

pub mod terminal;
pub mod draw;
pub mod errors;
#[cfg(feature = "widgets")]
pub mod widgets;

#[allow(rustdoc::missing_doc_code_examples)]
pub mod prelude {
    //! The crate's prelude includes items that you'd usually want imported in a project that uses
    //! Tuit.
    //!
    //! *This module is intended to be glob-imported.
    pub use crate::{
        draw::TerminalDrawTarget,
        terminal::{
            Terminal,
            TerminalExtended,
            TerminalObject,
        },
    };
}


#[doc(hidden)]
#[cfg(test)]
mod test {
    extern crate std;
    extern crate alloc;

    use std::prelude::rust_2021::*;

    use crate::prelude::*;
    use crate::terminal::ConstantSizeTerminal;
    use crate::terminal::TerminalCell;

    #[test]
    fn views() {
        let mut terminal: ConstantSizeTerminal<20, 20> = ConstantSizeTerminal::new();

        terminal.characters_mut()[21] = TerminalCell {
            character: 'h',
            style: Default::default(),
        };

        let my_array = terminal.copied_view::<8, 5>(1, 1).unwrap();

        let my_array = my_array.flatten();

        assert_eq!(my_array[0], TerminalCell {
            character: 'h',
            style: Default::default(),
        })
    }
}