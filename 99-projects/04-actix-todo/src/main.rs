mod api;
mod db;

use actix_web::{web::{self}, App, HttpResponse, HttpServer, Responder};
use db::repo::TodoRepo;
use api::handlers;

struct AppState {
    dbrepo: TodoRepo,
}

async fn health() -> impl Responder {
    HttpResponse::Ok().body("Looks Ok")
}

#[actix_web::main]
async fn main() -> std::io::Result<()>{
    let url = "postgres://postgres:password@localhost:5432/todo1";
    let pool = sqlx::postgres::PgPool::connect(url).await.expect("errror con");

    sqlx::migrate!("./migrations").run(&pool).await.unwrap();

    let repo = TodoRepo::new(pool);
    let app_state = web::Data::new(AppState{
        dbrepo: repo,
    });

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .service(
                // grouping under /api/v1
                web::scope("/api/v1")
                    .service(handlers::post_todo)
                    .service(handlers::list_todos)
            )
            // .service(echo)
            .route("/health", web::get().to(health))
    })
    .workers(4)
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
