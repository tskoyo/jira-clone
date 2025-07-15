// trait Database {
//     fn read_db();
//     fn write_db();
// }

pub struct DatabaseConnection {
    pub username: String,
    pub password: String,
    pub address: String,
    pub port: String,
    pub db_name: String,
}

impl DatabaseConnection {
    pub fn new(
        username: String,
        password: String,
        address: String,
        port: String,
        db_name: String,
    ) -> Self {
        DatabaseConnection {
            username,
            password,
            address,
            port,
            db_name,
        }
    }

    pub fn format_url(&self) -> String {
        format!(
            "mysql://{}:{}@{}:{}/{}",
            self.username, self.password, self.address, self.port, self.db_name
        )
    }
}
