use crate::{db::DatabaseConnection, models::EpicJsonRow, repository::Repository};
use sqlx::mysql::MySqlPool;

mod db;
mod models;
mod repository;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let db_conection = DatabaseConnection::new(
        "tskoyo".to_owned(),
        "12345".to_owned(),
        "192.168.0.63".to_owned(),
        "3306".to_owned(),
        "jira_clone".to_owned(),
    );

    let pool = MySqlPool::connect(&db_conection.format_url()).await?;
    let repository = Repository::new(&pool);
    let epic = repository.get_epic(2).await?;

    for e in &epic {
        println!("-------------------------");
        println!("[Epic] name: {}", e.epic_name);
        println!("Story name: {}", e.story_name);
        match &e.story_description {
            Some(desc) => println!("Description: {}", desc),
            None => println!("Description: <none>"),
        }
        match &e.story_status {
            Some(status) => println!("Status: {:?}", status),
            None => println!("Status: <none>"),
        }
    }

    Ok(())
}
