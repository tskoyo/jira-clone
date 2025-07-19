use crate::EpicJsonRow;
use sqlx::{MySql, Pool};

pub struct Repository<'a> {
    pub pool: &'a Pool<MySql>,
}

impl<'a> Repository<'a> {
    pub fn new(mysql_pool: &'a Pool<MySql>) -> Self {
        Repository { pool: mysql_pool }
    }

    pub async fn get_epic(&self, epic_id: u64) -> Result<Vec<EpicJsonRow>, sqlx::Error> {
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
        .fetch_all(self.pool)
        .await?;

        Ok(rows)
    }

    pub async fn insert_epic(
        &self,
        name: &str,
        description: &str,
        status: &str,
    ) -> Result<u64, sqlx::Error> {
        let result = sqlx::query!(
            r#"
            INSERT INTO epics (name, description, status)
            VALUES (?, ?, ?)
            "#,
            name,
            description,
            status
        );

        Ok(2)
        // .execute(self.pool)
        // .await?;
    }
}
