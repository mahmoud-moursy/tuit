//! The parts of Tuit that can be enabled when there is standard library support.

extern crate std;
/// A terminal that outputs to [`std::io::stdout`].
pub mod stdout_render;
mod errors;
