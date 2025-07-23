use sqlx::{query_as, MySql, Pool, Row};
use std::env;

pub fn init_env() {
    dotenv::dotenv().ok();
}

fn get_database_url() -> String {
    env::var("DATABASE_URL").expect("DATABASE_URL must be set")
}

pub async fn get_pool() -> Pool<MySql> {
    let database_url = get_database_url();
    sqlx::mysql::MySqlPoolOptions::new()
        .connect(&database_url)
        .await
        .unwrap()
}

#[derive(Debug)]
pub struct TestTableData {
    id: i32,
    name: Option<String>,
}

pub async fn get_all(pool: Pool<MySql>) -> Vec<TestTableData> {
    query_as!(TestTableData, "select * from test_table")
        .fetch_all(&pool)
        .await
        .unwrap()
}
