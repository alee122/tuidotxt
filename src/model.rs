use todo_txt::task::Task;
use std::vec::Vec;

#[derive(Debug, Default)]
pub struct Model {
    pub  counter: i32,
    pub running_state: RunningState,
    pub to_do : Vec<Task>,
    pub today : Vec<Task>,
    pub stuck : Vec<Task>
}

#[derive(Debug, Default, PartialEq, Eq)]
pub enum RunningState {
    #[default]
    Running,
    Done,
}
