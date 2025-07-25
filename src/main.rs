mod db;
mod models;

use anyhow::Ok;
use db::Database;

fn main() -> Result<(), anyhow::Error> {
    let db = db::JSONFileDatabase {
        file_path: "db_state.json".to_string(), // Path to your JSON file
    };

    let epic = models::Epic {
        name: "Epic 1".to_string(),
        description: "An epic description".to_string(),
        status: models::Status::Open,
        stories: vec![], // Empty stories for now
    };

    let story = models::Story {
        name: "Story 1".to_string(),
        description: "A story description".to_string(),
        status: models::Status::Open,
    };

    let mut epics = std::collections::HashMap::new();
    epics.insert(1, epic);

    let mut stories = std::collections::HashMap::new();
    stories.insert(1, story);

    let db_state = models::DBState {
        last_item_id: 2,
        epics,
        stories,
    };

    db.write_db(&db_state)?;

    println!("Database written successfully!");

    let db_state_read = db.read_db()?;

    println!("Read DBState: {:?}", db_state_read);

    Ok(())
}
