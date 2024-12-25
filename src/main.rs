use std::result::Result;
use async_std::main;
use sqlx::{sqlite::SqliteQueryResult,Sqlite,SqlitePool,migrate::MigrateDatabase};


async fn create_schema(db_url:&str)->Result<SqliteQueryResult, sqlx::Error>{
    let pool = SqlitePool::connect(&db_url).await?;
    let qry = "PRAGMA foreign_keys = ON;
    CREATE TABLE IF NOT EXISTS settings
    (
        setting_id   INTEGER PRIMARY KEY NOT NULL,
        description  TEXT                NOT NULL,
        created_on   DATABASE DEFAULT    (datetime('now','localtime')),
        updated_on   DATABASE DEFAULT    (datetime('now','localtime')),
        done         BOOLEAN             NOT NULL DEFAULT 0
    );
    CREATE TABLE IF NOT EXISTS project (
        project_id    INTEGER PRIMARY KEY AUTOINCREMENT, 
        product_name  TEXT
    );
     ";
    let result = sqlx::query(&qry).execute(&pool).await;
    pool.close().await;
    return result;

}

#[main]
async fn main() {
    println!("Hello, world!");
    let db_url = String::from("sqlite://sqlite.db");
    if !Sqlite::database_exists(&db_url).await.unwrap_or(false){
        Sqlite::create_database(&db_url).await.unwrap();
        match create_schema(&db_url).await{
            Ok(_)=>println!("Database is created"),
            Err(e)=>panic!("{}",e),
        }
    };
    let instance = SqlitePool::connect(&db_url).await.unwrap();
    let qry ="INSERT INTO settings (description) VALUES($1)";
    let result = sqlx::query(&qry).bind("testing").execute(&instance).await;
    
    match result {
        Ok(res) => println!("Insert successful: {:?}", res),
        Err(e) => println!("Error during insertion: {}", e),
    }
    instance.close().await;
}
