use crate::components::postgres::todo::data::{Todo, TodoRequest};
use crate::components::postgres::todo::todo::{
    create_todo,
};
use uuid::Uuid;
use crate::{DBPool};
use crate::helper::error;

pub async fn create_todo_business(name: String) -> Result<Todo, error::Error> {
    let id  = Uuid::new_v4().to_string();
    let request_to_db =  TodoRequest {
        id: Some(id),
        name: Some(name),
        ..Default::default()
    };

    let todo = create_todo(request_to_db).await;

    todo 
}