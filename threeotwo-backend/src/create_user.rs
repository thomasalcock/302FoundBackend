use rusqlite::{params, Connection, Result};

// Define a simple User struct
#[derive(Debug)]
struct User {
    id: i32,
    username: String,
    email: String,
}

// Function to create a users table (if it doesn't exist)
fn init_db(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS users (
            id          INTEGER PRIMARY KEY AUTOINCREMENT,
            username    TEXT NOT NULL UNIQUE,
            location_longitude REAL,
            location_latitude REAL
        )",
        [],
    )?;
    Ok(())
}

// Function to insert a new user into the database
fn create_user(conn: &Connection, username: &str, email: &str) -> Result<User> {
    conn.execute(
        "INSERT INTO users (username, email) VALUES (?1, ?2)",
        params![username, email],
    )?;

    let id = conn.last_insert_rowid() as i32;

    Ok(User {
        id,
        username: username.to_string(),
        email: email.to_string(),
    })
}

fn main() -> Result<()> {
    // Open or create the database file
    let conn = Connection::open("users.db")?;

    // Initialize the table
    init_db(&conn)?;

    // Create a new user
    let user = create_user(&conn, "alice", "alice@example.com")?;

    println!("Created user: {:?}", user);

    Ok(())
}
