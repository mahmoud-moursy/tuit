use alloc::boxed::Box;
use alloc::vec;
use core::ops::{Deref, DerefMut};
use crate::prelude::Metadata;
use crate::style::Style;
use crate::terminal::{Cell, TerminalConst, TerminalMut};

#[allow(unused_imports)] // used in docs.
use crate::terminal::ConstantSize;

/// A [`ConstantSize`] that initializes its [`Cell`]s inside a [`Box`] to avoid a stack overflow.
///
/// ## Why not a `Vec`?
///
/// `Vec` holds no intrinsic guarantee that the shape will be consistent.
/// A bug in a feature like, e.g. rescaling, could cause unexpected results.
///
/// If you want a resizable terminal, the `extras`+`alloc`
/// features will enable the [`super::extras::Rescale`]
/// terminal.
/// 
/// Enabling extras will also add a dependency on `ndarray`.
#[derive(Debug, Hash, Eq, PartialEq, Clone)]
pub struct ConstantBoxed<const WIDTH: usize, const HEIGHT: usize> {
    cells: Box<[Box<[Cell; WIDTH]>; HEIGHT]>,
    default_style: Style
}

impl<const WIDTH: usize, const HEIGHT: usize> Default for ConstantBoxed<WIDTH, HEIGHT> {
    fn default() -> Self {
        Self::new()
    }
}

impl<const WIDTH: usize, const HEIGHT: usize> ConstantBoxed<WIDTH, HEIGHT> {
    /// Create a new [`ConstantBoxed`]
    #[allow(clippy::missing_panics_doc)]
    #[must_use]
    pub fn new() -> Self {
        let rows:
            Box<[Box<[Cell; WIDTH]>; HEIGHT]>
            = vec![
                vec![Cell::default(); WIDTH]
                        .into_boxed_slice()
                        .try_into()
                        .expect("Always correct size. Never fails.");
                HEIGHT
                ]
                .into_boxed_slice()
                .try_into()
                .expect("Always correct size. Never fails.");

        Self {
            cells: rows,
            default_style: Style::default(),
        }
    }
}

impl<const WIDTH: usize, const HEIGHT: usize> Metadata for ConstantBoxed<WIDTH, HEIGHT> {
    fn dimensions(&self) -> (usize, usize) {
        (WIDTH, HEIGHT)
    }

    fn default_style(&self) -> Style {
        self.default_style
    }
}

impl<const WIDTH: usize, const HEIGHT: usize> TerminalConst for ConstantBoxed<WIDTH, HEIGHT> {
    fn cells(&self) -> impl Iterator<Item=&Cell> {
        self.cells.iter().flat_map(Deref::deref)
    }

    fn cell(&self, x: usize, y: usize) -> Option<&Cell> {
        self.cells.get(y).and_then(|row| row.get(x))
    }
}

impl<const WIDTH: usize, const HEIGHT: usize> TerminalMut for ConstantBoxed<WIDTH, HEIGHT> {
    fn cells_mut(&mut self) -> impl Iterator<Item=&mut Cell> {
        self.cells.iter_mut().flat_map(DerefMut::deref_mut)
    }

    fn cell_mut(&mut self, x: usize, y: usize) -> Option<&mut Cell> {
        self.cells.get_mut(y).and_then(|row| row.get_mut(x))
    }
}