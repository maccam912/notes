use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::path::PathBuf;
use serde::{Serialize, Deserialize};
use serde_json;
use dirs::home_dir;

/// Struct to represent a single todo item.
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Todo {
    /// The description of the todo item.
    pub description: String,
    /// The optional due date timestamp of the todo item.
    pub due_date: Option<i64>,
}

/// Struct to manage todos.
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Todos {
    /// A vector to store todo items.
    pub items: Vec<Todo>,
}

impl Todos {
    /// Creates a new `Todos` instance.
    ///
    /// # Returns
    ///
    /// A new `Todos` instance with an empty items vector.
    pub fn new() -> Todos {
        Todos {
            items: vec![],
        }
    }

    /// Adds a new todo to the items vector.
    ///
    /// # Arguments
    ///
    /// * `description` - A string representing the description of the todo.
    /// * `due_date` - An optional timestamp representing the due date of the todo.
    pub fn add(&mut self, description: String, due_date: Option<i64>) {
        self.items.push(Todo { description, due_date });
    }

    /// Saves the todos to a file.
    ///
    /// # Returns
    ///
    /// An `io::Result<()>` indicating success or failure.
    pub fn save_to_file(&self) -> io::Result<()> {
        let path = Self::get_todos_file_path()?;
        let mut file = File::create(path)?;
        let data = serde_json::to_string(&self)?;
        file.write_all(data.as_bytes())?;
        Ok(())
    }

    /// Loads the todos from a file.
    ///
    /// # Returns
    ///
    /// An `io::Result<Todos>` containing the loaded todos or an error.
    pub fn load_from_file() -> io::Result<Todos> {
        let path = Self::get_todos_file_path()?;
        let mut file = File::open(path)?;
        let mut data = String::new();
        file.read_to_string(&mut data)?;
        let todos: Todos = serde_json::from_str(&data)?;
        Ok(todos)
    }

    /// Returns the path to the `.todos` file in the `.notes` directory, creating the directory if it doesn't exist.
    ///
    /// # Returns
    ///
    /// An `io::Result<PathBuf>` containing the path to the `.todos` file or an error.
    fn get_todos_file_path() -> io::Result<PathBuf> {
        let home = home_dir().ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "Home directory not found"))?;
        let notes_dir = home.join(".notes");
        if !notes_dir.exists() {
            fs::create_dir_all(&notes_dir)?;
        }
        Ok(notes_dir.join(".todos"))
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
    fn test_add_todo() {
        let mut todos = Todos::new();
        todos.add("Test todo".to_string(), None);
        assert_eq!(todos.items.len(), 1);
        assert_eq!(todos.items[0].description, "Test todo");
    }

    #[test]
    fn test_save_and_load_todos() {
        let temp_notes_dir = setup_temp_notes_dir();
        env::set_var("HOME", temp_notes_dir.parent().unwrap());

        let mut todos = Todos::new();
        todos.add("Test todo".to_string(), Some(1627849200));
        todos.save_to_file().unwrap();

        let loaded_todos = Todos::load_from_file().unwrap();
        assert_eq!(loaded_todos.items.len(), 1);
        assert_eq!(loaded_todos.items[0].description, "Test todo");
        assert_eq!(loaded_todos.items[0].due_date, Some(1627849200));
    }
}