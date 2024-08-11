//! A shelved example.
use tuit::terminal::ConstantSize;
use tuit::widgets::builtins::{Shelved, Sweeper, Text};
use tuit::prelude::*;
use tuit::std::stdout_render::StdoutRenderer;
use tuit::style::{Ansi4, Colour};
use tuit::style::Colour::Ansi16;

fn main() {
    env_logger::builder().filter_level(log::LevelFilter::Trace).init();

    let mut terminal: ConstantSize<200, 50> = ConstantSize::new();

    Sweeper::of_colour(Colour::Ansi16(Ansi4::Cyan)).drawn(&mut terminal).expect("Always ok");

    let hello = Text::new("Hello!").with_margin(2);
    let world = Text::new("World!").with_margin(2);

    let shelved = hello.next_to(world).centered().with_margin(2);

    shelved.use_backdrop(Ansi16(Ansi4::Yellow)).drawn(&mut terminal).expect("Always ok");
    shelved.drawn(&mut terminal).ok();

    let mut renderer = StdoutRenderer::default();
    renderer.render(&terminal).expect("Always ok");
}