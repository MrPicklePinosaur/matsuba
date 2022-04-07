
use rusqlite::{Connection, Result};

pub fn init() -> Result<()> {
    let db = Connection::open_in_memory()?;

    db.execute("
        CREATE TABLE kanji (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            kana TEXT NOT NULL,
            kanji TEXT NOT NULL,
            frequency INTEGER DEFAULT 0,
        )
        ", []
    )?;

    Ok(())
}
