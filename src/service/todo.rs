use crate::{helper::error::Error::*, DBPool, components::postgres::todo::todo};
use warp::{http::StatusCode, reject, Reply, Rejection, reply::json};
use crate::models::data::{TodoCreateRequest, TodoResponse};
type Result<T> = std::result::Result<T, warp::Rejection>;

pub async fn create_todo_service(body: TodoCreateRequest) -> Result<impl Reply> {
    let todo = crate::components::business::todo::create_todo_business(body.name).await;
    if todo.is_err() == true {
        return Err(reject::custom(todo.err().unwrap()));
    }
    let todo = todo.unwrap();

    let resp = TodoResponse::of(todo);

    Ok(json(&resp))
}