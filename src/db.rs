use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::models::{DBState, Epic, Status, Story};

pub trait Database {
    // Make the trait methods public by adding `pub` here
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
            .map_err(|e| anyhow::anyhow!("Failed to read file: {}", e))?;

        let db_state: DBState = serde_json::from_str(&file_content)
            .map_err(|e| anyhow::anyhow!("Failed to parse JSON: {}", e))?;

        Ok(db_state)
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

            let result = db.read_db();

            assert_eq!(result.is_ok(), true);
        }

        #[test]
        fn write_db_should_work() {
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

            let write_result = db.write_db(&state);
            let read_result = db.read_db().unwrap();

            assert_eq!(write_result.is_ok(), true);
            assert_eq!(read_result, state);
        }
    }
}
