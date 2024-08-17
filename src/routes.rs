use sqlx::SqlitePool;
use warp::{Filter, Rejection, Reply};

use crate::api::{get_ayt, get_tyt};

pub fn routes(pool: SqlitePool) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    let get_tyt = warp::path!("api" / "tyt")
        .and(warp::get())
        .and(with_db(pool.clone()))
        .and_then(get_tyt);

    let get_ayt = warp::path!("api" / "ayt" / String)
        .and(warp::get())
        .and(with_db(pool.clone()))
        .and_then(get_ayt);

    get_tyt.or(get_ayt)
}

fn with_db(
    pool: SqlitePool,
) -> impl Filter<Extract = (SqlitePool,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || pool.clone())
}
