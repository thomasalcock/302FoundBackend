use sqlx::{Pool, Sqlite};

#[derive(Clone)]
pub struct AppState {
    conn: Pool<Sqlite>
}

impl AppState {
    pub fn new(conn: Pool<Sqlite>) -> Self {
        AppState {conn: conn}
    }

    pub fn conn(&self) -> &Pool<Sqlite> {&self.conn}
    pub fn conn_mut(&mut self) -> &mut Pool<Sqlite> {&mut self.conn}
}
