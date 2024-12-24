mod api;
use std::sync::Mutex;

use actix_web::{get, post, web::{self, Json}, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
struct TodoDb {
    todos: Mutex<Vec<api::models::Todo>>
}

#[get("/todo")]
async fn list_todos(db: web::Data<TodoDb>) -> impl Responder {
    let todos = db.todos.lock().unwrap();
    HttpResponse::Ok().json(&*todos)
}

#[post("/todo")]
async fn post_todo(todo_json: Json<api::models::Todo>, db: web::Data<TodoDb>) -> impl Responder {
    let mut todos = db.todos.lock().unwrap();
    let mut todo = todo_json.into_inner();
    todo.updated();
    todos.push(todo);
    HttpResponse::Ok().json(todos.last())
}

#[actix_web::main]
async fn main() -> std::io::Result<()>{
    let db = web::Data::new(TodoDb{
        todos: Mutex::new(vec![]),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(db.clone())
            .service(
                // grouping under /api/v1
                web::scope("/api/v1")
                    .service(post_todo)
                    .service(list_todos)
            )
            // .service(echo)
            // .route("/hello", web::get().to(manual_hello))
    })
    .workers(4)
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
