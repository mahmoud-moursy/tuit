//! Terminal features that need allocation to work properly. Check the [`extras`] module

#[cfg(feature = "extras")]
pub mod extras;
/// A [`crate::terminal::ConstantSize`] initialized inside a [`alloc::boxed::Box`], to avoid
/// overflowing the stack with larger terminal sizes.
pub mod constant_boxed;

pub use constant_boxed::ConstantBoxed;