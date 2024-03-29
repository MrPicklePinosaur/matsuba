use std::path::Path;

use rusqlite::Connection;
use rusqlite::{params, Result};

use crate::config::SETTINGS;

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
    let db_path = Path::new(&SETTINGS.database.cache_dir).join("dict.db3");
    Connection::open(db_path.to_str().unwrap())
}

pub fn init(conn: &Connection) -> Result<()> {
    conn.execute(
        "
        CREATE TABLE IF NOT EXISTS entry (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            r_ele TEXT NOT NULL,
            k_ele TEXT NOT NULL,
            frequency INTEGER DEFAULT 0
        )
        ",
        [],
    )?;

    Ok(())
}

pub fn insert_entry(conn: &Connection, entry: &Entry) -> Result<()> {
    conn.execute(
        "
        INSERT INTO entry (r_ele, k_ele)
        VALUES (?1, ?2)
        ",
        params![entry.r_ele, entry.k_ele],
    )?;

    Ok(())
}

pub fn search(conn: &Connection, reading: &str) -> Result<Vec<Entry>> {
    let mut query = conn.prepare(
        "
        SELECT r_ele, k_ele, frequency
        FROM entry
        WHERE r_ele = ?
        ",
    )?;

    let entry_it = query.query_map(&[reading], |row| Ok(Entry::new(row.get(0)?, row.get(1)?)))?;

    // TODO wonder if this can be better
    let mut output: Vec<Entry> = Vec::new();
    for entry in entry_it {
        output.push(entry?);
    }

    Ok(output)
}
