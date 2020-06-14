use std::fs::File;
use unicode_width::UnicodeWidthStr;
use termion::raw::IntoRawMode as _;
use termion::event::Key;
use termion::input::TermRead;
use tui::layout::Constraint;
use tui::widgets::{Row, Table};

use crate::tree::Tree;

pub fn render(tree: Tree) -> Result<(), Box<dyn std::error::Error>> {
    let (w, h) = tree.size();
    let widths = tree.into_iter()
        .map(|row| row.map(|cell| cell.map(|s| UnicodeWidthStr::width(s)).unwrap_or(0)))
        .fold(vec![0u16; w], |mut widths, row| {
            widths.iter_mut()
                .zip(row.chain(std::iter::repeat(0)).take(w))
                .map(|(a, b)| u16::max(*a, b as u16))
                .collect()
        })
        .into_iter()
        .map(Constraint::Length)
        .collect::<Vec<_>>();

    let mut keys = File::open("/dev/tty")?.keys();
    let stdout = std::io::stdout().into_raw_mode()?;
    let backend = tui::backend::TermionBackend::new(stdout);
    let mut terminal = tui::Terminal::new(backend)?;
    terminal.hide_cursor()?;

    let (mut x, mut y) = (0, 0);
    terminal.clear()?;
    'outer: loop {
        terminal.draw(|mut f| {
            let table = Table
                ::new(
                    std::iter::repeat("").skip(x).take(w),
                    tree.into_iter()
                        .skip(y)
                        .map(|row| row.skip(x).map(|data| data.unwrap_or("")))
                        .map(Row::Data),
                )
                .widths(&widths[x..]);
            f.render_widget(table, f.size());
        })?;
        loop {
            match keys.next().transpose()? {
                Some(Key::Left) | Some(Key::Char('h')) => {
                    x = x.saturating_sub(1);
                }
                Some(Key::Right) | Some(Key::Char('l')) => {
                    x = usize::min(x + 1, w.saturating_sub(1));
                }
                Some(Key::Up) | Some(Key::Char('k')) => {
                    y = y.saturating_sub(1);
                }
                Some(Key::Down) | Some(Key::Char('j')) => {
                    y = usize::min(y + 1, h.saturating_sub(1));
                }
                Some(Key::Char('q')) | None => break 'outer,
                _ => continue,
            }
            break;
        }
     }

    Ok(())
}
