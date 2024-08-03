//! An example demonstrating the [`Rescale`] terminal.

use std::io::stdout;
use tuit::allocations::terminal::extras::resize::Rescale;
use tuit::terminal::Rescalable;
use tuit::prelude::*;
use tuit::widgets::builtins::Uv;

fn main() {
    let mut rescalable = Rescale::default();
    let uv = Uv;

    rescalable.rescale((200, 20)).ok();

    uv.drawn(&mut rescalable).ok();

    rescalable.rescale((20, 20)).ok();

    print!("This should be a UV texture but... woah, what happened!?");
    rescalable.display(stdout()).ok();
    print!("\nYou just saw blue because the terminal wasn't redrawn after scaling.");
    uv.drawn(&mut rescalable).ok();

    rescalable.display(stdout()).ok();

    print!("\nHere is the correct UV texture. This is why you should re-draw after re-scaling");

    rescalable.rescale((200, 20)).ok();
    uv.drawn(&mut rescalable).ok();
    rescalable.display(stdout()).ok();
    println!("\n...and this is what the large-scale image is.");
}