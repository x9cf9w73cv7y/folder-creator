use eframe::{egui, App, Frame, NativeOptions};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Default)]
struct Config {
    base_path: String,
}

struct FolderCreatorApp {
    folder_name: String,
    base_path: String,
    status_message: String,
    status_color: egui::Color32,
    show_options: bool,
    config_path: Option<PathBuf>,
}

impl FolderCreatorApp {
    fn config_path() -> Option<PathBuf> {
        dirs::config_dir().map(|p| p.join("folder_creator").join("config.json"))
    }

    fn load_config() -> Config {
        if let Some(path) = Self::config_path() {
            if let Ok(contents) = fs::read_to_string(&path) {
                if let Ok(config) = serde_json::from_str::<Config>(&contents) {
                    return config;
                }
            }
        }
        Config::default()
    }

    fn save_config(&self) {
        if let Some(ref path) = self.config_path {
            if let Some(parent) = path.parent() {
                let _ = fs::create_dir_all(parent);
            }
            let config = Config {
                base_path: self.base_path.clone(),
            };
            if let Ok(json) = serde_json::to_string_pretty(&config) {
                let _ = fs::write(path, json);
            }
        }
    }
}

impl Default for FolderCreatorApp {
    fn default() -> Self {
        let config = Self::load_config();
        Self {
            folder_name: String::new(),
            base_path: if config.base_path.is_empty() {
                dirs::home_dir()
                    .map(|p| p.to_string_lossy().to_string())
                    .unwrap_or_else(|| String::from("."))
            } else {
                config.base_path
            },
            status_message: String::new(),
            status_color: egui::Color32::WHITE,
            show_options: false,
            config_path: Self::config_path(),
        }
    }
}

impl FolderCreatorApp {
    fn create_folder_structure(&mut self) {
        if self.folder_name.trim().is_empty() {
            self.status_message = "Bitte geben Sie einen Ordnernamen ein!".to_string();
            self.status_color = egui::Color32::RED;
            return;
        }

        let base = PathBuf::from(&self.base_path);
        let main_folder = base.join(&self.folder_name);
        let drive_c_folder = main_folder.join("drive_c");

        match fs::create_dir_all(&drive_c_folder) {
            Ok(_) => {
                self.status_message = format!(
                    "Erfolg! Ordner erstellt:\n{}\n{}\n",
                    main_folder.display(),
                    drive_c_folder.display()
                );
                self.status_color = egui::Color32::GREEN;
                self.folder_name.clear();
            }
            Err(e) => {
                self.status_message = format!("Fehler: {}", e);
                self.status_color = egui::Color32::RED;
            }
        }
    }
}

impl App for FolderCreatorApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Ordner Ersteller");
            ui.add_space(20.0);

            // Optionsbereich
            ui.collapsing("⚙️ Optionen", |ui| {
                ui.horizontal(|ui| {
                    ui.label("Basispfad:");
                    let old_path = self.base_path.clone();
                    ui.text_edit_singleline(&mut self.base_path);
                    if self.base_path != old_path {
                        self.save_config();
                    }
                    if ui.button("📁").clicked() {
                        if let Some(path) = rfd::FileDialog::new().pick_folder() {
                            self.base_path = path.to_string_lossy().to_string();
                            self.save_config();
                        }
                    }
                });
            });

            ui.add_space(30.0);

            // Hauptbereich - zentriert und prominent
            ui.vertical_centered(|ui| {
                ui.heading("📁 Neuer Ordner");
                ui.add_space(20.0);
                
                // Großes Eingabefeld für den Ordnernamen
                let text_edit = egui::TextEdit::multiline(&mut self.folder_name)
                    .hint_text("Ordnername eingeben...")
                    .desired_width(400.0)
                    .min_size(egui::vec2(400.0, 60.0))
                    .font(egui::TextStyle::Heading)
                    .margin(egui::vec2(12.0, 12.0));
                
                let response = ui.add(text_edit);
                
                // Enter-Taste zum Bestätigen
                if response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                    self.create_folder_structure();
                }
                
                ui.add_space(20.0);
                
                // Großer Button
                let button = egui::Button::new("✅ Ordner erstellen")
                    .min_size(egui::vec2(200.0, 50.0));
                if ui.add(button).clicked() {
                    self.create_folder_structure();
                }
            });

            ui.add_space(30.0);

            // Statusmeldung
            if !self.status_message.is_empty() {
                ui.vertical_centered(|ui| {
                    ui.colored_label(self.status_color, &self.status_message);
                });
            }

            ui.add_space(20.0);
            ui.separator();
            ui.label(format!("Basispfad: {}", self.base_path));
        });
    }
}

fn main() -> eframe::Result<()> {
    let options = NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([500.0, 400.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Ordner Ersteller",
        options,
        Box::new(|_cc| Ok(Box::new(FolderCreatorApp::default()))),
    )
}
