//! Demonstrates centered prompts.

use tuit::prelude::*;
use tuit::std::stdout_render::StdoutRenderer;
use tuit::style::{Ansi4, Colour::Ansi16};
use tuit::terminal::ConstantSize;
use tuit::widgets::builtins::centered_prompt::CenteredPrompt;
use tuit::widgets::builtins::sweeper::Sweeper;

#[cfg(not(feature = "ansi_terminal"))]
fn main() {
    println!("You must apply the stdout_render feature to view this example. Use `cargo --features stdout_render`");
}

#[cfg(feature = "ansi_terminal")]
fn main() {
    let mut terminal: ConstantSize<58, 14> = ConstantSize::new();
    //
    // let text = CenteredText::new("Hello world!");
    //
    // text.drawn(&mut terminal)
    //     .expect("This method CAN fail, but only if the prompt is too large. Here, it is not.");

    let mut stdio = StdoutRenderer::default();

    let sweeper = Sweeper::of_colour(Ansi16(Ansi4::BrightCyan));

    let mut x = 0;

    loop {
        let test_buttons: Vec<String> = (0..=x).map(|x| format!(" Test {x:>02} ")).collect();
        let test_buttons: Vec<&str> = test_buttons[..].iter().map(String::as_str).collect();

        sweeper.drawn(&mut terminal).ok();

        let mut prompt = CenteredPrompt::new("Hello world!", &test_buttons).select(2);

        prompt.centered_text.style = prompt
            .centered_text
            .style
            .bg(Ansi16(Ansi4::Red))
            .fg(Ansi16(Ansi4::BrightWhite));
        prompt.selected_button_style = prompt.selected_button_style.inverted();

        if prompt.drawn(&mut terminal).is_err() {
            x -= 1;
            break;
        }

        x += 1;


        stdio.render(terminal).ok();
    }

    println!("\nWe managed to draw a prompt with up to {x} buttons!");
}
