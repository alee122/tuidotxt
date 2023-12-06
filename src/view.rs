use ratatui::{prelude::*, widgets::*};
use crate::model::*;
use todo_txt::task::Task;
use std::vec::Vec;

fn tasks_to_list(task_vec : &mut Vec<Task>) -> List {
    let mut temp = Vec::<ListItem>::new();

    for task in task_vec.iter() {
        temp.push(ListItem::new(Line::from(Span::raw(
                        format!("{}", task.subject)
                        ))));
    }
    return List::new(temp)
}

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
    
    f.render_widget(tasks_to_list(&mut model.to_do)
        .block(Block::default()
               .title("To Do")
               .title_alignment(Alignment::Center)
               .borders(Borders::ALL)),
        layout[0]);
    f.render_widget(tasks_to_list(&mut model.today)
        .block(Block::default()
               .title("Today")
               .title_alignment(Alignment::Center)
               .borders(Borders::ALL)),
        layout[1]);
    f.render_widget(tasks_to_list(&mut model.stuck)
        .block(Block::default()
               .title("Stuck")
               .title_alignment(Alignment::Center)
               .borders(Borders::ALL)),
        layout[2]);
}
