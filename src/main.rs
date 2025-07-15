use crate::{db::DatabaseConnection, models::EpicJsonRow};
use sqlx::mysql::MySqlPool;

mod db;
mod models;

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

    let epic = get_epic(&pool, 2).await?;

    for e in &epic {
        println!("-------------------------");
        println!("Epic name: {}", e.epic_name);
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

async fn get_epic(pool: &MySqlPool, epic_id: u64) -> Result<Vec<EpicJsonRow>, sqlx::Error> {
    let rows = sqlx::query_as::<_, EpicJsonRow>(
        r#"
        SELECT
            e.name as epic_name,
            e.description as epic_description,
            e.status as epic_status,
            s.name as story_name,
            s.description as story_description,
            s.status as story_status
        FROM epics e
        LEFT JOIN stories s ON e.id = s.epic_id
        WHERE e.id = ?
        "#,
    )
    .bind(epic_id)
    .fetch_all(pool)
    .await?;

    Ok(rows)
}
