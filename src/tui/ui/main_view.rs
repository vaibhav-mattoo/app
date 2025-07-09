use crate::tui::app::App;
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph, Wrap},
};

pub fn render(f: &mut Frame, app: &App, area: ratatui::layout::Rect) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(70), Constraint::Percentage(30)])
        .split(area);

    let commands: Vec<ListItem> = app
        .filtered_commands
        .iter()
        .map(|cmd| {
            ListItem::new(Line::from(vec![
                Span::styled(
                    format!("Score: {} | ", cmd.score),
                    Style::default().fg(Color::Green),
                ),
                Span::raw(&cmd.command_text),
            ]))
        })
        .collect();

    let commands_list = List::new(commands)
        .block(Block::default().borders(Borders::ALL).title("Commands"))
        .highlight_style(Style::default().bg(Color::DarkGray))
        .highlight_symbol(">> ");

    f.render_stateful_widget(commands_list, chunks[0], &mut app.list_state.clone());

    let controls = Paragraph::new(
        "Controls:\n\n\
        a - Add alias\n\
        r - Remove alias\n\
        c - Change alias\n\
        s - Get suggestions\n\
        d - Delete suggestion\n\
        l - List aliases\n\
        q - Quit\n\n\
        ↑/↓ - Navigate\n\
        Esc - Cancel/Close",
    )
    .block(Block::default().borders(Borders::ALL).title("Controls"))
    .wrap(Wrap { trim: true });

    f.render_widget(controls, chunks[1]);
}
