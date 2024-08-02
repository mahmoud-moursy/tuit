//! This module contains the [`Extended`] trait that contains some extra methods.

use crate::prelude::*;
use crate::terminal::extended::view_mut::ViewMut;
use crate::widgets::Rectangle;

/// The [`ViewMut`] terminal.
pub mod view_mut;
mod view;
mod view_iterator;

/// Some things here require extra dependencies that I didn't want to add to needlessly add to the
/// crate... so they got broken off into their own thing.
pub trait Extended: Terminal {
    /// Returns a mutable view into
    fn view_mut(&mut self, rect: Rectangle) -> ViewMut<&mut Self> {
        ViewMut::new(self, rect)
    }

    // Returns a copied view to the [`Cell`]s within the specified area.
    // 
    // ```
    // use tuit::terminal::{ConstantSize, Cell};
    // use tuit::prelude::*;
    // 
    // let my_terminal: ConstantSize<20, 20> = ConstantSize::new();
    // 
    // // A get a copied view of height 2 and width 2, at x-y coords (1,1).
    // let characters: [[Cell; 2]; 2] = my_terminal.view_copied::<2, 2>(1, 1).expect("This should never fail!");
    // ```
    // 
    // # Errors
    // 
    // This will return an [`Err`] if the requested view reaches out-of-bounds.
    // fn view_copied<const WIDTH: usize, const HEIGHT: usize>(
    //     &self,
    //     x_offset: usize,
    //     y_offset: usize,
    // ) -> crate::Result<[[Cell; WIDTH]; HEIGHT]> {
    //     let (terminal_width, terminal_height) = self.dimensions();
    //     let characters = self.cells();
    // 
    //     if y_offset + HEIGHT > terminal_height {
    //         return Err(Error::OutOfBoundsCoordinate {
    //             x: Some(x_offset + WIDTH),
    //             y: Some(y_offset + HEIGHT),
    //         });
    //     }
    // 
    //     if x_offset + WIDTH > terminal_width {
    //         return Err(Error::OutOfBoundsCoordinate {
    //             x: Some(x_offset + WIDTH),
    //             y: Some(y_offset + HEIGHT),
    //         });
    //     }
    // 
    //     Ok(array::from_fn(|y| {
    //         array::from_fn(|x| characters[x_offset + x + (y_offset + y) * terminal_width])
    //     }))
    // }
    // 
    // Returns a reference view to the [`Cell`]s within the specified area.
    // 
    // ```
    // use tuit::terminal::{ConstantSize, Cell};
    // use tuit::prelude::*;
    // 
    // let my_terminal: ConstantSize<20, 20> = ConstantSize::new();
    // 
    // // A get a copied view of height 2 and width 2, at x-y coords (1,1).
    // let cells: [[&Cell; 2]; 2] = my_terminal.view::<2, 2>(1, 1).expect("This should never fail!");
    // ```
    // 
    // # Errors
    // 
    // This will return an [`Err`] if the requested view reaches out-of-bounds.
    // fn view<const WIDTH: usize, const HEIGHT: usize>(
    //     &self,
    //     x_offset: usize,
    //     y_offset: usize,
    // ) -> crate::Result<[[&Cell; WIDTH]; HEIGHT]> {
    //     let (terminal_width, terminal_height) = self.dimensions();
    //     let characters = self.cells();
    // 
    //     if y_offset + HEIGHT > terminal_height {
    //         return Err(Error::OutOfBoundsCoordinate {
    //             x: Some(x_offset + WIDTH),
    //             y: Some(y_offset + HEIGHT),
    //         });
    //     }
    // 
    //     if x_offset + WIDTH > terminal_width {
    //         return Err(Error::OutOfBoundsCoordinate {
    //             x: Some(x_offset + WIDTH),
    //             y: Some(y_offset + HEIGHT),
    //         });
    //     }
    // 
    //     Ok(array::from_fn(move |y| {
    //         array::from_fn(move |x| {
    //             characters.index(x_offset + x + (y_offset + y) * terminal_width)
    //         })
    //     }))
    // }
    // 
    // Returns a view to the [`Cell`]s within the specified area, as pointers instead of references.
    // 
    // ```
    // use tuit::terminal::{ConstantSize, Cell};
    // use tuit::prelude::*;
    // 
    // let my_terminal: ConstantSize<20, 20> = ConstantSize::new();
    // 
    // // A get a copied view of height 2 and width 2, at x-y coords (1,1).
    // let cells: [*const Cell; 2] = my_terminal.view_ptr::<2, 2>(1, 1).expect("This should never fail!");
    // ```
    // 
    // Note that *only* creating this is safe, but dereferencing anything inside is not.
    // Every value within is guaranteed to be initialized at time of creation, and is guaranteed to be of the correct size.
    // 
    // # Errors
    // 
    // This will return an [`Err`] if the requested view reaches out-of-bounds.
    // fn view_ptr<const WIDTH: usize, const HEIGHT: usize>(
    //     &self,
    //     x_offset: usize,
    //     y_offset: usize,
    // ) -> crate::Result<[*const Cell; HEIGHT]> {
    //     let (terminal_width, terminal_height) = self.dimensions();
    //     let characters = self.cells();
    // 
    //     if y_offset + HEIGHT > terminal_height {
    //         return Err(Error::OutOfBoundsCoordinate {
    //             x: Some(x_offset + WIDTH),
    //             y: Some(y_offset + HEIGHT),
    //         });
    //     }
    // 
    //     if x_offset + WIDTH > terminal_width {
    //         return Err(Error::OutOfBoundsCoordinate {
    //             x: Some(x_offset + WIDTH),
    //             y: Some(y_offset + HEIGHT),
    //         });
    //     }
    // 
    //     Ok(array::from_fn(|y| {
    //         characters[(y_offset + y) * terminal_width + x_offset..][..WIDTH].as_ptr()
    //     }))
    // }
    // 
    // Returns a view to the [`Cell`]s within the specified area, as pointers instead of references.
    // 
    // ```
    // use tuit::terminal::{ConstantSize, Cell};
    // use tuit::prelude::*;
    // 
    // let mut my_terminal: ConstantSize<20, 20> = ConstantSize::new();
    // 
    // // A get a pointer array of height 2, where each pointer points to 2 characters, at x-y coords (1,1).
    // let cells: [*mut Cell; 2] = my_terminal.view_ptr_mut::<2, 2>(1, 1).expect("This should never fail!");
    // ```
    // 
    // Note that *only* creating this is safe, but dereferencing anything inside is not. Writing anything especially is
    // unsafe, and Rust's borrow-checker cannot save you from yourself here.
    // 
    // Every value within is guaranteed to be initialized at time of creation, and is guaranteed to be of the correct size.
    // 
    // # Errors
    // 
    // This will return an [`Err`] if the requested view reaches out-of-bounds.
    // fn view_ptr_mut<const WIDTH: usize, const HEIGHT: usize>(
    //     &mut self,
    //     x_offset: usize,
    //     y_offset: usize,
    // ) -> crate::Result<[*mut Cell; HEIGHT]> {
    //     let (terminal_height, terminal_width) = self.dimensions();
    //     let characters = self.cells_mut();
    // 
    //     if y_offset + HEIGHT > terminal_height {
    //         return Err(Error::OutOfBoundsCoordinate {
    //             x: Some(x_offset + WIDTH),
    //             y: Some(y_offset + HEIGHT),
    //         });
    //     }
    // 
    //     if x_offset + WIDTH > terminal_width {
    //         return Err(Error::OutOfBoundsCoordinate {
    //             x: Some(x_offset + WIDTH),
    //             y: Some(y_offset + HEIGHT),
    //         });
    //     }
    // 
    //     Ok(array::from_fn(|y| {
    //         characters[(y_offset + y) * terminal_width + x_offset..][..WIDTH].as_mut_ptr()
    //     }))
    // }
    // 
    // Returns a mutable view to the [`Cell`]s within the specified area.
    // 
    // ```
    // use tuit::terminal::{ConstantSize, Cell};
    // use tuit::prelude::*;
    // 
    // let mut my_terminal: ConstantSize<20, 20> = ConstantSize::new();
    // 
    // // A get a mutable view of height 2 and width 2, at x-y coords (1,1).
    // let cells: [[&mut Cell; 2]; 2] = my_terminal.view_mut::<2, 2>(1, 1).expect("This should never fail!");
    // ```
    // 
    // # Errors
    // 
    // This will return an [`Err`] if the requested view reaches out-of-bounds.
    // fn view_mut<const WIDTH: usize, const HEIGHT: usize>(
    //     &mut self,
    //     x_offset: usize,
    //     y_offset: usize,
    // ) -> crate::Result<[[&mut Cell; WIDTH]; HEIGHT]> {
    //     let (terminal_width, terminal_height) = self.dimensions();
    //     let mut characters = self.cells_mut().iter_mut();
    // 
    //     if y_offset + HEIGHT > terminal_height {
    //         return Err(Error::OutOfBoundsCoordinate {
    //             x: Some(x_offset + WIDTH),
    //             y: Some(y_offset + HEIGHT),
    //         });
    //     }
    // 
    //     if x_offset + WIDTH > terminal_width {
    //         return Err(Error::OutOfBoundsCoordinate {
    //             x: Some(x_offset + WIDTH),
    //             y: Some(y_offset + HEIGHT),
    //         });
    //     }
    // 
    //     let index = x_offset + (y_offset * terminal_width);
    // 
    //     // Skips by n items.
    //     characters.nth(index - 1);
    // 
    //     Ok(array::from_fn(|_y_coord| {
    //         // Have to use iterator pattern here, borrow checker does not like arbitrary mutable refs.
    //         let row: [&mut Cell; WIDTH] =
    //             array::from_fn(|_x_coord| characters.next().expect("Should not error"));
    //         characters.nth((terminal_width) - 1);
    //         row
    //     }))
    // }
    // 
    // /// Gets a view similar to [`TerminalExtended::view`], but allows you to pass runtime values for width and height
    // /// of the view, instead of compile-time-known constants.
    // ///
    // /// To use this, you must have the `alloc` feature enabled (it is not on by default).
    // ///
    // /// # Errors
    // ///
    // /// This will return an [`Err`] if the requested view reaches out-of-bounds.
    // #[cfg(feature = "alloc")]
    // fn view_vec(
    //     &self,
    //     width: usize,
    //     height: usize,
    //     x_offset: usize,
    //     y_offset: usize,
    // ) -> crate::Result<Vec<Vec<Cell>>> {
    //     let (terminal_width, terminal_height) = self.dimensions();
    //     let characters = self.characters_slice();
    // 
    //     if y_offset + height > terminal_height {
    //         return Err(Error::OutOfBoundsCoordinate {
    //             x: Some(x_offset + width),
    //             y: Some(y_offset + height),
    //         });
    //     }
    // 
    //     if x_offset + width > terminal_width {
    //         return Err(Error::OutOfBoundsCoordinate {
    //             x: Some(x_offset + width),
    //             y: Some(y_offset + height),
    //         });
    //     }
    // 
    //     let mut output = Vec::new();
    // 
    //     for y in 0..height {
    //         output.push(Vec::new());
    //         for x in width..width + x_offset {
    //             output[y].push(characters[x + (y + y_offset) * terminal_width]);
    //         }
    //     }
    // 
    //     Ok(output)
    // }
}

impl<T: Terminal> Extended for T {}
