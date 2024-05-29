use std::{env, time::Duration};

use sqlx::{mysql::MySqlPoolOptions, MySql, Pool};

#[derive(Clone)]
pub struct Database {
    pub pool: Pool<MySql>,
}

impl Database {
    pub async fn new() -> Self {
        let url = env::var("DATABASE_URL").expect("DATABASE_URL is not defined");
        Self {
            pool: MySqlPoolOptions::new()
                .max_connections(20)
                .acquire_timeout(Duration::from_secs(3))
                .connect(&url)
                .await
                .expect("Can't connect Database")
        }
    }
}