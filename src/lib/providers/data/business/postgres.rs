use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[allow(dead_code)]
pub struct PostgresData {
    PG_DATABASE_NAME: String,
    PG_HOST: String,
    PG_PASSWORD: String,
    PG_PORT: String,
    PG_URL: String,
    PG_URL_PREFIX: String,
    PG_USER: String,
    // pg_database_name: ,
    // pg_host: String,
    // pg_password: String,
    // pg_port: i32,
    // pg_url: String,
    // pg_url_prefix: String,
    // pg_user: String,
}

pub struct Postgres {
    // client:
}
