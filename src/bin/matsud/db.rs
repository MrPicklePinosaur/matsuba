
use rusqlite::{Result, ToSql, params};
use rusqlite::Connection;

pub type DBConnection = Connection;

#[derive(Debug)]
pub struct Entry {
    pub r_ele: String,
    pub k_ele: String,
    pub frequency: u8,
}

impl Entry {
    
    pub fn new(r_ele: String, k_ele: String) -> Self {
        Entry {
            r_ele,
            k_ele,
            frequency: 0,
        }
    }
}

pub fn get_connection() -> Result<Connection> {
    // TODO make this config variable
    return Connection::open("./db/matsuba.db3");
}

pub fn init(conn: &Connection) -> Result<()> {

    conn.execute("
        CREATE TABLE entry (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            r_ele TEXT NOT NULL,
            k_ele TEXT NOT NULL,
            frequency INTEGER DEFAULT 0,
            last_updated TEXT NOT NULL
        )
        ", []
    )?;

    Ok(())
}

pub fn insert_entry(conn: &Connection, entry: &Entry) -> Result<()> {

    use chrono::{DateTime, Utc};
    use std::time::SystemTime;

    let now: DateTime<Utc> = SystemTime::now().into();
    let now = now.to_rfc3339();

    conn.execute("
        INSERT INTO entry (r_ele, k_ele, last_updated)
        VALUES (?1, ?2, ?3)
        ", params![entry.r_ele, entry.k_ele, now]
    )?;

    Ok(())
}

pub enum SearchHeuristic {
    None,
    Frequency,
    Recent,
}
pub fn search(conn: &Connection, reading: &str, heuristic: SearchHeuristic) -> Result<Vec<Entry>> {

    let mut query = conn.prepare("
        SELECT r_ele, k_ele, frequency
        FROM entry
        WHERE r_ele = ?
        "
    )?;

    let entry_it = query.query_map(&[reading], |row| {
        Ok(Entry::new(
            row.get(0)?,
            row.get(1)?
        ))
    })?;

    // TODO wonder if this can be better
    let mut output: Vec<Entry> = Vec::new();
    for entry in entry_it {
        output.push(entry?);
    }

    Ok(output)
}

