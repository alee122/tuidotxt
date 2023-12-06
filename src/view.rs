use ratatui::{prelude::*, widgets::*};

pub fn view(f: &mut Frame) {
    use ratatui::prelude::*;
    let layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![
                     Constraint::Percentage(33),
                     Constraint::Percentage(33),
                     Constraint::Percentage(33),
        ])
        .split(f.size());
    f.render_widget(
        Paragraph::new("")
        .block(Block::default()
               .title("To Do")
               .title_alignment(Alignment::Center)
               .borders(Borders::ALL)),
        layout[0]);
    f.render_widget(
        Paragraph::new("")
        .block(Block::default()
               .title("Today")
               .title_alignment(Alignment::Center)
               .borders(Borders::ALL)),
        layout[1]);
    f.render_widget(
        Paragraph::new("")
        .block(Block::default()
               .title("Stuck")
               .title_alignment(Alignment::Center)
               .borders(Borders::ALL)),
        layout[2]);
}
