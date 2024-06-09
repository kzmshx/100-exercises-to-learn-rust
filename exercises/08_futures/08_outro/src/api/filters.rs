use crate::api::handlers;
use crate::db::Db;
use std::convert::Infallible;
use warp::Filter;

pub fn all(db: Db) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    hello().or(tickets(db))
}

pub fn hello() -> impl Filter<Extract = (&'static str,), Error = warp::Rejection> + Clone {
    warp::path!("hello")
        .and(warp::get())
        .map(|| "Hello, world!")
}

pub fn tickets(db: Db) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    tickets_get(db.clone())
        .or(tickets_post(db.clone()))
        .or(tickets_id_get(db.clone()))
        .or(tickets_id_put(db.clone()))
}

/// GET /tickets
pub fn tickets_get(
    db: Db,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("tickets")
        .and(warp::get())
        .and(with_db(db))
        .and_then(handlers::tickets::list)
}

/// POST /tickets
pub fn tickets_post(
    db: Db,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("tickets")
        .and(warp::post())
        .and(warp::body::json())
        .and(with_db(db))
        .and_then(handlers::tickets::create)
}

/// GET /tickets/:id
pub fn tickets_id_get(
    db: Db,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("tickets" / u64)
        .and(warp::get())
        .and(with_db(db))
        .and_then(handlers::tickets::get)
}

/// PUT /tickets/:id
pub fn tickets_id_put(
    db: Db,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("tickets" / u64)
        .and(warp::put())
        .and(warp::body::json())
        .and(with_db(db))
        .and_then(handlers::tickets::update)
}

fn with_db(db: Db) -> impl Filter<Extract = (Db,), Error = Infallible> + Clone {
    warp::any().map(move || db.clone())
}
