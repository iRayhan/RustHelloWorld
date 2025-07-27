use sqlx::mysql::MySqlQueryResult;
use sqlx::{query, query_as, Error, MySql, Pool};
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
    pub(crate) id: i32,
    pub(crate) name: Option<String>,
}

pub async fn get_all(pool: &Pool<MySql>) -> Vec<TestTableData> {
    query_as!(TestTableData, "select * from test_table")
        .fetch_all(pool)
        .await
        .unwrap()
}

pub async fn insert(pool: &Pool<MySql>, data: TestTableData) -> Result<MySqlQueryResult, Error> {
    query!("insert into test_table(name) values (?)", data.name)
        .execute(pool)
        .await
}

pub async fn delete(pool: &Pool<MySql>, delete_id: i32) -> Result<MySqlQueryResult, Error> {
    query!("delete from test_table where id=?", delete_id)
        .execute(pool)
        .await
}
