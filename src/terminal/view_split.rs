use crate::style::Style;
use crate::terminal::{Cell, Metadata, Rectangle, Rescalable, TerminalConst, TerminalMut};
use crate::terminal::view::View;
use crate::widgets::Direction;

#[allow(unused_imports)] // used in docs.
use crate::terminal::Terminal;

/// A view splitter -- can split views both horizontally and vertically.
pub struct ViewSplit<T> {
    child: T
}

impl<T> ViewSplit<T> {
    /// Make a new [`ViewSplit`] using the `Terminal` you want to split.
    pub const fn new(child: T) -> Self {
        Self {
            child
        }
    }

    /// Convert the [`ViewSplit`] into its inner value (the terminal with which it was created).
    pub fn into_inner(self) -> T {
        self.child
    }

    /// Get a reference to the [`Terminal`] inside the [`ViewSplit`].
    pub const fn inner(&self) -> &T {
        &self.child
    }

    /// Splits the terminal into left/right, and returns the left [`View`].
    #[allow(clippy::missing_panics_doc)]
    pub fn split_left(&self) -> View<&T>
    where T: TerminalConst {
        let min_x = self.child.bounding_box().left();
        let max_x = self.child.bounding_box().center_x();
        let min_y = self.child.bounding_box().top();
        let max_y = self.child.bounding_box().bottom();
        let bounding_box = Rectangle::new((min_x, min_y), (max_x, max_y));

        self.child.view(bounding_box).expect("View should've been valid.")
    }

    /// Splits the terminal into left/right, and returns the right [`View`].
    #[allow(clippy::missing_panics_doc)]
    pub fn split_right(&self) -> View<&T>
    where T: TerminalConst {
        let min_x = self.child.bounding_box().center_x();
        let max_x = self.child.bounding_box().right();
        let min_y = self.child.bounding_box().top();
        let max_y = self.child.bounding_box().bottom();
        let bounding_box = Rectangle::new((min_x, min_y), (max_x, max_y));

        self.child.view(bounding_box).expect("View should've been valid.")
    }

    /// Splits the terminal into top/bottom, and returns the top [`View`].
    #[allow(clippy::missing_panics_doc)]
    pub fn split_top(&self) -> View<&T>
    where T: TerminalConst {
        let min_x = self.child.bounding_box().left();
        let max_x = self.child.bounding_box().right();
        let min_y = self.child.bounding_box().top();
        let max_y = self.child.bounding_box().center_y();
        let bounding_box = Rectangle::new((min_x, min_y), (max_x, max_y));

        self.child.view(bounding_box).expect("View should've been valid.")
    }

    /// Splits the terminal into top/bottom, and returns the bottom [`View`].
    #[allow(clippy::missing_panics_doc)]
    pub fn split_bottom(&self) -> View<&T>
    where T: TerminalConst {
        let min_x = self.child.bounding_box().left();
        let max_x = self.child.bounding_box().right();
        let min_y = self.child.bounding_box().center_y();
        let max_y = self.child.bounding_box().bottom();
        let bounding_box = Rectangle::new((min_x, min_y), (max_x, max_y));

        self.child.view(bounding_box).expect("View should've been valid.")
    }

    /// Splits the terminal into left/right, and returns the left [`View`].
    #[allow(clippy::missing_panics_doc)]
    pub fn split_left_mut(&mut self) -> View<&mut T>
    where T: TerminalMut {
        let min_x = self.child.bounding_box().left();
        let max_x = self.child.bounding_box().center_x();
        let min_y = self.child.bounding_box().top();
        let max_y = self.child.bounding_box().bottom();
        let bounding_box = Rectangle::new((min_x, min_y), (max_x, max_y));

        self.child.view_mut(bounding_box).expect("View should've been valid.")
    }

    /// Splits the terminal into left/right, and returns the right [`View`].
    #[allow(clippy::missing_panics_doc)]
    pub fn split_right_mut(&mut self) -> View<&mut T>
    where T: TerminalMut {
        let min_x = self.child.bounding_box().center_x();
        let max_x = self.child.bounding_box().right();
        let min_y = self.child.bounding_box().top();
        let max_y = self.child.bounding_box().bottom();
        let bounding_box = Rectangle::new((min_x, min_y), (max_x, max_y));

        self.child.view_mut(bounding_box).expect("View should've been valid.")
    }
    /// Splits the terminal into top/bottom, and returns the top [`View`].
    #[allow(clippy::missing_panics_doc)]
    pub fn split_top_mut(&mut self) -> View<&mut T>
    where T: TerminalMut {
        let min_x = self.child.bounding_box().left();
        let max_x = self.child.bounding_box().right();
        let min_y = self.child.bounding_box().top();
        let max_y = self.child.bounding_box().center_y();
        let bounding_box = Rectangle::new((min_x, min_y), (max_x, max_y));

        self.child.view_mut(bounding_box).expect("View should've been valid.")
    }
    /// Splits the terminal into top/bottom, and returns the bottom [`View`].
    #[allow(clippy::missing_panics_doc)]
    pub fn split_bottom_mut(&mut self) -> View<&mut T>
    where T: TerminalMut {
        let min_x = self.child.bounding_box().left();
        let max_x = self.child.bounding_box().right();
        let min_y = self.child.bounding_box().center_y();
        let max_y = self.child.bounding_box().bottom();
        let bounding_box = Rectangle::new((min_x, min_y), (max_x, max_y));

        self.child.view_mut(bounding_box).expect("View should've been valid.")
    }

    /// Select a split based on the given [`Direction`].
    ///
    /// - [`Direction::Down`] returns a [`ViewSplit::split_bottom`]
    /// - [`Direction::Up`] returns a [`ViewSplit::split_top`]
    /// - [`Direction::Left`] returns a [`ViewSplit::split_left`]
    /// - [`Direction::Right`] returns a [`ViewSplit::split_right`]
    pub fn split(&self, direction: Direction) -> View<&T>
    where T: TerminalConst {
        match direction {
            Direction::Down => self.split_bottom(),
            Direction::Up => self.split_top(),
            Direction::Left => self.split_left(),
            Direction::Right => self.split_right(),
        }
    }

    /// Select a split based on the given [`Direction`].
    ///
    /// - [`Direction::Down`] returns a [`ViewSplit::split_bottom_mut`]
    /// - [`Direction::Up`] returns a [`ViewSplit::split_top_mut`]
    /// - [`Direction::Left`] returns a [`ViewSplit::split_left_mut`]
    /// - [`Direction::Right`] returns a [`ViewSplit::split_right_mut`]
    pub fn split_mut(&mut self, direction: Direction) -> View<&mut T>
    where T: TerminalMut {
        match direction {
            Direction::Down => self.split_bottom_mut(),
            Direction::Up => self.split_top_mut(),
            Direction::Left => self.split_left_mut(),
            Direction::Right => self.split_right_mut(),
        }
    }
}

impl<T: Metadata> Metadata for ViewSplit<T> {
    fn dimensions(&self) -> (usize, usize) {
        self.child.dimensions()
    }

    fn default_style(&self) -> Style {
        self.child.default_style()
    }
}

impl<T: TerminalConst> TerminalConst for ViewSplit<T> {
    fn cells(&self) -> impl Iterator<Item=&Cell> {
        self.child.cells()
    }

    fn cell(&self, x: usize, y: usize) -> Option<&Cell> {
        self.child.cell(x, y)
    }
}

impl<T: TerminalMut> TerminalMut for ViewSplit<T> {
    fn cells_mut(&mut self) -> impl Iterator<Item=&mut Cell> {
        self.child.cells_mut()
    }

    fn cell_mut(&mut self, x: usize, y: usize) -> Option<&mut Cell> {
        self.child.cell_mut(x, y)
    }
}

impl<T: Rescalable> Rescalable for ViewSplit<T> {
    fn rescale(&mut self, new_size: (usize, usize)) -> Result<(), (usize, usize)> {
        self.child.rescale(new_size)
    }
}