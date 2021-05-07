use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct TaskModel {
    id: i32,
    name: String,
    workload: f32,
    points: f32
}