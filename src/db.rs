use anyhow::{Ok, Result};
use serde::{Deserialize, Serialize};

use crate::models::{DBState, Epic, Status, Story};

pub struct JiraDatabase {
    database: Box<dyn Database>,
}

impl JiraDatabase {
    pub fn new(file_path: String) -> Self {
        JiraDatabase {
            database: Box::new(JSONFileDatabase { file_path }),
        }
    }

    pub fn read_db(&self) -> Result<DBState> {
        let db_state = self.database.read_db()?;

        Ok(db_state)
    }

    pub fn create_epic(&self, epic: Epic) -> Result<u32> {
        let mut db_state = self.read_db()?;
        let epic_id = db_state.last_item_id + 1;

        db_state.last_item_id = epic_id;
        db_state.epics.insert(epic_id, epic);

        self.database.write_db(&db_state)?;

        Ok(epic_id)
    }

    pub fn create_story(&self, story: Story, epic_id: u32) -> Result<u32> {
        let mut db_state = self.read_db()?;
        let story_id = db_state.last_item_id + 1;

        db_state.last_item_id = story_id;
        db_state.stories.insert(story_id, story);

        if let Some(epic) = db_state.epics.get_mut(&epic_id) {
            epic.stories.push(story_id);
        } else {
            return Err(anyhow::anyhow!("Epic not found!"));
        }

        self.database.write_db(&db_state)?;

        Ok(story_id)
    }

    pub fn delete_epic(&self, epic_id: u32) -> Result<()> {
        let mut db_state = self.read_db()?;

        if let Some(_) = db_state.epics.get(&epic_id) {
            db_state.epics.remove(&epic_id);
        } else {
            return Err(anyhow::anyhow!("Epic not found!"));
        }

        self.database.write_db(&db_state)?;
        Ok(())
    }

    pub fn delete_story(&self, epic_id: u32, story_id: u32) -> Result<()> {
        let mut db_state = self.read_db()?;

        if let Some(_) = db_state.stories.get(&epic_id) {
            db_state.stories.remove(&epic_id);
        } else {
            return Err(anyhow::anyhow!("Epic not found!"));
        }

        self.database.write_db(&db_state)?;
        Ok(())
    }

    pub fn update_epic_status(&self, epic_id: u32, status: Status) -> Result<()> {
        let mut db_state = self.read_db()?;

        if let Some(epic) = db_state.epics.get_mut(&epic_id) {
            epic.status = status;
        } else {
            return Err(anyhow::anyhow!("Epic not found!"));
        }

        self.database.write_db(&db_state)?;
        Ok(())
    }

    pub fn update_story_status(&self, story_id: u32, status: Status) -> Result<()> {
        let mut db_state = self.read_db()?;

        if let Some(story) = db_state.stories.get_mut(&story_id) {
            story.status = status;
        } else {
            return Err(anyhow::anyhow!("Epic not found!"));
        }

        self.database.write_db(&db_state)?;
        Ok(())
    }
}

pub trait Database {
    fn read_db(&self) -> Result<DBState>;
    fn write_db(&self, db_state: &DBState) -> Result<()>;
}

#[derive(Serialize, Deserialize, Debug)]
pub struct JSONFileDatabase {
    pub file_path: String,
}

impl Database for JSONFileDatabase {
    fn read_db(&self) -> Result<DBState> {
        let file_content = std::fs::read_to_string(&self.file_path)
            .map_err(|e| anyhow::anyhow!("Failed to read to string: {}", e))?;

        let deserialized_content: DBState = serde_json::from_str(&file_content)
            .map_err(|e| anyhow::anyhow!("Failed to deserialize: {}", e))?;

        Ok(deserialized_content)
    }

    fn write_db(&self, db_state: &DBState) -> Result<()> {
        let json_content = serde_json::to_string_pretty(db_state)
            .map_err(|e| anyhow::anyhow!("Failed to serialize DBState: {}", e))?;

        std::fs::write(&self.file_path, json_content)
            .map_err(|e| anyhow::anyhow!("Failed to write file: {}", e))?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod database {
        use std::collections::HashMap;
        use std::io::Write;

        use super::*;

        fn create_test_db() -> (tempfile::NamedTempFile, JSONFileDatabase) {
            let mut tmpfile = tempfile::NamedTempFile::new().unwrap();
            let file_contents = r#"{ "last_item_id": 0, "epics": {}, "stories": {} }"#;
            write!(tmpfile, "{}", file_contents).unwrap();

            let db = JSONFileDatabase {
                file_path: tmpfile
                    .path()
                    .to_str()
                    .expect("failed to convert tmpfile path to str")
                    .to_string(),
            };

            (tmpfile, db)
        }

        #[test]
        fn create_epic_should_work() {
            let (_tmpfile, test_db) = create_test_db();

            let jira_database = JiraDatabase::new(test_db.file_path.clone());

            let epic = Epic {
                name: "New Epic".to_owned(),
                description: "Epic description".to_owned(),
                status: Status::Open,
                stories: vec![],
            };

            jira_database.create_epic(epic).unwrap();
            let db_state = jira_database.read_db().unwrap();
            assert_eq!(db_state.last_item_id, 1);
            assert_eq!(db_state.epics.len(), 1);
        }

        #[test]
        fn create_story_should_work() {
            let (_tmpfile, test_db) = create_test_db();
            let jira_database = JiraDatabase::new(test_db.file_path.clone());

            let story = Story {
                name: "Story".to_owned(),
                description: "Story description".to_owned(),
                status: Status::Open,
            };

            let epic = Epic {
                name: "New Epic".to_owned(),
                description: "Epic description".to_owned(),
                status: Status::Open,
                stories: vec![],
            };
            let epic_id = jira_database.create_epic(epic).unwrap();

            jira_database.create_story(story, epic_id);

            let db_state = jira_database.read_db().unwrap();
            assert_eq!(db_state.last_item_id, 2);
            assert_eq!(db_state.epics.len(), 1);
            assert_eq!(db_state.stories.len(), 1);
        }

        #[test]
        fn read_db_should_fail_with_invalid_path() {
            let db = JSONFileDatabase {
                file_path: "INVALID_PATH".to_owned(),
            };
            assert_eq!(db.read_db().is_err(), true);
        }

        #[test]
        fn read_db_should_fail_with_invalid_json() {
            let mut tmpfile = tempfile::NamedTempFile::new().unwrap();

            let file_contents = r#"{ "last_item_id": 0 epics: {} stories {} }"#;
            write!(tmpfile, "{}", file_contents).unwrap();

            let db = JSONFileDatabase {
                file_path: tmpfile
                    .path()
                    .to_str()
                    .expect("failed to convert tmpfile path to str")
                    .to_string(),
            };

            let result = db.read_db();

            assert_eq!(result.is_err(), true);
        }

        #[test]
        fn read_db_should_parse_json_file() {
            let (_tmpfile, test_db) = create_test_db();
            let result = test_db.read_db();

            assert_eq!(result.is_ok(), true);
        }

        #[test]
        fn write_db_should_work() {
            let (_tmpfile, test_db) = create_test_db();

            let story = Story {
                name: "epic 1".to_owned(),
                description: "epic 1".to_owned(),
                status: Status::Open,
            };
            let epic = Epic {
                name: "epic 1".to_owned(),
                description: "epic 1".to_owned(),
                status: Status::Open,
                stories: vec![2],
            };

            let mut stories = HashMap::new();
            stories.insert(2, story);

            let mut epics = HashMap::new();
            epics.insert(1, epic);

            let state = DBState {
                last_item_id: 2,
                epics,
                stories,
            };

            let write_result = test_db.write_db(&state);
            let read_result = test_db.read_db().unwrap();

            assert_eq!(write_result.is_ok(), true);
            assert_eq!(read_result, state);
        }
    }
}
