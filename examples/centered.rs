//! Show how the [`Centered`] widget can be used.

use log::LevelFilter;
use tuit::prelude::*;
use tuit::std::stdout_render::StdoutRenderer;
use tuit::style::{Ansi4, Colour};
use tuit::terminal::ConstantSize;
use tuit::widgets::BoundingBox;
use tuit::widgets::builtins::{Buttons, Sweeper, Text};

fn main() {
    env_logger::builder().filter_level(LevelFilter::Trace).init();

    let mut terminal: ConstantSize<50,20> = ConstantSize::new();

    let yellow = Sweeper::of_colour(Colour::Ansi16(Ansi4::Yellow));

    yellow.drawn(&mut terminal).unwrap();

    let mut buttons = Buttons::new(&[" No ", " Yes "]).select_last();

    buttons.selected_button_style = buttons.selected_button_style.bg(Colour::Ansi16(Ansi4::Red)).underlined().font_weight(600);
    buttons.unselected_button_style = buttons.unselected_button_style.bg(Colour::Ansi16(Ansi4::BrightRed));

    let mut continue_str = String::from("T");

    loop {
        yellow.drawn(&mut terminal).unwrap();

        let prompt = Text::new(&continue_str)
            .with_margin(2)
            .centered();

        let blue_bg_area = prompt.bounding_box_in(&terminal).expect("Bounding box should always be Ok");
        let blue_bg_view = terminal.view_mut(blue_bg_area).expect("This should always be Ok");

        let blue_bg = Sweeper::of_colour(Colour::Ansi16(Ansi4::Blue));
        blue_bg.drawn(blue_bg_view).expect("Oop! Ran out of space!");
        prompt.drawn(&mut terminal).expect("Hopefully there is enough space!");

        terminal.display(StdoutRenderer::default()).expect("Oop! Ran out of space!");
        println!();

        continue_str.push_str("T");
    }
}