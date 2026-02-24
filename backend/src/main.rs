use actix_web::{App, HttpResponse, HttpServer, web};
mod config;
mod routes;
mod db;
mod errors;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let cfg = config::Config::from_env();

    let pool = db::create_pool(&cfg.database_url()).await;

    HttpServer::new(move || {
        App::new()
            /*
            .app_data(web::Data::new(pool.clone()))
            .configure(routes::init)
            */
    })
        .bind(("0.0.0.0", 8080))?
        .run()
    .await
}

/*
async fn get_posts(
    query: web::Query<PostsQuery>,
    pool: web::Data<sqlx::PgPool>,
) -> HttpResponse {
    let page = query.page.unwrap_or(1);
    let project = query.project;

    match db::fetch_posts(&pool, page, project).await {
        Ok(posts) => HttpResponse::Ok().json(posts),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
*/
#[derive(serde::Deserialize)]
struct PostsQuery {
    page: Option<i64>,
    project: Option<i64>,
}
