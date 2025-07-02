use crate::models::Epic;

mod models;

fn main() {
    let epic = Epic::new(
        "API design".to_owned(),
        "We should create a new API design".to_owned(),
    );
}
