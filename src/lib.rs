//! # Tuit is a `no_std` TUI library intended for use in bare-metal applications
//!
//! Tuit is a library for the hardcore, `no_std`, `no_alloc` Rust user.
//!
//! When I was began on my operating system journey, I noticed that there was not a single `no_std` TUI library. So I
//! decided to make my own, and share it!
//!
//! API is inspired by [`embedded_graphics`], my beloved.
//!
//! ```
//! use tuit::terminal::ConstantSize;
//! use tuit::widgets::CenteredText;
//! use tuit::prelude::*;
//!
//! let mut terminal = ConstantSize::<20, 20>::new();
//! let prompt = CenteredText::new("Hello world!");
//!
//! prompt.drawn(&mut terminal).expect("This won't fail."); // Draws "Hello world!" in the center of the screen.
//! ```
#![feature(int_roundings, associated_type_bounds, slice_flatten, iter_advance_by)]
#![feature(rustdoc_missing_doc_code_examples)]
// Never include the standard library unless the "std" feature is specified.
#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "alloc")]
extern crate alloc;

#[doc(hidden)]
pub use errors::Error;

/// This is a type alias used by `tuit` for its errors.
pub type Result<T> = core::result::Result<T, Error>;

pub mod draw;
pub mod errors;
pub mod terminal;
#[cfg(feature = "widgets")]
pub mod widgets;

pub mod prelude {
    //! The crate's prelude includes items that you'd usually want imported in a project that uses
    //! Tuit. It imports all the traits related to terminals.
    //!
    //! *This module is intended to be glob-imported.
    pub use crate::{
        draw::TerminalDrawTarget,
        terminal::{Terminal, TerminalExtended, Widget},
    };
}

#[doc(hidden)]
#[cfg(test)]
mod test {
    extern crate alloc;
    extern crate std;

    use std::prelude::rust_2021::*;

    use crate::prelude::*;
    use crate::terminal::Character;
    use crate::terminal::{ConstantSize, Style};

    #[test]
    fn views() {
        let mut terminal: ConstantSize<20, 20> = ConstantSize::new();

        terminal.characters_slice_mut()[21] = Character {
            character: 'h',
            style: Style::default(),
        };

        let my_array = terminal
            .view_copied::<8, 5>(1, 1)
            .expect("Should never fail!");

        let my_array = my_array.flatten();

        assert_eq!(my_array[0].character, 'h');
    }

    #[test]
    fn mutable_views() {
        let mut terminal: ConstantSize<20, 20> = ConstantSize::new();

        let view: [[&mut Character; 5]; 5] = terminal.view_mut(10, 10).expect("Should not fail.");

        view[0][0].character = 'h';

        assert_eq!(
            terminal
                .character(15, 15)
                .expect("Won't fail because we are indexing into a valid location")
                .character,
            'h'
        );
    }
}
