use serde::{Deserialize, Serialize};
use crate::models::task::TaskModel;

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct ProblemModel {
    tasks_list: Vec<TaskModel>,
    time_available: f32,
    accept_incomplete_tasks: bool
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct ProblemSolvedModel {
    //todo
}