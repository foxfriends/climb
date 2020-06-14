use std::fs::File;
use unicode_width::UnicodeWidthStr;
use termion::raw::IntoRawMode as _;
use termion::event::Key;
use termion::input::TermRead;
use tui::layout::{Layout, Constraint, Direction};
use tui::widgets::{List, Text, Block, Borders, BorderType};

use crate::tree::Tree;

pub fn render(tree: Tree) -> Result<(), Box<dyn std::error::Error>> {
    const BORDER_WIDTH: u16 = 1;
    let (w, h) = tree.size();
    let widths = tree.into_iter()
        .map(|row| row.map(|cell| cell.map(|s| UnicodeWidthStr::width(s)).unwrap_or(0)))
        .fold(vec![0u16; w], |mut widths, row| {
            widths.iter_mut()
                .zip(row.chain(std::iter::repeat(0)).take(w))
                .map(|(a, b)| u16::max(*a, b as u16 + BORDER_WIDTH))
                .collect()
        })
        .into_iter()
        .collect::<Vec<_>>();

    let mut keys = File::open("/dev/tty")?.keys();
    let stdout = std::io::stdout().into_raw_mode()?;
    let backend = tui::backend::TermionBackend::new(stdout);
    let mut terminal = tui::Terminal::new(backend)?;
    terminal.hide_cursor()?;

    let (mut vx, mut vy) = (0, 0);
    terminal.clear()?;
    'outer: loop {
        terminal.draw(|mut f| {
            let size = f.size();
            let cols_to_show = widths
                .iter()
                .skip(vx)
                .take_while({
                    let mut max_width = size.width;
                    move |&&width| {
                        let ok = max_width > width;
                        max_width = max_width.saturating_sub(width);
                        ok
                    }
                })
                .count();

            let layout = Layout::default()
                .constraints(widths
                    .iter()
                    .skip(vx)
                    .take(cols_to_show)
                    .copied()
                    .map(Constraint::Length)
                    .collect::<Vec<_>>())
                .direction(Direction::Horizontal)
                .horizontal_margin(1)
                .split(size);

            let rows = tree.into_iter()
                .skip(vy)
                .take(size.height as usize)
                .map(|row| row
                    .skip(vx)
                    .take(cols_to_show)
                    .map(|cell| cell.unwrap_or(""))
                    .map(Text::raw));

            let mut cols = vec![vec![]; cols_to_show];
            for row in rows {
                for (i, text) in row.chain(std::iter::repeat(Text::raw(""))).take(cols_to_show).enumerate() {
                    cols[i].push(text);
                }
            }

            let lists = cols
                .into_iter()
                .map(|col| List::new(col.into_iter())
                    .block(Block::default()
                        .borders(Borders::RIGHT)
                        .border_type(BorderType::Plain)));

            for (i, list) in lists.enumerate() {
                f.render_widget(list, layout[i]);
            }
        })?;
        loop {
            match keys.next().transpose()? {
                Some(Key::Left) | Some(Key::Char('h')) => {
                    vx = vx.saturating_sub(1);
                }
                Some(Key::Char('H')) => {
                    vx = vx.saturating_sub(10);
                }
                Some(Key::Right) | Some(Key::Char('l')) => {
                    vx = usize::min(vx + 1, w.saturating_sub(1));
                }
                Some(Key::Char('L')) => {
                    vx = usize::min(vx + 10, w.saturating_sub(1));
                }
                Some(Key::Up) | Some(Key::Char('k')) => {
                    vy = vy.saturating_sub(1);
                }
                Some(Key::Char('K')) => {
                    vy = vy.saturating_sub(10);
                }
                Some(Key::Down) | Some(Key::Char('j')) => {
                    vy = usize::min(vy + 1, h.saturating_sub(1));
                }
                Some(Key::Char('J')) => {
                    vy = usize::min(vy + 10, h.saturating_sub(1));
                }
                Some(Key::Char('q')) | None => break 'outer,
                _ => continue,
            }
            break;
        }
     }

    Ok(())
}
