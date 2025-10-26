use sqlx::{query, Pool, Sqlite};

use crate::error::{Error, Result};

#[derive(Clone)]
pub struct AppState {
    conn: Pool<Sqlite>
}

impl AppState {
    pub fn new(conn: Pool<Sqlite>) -> Self {
        AppState {conn: conn}
    }

    pub async fn init_database(&self) -> Result<()> {
        let query_string = r#"    
        CREATE TABLE IF NOT EXISTS user (
            id INTEGER PRIMARY KEY autoincrement,
            user_name VARCHAR(255),
            full_name VARCHAR(255) NOT NULL,
            email VARCHAR(255),
            phone_number VARCHAR(255),
            alias INTEGER,
            FOREIGN KEY (alias) REFERENCES user(id)
        );
        DELETE FROM user;

        CREATE TABLE IF NOT EXISTS trust (
            user INTEGER NOT NULL,
            trustee INTEGER NOT NULL,
            FOREIGN KEY (user) REFERENCES user(id),
            FOREIGN KEY (trustee) REFERENCES user(id),
            PRIMARY KEY (user, trustee)
        );
        DELETE FROM trust;

        CREATE TABLE IF NOT EXISTS location (
            id INTEGER PRIMARY KEY autoincrement,
            user_id INTEGER NOT NULL,
            timestamp INTEGER NOT NULL,
            latitude REAL NOT NULL,
            longitude REAL NOT NULL,
            FOREIGN KEY (user_id) REFERENCES user(id)
        );
        DELETE FROM location;
        "#;     
        query(query_string)
            .fetch_all(self.conn())
            .await
            .map_err(|e| {
                println!("{}", e);
                Error::DatabaseError

            })?;
        Ok(())
    }

    pub fn conn(&self) -> &Pool<Sqlite> {&self.conn}
    //pub fn conn_mut(&mut self) -> &mut Pool<Sqlite> {&mut self.conn}
}
