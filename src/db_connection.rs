use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use dotenv;
use std::error::Error;

pub type PgPooledConnection = PooledConnection<ConnectionManager<PgConnection>>;

pub fn pg_pool_handler() -> Result<PgPooledConnection, Box<dyn Error>> {
    let manager = ConnectionManager::<PgConnection>::new(dotenv::var("DATABASE_URL").unwrap());
    Pool::builder().build(manager)?.get().map_err(|e| e.into())
}
