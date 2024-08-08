use crate::style::Style;
use crate::terminal::{Cell, Metadata, TerminalConst};
use crate::terminal::view_iterator::ViewIterator;
use crate::terminal::TerminalMut;
use crate::terminal::Rectangle;

/// A mutable view into another [`TerminalMut`].
pub struct View<T>
{
    /// The parent terminal containing the characters inside the view
    parent: T,
    /// The default style of the parent terminal
    default_style: Style,
    /// The area that the view draws into.
    rect: Rectangle,
}

impl<T> Metadata for View<T>
where T: Metadata {
    fn dimensions(&self) -> (usize, usize) {
        self.rect.dimensions()
    }

    fn default_style(&self) -> Style {
        self.default_style
    }
}

impl<T> TerminalConst for View<T>
where T: TerminalConst {
    fn cells(&self) -> impl Iterator<Item=&Cell> {
        let parent_dimensions @ (width, height) = self.parent.dimensions();
        let view_top = self.rect.top();
        let view_left = self.rect.left();
        let cells = self.parent.cells();

        ViewIterator {
            child: cells
                .skip(view_left)
                .skip(view_top * width),
            current_coord: (0,0),
            parent_dimensions,
            view_rect: self.rect
        }
    }

    fn cell(&self, x: usize, y: usize) -> Option<&Cell> {
        let x = x + self.rect.left();
        let y = y + self.rect.top();

        if self.rect.contains((x, y)) {
            self.parent.cell(x, y)
        } else {
            None
        }
    }
}

impl<T> TerminalMut for View<T>
where T: TerminalMut {
    fn cells_mut(&mut self) -> impl Iterator<Item=&mut Cell> {
        let parent_dimensions @ (width, height) = self.parent.dimensions();
        let view_top = self.rect.top();
        let view_left = self.rect.left();
        let cells = self.parent.cells_mut();

        ViewIterator {
            child: cells
                .skip(view_left)
                .skip(view_top * width),
            current_coord: (0,0),
            parent_dimensions,
            view_rect: self.rect
        }
    }

    fn cell_mut(&mut self, x: usize, y: usize) -> Option<&mut Cell> {
        let x = x + self.rect.left();
        let y = y + self.rect.top();

        if self.rect.contains((x, y)) {
            self.parent.cell_mut(x, y)
        } else {
            None
        }
    }
}


impl<T> View<T> {
    /// Creates a new [`View`] from the given [`TerminalMut`] or [`TerminalConst`] and the given [`Rectangle`]
    pub fn new(terminal: T, view_rect: Rectangle) -> Option<Self>
    where T: Metadata {
        if terminal.bounding_box().contains_rect(view_rect) {
            Some(Self {
                default_style: terminal.default_style(),
                parent: terminal,
                rect: view_rect
            })
        } else {
            None
        }
    }
}

