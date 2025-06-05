use crate::tui::app::App;
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{Block, Borders, List, ListItem, Paragraph},
};

pub fn render(f: &mut Frame, app: &App, area: ratatui::layout::Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(0)])
        .split(area);

    let input = Paragraph::new(app.input.as_str())
        .style(Style::default().fg(Color::Yellow))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(format!("{:?}", app.mode)),
        );
    f.render_widget(input, chunks[0]);

    f.set_cursor_position((
        chunks[0].x + app.cursor_position as u16 + 1,
        chunks[0].y + 1,
    ));

    if !app.filtered_commands.is_empty() {
        let commands: Vec<ListItem> = app
            .filtered_commands
            .iter()
            .map(|cmd| ListItem::new(cmd.command_text.as_str()))
            .collect();

        let commands_list = List::new(commands).block(
            Block::default()
                .borders(Borders::ALL)
                .title("Matching Commands"),
        );

        f.render_widget(commands_list, chunks[1]);
    }
}
