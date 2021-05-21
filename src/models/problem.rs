use serde::{Deserialize, Serialize};
use crate::models::task::TaskModel;

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct ProblemModel {
    pub tasks_list: Vec<TaskModel>,
    pub time_available: f32,
    pub accept_incomplete_tasks: bool
}