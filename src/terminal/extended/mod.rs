//! This module contains the [`Extended`] trait that contains some extra methods.

use crate::prelude::*;
use crate::terminal::extended::view::View;
use crate::widgets::Rectangle;

/// The [`View`] terminal, that can provide mutable or immutable views into terminals.
mod view;
mod view_iterator;

/// Some things here require extra dependencies that I didn't want to add to needlessly add to the
/// crate... so they got broken off into their own thing.
pub trait Extended: Terminal {
    /// Returns a mutable [`View`] into the [`Terminal`] within the specified [`Rectangle`].
    fn view_mut(&mut self, rect: Rectangle) -> Option<View<&mut Self>> {
        View::new(self, rect)
    }


    /// Return an immutable [`View`] into the [`Terminal`] within the specified [`Rectangle`].
    fn view(&self, rect: Rectangle) -> Option<View<&Self>> {
        View::new(self, rect)
    }
}

impl<T: Terminal> Extended for T {}
