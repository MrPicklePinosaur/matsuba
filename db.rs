
use rusqlite::{Result, params};
pub use rusqlite::Connection;

pub struct Entry {
    pub r_ele: Vec<String>,
    pub k_ele: String,
}

pub fn get_connection() -> Result<Connection> {
    return Connection::open_in_memory();
}

pub fn init(conn: &Connection) -> Result<()> {

    conn.execute("
        CREATE TABLE entry (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            r_ele TEXT NOT NULL,
            k_ele TEXT NOT NULL,
            frequency INTEGER DEFAULT 0
        )
        ", []
    )?;

    Ok(())
}

pub fn insert_entry(conn: &Connection, entry: &Entry) -> Result<()> {

    // TODO maybe batch into a transaction for faster
    for reading in entry.r_ele.iter() {
        conn.execute("
            INSERT INTO entry (r_ele, k_ele)
            VALUES (?1, ?2)
            ", params![reading, entry.k_ele]
        )?;
    }

    Ok(())
}

