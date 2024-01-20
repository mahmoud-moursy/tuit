//! # Tuit is a `no_std` TUI library intended for use in bare-metal applications
//!
//! Tuit is meant to be the successor to my (very infant) library made to fill the same gap called Tooey.
//!
//! Tooey relied upon `alloc` and was generally not very easy-to-use, well-documented or extensible. Tuit
//! is made to change that, using the lessons learnt from Tooey.
//!
//!
//!
#![feature(int_roundings, associated_type_bounds, slice_flatten, iter_advance_by)]

// Never include the standard library unless the "std" feature is specified.
#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "alloc")]
extern crate alloc;

#[doc(hidden)]
pub use errors::Error;

/// This is a type alias used by `tuit` for its errors.
pub type Result<T> = core::result::Result<T, Error>;

pub mod terminal;
pub mod draw;
pub mod errors;
#[cfg(feature = "widgets")]
pub mod widgets;

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
            Widget,
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
    use crate::terminal::{ConstantSize, Style};
    use crate::terminal::Character;

    #[test]
    fn views() {
        let mut terminal: ConstantSize<20, 20> = ConstantSize::new();

        terminal.characters_slice_mut()[21] = Character {
            character: 'h',
            style: Style::default(),
        };

        let my_array = terminal.view_copied::<8, 5>(1, 1).expect("Should never fail!");

        let my_array = my_array.flatten();

        assert_eq!(my_array[0].character, 'h');
    }

    #[test]
    fn mutable_views() {
        let mut terminal: ConstantSize<20, 20> = ConstantSize::new();

        let view: [[&mut Character; 5]; 5] = terminal.view_mut(10, 10).expect("Should not fail.");

        view[0][0].character = 'h';

        assert_eq!(terminal.character(15, 15).expect("Won't fail because we are indexing into a valid location").character, 'h');
    }
}