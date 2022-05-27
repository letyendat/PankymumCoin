use serde_derive::{Deserialize, Serialize};
use chrono::prelude::*;

#[derive(Deserialize)]
pub struct Todo {
    pub id: String,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub checked: bool,
}

#[derive(Deserialize, Default)]
pub struct TodoRequest {
    pub id: Option<String>,
    pub name: Option<String>,
    pub created_at: Option<i32>,
    pub checked: Option<bool>,
}