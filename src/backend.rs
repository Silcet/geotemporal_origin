use dioxus::prelude::*;
use server_fn::ServerFn;

#[cfg(feature = "server")]
thread_local! {
    pub static DB: rusqlite::Connection = {
        let conn = rusqlite::Connection::open("hotdog.db").expect("Failed to open the database");

        conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS dogs (
                id INTEGER PRIMARY KEY,
                url TEXT NOT NULL
            );"
        ).unwrap();

        conn
    }
}

#[server]
pub async fn save_dog(image: String) -> Result<(), ServerFnError> {
    DB.with(|f| f.execute("INSERT INTO dogs (url) VALUES (?1)", &[&image]))?;
    Ok(())
}

#[server]
pub async fn list_dogs() -> Result<Vec<(usize, String)>, ServerFnError> {
    let dogs = DB.with(|f| {
        f.prepare("SELECT id, url FROM dogs ORDER BY id  DESC LIMIT 10")
            .unwrap()
            .query_map([], |row| Ok((row.get(0)?, row.get(1)?)))
            .unwrap()
            .map(|r| r.unwrap())
            .collect()
    });

    Ok(dogs)
}

#[server]
pub async fn get_dog(id: usize) -> Result<String, ServerFnError> {
    let dog = DB
        .with(|f| {
            f.query_row("SELECT url FROM dogs WHERE id = (?1)", &[&id], |row| {
                row.get(0)
            })
        })
        .unwrap();

    Ok(dog)
}

#[server]
pub async fn delete_dog(id: usize) -> Result<(), ServerFnError> {
    DB.with(|f| f.execute("DELETE FROM dogs WHERE id = (?1)", &[&id]))?;
    Ok(())
}
