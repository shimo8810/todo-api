use actix_web::{web, App, HttpServer};
use anyhow::Result;

use todo_api::db::establish_connection;
use todo_api::route::{add_task, delete_task, hello, list_tasks};

#[actix_web::main]
async fn main() -> Result<()> {
    //
    dotenvy::dotenv().ok();

    // db conn pool の準備
    let pool = establish_connection()?;

    HttpServer::new(move || {
        //
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(hello)
            .service(list_tasks)
            .service(add_task)
            .service(delete_task)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await?;

    Ok(())
}
