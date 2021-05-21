use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct TaskModel {
    pub id: i32,
    pub name: String,
    pub points: f32,
    pub workload: f32,
    pub workload_percent: Option<f64>
}