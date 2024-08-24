use std::sync::Arc;

use rosu_v2::Osu;
use sqlx::SqlitePool;
use warp::{body::json, Filter, Rejection, Reply};

use crate::api::{
    estimate_cs2_rank, estimate_valorant_rank, feedback, get_ayt, get_osu_user, get_tyt,
};

pub fn routes(
    pool: SqlitePool,
    osu: Arc<Osu>,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    // GET routes
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

    let get_estimate_valorant_rank = warp::path!("api" / "estimate" / "valorant" / String)
        .and(warp::get())
        .and_then(estimate_valorant_rank);

    let get_estimate_cs2_rank = warp::path!("api" / "estimate" / "cs2" / usize)
        .and(warp::get())
        .and_then(estimate_cs2_rank);

    // POST routes
    let post_feedback = warp::path!("api" / "feedback")
        .and(warp::post())
        .and(json())
        .and(with_db(pool.clone()))
        .and_then(feedback);

    get_tyt
        .or(get_ayt)
        .or(get_osu_user)
        .or(get_estimate_valorant_rank)
        .or(get_estimate_cs2_rank)
        .or(post_feedback)
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
