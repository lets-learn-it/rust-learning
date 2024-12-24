use actix_web::{get, post, web::{self, Json}, HttpResponse, Responder};

use crate::AppState;
use crate::api::models;

#[get("/todo")]
async fn list_todos(state: web::Data<AppState>) -> impl Responder {
    let todos = state.dbrepo.list().await.unwrap();
    HttpResponse::Ok().json(todos)
}

#[post("/todo")]
async fn post_todo(todo_json: Json<models::CreateTodoRequest>, state: web::Data<AppState>) -> impl Responder {
    let todo_req = todo_json.into_inner();
    let todo = todo_req.into();
    let created_todo = state.dbrepo.add(&todo).await.unwrap();
    let reponse: models::CreateTodoResponse = created_todo.into();
    HttpResponse::Ok().json(reponse)
}
