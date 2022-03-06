pub use crossterm::{
    event::{poll, read, Event, KeyCode, KeyEvent},
    terminal::size,
};
pub use masof::{Color, ContentStyle, Renderer};
use std::io::Stdout;

use super::draw::*;
use super::*;

pub fn render_progress(
    renderer: &mut Renderer,
    style: ContentStyle,
    stdout: &mut Stdout,
    title: &str,
    progress: f64,
) -> Result<(), Error> {
    let progress_size = (title.len() as i32 + 2, 4);
    let pos = box_center_screen(progress_size)?;

    renderer.begin()?;

    {
        let mut context = DrawContext { renderer, style };

        context.draw_box(pos, progress_size);
        if pos.1 + 1 >= 0 {
            context.draw_str(pos.0 + 1, pos.1 + 1, title);
        }
        if pos.1 + 2 >= 0 {
            context.draw_str(
                pos.0 + 1,
                pos.1 + 2,
                &"#".repeat((title.len() as f64 * progress) as usize),
            );
        }
    }

    renderer.end(stdout)?;

    Ok(())
}
