use rhb::{
    config::user,
    tui::{event, help},
};

use std::io;

use tui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans, Text},
    widgets::{Block, Borders, List, ListItem},
    Terminal,
};

fn main() {
    test_main().unwrap();
}
fn test_main() -> Result<(), io::Error> {
    let help = help::Help::default();

    let n1 = vec![
        Span::raw("Press "),
        Span::styled("q", Style::default().add_modifier(Modifier::BOLD)),
        Span::raw(" to exit, "),
        Span::styled("e", Style::default().add_modifier(Modifier::BOLD)),
        Span::raw(" to start editing."),
    ];

    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    terminal.clear()?;
    terminal.hide_cursor()?;

    terminal.draw(|f| {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints([Constraint::Percentage(10), Constraint::Percentage(80)].as_ref())
            .split(f.size());

        let block = Block::default().title("Block 1").borders(Borders::ALL);
        f.render_widget(block, chunks[0]);

        let message: Vec<String> = vec!["aw".to_string(), "awd".to_string()];
        let messages: Vec<ListItem> = message
            .iter()
            .enumerate()
            .map(|(i, m)| {
                let content = vec![Spans::from(Span::raw(format!("{}: {}", i, m)))];
                ListItem::new(content)
            })
            .collect();
        let me =
            List::new(messages).block(Block::default().borders(Borders::ALL).title("Messages"));
        f.render_widget(me, chunks[1]);
    })?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_profile() {
        let p = user::Profile::new("text", "text", &1);
        println!("{}", format!("{:#?}", p));
    }
}
