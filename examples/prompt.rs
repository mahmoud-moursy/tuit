//! Demonstrates centered prompts.

use tuit::prelude::*;
use tuit::std::stdout_render::StdoutRenderer;
use tuit::style::{Ansi4, Colour::Ansi16};
use tuit::terminal::ConstantSize;
use tuit::widgets::builtins::sweeper::Sweeper;
use tuit::widgets::builtins::{Buttons, Text};

#[cfg(not(feature = "ansi_renderer"))]
fn main() {
    println!("You must apply the stdout_render feature to view this example. Use `cargo --features stdout_render`");
}

#[cfg(feature = "ansi_renderer")]
fn main() {
    let mut terminal: ConstantSize<58, 14> = ConstantSize::new();
    //
    // let text = CenteredText::new("Hello world!");
    //
    // text.drawn(&mut terminal)
    //     .expect("This method CAN fail, but only if the prompt is too large. Here, it is not.");

    let mut renderer = StdoutRenderer::default();

    let sweeper = Sweeper::of_colour(Ansi16(Ansi4::BrightCyan));

    let query = Text::new("Continue?").with_margin(1);
    let buttons = Buttons::new(&[" Yes ", " No "]).select_last().centered();

    let prompt = query.on_top_of(buttons).centered();


    sweeper.drawn(&mut terminal).ok();
    prompt.drawn(&mut terminal).ok();

    renderer.render(terminal).ok();
}
