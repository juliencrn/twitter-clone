use crate::errors::ApiError;
use diesel::pg::PgConnection;
use diesel::r2d2::{Builder, ConnectionManager, Pool, PooledConnection};
use lazy_static::lazy_static;
use r2d2;

pub type DBPool = Pool<ConnectionManager<PgConnection>>;
pub type DBPooledConnection = PooledConnection<ConnectionManager<PgConnection>>;

pub async fn init_pool(database_url: &str) -> Result<DBPool, r2d2::Error> {
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    Pool::builder().build(manager)
}

lazy_static! {
    static ref POOL: DBPool = {
        let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL missing");
        let manager = ConnectionManager::<PgConnection>::new(db_url);

        // test_transaction doesn't support multiple tx
        let pool_max_size = match cfg!(test) {
            true => 1,
            false => 10,
        };

        Builder::new()
            .max_size(pool_max_size)
            .build(manager)
            .expect("Failed to create db pool")
    };
}

pub fn init() {
    info!("Initializing DB");
    lazy_static::initialize(&POOL);

    // Setup test db mode while testing only
    if cfg!(test) {
        use crate::diesel::Connection;
        let conn = connection().expect("Failed to get db connection");
        conn.begin_test_transaction()
            .expect("Failed to start transaction");
    }
}

pub fn connection() -> Result<DBPooledConnection, ApiError> {
    POOL.get()
        .map_err(|e| ApiError::new(500, format!("Failed getting db connection: {}", e)))
}
