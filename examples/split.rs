//! Show how split views can be used.

use std::io::stdout;
use tuit::style::{Ansi4, Colour};
use tuit::terminal::{ConstantSize, ViewSplit};
use tuit::widgets::builtins::{Margin, Sweeper, Text};
use tuit::prelude::*;
use tuit::widgets::BoundingBox;

fn main() {
    let mut terminal: ConstantSize<50,20> = ConstantSize::new();

    let yellow = Sweeper::of_colour(Colour::Ansi16(Ansi4::Yellow));
    let magenta = Sweeper::of_colour(Colour::Ansi16(Ansi4::Magenta));
    let blue = Margin::new(Sweeper::of_colour(Colour::Ansi16(Ansi4::Blue))).margin(2);

    let long_left_text = Margin::new(Text::new(
        "Here's some really long text that will probably, or at least I hope, wrap around when drawn on the left side of the terminal! It even has some extra padding to add space! Wow, isn't that cool!?"
    )).margin(2);
    let short_right_text = Margin::new(Text::new("The guy next to me is too loud...")).margin(2);
    
    println!("yellow full box: {:?}", yellow.bounding_box(&terminal));
    println!("right text: {:?}", short_right_text.bounding_box(&terminal));
    println!("right text area: {}", short_right_text.bounding_box(&terminal).area());
    
    let mut split = ViewSplit::new(&mut terminal);

    let mut right = split.split_right_mut();
    println!("yellow with margin: {:?}", short_right_text.bounding_box(&right));
    println!("yellow inner: {:?}", short_right_text.inner().bounding_box(&right));
    yellow.drawn(&mut right).ok();
    blue.drawn(&mut right).ok();
    short_right_text.drawn(&mut right).ok();

    let mut left = split.split_left_mut();
    println!("magenta with margin: {:?}", long_left_text.bounding_box(&left));
    println!("magenta inner: {:?}", long_left_text.inner().bounding_box(&left));
    magenta.drawn(&mut left).ok();
    blue.drawn(&mut left).ok();
    long_left_text.drawn(&mut left).ok();
    
    terminal.display(stdout()).expect("Can't fail");
}