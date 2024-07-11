use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::path::PathBuf;
use dirs::home_dir;

/// Struct to manage notes.
pub struct Notes {
    /// A vector to store note items.
    pub items: Vec<String>,
}

impl Notes {
    /// Creates a new `Notes` instance.
    ///
    /// # Returns
    ///
    /// A new `Notes` instance with an empty items vector.
    pub fn new() -> Notes {
        Notes {
            items: vec![],
        }
    }

    /// Adds a new note to the items vector.
    ///
    /// # Arguments
    ///
    /// * `note` - A string representing the note to be added.
    pub fn add(&mut self, note: String) {
        self.items.push(note);
    }

    /// Creates a new note file with the given title and content.
    ///
    /// # Arguments
    ///
    /// * `title` - The title of the note.
    /// * `content` - The content of the note.
    ///
    /// # Returns
    ///
    /// An `io::Result<()>` indicating success or failure.
    pub fn create_note_file(title: &str, content: &str) -> io::Result<()> {
        let path = Self::get_notes_dir()?.join(format!("{}.txt", title));
        let mut file = File::create(path)?;
        file.write_all(content.as_bytes())?;
        Ok(())
    }

    /// Reads the content of a note file with the given title.
    ///
    /// # Arguments
    ///
    /// * `title` - The title of the note to be read.
    ///
    /// # Returns
    ///
    /// An `io::Result<String>` containing the content of the note or an error.
    pub fn read_note_file(title: &str) -> io::Result<String> {
        let path = Self::get_notes_dir()?.join(format!("{}.txt", title));
        let mut file = File::open(path)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;
        Ok(content)
    }

    /// Updates the content of an existing note file with the given title.
    ///
    /// # Arguments
    ///
    /// * `title` - The title of the note to be updated.
    /// * `new_content` - The new content for the note.
    ///
    /// # Returns
    ///
    /// An `io::Result<()>` indicating success or failure.
    pub fn update_note_file(title: &str, new_content: &str) -> io::Result<()> {
        let path = Self::get_notes_dir()?.join(format!("{}.txt", title));
        let mut file = File::create(path)?;
        file.write_all(new_content.as_bytes())?;
        Ok(())
    }

    /// Deletes a note file with the given title.
    ///
    /// # Arguments
    ///
    /// * `title` - The title of the note to be deleted.
    ///
    /// # Returns
    ///
    /// An `io::Result<()>` indicating success or failure.
    pub fn delete_note_file(title: &str) -> io::Result<()> {
        let path = Self::get_notes_dir()?.join(format!("{}.txt", title));
        fs::remove_file(path)?;
        Ok(())
    }

    /// Lists all note files in the `.notes` directory.
    ///
    /// # Returns
    ///
    /// An `io::Result<Vec<String>>` containing the list of note titles or an error.
    pub fn list_notes() -> io::Result<Vec<String>> {
        let path = Self::get_notes_dir()?;
        let mut notes = Vec::new();
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_file() {
                if let Some(name) = path.file_stem() {
                    if let Some(name_str) = name.to_str() {
                        notes.push(name_str.to_string());
                    }
                }
            }
        }
        Ok(notes)
    }

    /// Returns the path to the `.notes` directory, creating it if it doesn't exist.
    ///
    /// # Returns
    ///
    /// An `io::Result<PathBuf>` containing the path to the `.notes` directory or an error.
    fn get_notes_dir() -> io::Result<PathBuf> {
        let home = home_dir().ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "Home directory not found"))?;
        let notes_dir = home.join(".notes");
        if !notes_dir.exists() {
            fs::create_dir_all(&notes_dir)?;
        }
        Ok(notes_dir)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use tempfile::tempdir;

    fn setup_temp_notes_dir() -> PathBuf {
        let temp_dir = tempdir().unwrap();
        let temp_notes_dir = temp_dir.path().join(".notes");
        fs::create_dir_all(&temp_notes_dir).unwrap();
        temp_notes_dir
    }

    #[test]
    fn test_create_note_file() {
        let temp_notes_dir = setup_temp_notes_dir();
        env::set_var("HOME", temp_notes_dir.parent().unwrap());

        let title = "test_note";
        let content = "This is a test note.";
        Notes::create_note_file(title, content).unwrap();

        let note_path = temp_notes_dir.join(format!("{}.txt", title));
        assert!(note_path.exists());

        let mut file = File::open(note_path).unwrap();
        let mut file_content = String::new();
        file.read_to_string(&mut file_content).unwrap();
        assert_eq!(file_content, content);
    }

    #[test]
    fn test_read_note_file() {
        let temp_notes_dir = setup_temp_notes_dir();
        env::set_var("HOME", temp_notes_dir.parent().unwrap());

        let title = "test_note";
        let content = "This is a test note.";
        Notes::create_note_file(title, content).unwrap();

        let read_content = Notes::read_note_file(title).unwrap();
        assert_eq!(read_content, content);
    }

    #[test]
    fn test_update_note_file() {
        let temp_notes_dir = setup_temp_notes_dir();
        env::set_var("HOME", temp_notes_dir.parent().unwrap());

        let title = "test_note";
        let content = "This is a test note.";
        let new_content = "This is updated content.";
        Notes::create_note_file(title, content).unwrap();
        Notes::update_note_file(title, new_content).unwrap();

        let read_content = Notes::read_note_file(title).unwrap();
        assert_eq!(read_content, new_content);
    }

    #[test]
    fn test_delete_note_file() {
        let temp_notes_dir = setup_temp_notes_dir();
        env::set_var("HOME", temp_notes_dir.parent().unwrap());

        let title = "test_note";
        let content = "This is a test note.";
        Notes::create_note_file(title, content).unwrap();

        let note_path = temp_notes_dir.join(format!("{}.txt", title));
        assert!(note_path.exists());

        Notes::delete_note_file(title).unwrap();
        assert!(!note_path.exists());
    }

    #[test]
    fn test_list_notes() {
        let temp_notes_dir = setup_temp_notes_dir();
        env::set_var("HOME", temp_notes_dir.parent().unwrap());

        let titles = vec!["note1", "note2", "note3"];
        for title in &titles {
            Notes::create_note_file(title, "content").unwrap();
        }

        let listed_notes = Notes::list_notes().unwrap();
        assert_eq!(listed_notes.len(), titles.len());
        for title in &titles {
            assert!(listed_notes.contains(&title.to_string()));
        }
    }
}