use crate::Error;
use crate::prelude::{Terminal, TerminalConst, Widget};
use crate::style::Style;
use crate::terminal::{UpdateInfo, UpdateResult, Rectangle};
use crate::widgets::{BoundingBox, };

/// Text at the top-left of the terminal.
pub struct Text<'a> {
    /// The text to display.
    pub text: &'a str,
    /// The style with which to display it.
    pub style: Style
}

impl<'a> Text<'a> {
    /// Create a new [`Text`] with the default style.
    ///
    /// ```
    /// use tuit::style::Style;
    /// use tuit::widgets::builtins::Text;
    ///
    /// let text_widget = Text::new("Hello!");
    ///
    /// assert_eq!(text_widget.text, "Hello!");
    /// assert_eq!(text_widget.style, Style::new());
    /// ```
    #[must_use]
    pub const fn new(text: &'a str) -> Self {
        Self {
            text,
            style: Style::new()
        }
    }
}

impl Widget for Text<'_> {
    fn update(&mut self, _update_info: UpdateInfo, _terminal: impl TerminalConst) -> crate::Result<UpdateResult> {
        Ok(UpdateResult::NoEvent)
    }

    fn draw(&self, _update_info: UpdateInfo, mut terminal: impl Terminal) -> crate::Result<UpdateResult> {
        let mut cells = terminal.cells_mut();

        for (idx, character) in self.text.chars().enumerate() {
            let current_cell = cells
                .next()
                .ok_or(Error::OutOfBoundsCharacter(idx))?;

            current_cell.character = character;
            current_cell.style = self.style;
        }

        Ok(UpdateResult::NoEvent)
    }
}

impl BoundingBox for Text<'_> {
    fn bounding_box(&self, terminal: impl TerminalConst) -> Rectangle {
        let height = self.text.len() / terminal.width();
        let width = self.text.len().min(terminal.width());

        Rectangle::of_size(width, height)
    }

    fn completely_covers(&self, rectangle: Rectangle) -> bool {
        self.text.len() >= rectangle.area()
    }
}