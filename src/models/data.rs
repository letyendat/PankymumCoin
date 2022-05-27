use serde_derive::{Deserialize, Serialize};
use crate::components::postgres::todo::data::Todo;

#[derive(Deserialize)]
pub struct TodoCreateRequest {
    pub name: String,
}

#[derive(Serialize)]
pub struct TodoResponse {
    pub id: String,
    pub name: String,
    pub checked: bool,
}

impl TodoResponse {
    pub fn of(todo: Todo) -> TodoResponse {
            TodoResponse {
                id: todo.id,
                name: todo.name,
                checked: todo.checked,
            }
    }
}