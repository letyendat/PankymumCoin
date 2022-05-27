extern crate PankymumCoin;

use std::convert::Infallible;

use warp::{Filter, hyper::StatusCode};
use mobc::{Connection, Pool};
use mobc_postgres::{tokio_postgres, PgConnectionManager};
use tokio_postgres::NoTls;
use PankymumCoin::components::postgres::func;
use PankymumCoin::service;
use PankymumCoin::helper::error;
use PankymumCoin::DBPool;
use PankymumCoin::DBPOOL;

#[tokio::main]
async fn main() {
    func::init_db(&DBPOOL)
        .await
        .expect("database can be initialized");

    let todo = warp::path("todo");
    let todo_routes = todo
        .and(warp::post())
        .and(warp::body::json())
        // .and(with_db(db_pool.clone()))
        .and_then(service::todo::create_todo_service);
    // .or(todo
    //         .and(warp::post())
    //         .and(warp::body::json())
    //         .and(with_db(db_pool.clone()))
    //         .and_then(handler::create_todo_handler))
    // .or(todo
    //         .and(warp::put())
    //         .and(warp::path::param())
    //         .and(warp::body::json())
    //         .and(with_db(db_pool.clone()))
    //         .and_then(handler::update_todo_handler))
    // .or(todo
    //         .and(warp::delete())
    //         .and(warp::path::param())
    //         .and(with_db(db_pool.clone()))
    //         .and_then(handler::delete_todo_handler));


    let routes = todo_routes
        // .or(todo_routes)
        .with(warp::cors().allow_any_origin())
        .recover(error::handle_rejection);

    warp::serve(routes).run(([127, 0, 0, 1], 8000)).await;
}

// fn with_db(db_pool: DBPool) -> impl Filter<Extract = (DBPool,), Error = Infallible> + Clone {
//     warp::any().map(move || db_pool.clone())
// } Ko hiu