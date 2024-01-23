use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[allow(dead_code)]
pub struct PostgresData {
    pg_database_name: String,
    pg_host: String,
    pg_password: String,
    pg_port: i32,
    pg_url: String,
    pg_url_prefix: String,
    pg_user: String,
}

pub struct Postgres {
    // client:
}
