//! A bad benchmark.

use std::time::Instant;

use tuit::allocations::terminal::constant_boxed::ConstantBoxed;
use tuit::terminal::{Rectangle, TerminalMut};
use tuit::widgets::builtins::{ Text, Uv};
use tuit::widgets::Widget;

const TARGET_FPS: u16 = 60;

fn main() {
    let mut terminal: ConstantBoxed<1000, 1000> = ConstantBoxed::new();

    let uv = Uv;
    let text = Text::new("Hello! I am a text!");
    let centered_prompt = Text::new("Test!");

    for _ in 0..100 {
        let start = Instant::now();

        for _ in 0..TARGET_FPS {
            // centered_prompt.drawn(&mut terminal).ok();
            uv.drawn(&mut terminal).ok();
            text.drawn(&mut terminal).ok();
            centered_prompt.drawn(&mut terminal).ok();

            let mut view = terminal.view_mut(Rectangle::of_size((500, 500)).at((200, 200))).expect("Always valid");

            uv.drawn(&mut view).ok();
            text.drawn(&mut view).ok();
        }


        let duration = start.elapsed();

        println!("That took a (hopefully?) speedy (please?) {} seconds to draw {TARGET_FPS} times!", duration.as_secs_f32());
        println!("That is {} FPS.", 1.0/duration.as_secs_f32() * f32::from(TARGET_FPS));
    }
}