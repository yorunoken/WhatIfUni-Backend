use dotenvy::dotenv;

mod api;
mod database;
mod models;
mod routes;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let pool = database::create_pool().await;

    let api = routes::routes(pool);

    println!("Listening on http://localhost:3030");
    warp::serve(api).run(([127, 0, 0, 1], 3030)).await;
}
