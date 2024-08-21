use std::sync::Arc;

use rosu_v2::Osu;
use sqlx::SqlitePool;
use warp::{Filter, Rejection, Reply};

use crate::api::{get_ayt, get_osu_user, get_tyt};

pub fn routes(
    pool: SqlitePool,
    osu: Arc<Osu>,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    let get_tyt = warp::path!("api" / "tyt")
        .and(warp::get())
        .and(warp::query::<std::collections::HashMap<String, String>>())
        .and(with_db(pool.clone()))
        .and_then(get_tyt);

    let get_ayt = warp::path!("api" / "ayt" / String)
        .and(warp::get())
        .and(warp::query::<std::collections::HashMap<String, String>>())
        .and(with_db(pool.clone()))
        .and_then(get_ayt);

    let get_osu_user = warp::path!("api" / "osu" / "user" / String)
        .and(warp::get())
        .and(with_osu(osu.clone()))
        .and_then(get_osu_user);

    get_tyt.or(get_ayt).or(get_osu_user)
}

fn with_db(
    pool: SqlitePool,
) -> impl Filter<Extract = (SqlitePool,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || pool.clone())
}

fn with_osu(
    osu: Arc<Osu>,
) -> impl Filter<Extract = (Arc<Osu>,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || osu.clone())
}
