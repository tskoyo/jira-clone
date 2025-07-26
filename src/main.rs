mod db;
mod models;

use anyhow::Ok;
use db::Database;

fn main() -> Result<(), anyhow::Error> {
    let db = db::JSONFileDatabase {
        file_path: "./data/db.json".to_string(),
    };

    let mut db_state = db.read_db()?;

    let story = models::Story {
        name: "Story 1".to_string(),
        description: "A story description".to_string(),
        status: models::Status::Open,
    };

    let epic = models::Epic {
        name: "Epic 1".to_string(),
        description: "An epic description".to_string(),
        status: models::Status::Open,
        stories: vec![4],
    };

    db_state.epics.insert(2, epic);
    db_state.stories.insert(4, story);
    db_state.last_item_id = 4;

    db.write_db(&db_state)?;

    println!("Database written successfully!");

    Ok(())
}
