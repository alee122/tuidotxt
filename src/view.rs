use ratatui::{prelude::*, widgets::*};
use crate::model::*;
use todo_txt::task::Task;
use std::vec::Vec;

fn tasks_to_tui_list(task_vec : &mut Vec<Task>, list_vec : &mut Vec<ListItem>) {
    for task in task_vec.iter() {
        list_vec.push(ListItem::new(Line::from(Span::raw(
                        format!("{}", task.subject)
                        ))));
    }
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

    let mut list1 = Vec::<ListItem>::new();
    let mut list2 = Vec::<ListItem>::new();
    let mut list3 = Vec::<ListItem>::new();
    tasks_to_tui_list(&mut model.to_do, &mut list1);
    tasks_to_tui_list(&mut model.stuck, &mut list2);
    tasks_to_tui_list(&mut model.today, &mut list3);
    let todo_list = List::new(list1);
    let stuck_list = List::new(list2);
    let today_list = List::new(list3);
    
    f.render_widget(todo_list
        .block(Block::default()
               .title("To Do")
               .title_alignment(Alignment::Center)
               .borders(Borders::ALL)),
        layout[0]);
    f.render_widget(today_list
        .block(Block::default()
               .title("Today")
               .title_alignment(Alignment::Center)
               .borders(Borders::ALL)),
        layout[1]);
    f.render_widget(stuck_list
        .block(Block::default()
               .title("Stuck")
               .title_alignment(Alignment::Center)
               .borders(Borders::ALL)),
        layout[2]);
}
