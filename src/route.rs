use actix_web::HttpResponse;
use actix_web::{get, post, web, Responder};
use diesel::RunQueryDsl;
use uuid::Uuid;

use crate::db::DbPool;
use crate::models::{NewTask, Task};
use crate::schema::tasks;

#[get("/")]
async fn hello() -> impl Responder {
    "test"
}

#[get("/tasks")]
async fn list_tasks(pool: web::Data<DbPool>) -> HttpResponse {
    let mut conn = pool.get().unwrap();
    let a: Vec<Task> = tasks::table.load(&mut conn).unwrap();

    HttpResponse::Ok().json(a)
}

#[post("/tasks")]
async fn add_task(pool: web::Data<DbPool>, form: web::Json<NewTask>) -> HttpResponse {
    let mut conn = pool.get().unwrap();

    let id = Uuid::new_v4().to_string();
    let new_task = Task {
        id,
        title: form.title.clone(),
        done: false,
    };

    diesel::insert_into(tasks::table)
        .values(&new_task)
        .execute(&mut conn)
        .unwrap();

    HttpResponse::Ok().json(new_task)
}
