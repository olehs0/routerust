use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use dotenv;

pub type PgPooledConnection = PooledConnection<ConnectionManager<PgConnection>>;

pub fn pg_pool_handler() -> PgPooledConnection {
    let manager = ConnectionManager::<PgConnection>::new(dotenv::var("DATABASE_URL").unwrap());
    Pool::builder()
        .build(manager)
        .unwrap()
        .get()
        .expect("Failed")
}
