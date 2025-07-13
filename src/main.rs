use crate::models::{Epic, EpicJsonRow, Status};
use sqlx::FromRow;
use sqlx::mysql::MySqlPool;

mod models;
mod repository;

#[derive(Debug, FromRow)]
pub struct EpicWithStoryRow {
    pub epic_name: String,
    pub epic_description: String,
    pub epic_status: Status,
    pub story_name: Option<String>,
    pub story_description: Option<String>,
    pub story_status: Option<Status>,
}

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let url = "mysql://tskoyo:12345@192.168.0.63:3306/jira_clone";
    let pool = MySqlPool::connect(url).await?;

    let epic = get_epic(&pool, 2).await?;
    println!("{:?}", epic);

    Ok(())
}

async fn get_epic(pool: &MySqlPool, epic_id: u64) -> Result<Vec<EpicJsonRow>, sqlx::Error> {
    let rows = sqlx::query_as::<_, EpicJsonRow>(
        r#"SELECT
            e.id,
            e.name,
            e.description,
            CAST(e.status AS CHAR),
            JSON_ARRAYAGG(
                JSON_OBJECT(
                    'name',        s.name,
                    'description', s.description,
                    'status',      CAST(s.status AS CHAR)
                )
            ) AS stories
        FROM    epics e
        LEFT JOIN stories s ON s.epic_id = e.id
        WHERE   e.id = ?
        GROUP BY e.id;
        "#,
    )
    .bind(epic_id)
    .fetch_all(pool)
    .await?;

    Ok(rows)
}
