use crate::{DBPath, Data};
use rusqlite::Connection;

fn connect() -> rusqlite::Result<Connection> {
    let db_path = DBPath { name: "movie" }.new().expect("");
    let conn = Connection::open(db_path)?;
    Ok(conn)
}
fn read_db(conn: &Connection) -> rusqlite::Result<Vec<Data>> {
    let query = "SELECT link FROM movie";
    let mut stmt = conn.prepare(query)?;
    let content_iter = stmt.query_map([], |row| Ok(Data { path: row.get(0)? }))?;
    let vec = content_iter
        .map(|val| {
            val.unwrap_or(Data {
                path: "None".to_string(),
            })
        })
        .collect::<Vec<Data>>();
    Ok(vec)
}
pub fn db_main() -> rusqlite::Result<Vec<Data>> {
    let conn = connect()?;
    Ok(read_db(&conn).expect(""))
}
