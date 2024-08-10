//! An example demonstrating the [`Rescale`] terminal.

use tuit::allocations::terminal::extras::Rescale;
use tuit::prelude::*;
use tuit::std::stdout_render::StdoutRenderer;
use tuit::terminal::Rescalable;
use tuit::widgets::builtins::Uv;

fn main() {
    let mut rescalable = Rescale::default();
    let uv = Uv;

    rescalable.rescale((200, 20)).ok();

    uv.drawn(&mut rescalable).ok();

    rescalable.rescale((20, 20)).ok();

    print!("This should be a UV texture but... woah, what happened!?");
    rescalable.display(StdoutRenderer::default()).ok();
    print!("\nYou just saw blue because the terminal wasn't redrawn after scaling.");
    uv.drawn(&mut rescalable).ok();

    rescalable.display(StdoutRenderer::default()).ok();

    print!("\nHere is the correct UV texture. This is why you should re-draw after re-scaling");

    rescalable.rescale((200, 20)).ok();
    uv.drawn(&mut rescalable).ok();
    rescalable.display(StdoutRenderer::default()).ok();
    println!("\n...and this is what the large-scale image is.");
}