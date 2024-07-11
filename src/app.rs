use eframe::egui::{self, CentralPanel, SidePanel, TopBottomPanel};
use std::sync::Arc;
use std::sync::Mutex;

use crate::notes::Notes;
use crate::todos::Todos;

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct TemplateApp {
    #[serde(skip)]
    notes: Arc<Mutex<Notes>>,
    #[serde(skip)]
    todos: Arc<Mutex<Todos>>,
    selected_note: Option<String>,
    command_input: String,
    mode: Mode,
}

impl Default for TemplateApp {
    fn default() -> Self {
        let mut notes = Notes::new();

        // Load notes from the file system
        let loaded_notes = Notes::list_notes().unwrap_or_default();
        for note in loaded_notes {
            notes.add(note);
        }

        // Load todos from the file system
        let todos = Todos::load_from_file().unwrap_or_default();

        Self {
            notes: Arc::new(Mutex::new(notes)),
            todos: Arc::new(Mutex::new(todos)),
            selected_note: None,
            command_input: String::new(),
            mode: Mode::Command,
        }
    }
}

impl TemplateApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }
        Default::default()
    }

    fn create_note(&mut self, title: &str, content: &str) {
        let mut notes = self.notes.lock().unwrap();
        notes.add(title.to_string());
        Notes::create_note_file(title, content).unwrap();
    }

    fn delete_note(&mut self, title: &str) {
        let mut notes = self.notes.lock().unwrap();
        notes.items.retain(|note| note != title);
        Notes::delete_note_file(title).unwrap();
    }

    fn create_todo(&mut self, description: &str, due_date: Option<i64>) {
        let mut todos = self.todos.lock().unwrap();
        todos.add(description.to_string(), due_date);
        todos.save_to_file().unwrap();
    }

    fn delete_todo(&mut self, index: usize) {
        let mut todos = self.todos.lock().unwrap();
        if index < todos.items.len() {
            todos.items.remove(index);
            todos.save_to_file().unwrap();
        }
    }

    fn save_active_note_to_disk(&self) {
        if let Some(selected_note) = &self.selected_note {
            if let Some(content) = Notes::read_note_file(selected_note).ok() {
                Notes::update_note_file(selected_note, &content).unwrap();
            }
        }
    }
}

impl eframe::App for TemplateApp {
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Periodically save the active note to disk
        ctx.request_repaint_after(std::time::Duration::from_secs(10));
        self.save_active_note_to_disk();

        TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                let is_web = cfg!(target_arch = "wasm32");
                if !is_web {
                    ui.menu_button("File", |ui| {
                        if ui.button("Quit").clicked() {
                            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                    });
                    ui.add_space(16.0);
                }
                egui::widgets::global_dark_light_mode_buttons(ui);
            });
        });

        SidePanel::left("left_panel").show(ctx, |ui| {
            ui.heading("Notes");
            let notes = self.notes.lock().unwrap();
            for note in &notes.items {
                if ui.button(note).clicked() {
                    self.selected_note = Some(note.clone());
                }
            }
            if ui.button("Create Note").clicked() {
                self.create_note("New Note", "This is a new note.");
            }
            if let Some(selected_note) = &self.selected_note {
                if ui.button("Delete Note").clicked() {
                    self.delete_note(selected_note);
                    self.selected_note = None;
                }
            }
        });

        SidePanel::right("right_panel").show(ctx, |ui| {
            ui.heading("Todos");
            let todos = self.todos.lock().unwrap();
            for (index, todo) in todos.items.iter().enumerate() {
                ui.horizontal(|ui| {
                    ui.label(&todo.description);
                    if ui.button("Delete").clicked() {
                        self.delete_todo(index);
                    }
                });
            }
            if ui.button("Create Todo").clicked() {
                self.create_todo("New Todo", None);
            }
        });

        CentralPanel::default().show(ctx, |ui| {
            if let Some(selected_note) = &self.selected_note {
                if let Some(mut content) = Notes::read_note_file(selected_note).ok() {
                    ui.text_edit_multiline(&mut content);
                    Notes::update_note_file(selected_note, &content).unwrap();
                }
            } else {
                ui.label("Select a note to edit");
            }
        });

        TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label("Command:");
                ui.text_edit_singleline(&mut self.command_input);
                if ui.button("Enter").clicked() {
                    // Handle command input
                }
            });
        });
    }
}

#[derive(serde::Deserialize, serde::Serialize)]
enum Mode {
    Command,
    CommandInput,
    Edit,
}

fn powered_by_egui_and_eframe(ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 0.0;
        ui.label("Powered by ");
        ui.hyperlink_to("egui", "https://github.com/emilk/egui");
        ui.label(" and ");
        ui.hyperlink_to(
            "eframe",
            "https://github.com/emilk/egui/tree/master/crates/eframe",
        );
        ui.label(".");
    });
}