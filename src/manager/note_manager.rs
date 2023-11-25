use crate::data::note_data::NoteData;

pub struct NoteManager {
    note_storage: Vec<NoteData>,
}

impl NoteManager {
    pub fn new() -> Self {
        NoteManager {
            note_storage: Vec::new(),
        }
    }

    pub fn add_note(&mut self, title: &str, content: &str) -> usize {
        let current_size = self.note_storage.len();
        let note = NoteData::new(title, content);
        self.note_storage.push(note);

        current_size
    }

    pub fn get_note(&mut self, id: usize) -> Option<&NoteData> {
        self.note_storage.get(id)
    }
}
