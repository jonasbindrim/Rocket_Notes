use std::collections::HashMap;

use crate::data::note::NoteData;

pub struct NoteManager {
    note_storage: HashMap<String, NoteData>,
}

impl NoteManager {
    pub fn new() -> Self {
        NoteManager {
            note_storage: HashMap::new(),
        }
    }

    pub fn add_note(&mut self, title: &str, content: &str) -> Result<(), ()> {
        let note = NoteData::new(title, content);

        if self.note_storage.contains_key(title) {
            Err(())
        } else {
            self.note_storage.insert(String::from(title), note);
            Ok(())
        }
    }

    pub fn delete_note(&mut self, title: &str) -> Result<(), ()> {
        match self.note_storage.remove(title) {
            Some(_) => Ok(()),
            None => Err(()),
        }
    }

    pub fn get_note(&mut self, title: &str) -> Option<&NoteData> {
        self.note_storage.get(title)
    }

    pub fn get_all_notes(&self) -> Vec<NoteData> {
        let mut notes = Vec::<NoteData>::new();

        self.note_storage.values().for_each(|elem| {
            notes.push(elem.clone());
        });

        notes
    }
}
