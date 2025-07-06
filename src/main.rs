use crate::models::Epic;
use mysql::prelude::*;
use mysql::*;

mod models;

#[derive(Debug)]
struct EpicRow {
    id: i32,
    name: String,
    description: String,
    status: String,
}

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let url = "mysql://root:12345@192.168.0.63:3306/jira_clone";
    let pool = Pool::new(url)?;

    let mut conn = pool.get_conn()?;
    let rows: Vec<EpicRow> = conn.query_map(
        "SELECT id, name, description, status FROM epics",
        |(id, name, description, status)| EpicRow {
            id,
            name,
            description,
            status,
        },
    )?;

    for row in rows {
        println!("{:?}", row);
    }

    Ok(())
}
