
use crate::models::{problem::{ProblemModel}, task::TaskModel};
use good_lp::{Constraint, Expression, Solution, SolverModel, Variable, constraint, default_solver, variables, variable::variable};
use rocket_contrib::json::Json;

#[post("/solve", format = "json", data = "<problem_md>")]
pub fn solve(problem_md: Json<ProblemModel>) -> Json<Vec<TaskModel>> {
    let mut variables = variables!();

    let devo_fazer_tx = if problem_md.accept_incomplete_tasks {
        variables.add_vector(variable().min(0).max(1), problem_md.tasks_list.len())
    } else {
        variables.add_vector(variable().binary(), problem_md.tasks_list.len())
    };

    let problem = variables.maximise(objective_max_pts(&devo_fazer_tx, &problem_md.tasks_list))
        .using(default_solver)
        .with(constraint_time_available(&devo_fazer_tx, &problem_md.tasks_list, problem_md.time_available));

    let solution = problem.solve().unwrap();

    let solved = devo_fazer_tx.iter()
            .enumerate()
            .map(|(i, v)| {
                let mut t = problem_md.tasks_list[i].clone();
                t.workload_percent = Some(solution.value(*v));
                t
            })
            .collect();

    Json(solved)
}

fn constraint_time_available(devo_fazer_tx: &Vec<Variable>, tasks: &Vec<TaskModel>, time_available: f32) -> Constraint {
    let mut expression: Expression = 0.into();
    for (i, task) in tasks.iter().enumerate() {
        expression += devo_fazer_tx[i] * task.workload;
    }
    constraint!(expression <= time_available)
}

fn objective_max_pts(devo_fazer_tx: &Vec<Variable>, tasks: &Vec<TaskModel>) -> Expression {
    let mut objective: Expression = 0.into();
    for (i, task) in tasks.iter().enumerate() {
        objective += devo_fazer_tx[i] * task.points;
    }
    objective
}