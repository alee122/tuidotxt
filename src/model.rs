use todo_txt::task::Task;
use std::vec::Vec;

#[derive(Debug, Default)]
pub struct Model {
    pub  counter: i32,
    pub running_state: RunningState,
    _to_do : Vec<Task>,
    _today : Vec<Task>,
    _stuck : Vec<Task>
}

#[derive(Debug, Default, PartialEq, Eq)]
pub enum RunningState {
    #[default]
    Running,
    Done,
}
