use crate::DBPOOL;
use crate::components::postgres::func::get_db_con;
use crate::helper::error::Error::*;
use chrono::{DateTime, Utc};
use mobc::Pool;
use mobc_postgres::tokio_postgres::Row;
use mobc_postgres::{tokio_postgres, PgConnectionManager};
use tokio_postgres::{Config, Error, NoTls};
use std::str::FromStr;
use std::time::Duration;
use crate::{DBPool, helper::error};
use super::data::{Todo, TodoRequest};

const TABLE: &str = "todo";
const SELECT_FIELDS: &str = "id, name, created_at, checked";

//create
pub async fn create_todo(body: TodoRequest) -> Result<Todo, error::Error> {
    let con = get_db_con(&DBPOOL).await?;
    let query = format!("INSERT INTO {} (id, name) VALUES ($1, $2) RETURNING *", TABLE);
    let row = con
            .query_one(query.as_str(), &[&body.id, &body.name])
            .await
            .map_err(DBQueryError)?;
    Ok(row_to_todo(&row))
}

fn row_to_todo(row: &Row) -> Todo {
    let id: String = row.get(0);
    let name: String = row.get(1);
    let created_at: DateTime<Utc> = row.get(2);
    let checked: bool = row.get(3);
    Todo {
            id,
            name,
            created_at,
            checked,
    }
}