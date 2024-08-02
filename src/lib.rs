//! # Tuit is a `no_std` TUI library intended for use in bare-metal applications
//!
//! Tuit is a library for the hardcore, `no_std`, `no_alloc` Rust user.
//!
//! When I began on my operating system journey, I noticed that there was not a single `no_std` TUI library. So I
//! decided to make my own, and share it!
//!
//! API is partially inspired by `embedded_graphics`, my beloved.
//!
//! ```
//! use tuit::terminal::ConstantSize;
//! use tuit::widgets::builtins::CenteredText;
//! use tuit::prelude::*;
//!
//! let mut terminal = ConstantSize::<20, 20>::new();
//! let prompt = CenteredText::new("Hello world!");
//!
//! prompt.drawn(&mut terminal).expect("This won't fail."); // Draws "Hello world!" in the center of the screen.
//! ```
#![feature(iter_array_chunks)]
#![allow(clippy::collapsible_if)]
#![no_std]

#[doc(hidden)]
pub use errors::Error;

#[cfg(feature = "alloc")]
pub mod allocations;
pub mod draw;
pub mod errors;
#[cfg(feature = "std")]
pub mod std;
pub mod terminal;
pub mod widgets;
/// Structs related to styles -- includes [`style::Style`] and [`style::Colour`] <br>
pub mod style;
/// Default implementations of traits for builtin structs. <br>
mod default_impls;

pub mod prelude {
    //! The crate's prelude includes items that you'd usually want imported in a project that uses
    //! Tuit. It imports all the traits related to terminals.
    //!
    //! *This module is intended to be glob-imported.
    pub use crate::{
        draw::Renderer,
        terminal::{Metadata, Terminal, TerminalConst, TerminalMut},
        widgets::Widget,
    };
}

/// This is a type alias used by `tuit` for its errors.
pub type Result<T> = core::result::Result<T, Error>;

#[doc(hidden)]
#[cfg(test)]
mod test {
    extern crate alloc;
    extern crate std;

    use std::prelude::rust_2021::*;

    use crate::prelude::*;
    use crate::style::Style;
    use crate::terminal::Cell;
    use crate::terminal::ConstantSize;

    #[test]
    fn views() {
        let mut terminal: ConstantSize<20, 20> = ConstantSize::new();

        *(terminal.cells_mut().nth(20).unwrap()) = Cell {
            character: 'h',
            style: Style::default(),
        };

        // let my_array = terminal
        //     .view_copied::<8, 5>(1, 1)
        //     .expect("Should never fail!");
        //
        // let my_array = my_array.as_flattened();
        //
        // assert_eq!(my_array[0].character, 'h');
    }

    // #[test]
    // fn mutable_views() {
    //     let mut terminal: ConstantSize<20, 20> = ConstantSize::new();
    //
    //     let view: [[&mut Cell; 5]; 5] = terminal.view_mut(10, 10).expect("Should not fail.");
        //
        // view[0][0].character = 'h';
        //
        // assert_eq!(
        //     terminal
        //         .character(10, 10)
        //         .expect("Won't fail because we are indexing into a valid location")
        //         .character,
        //     'h'
        // );
    // }
}
