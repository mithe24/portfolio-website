use actix_web::{App, HttpResponse, HttpServer, Responder, get, web};
use serde::Deserialize;

use crate::db::repository::PostRepository;
mod config;
mod routes;
mod db;
mod errors;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let cfg = config::Config::from_env();

    let pool = db::create_pool(&cfg.database_url()).await.unwrap();

    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Migration failed");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            /* .configure(routes::init) */
        })
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}

#[derive(Deserialize)]
struct PaginationQuery {
    page: u64,
}

#[get("/posts")]
async fn get_posts(
    query: web::Query<PaginationQuery>,
    db: web::Data<PostRepository>,
) -> impl Responder {
    println!("End point was hit");
    let page_size = 10;
    let offset = (query.page.saturating_sub(1)) * page_size;

    match db.get_posts(page_size, offset).await {
        Ok(posts) => HttpResponse::Ok().json(posts),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}
