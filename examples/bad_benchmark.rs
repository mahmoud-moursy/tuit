//! A bad benchmark.

use std::time::Instant;
use tuit::widgets::builtins::{Text, Uv};

const TARGET_FPS: u16 = 60;

fn main() {
    let uv = Uv;
    let text = Text::new("Hello! I am a text!");

    loop {
        let start = Instant::now();

        todo!("Add the benchmark");

        let duration = start.elapsed();

        println!("That took a (hopefully?) speedy (please?) {} seconds to draw 60 times!", duration.as_secs_f32());
        println!("That is {} FPS.", 1.0/duration.as_secs_f32() * f32::from(TARGET_FPS));
    }
}