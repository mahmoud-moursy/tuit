use crate::style::Style;
use crate::terminal::{Cell, Metadata};
use crate::terminal::TerminalMut;
use crate::widgets::Rectangle;

/// A mutable view into another [`TerminalMut`].
pub struct ViewMut<const WIDTH: usize, const HEIGHT: usize, T>
 {
    /// The parent terminal containing the characters inside the view
    parent: T,
    /// The default style of the parent terminal
    default_style: Style,
    /// The area that the view draws into.
    view_area: Rectangle,
}

impl<const WIDTH: usize, const HEIGHT: usize, T> Metadata for ViewMut<WIDTH, HEIGHT, T>
where T: TerminalMut {
    fn dimensions(&self) -> (usize, usize) {
        self.parent.dimensions()
    }

    fn default_style(&self) -> Style {
        self.default_style
    }
}

impl<const WIDTH: usize, const HEIGHT: usize, T> TerminalMut for ViewMut<WIDTH, HEIGHT, T>
where T: TerminalMut {
    fn cells_mut(&mut self) -> impl Iterator<Item=&mut Cell> {
        let parent_width = self.parent.width();
        let view_top = self.view_area.top();
        let view_left = self.view_area.left();
        let view_height = self.view_area.height();
        let view_width = self.view_area.width();
        let cells = self.parent.cells_mut();

        assert_eq!(view_height, HEIGHT);
        assert_eq!(view_width, WIDTH);

        todo!("This currently returns incorrect info.");

        cells
            .skip(view_left)
            .skip(view_top * parent_width)
            .array_chunks::<WIDTH>()
            .step_by(parent_width)
            .take(view_height)
            .flatten()
    }
}

impl<const WIDTH: usize, const HEIGHT: usize, T> ViewMut<WIDTH, HEIGHT, T> {
    /// Creates a new [`ViewMut`] from the given [`TerminalMut`] and the left-top
    /// coordinate.
    pub fn new(terminal: T, left_top: (usize, usize)) -> Self
    where T: Metadata {
        Self {
            default_style: terminal.default_style(),
            parent: terminal,
            view_area: Rectangle::of_size(WIDTH, HEIGHT).to(left_top)
        }
    }
}

