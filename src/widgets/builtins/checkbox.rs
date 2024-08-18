use crate::prelude::{Terminal, TerminalConst, WithLayout};
use crate::style::Style;
use crate::terminal::{KeyState, MouseButton, Rectangle, UpdateInfo, UpdateResult};
use crate::widgets::builtins::Text;
use crate::widgets::{BoundingBox, Widget};

/// A checkbox widget.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct Checkbox<'a> {
    /// The entry of the checkbox.
    pub entry: &'a str,
    /// Whether the checkbox is checked.
    pub checked: bool,
    /// The style of the tickbox.
    pub box_style: Style,
    /// The style of the entry text.
    pub entry_style: Style,
}

impl<'a> Checkbox<'a> {
    const CHECKED: &'static str = "[x] ";
    const UNCHECKED: &'static str = "[ ] ";
    const CHECKBOX_WIDTH: usize = Self::CHECKED.len();

    /// Create a new [`Checkbox`] with the specified value.
    #[must_use]
    pub const fn new(entry: &'a str) -> Self {
        Self { entry, checked: false, box_style: Style::new(), entry_style: Style::new() }
    }

    /// Check the [`Checkbox`]!
    ///
    /// `[ ]` -> `[x]`
    #[must_use]
    pub const fn checked(mut self) -> Self {
        self.checked = true;

        self
    }

    /// Uncheck the [`Checkbox`]!
    ///
    /// `[x]` -> `[ ]`
    #[must_use]
    pub const fn unchecked(mut self) -> Self {
        self.checked = false;

        self
    }
}

impl Widget for Checkbox<'_> {
    fn update(&mut self, update_info: UpdateInfo, terminal: impl TerminalConst) -> crate::Result<UpdateResult> {
        #[cfg(feature = "debug")]
        log::trace!("Checkbox update: {:?}", update_info);

        match update_info {
            UpdateInfo::CellClicked(x, y, MouseButton::Primary) => {
                #[cfg(feature = "debug")]
                {
                    log::trace!("Checkbox saw click at ({x}, {y})");
                    let bb = self.bounding_box_in(&terminal)?;
                    log::trace!("Checkbox bounding box: {bb:?}");
                }


                if self.bounding_box_in(&terminal)?.contains((x, y)) {
                    self.checked = !self.checked;

                    return Ok(UpdateResult::Interacted)
                }
            }
            UpdateInfo::KeyboardInput(0x28, KeyState::KeyDown) => {
                self.checked = !self.checked;

                return Ok(UpdateResult::Interacted)
            }
            _ => {}
        }

        Ok(UpdateResult::NoEvent)
    }

    fn draw(&self, terminal: impl Terminal) -> crate::Result<UpdateResult> {
        let box_text = if self.checked { Self::CHECKED } else { Self::UNCHECKED };
        let mut box_widget = Text::new(box_text);
        box_widget.style = self.box_style;

        let mut entry = Text::new(self.entry);
        entry.style = self.entry_style;

        let checkbox = box_widget.next_to(entry);

        checkbox.draw(terminal)
    }
}

impl BoundingBox for Checkbox<'_> {
    fn bounding_box(&self, rect: Rectangle) -> crate::Result<Rectangle> {
        let box_widget = Text::new(Self::UNCHECKED);
        let entry = Text::new(self.entry);

        let checkbox = box_widget.next_to(entry);

        checkbox.bounding_box(rect)
    }

    fn completely_covers(&self, rect: Rectangle) -> bool {
        let box_widget = Text::new(Self::UNCHECKED);
        let entry = Text::new(self.entry);

        let checkbox = box_widget.next_to(entry);

        checkbox.completely_covers(rect)
    }
}