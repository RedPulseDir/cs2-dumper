use eframe::egui;
use parking_lot::Mutex;
use std::sync::Arc;
use std::thread;

#[derive(Clone, Debug)]
pub struct ProcessInfo {
    pub name: String,
    pub pid: u32,
}

pub struct ProcessSelector {
    processes: Arc<Mutex<Vec<ProcessInfo>>>,
    selected_process: Option<String>,
    search_query: String,
    loading: bool,
}

impl ProcessSelector {
    pub fn new() -> Self {
        let mut selector = Self {
            processes: Arc::new(Mutex::new(Vec::new())),
            selected_process: None,
            search_query: String::new(),
            loading: false,
        };

        selector.refresh_processes();
        selector
    }

    pub fn refresh_processes(&mut self) {
        if self.loading {
            return;
        }

        self.loading = true;
        let processes = self.processes.clone();

        thread::spawn(move || {
            let mut proc_list = Vec::new();

            #[cfg(windows)]
            {
                use memflow::prelude::v1::*;

                if let Ok(os) = memflow_native::create_os(&OsArgs::default(), LibArc::default()) {
                    if let Ok(list) = os.process_info_list() {
                        for info in list {
                            proc_list.push(ProcessInfo {
                                name: info.name.to_string(),
                                pid: info.pid.0 as u32,
                            });
                        }
                    }
                }
            }

            proc_list.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));

            *processes.lock() = proc_list;
        });

        self.loading = false;
    }

    pub fn ui(&mut self, ui: &mut egui::Ui) -> Option<String> {
        ui.vertical(|ui| {
            ui.add_space(10.0);

            ui.horizontal(|ui| {
                ui.label(super::theme::subheading("Process Selection"));
                ui.add_space(10.0);

                if ui.button("🔄 Refresh").clicked() {
                    self.refresh_processes();
                }
            });

            ui.add_space(10.0);

            ui.horizontal(|ui| {
                ui.label("🔍 Search:");
                ui.text_edit_singleline(&mut self.search_query);
            });

            ui.add_space(10.0);

            let processes = self.processes.lock();
            let cs2_exists = processes.iter().any(|p| p.name.to_lowercase() == "cs2.exe");

            if cs2_exists && self.selected_process.is_none() {
                self.selected_process = Some("cs2.exe".to_string());
            }

            if !cs2_exists {
                ui.colored_label(
                    egui::Color32::from_rgb(255, 200, 100),
                    "⚠ cs2.exe not found - please select process manually",
                );
                ui.add_space(5.0);
            }

            egui::ScrollArea::vertical()
                .max_height(300.0)
                .show(ui, |ui| {
                    for proc in processes.iter() {
                        if !self.search_query.is_empty()
                            && !proc
                                .name
                                .to_lowercase()
                                .contains(&self.search_query.to_lowercase())
                        {
                            continue;
                        }

                        let is_selected = self
                            .selected_process
                            .as_ref()
                            .map(|s| s == &proc.name)
                            .unwrap_or(false);

                        if ui
                            .selectable_label(
                                is_selected,
                                format!("{} (PID: {})", proc.name, proc.pid),
                            )
                            .clicked()
                        {
                            self.selected_process = Some(proc.name.clone());
                        }
                    }
                });

            ui.add_space(10.0);

            if let Some(selected) = &self.selected_process {
                ui.label(super::theme::success_text(&format!(
                    "✓ Selected: {}",
                    selected
                )));
            }
        });

        self.selected_process.clone()
    }

    pub fn get_selected(&self) -> Option<String> {
        self.selected_process.clone()
    }
}
