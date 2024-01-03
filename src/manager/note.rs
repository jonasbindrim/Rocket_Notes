use rusqlite::{Connection, params};

use crate::data::note::NoteData;

pub struct NoteManager {
    note_db_connection: Connection,
}

impl NoteManager {
    pub fn new() -> Self {

        let connection: Connection = Connection::open("notes.db").unwrap();
        Self::setup_database(&connection);

        NoteManager {
            note_db_connection: connection
        }
    }

    pub fn add_note(&mut self, title: &str, content: &str) -> Result<(), ()> {
        let note = NoteData::new(title, content);

        let insert_result = self.note_db_connection.execute(
            "insert into notes (title, content) values (?1, ?2)",
            [note.get_title(), note.get_content()]
        );

        match insert_result {
            Ok(_) => Ok(()),
            Err(_) => Err(()),
        }
    }

    pub fn delete_note(&mut self, title: &str) -> Result<(), ()> {
        let remove_result = self.note_db_connection.execute(
            "delete from notes where title=\"?1\"",
            [title]
        );        

        match remove_result {
            Ok(1) => Ok(()),
            Ok(_) | Err(_) => Err(())
        }
    }

    pub fn get_note(&mut self, title: &str) -> Option<NoteData> {
        let select_statement = self.note_db_connection.prepare(
            "select title, content from notes where title=:title;"
        );

        let Ok(mut select_statement) = select_statement else {
            return None;
        };

        let select_iter = select_statement.query_map(
            &[(":title", title)],
            |row| {
                Ok(
                    NoteData::new(
                        &row.get::<usize, String>(0).unwrap(),
                        &row.get::<usize, String>(1).unwrap()
                    )
                )
            }
        );

        let Ok(select_iter) = select_iter else {
            return None;
        };

        let select_results: Vec<NoteData> = select_iter.filter_map(std::result::Result::ok).collect();
        select_results.first().cloned()
    }

    pub fn get_all_notes(&self) -> Vec<NoteData> {
        let select_statement = self.note_db_connection.prepare(
            "select title, content from notes"
        );

        let Ok(mut select_statement) = select_statement else {
            return Vec::new();
        };

        let select_iter = select_statement.query_map(
            [],
            |row| {
                Ok(
                    NoteData::new(
                        &row.get::<usize, String>(0).unwrap(),
                        &row.get::<usize, String>(1).unwrap()
                    )
                )
            }
        );

        let Ok(select_iter) = select_iter else {
            return Vec::new();
        };

        select_iter.filter_map(std::result::Result::ok).collect::<Vec<NoteData>>()
    }

    fn setup_database(connection: &Connection) {
        connection.execute("create table if not exists notes (
            title text primary key,
            content text not null
        )", params![]).unwrap();
    }
}
