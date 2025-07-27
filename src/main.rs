mod db;
mod models;

use anyhow::Ok;
use db::Database;

use crate::models::Status;

fn main() -> Result<(), anyhow::Error> {
    let database = db::JiraDatabase::new("./data/db.json".to_string());

    let epic = models::Epic {
        name: "New Epic 1".to_string(),
        description: "New epic description".to_string(),
        status: models::Status::Open,
        stories: vec![],
    };

    let new_epic_id = database
        .create_epic(epic)
        .map_err(|e| anyhow::anyhow!("Error creating epic: {}", e))?;
    println!("Created epic: {:?}", new_epic_id);

    database
        .update_epic_status(new_epic_id, models::Status::InProgress)
        .map_err(|e| anyhow::anyhow!("Error updating epic status: {}", e))?;

    println!("Updated epic status to InProgress");
    Ok(())
}
