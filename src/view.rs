use ratatui::{prelude::*, widgets::*};
//use ratatui::text::{Line, Span};
use crate::model::*;

pub fn view(model:&mut Model, f: &mut Frame) {
    use ratatui::prelude::*;
    let layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![
                     Constraint::Percentage(33),
                     Constraint::Percentage(33),
                     Constraint::Percentage(33),
        ])
        .split(f.size());

    let mut todo_list = Vec::<ListItem>::new();
    for task in model.to_do.iter() {
        todo_list.push(ListItem::new(Line::from(Span::raw(
                        format!("{}", task.subject)
                        ))));
    }
    let list = List::new(todo_list);
    f.render_widget(list
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
