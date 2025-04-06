use chrono::{DateTime, Duration, Utc};
use rusqlite::{params, Connection, Result};

pub fn init_db() -> Result<Connection> {
    let conn = Connection::open("cache.db")?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS cache (
            key TEXT PRIMARY KEY,
            data TEXT NOT NULL,
            timestamp TEXT NOT NULL
        )",
        [],
    )?;
    Ok(conn)
}

pub fn get_from_cache(conn: &Connection, key: &str, max_age_mins: i64) -> Option<String> {
    let mut stmt = conn
        .prepare("SELECT data, timestamp FROM cache WHERE key = ?1")
        .ok()?;
    let mut rows = stmt.query(params![key]).ok()?;

    if let Some(row) = rows.next().ok()? {
        let data: String = row.get(0).ok()?;
        let timestamp: String = row.get(1).ok()?;
        let time: DateTime<Utc> = timestamp.parse().ok()?;
        if Utc::now() - time < Duration::minutes(max_age_mins) {
            return Some(data);
        }
    }
    None
}

pub fn save_to_cache(conn: &Connection, key: &str, data: &str) -> Result<()> {
    let now = Utc::now().to_rfc3339();
    conn.execute(
        "REPLACE INTO cache (key, data, timestamp) VALUES (?1, ?2, ?3)",
        params![key, data, now],
    )?;
    Ok(())
}
