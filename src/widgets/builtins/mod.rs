pub use centered_prompt::CenteredPrompt;
pub use centered_text::CenteredText;
pub use ruler::Ruler;
pub use sweeper::Sweeper;
pub use text::Text;
pub use uv::Uv;
pub use margin::Margin;
pub use centered::Centered;
pub use stacked::Stacked;
pub use buttons::Buttons;
use crate::widgets::BoundingBox;

/// The code for the [`Sweeper`] widget.
pub mod sweeper;
/// The code for the [`CenteredText`] widget.
pub mod centered_text;
/// The code for the [`CenteredPrompt`] widget.
pub mod centered_prompt;
/// The code for the [`Ruler`] widget.
pub mod ruler;
/// The code for the [`Text`] widget.
pub mod text;
/// The code for the [`Uv`] widget.
pub mod uv;
mod margin;
/// The code the [`dummy::Dummy`] widget.
pub mod dummy;
/// The code for the [`Centered`] widget.
pub mod centered;
/// The code for the [`Stacked`] widget.
pub mod stacked;
/// The code for the [`Buttons`] widget.
pub mod buttons;

impl<T: BoundingBox> From<T> for Centered<T> {
    fn from(value: T) -> Self {
        Self::new(value)
    }
}

impl<T: BoundingBox> From<T> for Margin<T> {
    fn from(value: T) -> Self {
        Self::new(value)
    }
}

/// An auto-implemented trait for widgets that provides some convenience methods for layouts.
pub trait WithLayout: Sized {
    /// Adds a margin by the specified distance -- can be negative to expand the widget.
    fn with_margin(self, margin: isize) -> Margin<Self> {
        Margin::new(self).margin(margin)
    }

    /// Centers the widget.
    fn centered(self) -> Centered<Self> {
        Centered::new(self)
    }

    /// Stacks the widget on top of another widget.
    fn on_top_of<T>(self, other: T) -> Stacked<Self, T> {
        Stacked::new(self, other)
    }
}

impl<T: BoundingBox> WithLayout for T {}
