use rocket::{Build, Rocket};
use rocket_sync_db_pools::{
    database,
    rusqlite::{self, params},
};

use crate::data::note::NoteData;

/// Sets up the database for storing notes
#[database("rusqlite")]
pub struct NotesDb(rusqlite::Connection);

/// Initializes the database connection and tables for the notes database
pub async fn init_notesdb(rocket: Rocket<Build>) -> Rocket<Build> {
    let Some(database_connection) = NotesDb::get_one(&rocket).await else {
        panic!("Unable to mount database \"NotesDb\"");
    };

    let create_table = database_connection
        .run(|connection| connection.execute(SQL_NOTES_INIT, params![]))
        .await;

    match create_table {
        Ok(_) => rocket,
        Err(error) => panic!("Unable to prepare database \"NotesDb\": {error}"),
    }
}

/// Supplies methods to execute a variety of prepared actions on the notes sql database
pub struct NotesManager {}

impl NotesManager {
    /// Adds the given note to the given database
    /// - `database` The database to which a note should be added
    /// - `note` The note which is written into the given database
    /// - `returns` Whether the note was added successfully
    pub async fn add_note(database: NotesDb, note: NoteData) -> Result<(), ()> {
        let insert_result = database
            .run(move |connection| {
                connection.execute(SQL_NOTES_INSERT, [note.get_title(), note.get_content()])
            })
            .await;

        match insert_result {
            Ok(_) => Ok(()),
            Err(_) => Err(()),
        }
    }

    /// Deletes the given note to the given database
    /// - `database` The database from which a note should be deleted
    /// - `title` The title of the note which is deleted from the given database
    /// - `returns` Whether the note was deleted successfully
    pub async fn delete_note(database: NotesDb, title: String) -> Result<(), ()> {
        let delete_result = database
            .run(move |connection| connection.execute(SQL_NOTES_DELETE, [title]))
            .await;

        match delete_result {
            Ok(1) => Ok(()),
            Ok(_) | Err(_) => Err(()),
        }
    }

    /// Retrieves a single note from the given database
    /// - `database` The database from which a note should be retrieved
    /// - `title` The title of the note which is retrieved from the given database
    /// - `returns` The note entry if available
    pub async fn get_note(database: NotesDb, title: String) -> Option<NoteData> {
        database
            .run(move |connection| {
                let Ok(mut select_statement) = connection.prepare(SQL_NOTES_SELECT_BY_TITLE) else {
                    return None;
                };

                let select_iter = select_statement.query_map(&[(":title", &title)], |row| {
                    Ok(NoteData::new(
                        &row.get::<usize, String>(0).unwrap(),
                        &row.get::<usize, String>(1).unwrap(),
                    ))
                });

                let Ok(select_iter) = select_iter else {
                    return None;
                };

                select_iter
                    .filter_map(std::result::Result::ok)
                    .collect::<Vec<NoteData>>()
                    .first()
                    .cloned()
            })
            .await
    }

    /// Retrieves all notes from the given database
    /// - `database` The database from which the notes should be retrieved
    /// - `returns` All available note entries
    pub async fn get_all_notes(database: NotesDb) -> Vec<NoteData> {
        database
            .run(move |connection| {
                let Ok(mut select_statement) = connection.prepare(SQL_NOTES_SELECT) else {
                    return Vec::new();
                };

                let select_iter = select_statement.query_map([], |row| {
                    Ok(NoteData::new(
                        &row.get::<usize, String>(0).unwrap(),
                        &row.get::<usize, String>(1).unwrap(),
                    ))
                });

                let Ok(select_iter) = select_iter else {
                    return Vec::new();
                };

                select_iter
                    .filter_map(std::result::Result::ok)
                    .collect::<Vec<NoteData>>()
            })
            .await
    }
}

static SQL_NOTES_INIT: &str = r"
    CREATE TABLE IF NOT EXISTS notes (
        title TEXT PRIMARY KEY,
        content TEXT NOT NULL
    )
";

static SQL_NOTES_INSERT: &str = "INSERT INTO notes (title, content) VALUES (?1, ?2)";
static SQL_NOTES_DELETE: &str = "DELETE FROM notes WHERE title=?1";
static SQL_NOTES_SELECT_BY_TITLE: &str = "SELECT title, content FROM notes WHERE title=:title";
static SQL_NOTES_SELECT: &str = "SELECT title, content FROM notes";
