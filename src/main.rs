use std::time::Duration;

use crossterm::event::{self, Event, KeyCode};
// cargo add anyhow crossterm ratatui
use ratatui::{prelude::*, widgets::*};
use anyhow::*;
use std::vec::Vec;
use todo_txt::task::Task;

#[derive(Debug, Default)]
struct Model {
    counter: i32,
    running_state: RunningState,
    _to_do : Vec<Task>,
    _today : Vec<Task>,
    _stuck : Vec<Task>
}

#[derive(Debug, Default, PartialEq, Eq)]
enum RunningState {
    #[default]
    Running,
    Done,
}

#[derive(PartialEq)]
enum Message {
    Increment,
    Decrement,
    Reset,
    Quit,
}

fn main() -> anyhow::Result<()> {
    tui::install_panic_hook();
    let mut terminal = tui::init_terminal()?;
    let mut model = Model::default();

    while model.running_state != RunningState::Done {
        // Render the current view
        terminal.draw(|f| view(&mut model, f))?;

        // Handle events and map to a Message
        let mut current_msg = handle_event(&model)?;

        // Process updates as long as they return a non-None message
        while current_msg != None {
            current_msg = update(&mut model, current_msg.unwrap());
        }
    }

    tui::restore_terminal()?;
    Ok(())
}

fn view(_model: &mut Model, f: &mut Frame) {
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
        Paragraph::new("To Do")
        .block(Block::new().borders(Borders::ALL)),
        layout[0]);
    f.render_widget(
        Paragraph::new("Today")
        .block(Block::new().borders(Borders::ALL)),
        layout[1]);
    f.render_widget(
        Paragraph::new("Stuck")
        .block(Block::new().borders(Borders::ALL)),
        layout[2]);
}

/// Convert Event to Message
///
/// We don't need to pass in a `model` to this function in this example
/// but you might need it as your project evolves
fn handle_event(_: &Model) -> anyhow::Result<Option<Message>> {
    if event::poll(Duration::from_millis(250))? {
        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Press {
                return Ok(handle_key(key));
            }
        }
    }
    Ok(None)
}

fn handle_key(key: event::KeyEvent) -> Option<Message> {
    match key.code {
        KeyCode::Char('j') => Some(Message::Increment),
        KeyCode::Char('k') => Some(Message::Decrement),
        KeyCode::Char('q') => Some(Message::Quit),
        _ => None,
    }
}

fn update(model: &mut Model, msg: Message) -> Option<Message> {
    match msg {
        Message::Increment => {
            model.counter += 1;
            if model.counter > 50 {
                return Some(Message::Reset);
            }
        }
        Message::Decrement => {
            model.counter -= 1;
            if model.counter < -50 {
                return Some(Message::Reset);
            }
        }
        Message::Reset => model.counter = 0,
        Message::Quit => {
            // You can handle cleanup and exit here
            model.running_state = RunningState::Done;
        }
    };
    None
}

mod tui {
    use crossterm::{
        terminal::{
            disable_raw_mode, enable_raw_mode, EnterAlternateScreen,
            LeaveAlternateScreen,
        },
        ExecutableCommand,
    };
    use ratatui::prelude::*;
    use std::{io::stdout, panic};

    pub fn init_terminal() -> anyhow::Result<Terminal<impl Backend>> {
        enable_raw_mode()?;
        stdout().execute(EnterAlternateScreen)?;
        let terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
        Ok(terminal)
    }

    pub fn restore_terminal() -> anyhow::Result<()> {
        stdout().execute(LeaveAlternateScreen)?;
        disable_raw_mode()?;
        Ok(())
    }

    pub fn install_panic_hook() {
        let original_hook = panic::take_hook();
        panic::set_hook(Box::new(move |panic_info| {
            stdout().execute(LeaveAlternateScreen).unwrap();
            disable_raw_mode().unwrap();
            original_hook(panic_info);
        }));
    }
}
