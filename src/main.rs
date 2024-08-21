use std::{env, sync::Arc};

use dotenvy::dotenv;

use rosu_v2::prelude::*;

mod api;
mod database;
mod models;
mod routes;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let osu_client_id: u64 = env::var("OSU_CLIENT_ID")
        .expect("Expected OSU_CLIENT_ID to be defined in environment.")
        .parse()
        .expect("OSU_CLIENT_ID is not a number!");

    let osu_client_secret = env::var("OSU_CLIENT_SECRET")
        .expect("Expected OSU_CLIENT_SECRET to be defined in environment.");

    let osu = Arc::new(Osu::new(osu_client_id, osu_client_secret).await.unwrap());
    let pool = database::create_pool().await;

    let api = routes::routes(pool, osu);

    println!("Listening on http://localhost:3031");
    warp::serve(api).run(([127, 0, 0, 1], 3031)).await;
}
