//! A shelved example.
use tuit::prelude::*;
use tuit::std::stdout_render::StdoutRenderer;
use tuit::style::Colour::Ansi16;
use tuit::style::{Ansi4, Colour};
use tuit::terminal::ConstantSize;
use tuit::widgets::builtins::{Sweeper, Text};

fn main() {
    env_logger::builder().filter_level(log::LevelFilter::Trace).init();

    let mut terminal: ConstantSize<200, 50> = ConstantSize::new();

    Sweeper::of_colour(Colour::Ansi16(Ansi4::Cyan)).drawn(&mut terminal).expect("Always ok");

    let hi = Text::new("Yello!").with_margin(2);
    let there = Text::new("There!").with_margin(2);
    let hi_there = hi.next_to(there).with_margin(2);
    let hello = Text::new("Hello!").with_margin(2);
    let world = Text::new("World!").with_margin(2);

    let hello_world = hello.next_to(world).centered().with_margin(2).on_top_of(hi_there);

    hello_world.use_backdrop(Ansi16(Ansi4::Yellow)).drawn(&mut terminal).expect("Always ok");
    hello_world.drawn(&mut terminal).ok();

    let mut renderer = StdoutRenderer::default();
    renderer.render(&terminal).expect("Always ok");
}