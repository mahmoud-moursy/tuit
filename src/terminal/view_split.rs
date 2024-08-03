use crate::style::Style;
use crate::terminal::{Cell, Metadata, Rectangle, TerminalConst, TerminalMut};
use crate::terminal::view::View;
use crate::widgets::Direction;

pub struct ViewSplit<T> {
    parent: T
}

impl<T> ViewSplit<T> {
    pub const fn new(parent: T) -> Self {
        Self {
            parent
        }
    }

    pub fn into_inner(self) -> T {
        self.parent
    }

    pub fn left_view(&self) -> View<&T>
    where T: TerminalConst {
        let min_x = self.parent.bounding_box().left();
        let max_x = self.parent.bounding_box().center_x();
        let min_y = self.parent.bounding_box().top();
        let max_y = self.parent.bounding_box().bottom();
        let bounding_box = Rectangle::new((min_x, min_y), (max_x, max_y));

        self.parent.view(bounding_box).expect("View should've been valid.")
    }

    pub fn right_view(&self) -> View<&T>
    where T: TerminalConst {
        let min_x = self.parent.bounding_box().center_x();
        let max_x = self.parent.bounding_box().right();
        let min_y = self.parent.bounding_box().top();
        let max_y = self.parent.bounding_box().bottom();
        let bounding_box = Rectangle::new((min_x, min_y), (max_x, max_y));

        self.parent.view(bounding_box).expect("View should've been valid.")
    }

    pub fn top_view(&self) -> View<&T>
    where T: TerminalConst {
        let min_x = self.parent.bounding_box().left();
        let max_x = self.parent.bounding_box().right();
        let min_y = self.parent.bounding_box().top();
        let max_y = self.parent.bounding_box().center_y();
        let bounding_box = Rectangle::new((min_x, min_y), (max_x, max_y));

        self.parent.view(bounding_box).expect("View should've been valid.")
    }
    pub fn bottom_view(&self) -> View<&T>
    where T: TerminalConst {
        let min_x = self.parent.bounding_box().left();
        let max_x = self.parent.bounding_box().right();
        let min_y = self.parent.bounding_box().center_y();
        let max_y = self.parent.bounding_box().bottom();
        let bounding_box = Rectangle::new((min_x, min_y), (max_x, max_y));

        self.parent.view(bounding_box).expect("View should've been valid.")
    }

    pub fn left_view_mut(&mut self) -> View<&mut T>
    where T: TerminalMut {
        let min_x = self.parent.bounding_box().left();
        let max_x = self.parent.bounding_box().center_x();
        let min_y = self.parent.bounding_box().top();
        let max_y = self.parent.bounding_box().bottom();
        let bounding_box = Rectangle::new((min_x, min_y), (max_x, max_y));

        self.parent.view_mut(bounding_box).expect("View should've been valid.")
    }

    pub fn right_view_mut(&mut self) -> View<&mut T>
    where T: TerminalMut {
        let min_x = self.parent.bounding_box().center_x();
        let max_x = self.parent.bounding_box().right();
        let min_y = self.parent.bounding_box().top();
        let max_y = self.parent.bounding_box().bottom();
        let bounding_box = Rectangle::new((min_x, min_y), (max_x, max_y));

        self.parent.view_mut(bounding_box).expect("View should've been valid.")
    }

    pub fn top_view_mut(&mut self) -> View<&mut T>
    where T: TerminalMut {
        let min_x = self.parent.bounding_box().left();
        let max_x = self.parent.bounding_box().right();
        let min_y = self.parent.bounding_box().top();
        let max_y = self.parent.bounding_box().center_y();
        let bounding_box = Rectangle::new((min_x, min_y), (max_x, max_y));

        self.parent.view_mut(bounding_box).expect("View should've been valid.")
    }

    pub fn bottom_view_mut(&mut self) -> View<&mut T>
    where T: TerminalMut {
        let min_x = self.parent.bounding_box().left();
        let max_x = self.parent.bounding_box().right();
        let min_y = self.parent.bounding_box().center_y();
        let max_y = self.parent.bounding_box().bottom();
        let bounding_box = Rectangle::new((min_x, min_y), (max_x, max_y));

        self.parent.view_mut(bounding_box).expect("View should've been valid.")
    }

    pub fn select_view(&self, direction: Direction) -> View<&T>
    where T: TerminalConst {
        match direction {
            Direction::Down => self.bottom_view(),
            Direction::Up => self.top_view(),
            Direction::Left => self.left_view(),
            Direction::Right => self.right_view(),
        }
    }

    pub fn select_view_mut(&mut self, direction: Direction) -> View<&mut T>
    where T: TerminalMut {
        match direction {
            Direction::Down => self.bottom_view_mut(),
            Direction::Up => self.top_view_mut(),
            Direction::Left => self.left_view_mut(),
            Direction::Right => self.right_view_mut(),
        }
    }
}

impl<T: Metadata> Metadata for ViewSplit<T> {
    fn dimensions(&self) -> (usize, usize) {
        self.parent.dimensions()
    }

    fn default_style(&self) -> Style {
        self.parent.default_style()
    }
}

impl<T: TerminalConst> TerminalConst for ViewSplit<T> {
    fn cells(&self) -> impl Iterator<Item=&Cell> {
        self.parent.cells()
    }

    fn cell(&self, x: usize, y: usize) -> Option<&Cell> {
        self.parent.cell(x, y)
    }
}

impl<T: TerminalMut> TerminalMut for ViewSplit<T> {
    fn cells_mut(&mut self) -> impl Iterator<Item=&mut Cell> {
        self.parent.cells_mut()
    }

    fn cell_mut(&mut self, x: usize, y: usize) -> Option<&mut Cell> {
        self.parent.cell_mut(x, y)
    }
}