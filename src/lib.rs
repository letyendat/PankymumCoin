pub mod components;
pub mod models;
pub mod service;
pub mod helper;

use std::{convert::Infallible, sync::Arc};
use warp::{Filter, hyper::StatusCode};
use mobc::{Connection, Pool};
use mobc_postgres::{tokio_postgres, PgConnectionManager};
use tokio_postgres::NoTls;

pub type DBCon = Connection<PgConnectionManager<NoTls>>;
pub type DBPool = Pool<PgConnectionManager<NoTls>>;
type Result<T> = std::result::Result<T, warp::Rejection>;



#[macro_use]
extern crate lazy_static;


lazy_static! {
    pub static ref DBPOOL: DBPool = {
        let db_pool = crate::components::postgres::func::create_pool().expect("database pool can be created");
        db_pool
    };
}
