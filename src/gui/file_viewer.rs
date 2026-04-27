use eframe::egui;
use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

pub struct FileViewer {
    output_dir: PathBuf,
    files: BTreeMap<String, String>,
    selected_file: Option<String>,
}

impl FileViewer {
    pub fn new(output_dir: PathBuf) -> Self {
        Self {
            output_dir,
            files: BTreeMap::new(),
            selected_file: None,
        }
    }

    pub fn refresh(&mut self) {
        self.files.clear();

        if let Ok(entries) = fs::read_dir(&self.output_dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_file() {
                    if let Some(name) = path.file_name() {
                        let name_str = name.to_string_lossy().to_string();
                        if let Ok(content) = fs::read_to_string(&path) {
                            self.files.insert(name_str, content);
                        }
                    }
                }
            }
        }

        if self.selected_file.is_none() && !self.files.is_empty() {
            self.selected_file = self.files.keys().next().cloned();
        }
    }

    pub fn ui(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.vertical(|ui| {
                ui.set_width(250.0);
                ui.label(super::theme::subheading("Files"));
                ui.separator();

                egui::ScrollArea::vertical().show(ui, |ui| {
                    for file_name in self.files.keys() {
                        let is_selected = self
                            .selected_file
                            .as_ref()
                            .map(|s| s == file_name)
                            .unwrap_or(false);

                        let icon = if file_name.ends_with(".json") {
                            "📄"
                        } else if file_name.ends_with(".cs") {
                            "📘"
                        } else if file_name.ends_with(".hpp") {
                            "📗"
                        } else if file_name.ends_with(".rs") {
                            "🦀"
                        } else if file_name.ends_with(".zig") {
                            "⚡"
                        } else {
                            "📋"
                        };

                        if ui
                            .selectable_label(is_selected, format!("{} {}", icon, file_name))
                            .clicked()
                        {
                            self.selected_file = Some(file_name.clone());
                        }
                    }
                });
            });

            ui.separator();

            ui.vertical(|ui| {
                if let Some(selected) = &self.selected_file {
                    ui.label(super::theme::subheading(selected));
                    ui.separator();

                    if let Some(content) = self.files.get(selected) {
                        egui::ScrollArea::both().show(ui, |ui| {
                            ui.add(
                                egui::TextEdit::multiline(&mut content.as_str())
                                    .font(egui::TextStyle::Monospace)
                                    .code_editor()
                                    .desired_width(f32::INFINITY),
                            );
                        });
                    }
                } else {
                    ui.centered_and_justified(|ui| {
                        ui.label(super::theme::body("No file selected"));
                    });
                }
            });
        });
    }
}
