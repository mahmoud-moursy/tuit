use crate::prelude::{Terminal, TerminalConst};
use crate::style::Colour;
use crate::terminal::{UpdateInfo, UpdateResult};
use crate::widgets::Widget;

/// A UV Map for your terminal. Changes only the background colour. Useful for debugging!
///
/// ... I should know...
///
/// Very...
///
/// Useful...
///
/// 😭
pub struct Uv;

impl Widget for Uv {
    fn update(&mut self, _update_info: UpdateInfo, _terminal: impl TerminalConst) ->
                                                                                  crate::Result<UpdateResult> {
        Ok(UpdateResult::NoEvent)
    }


    fn draw(&self, _update_info: UpdateInfo, mut terminal: impl Terminal) ->
                                                                      crate::Result<UpdateResult> {
        let width = terminal.width();
        let height = terminal.height();


        #[allow(clippy::cast_precision_loss)]
        #[allow(clippy::cast_sign_loss)]
        #[allow(clippy::cast_possible_truncation)]
        for (idx, cell) in terminal.cells_mut().enumerate() {
            let x = idx % width;
            let y = idx / width;

            let r = (x as f32 / width as f32) * 255.0;
            let b = (y as f32 / height as f32) * 255.0;

            cell.style.bg_colour = Some(Colour::Rgb24(r as u8, 0, b as u8));
        }

        Ok(UpdateResult::NoEvent)
    }
}