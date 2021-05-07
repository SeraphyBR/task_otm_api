
use crate::models::problem::{ProblemModel, ProblemSolvedModel};
use rocket_contrib::json::Json;

#[post("/", format = "json", data = "<problem>")]
pub fn solve(problem: Json<ProblemModel>) -> Json<ProblemSolvedModel> {
    //
    todo!()
}