use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Type};

#[derive(Debug, Serialize, Deserialize, Type)]
#[repr(i32)]
#[allow(dead_code)]
pub enum Status {
    Open = 0,
    InProgress = 1,
    Resolved = 2,
    Closed = 3,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
#[allow(dead_code)]
pub struct Epic {
    pub name: String,
    pub description: String,
    pub status: Status,
    pub stories: Vec<Story>,
}

#[derive(Debug, FromRow)]
#[allow(dead_code)]
pub struct EpicJsonRow {
    pub epic_name: String,
    pub epic_description: String,
    pub epic_status: Status,
    pub story_name: Option<String>,
    pub story_description: Option<String>,
    pub story_status: Option<Status>,
}

impl Epic {
    pub fn new(name: String, description: String) -> Self {
        Epic {
            name: name,
            description: description,
            status: Status::Open,
            stories: Vec::new(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct Story {
    name: String,
    description: String,
    status: Status,
}

impl Story {
    pub fn new(name: String, description: String) -> Self {
        Story {
            name: name,
            description: description,
            status: Status::Open,
        }
    }
}
