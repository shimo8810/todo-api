use actix_web::HttpResponse;
use actix_web::{delete, get, post, web, Error, Responder};
use diesel::{QueryDsl, RunQueryDsl};
use uuid::Uuid;

use crate::db::DbPool;
use crate::models::{NewTask, Task};
use crate::schema::tasks;

#[get("/")]
async fn hello() -> impl Responder {
    "test"
}

#[get("/tasks")]
async fn list_tasks(pool: web::Data<DbPool>) -> Result<HttpResponse, Error> {
    let task_list: Vec<Task> = web::block(move || {
        let mut conn = pool.get().unwrap();
        tasks::table.load(&mut conn)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(task_list))
}

#[post("/tasks")]
async fn add_task(
    pool: web::Data<DbPool>,
    form: web::Json<NewTask>,
) -> Result<HttpResponse, Error> {
    let id = Uuid::new_v4().to_string();

    let task = web::block(move || {
        let new_task = Task {
            id,
            title: form.title.clone(),
            done: false,
        };
        let mut conn = pool.get().unwrap();
        diesel::insert_into(tasks::table)
            .values(&new_task)
            .execute(&mut conn)
            .unwrap();

        new_task
    })
    .await?;

    Ok(HttpResponse::Ok().json(task))
}

#[delete("/tasks/{task_id}")]
pub async fn delete_task(
    pool: web::Data<DbPool>,
    id: web::Path<String>,
) -> Result<HttpResponse, Error> {
    let id = id.into_inner();

    web::block(move || {
        let mut conn = pool.get().unwrap();
        diesel::delete(tasks::table.find(id))
            .execute(&mut conn)
            .unwrap();
    })
    .await
    .unwrap();

    Ok(HttpResponse::Ok().body("delete"))
}
