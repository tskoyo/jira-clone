use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Type, types::Json};

#[derive(Debug, Serialize, Deserialize, Type)]
#[sqlx(type_name = "varchar")]
#[allow(dead_code)]
pub enum Status {
    Open,
    InProgress,
    Resolved,
    Closed,
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
    pub name: String,
    pub description: String,
    pub status: Status,
    pub stories: Json<Vec<Story>>,
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
