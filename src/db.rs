use anyhow::Result;
use diesel::{
    r2d2::{self, ConnectionManager},
    PgConnection,
};

pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub fn establish_connection() -> Result<DbPool> {
    dotenvy::dotenv().ok();

    let conn_spec = std::env::var("DATABASE_URL")?;
    let manager = ConnectionManager::<PgConnection>::new(conn_spec);
    Ok(r2d2::Pool::builder().build(manager)?)
}
