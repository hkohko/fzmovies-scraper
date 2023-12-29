use rusqlite::Connection;
use anyhow::Result;
use crate::DBPath;

pub fn db_main(data: Vec<String>) -> Result<()>{
    let conn = connect().expect("");
    create_table(&conn)?;
    insert_data(&conn, data)?;
    Ok(())
}
fn connect() -> rusqlite::Result<Connection> {
    let db_path = DBPath{name: "movie"}.new().expect("");
    let conn = Connection::open(db_path);
    conn
}
fn create_table(conn: &Connection) -> Result<()>{
    let query = "CREATE TABLE IF NOT EXISTS movie (
       id INTEGER PRIMARY KEY, 
       link TEXT
       )STRICT";
    conn.execute(query, ())?;
    Ok(())
}
fn insert_data(conn: &Connection, data: Vec<String>) -> Result<()>{
    let query = "INSERT OR IGNORE INTO movie(
        link
    )VALUES (
        ?1
    )";
    for string in data.iter() {
        conn.execute(query, (string,))?;
    }
    Ok(()) 
}