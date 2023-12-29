use rusqlite::Connection;
use anyhow::Result;
use crate::DBPath;

#[derive(Debug)]
struct Data {
    path: String
}
fn connect() -> rusqlite::Result<Connection> {
    let db_path = DBPath {name: "movie"}.new().expect("");
    let conn = Connection::open(db_path)?;
    Ok(conn)
}
fn read_db(conn: &Connection) -> rusqlite::Result<()>{
    let query = "SELECT link FROM movie";
    let mut stmt = conn.prepare(query)?;
    let content_iter = stmt.query_map([], |row| {
        Ok(Data {
            path: row.get(0)?,
        })
    })?;
    for data in content_iter {
        println!("{:?}", data?)
    }
    Ok(())
}
pub fn db_main() -> Result<()> {
    let conn = connect()?;
    let _ = read_db(&conn).expect("");
    
    Ok(())
}