pub enum Status {
    Open,
    InProgress,
    Resolved,
    Closed,
}

pub struct Epic {
    name: String,
    description: String,
    status: Status,
    stories: Vec<Story>,
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
