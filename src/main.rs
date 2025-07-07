use crate::models::Epic;
use crate::models::Status;
use sqlx::mysql::MySqlPool;
use sqlx::mysql::MySqlPoolOptions;
use sqlx::mysql::MySqlRow;

mod models;
mod repository;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let url = "mysql://tskoyo:12345@192.168.0.63:3306/jira_clone";
    let pool = MySqlPool::connect(url).await?;

    let epic = get_epic(&pool, 2).await?;
    println!("{:?}", epic);

    Ok(())
}

async fn get_epic(pool: &MySqlPool, epic_id: u64) -> Result<Vec<MySqlRow>, sqlx::Error> {
    let rows: Vec<Epic> = sqlx::query_as::<_, Epic>(
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

    Ok(records)
}

// async fn list_todos(pool: &MySqlPool) -> Result<(), sqlx::Error> {
//     let recs = sqlx::query!(
//         r#"
// SELECT id, description, done
// FROM todos
// ORDER BY id
//         "#
//     )
//     .fetch_all(pool)
//     .await?;

//     // NOTE: Booleans in MySQL are stored as `TINYINT(1)` / `i8`
//     //       0 = false, non-0 = true
//     for rec in recs {
//         println!(
//             "- [{}] {}: {}",
//             if rec.done != 0 { "x" } else { " " },
//             rec.id,
//             &rec.description,
//         );
//     }

//     Ok(())
// }

// fn create_db_pool(database_url: &string) -> MySqlPool {}
