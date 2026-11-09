use crate::generator::{CodeGenerator, GeneratedFile};
use crate::models::{Column, ColumnType, Project, RelationType, Relationship, Table};
use eframe::egui;

#[derive(PartialEq, Eq)]
pub enum ActiveTab {
    SchemaEditor,
    CodePreview,
    ProjectSettings,
}

pub struct VemtoApp {
    pub project: Project,
    pub active_tab: ActiveTab,
    pub selected_table_index: usize,
    pub generated_files: Vec<GeneratedFile>,
    pub selected_file_index: usize,
    pub export_status_message: Option<String>,

    // Form inputs for creating a new table
    pub new_table_name: String,
    pub new_model_name: String,

    // Form inputs for creating a new column
    pub new_col_name: String,
    pub new_col_type: ColumnType,
    pub new_col_nullable: bool,
    pub new_col_unique: bool,
    pub new_col_default: String,

    // Form inputs for creating a relationship
    pub new_rel_name: String,
    pub new_rel_type: RelationType,
    pub new_rel_target: String,
}

impl Default for VemtoApp {
    fn default() -> Self {
        let project = Project::default_laravel_project();
        let generated_files = CodeGenerator::generate_all(&project);

        Self {
            project,
            active_tab: ActiveTab::SchemaEditor,
            selected_table_index: 0,
            generated_files,
            selected_file_index: 0,
            export_status_message: None,

            new_table_name: String::new(),
            new_model_name: String::new(),

            new_col_name: String::new(),
            new_col_type: ColumnType::String,
            new_col_nullable: false,
            new_col_unique: false,
            new_col_default: String::new(),

            new_rel_name: String::new(),
            new_rel_type: RelationType::BelongsTo,
            new_rel_target: String::new(),
        }
    }
}

impl VemtoApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }

    pub fn refresh_generated_code(&mut self) {
        self.generated_files = CodeGenerator::generate_all(&self.project);
        if self.selected_file_index >= self.generated_files.len() {
            self.selected_file_index = 0;
        }
    }
}

impl eframe::App for VemtoApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Top Menu Bar
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.heading("༄ Vemto (Native Rust)");
                ui.separator();

                if ui.button("Save Project").clicked() {
                    if let Some(path) = rfd::FileDialog::new()
                        .set_file_name("vemto.json")
                        .add_filter("Vemto Project", &["json"])
                        .save_file()
                    {
                        match self.project.save_to_file(&path) {
                            Ok(_) => {
                                self.export_status_message =
                                    Some(format!("Project saved to {:?}", path.file_name().unwrap()));
                            }
                            Err(e) => {
                                self.export_status_message = Some(format!("Error saving project: {}", e));
                            }
                        }
                    }
                }

                if ui.button("Open Project").clicked() {
                    if let Some(path) = rfd::FileDialog::new()
                        .add_filter("Vemto Project", &["json"])
                        .pick_file()
                    {
                        match Project::load_from_file(&path) {
                            Ok(loaded_project) => {
                                self.project = loaded_project;
                                self.selected_table_index = 0;
                                self.refresh_generated_code();
                                self.export_status_message =
                                    Some(format!("Project loaded from {:?}", path.file_name().unwrap()));
                            }
                            Err(e) => {
                                self.export_status_message = Some(format!("Error loading project: {}", e));
                            }
                        }
                    }
                }

                if ui.button("Generate & Export Code").clicked() {
                    if let Some(folder) = rfd::FileDialog::new().pick_folder() {
                        match CodeGenerator::write_to_directory(&self.project, &folder) {
                            Ok(count) => {
                                self.export_status_message =
                                    Some(format!("Successfully generated {} Laravel files to {:?}", count, folder));
                            }
                            Err(e) => {
                                self.export_status_message = Some(format!("Error exporting code: {}", e));
                            }
                        }
                    }
                }

                ui.separator();

                ui.selectable_value(&mut self.active_tab, ActiveTab::SchemaEditor, "Schema Editor");
                ui.selectable_value(&mut self.active_tab, ActiveTab::CodePreview, "Code Preview");
                ui.selectable_value(&mut self.active_tab, ActiveTab::ProjectSettings, "Settings");
            });
        });

        // Bottom Status Bar
        egui::TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                if let Some(ref msg) = self.export_status_message {
                    ui.label(msg);
                } else {
                    ui.label(format!(
                        "Project: {} | Tables: {} | Generated Files: {}",
                        self.project.name,
                        self.project.tables.len(),
                        self.generated_files.len()
                    ));
                }
            });
        });

        // Main Central View
        egui::CentralPanel::default().show(ctx, |ui| match self.active_tab {
            ActiveTab::SchemaEditor => self.show_schema_editor(ui),
            ActiveTab::CodePreview => self.show_code_preview(ui),
            ActiveTab::ProjectSettings => self.show_project_settings(ui),
        });
    }
}

impl VemtoApp {
    fn show_schema_editor(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            // Left sidebar: Tables list
            egui::SidePanel::left("tables_panel")
                .resizable(false)
                .default_width(220.0)
                .show_inside(ui, |ui| {
                    ui.heading("Tables");
                    ui.separator();

                    let mut table_to_remove = None;

                    egui::ScrollArea::vertical().show(ui, |ui| {
                        for (idx, table) in self.project.tables.iter().enumerate() {
                            ui.horizontal(|ui| {
                                let is_selected = self.selected_table_index == idx;
                                if ui
                                    .selectable_label(is_selected, format!("{} ({})", table.name, table.model_name))
                                    .clicked()
                                {
                                    self.selected_table_index = idx;
                                }

                                if ui.small_button("✕").clicked() {
                                    table_to_remove = Some(idx);
                                }
                            });
                        }
                    });

                    if let Some(idx) = table_to_remove {
                        if self.project.tables.len() > 1 {
                            self.project.tables.remove(idx);
                            if self.selected_table_index >= self.project.tables.len() {
                                self.selected_table_index = self.project.tables.len().saturating_sub(1);
                            }
                            self.refresh_generated_code();
                        }
                    }

                    ui.separator();
                    ui.label("Add New Table:");
                    ui.horizontal(|ui| {
                        ui.label("Table:");
                        ui.text_edit_singleline(&mut self.new_table_name);
                    });
                    ui.horizontal(|ui| {
                        ui.label("Model:");
                        ui.text_edit_singleline(&mut self.new_model_name);
                    });

                    if ui.button("+ Add Table").clicked() {
                        if !self.new_table_name.is_empty() {
                            let model_name = if self.new_model_name.is_empty() {
                                heck::ToPascalCase::to_pascal_case(self.new_table_name.as_str())
                            } else {
                                self.new_model_name.clone()
                            };

                            let new_table = Table::new(&self.new_table_name, model_name);
                            self.project.tables.push(new_table);
                            self.selected_table_index = self.project.tables.len() - 1;
                            self.new_table_name.clear();
                            self.new_model_name.clear();
                            self.refresh_generated_code();
                        }
                    }
                });

            // Right area: Selected Table Details
            let mut should_refresh_code = false;
            let mut new_col_to_add = None;
            let mut new_rel_to_add = None;

            if let Some(table) = self.project.tables.get_mut(self.selected_table_index) {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    ui.heading(format!("Table: {}", table.name));
                    ui.separator();

                    ui.horizontal(|ui| {
                        ui.label("Model Name:");
                        ui.text_edit_singleline(&mut table.model_name);

                        ui.checkbox(&mut table.timestamps, "Timestamps");
                        ui.checkbox(&mut table.soft_deletes, "Soft Deletes");
                    });

                    ui.add_space(10.0);
                    ui.heading("Columns");

                    let mut col_to_remove = None;

                    egui::Grid::new("columns_grid")
                        .striped(true)
                        .min_col_width(100.0)
                        .show(ui, |ui| {
                            ui.strong("Name");
                            ui.strong("Type");
                            ui.strong("Nullable");
                            ui.strong("Unique");
                            ui.strong("Default");
                            ui.strong("Action");
                            ui.end_row();

                            for (c_idx, col) in table.columns.iter_mut().enumerate() {
                                ui.text_edit_singleline(&mut col.name);

                                egui::ComboBox::from_id_salt(format!("col_type_{}", c_idx))
                                    .selected_text(format!("{:?}", col.col_type))
                                    .show_ui(ui, |ui| {
                                        for t in ColumnType::all() {
                                            ui.selectable_value(&mut col.col_type, t.clone(), format!("{:?}", t));
                                        }
                                    });

                                ui.checkbox(&mut col.nullable, "");
                                ui.checkbox(&mut col.unique, "");

                                let mut def_text = col.default_value.clone().unwrap_or_default();
                                if ui.text_edit_singleline(&mut def_text).changed() {
                                    col.default_value = if def_text.is_empty() {
                                        None
                                    } else {
                                        Some(def_text)
                                    };
                                }

                                if ui.small_button("Delete").clicked() {
                                    col_to_remove = Some(c_idx);
                                }
                                ui.end_row();
                            }
                        });

                    if let Some(c_idx) = col_to_remove {
                        table.columns.remove(c_idx);
                    }

                    ui.add_space(8.0);
                    ui.horizontal(|ui| {
                        ui.label("New Column:");
                        ui.text_edit_singleline(&mut self.new_col_name);

                        egui::ComboBox::from_id_salt("new_col_type_select")
                            .selected_text(format!("{:?}", self.new_col_type))
                            .show_ui(ui, |ui| {
                                for t in ColumnType::all() {
                                    ui.selectable_value(&mut self.new_col_type, t.clone(), format!("{:?}", t));
                                }
                            });

                        ui.checkbox(&mut self.new_col_nullable, "Nullable");
                        ui.checkbox(&mut self.new_col_unique, "Unique");

                        if ui.button("+ Add Column").clicked() && !self.new_col_name.is_empty() {
                            let mut col = Column::new(&self.new_col_name, self.new_col_type.clone());
                            col.nullable = self.new_col_nullable;
                            col.unique = self.new_col_unique;
                            if !self.new_col_default.is_empty() {
                                col.default_value = Some(self.new_col_default.clone());
                            }
                            new_col_to_add = Some(col);
                            self.new_col_name.clear();
                            self.new_col_default.clear();
                        }
                    });

                    ui.add_space(15.0);
                    ui.heading("Relationships");

                    let mut rel_to_remove = None;

                    for (r_idx, rel) in table.relationships.iter_mut().enumerate() {
                        ui.horizontal(|ui| {
                            ui.label(format!("• {} -> {:?} {}", rel.name, rel.rel_type, rel.target_model));
                            if ui.small_button("Delete").clicked() {
                                rel_to_remove = Some(r_idx);
                            }
                        });
                    }

                    if let Some(r_idx) = rel_to_remove {
                        table.relationships.remove(r_idx);
                    }

                    ui.add_space(8.0);
                    ui.horizontal(|ui| {
                        ui.label("Name:");
                        ui.text_edit_singleline(&mut self.new_rel_name);

                        egui::ComboBox::from_id_salt("new_rel_type_select")
                            .selected_text(format!("{:?}", self.new_rel_type))
                            .show_ui(ui, |ui| {
                                ui.selectable_value(&mut self.new_rel_type, RelationType::BelongsTo, "BelongsTo");
                                ui.selectable_value(&mut self.new_rel_type, RelationType::HasMany, "HasMany");
                                ui.selectable_value(&mut self.new_rel_type, RelationType::HasOne, "HasOne");
                                ui.selectable_value(&mut self.new_rel_type, RelationType::BelongsToMany, "BelongsToMany");
                            });

                        ui.label("Target Model:");
                        ui.text_edit_singleline(&mut self.new_rel_target);

                        if ui.button("+ Add Relationship").clicked()
                            && !self.new_rel_name.is_empty()
                            && !self.new_rel_target.is_empty()
                        {
                            new_rel_to_add = Some(Relationship {
                                name: self.new_rel_name.clone(),
                                rel_type: self.new_rel_type.clone(),
                                target_model: self.new_rel_target.clone(),
                                foreign_key: None,
                                local_key: None,
                            });
                            self.new_rel_name.clear();
                            self.new_rel_target.clear();
                        }
                    });

                    ui.add_space(15.0);
                    if ui.button("Apply Changes & Update Generated Code").clicked() {
                        should_refresh_code = true;
                    }
                });
            }

            if let Some(col) = new_col_to_add {
                if let Some(table) = self.project.tables.get_mut(self.selected_table_index) {
                    table.columns.push(col);
                }
            }

            if let Some(rel) = new_rel_to_add {
                if let Some(table) = self.project.tables.get_mut(self.selected_table_index) {
                    table.relationships.push(rel);
                }
            }

            if should_refresh_code {
                self.refresh_generated_code();
                self.export_status_message = Some("Code preview refreshed!".to_string());
            }
        });
    }

    fn show_code_preview(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            // Left sidebar: file list
            egui::SidePanel::left("files_panel")
                .resizable(false)
                .default_width(280.0)
                .show_inside(ui, |ui| {
                    ui.heading("Generated Files");
                    ui.separator();

                    egui::ScrollArea::vertical().show(ui, |ui| {
                        for (idx, file) in self.generated_files.iter().enumerate() {
                            let label = file.relative_path.to_string_lossy();
                            let is_selected = self.selected_file_index == idx;
                            if ui.selectable_label(is_selected, label).clicked() {
                                self.selected_file_index = idx;
                            }
                        }
                    });
                });

            // Right area: file content preview
            egui::CentralPanel::default().show_inside(ui, |ui| {
                if let Some(file) = self.generated_files.get(self.selected_file_index) {
                    ui.heading(file.relative_path.to_string_lossy());
                    ui.separator();

                    egui::ScrollArea::both().show(ui, |ui| {
                        let mut text = file.content.as_str();
                        ui.add(
                            egui::TextEdit::multiline(&mut text)
                                .font(egui::TextStyle::Monospace)
                                .desired_width(f32::INFINITY)
                                .interactive(false),
                        );
                    });
                }
            });
        });
    }

    fn show_project_settings(&mut self, ui: &mut egui::Ui) {
        ui.heading("Project Settings");
        ui.separator();

        ui.horizontal(|ui| {
            ui.label("Project Name: ");
            ui.text_edit_singleline(&mut self.project.name);
        });

        ui.horizontal(|ui| {
            ui.label("Root Namespace: ");
            ui.text_edit_singleline(&mut self.project.namespace);
        });

        ui.add_space(10.0);
        if ui.button("Save Settings").clicked() {
            self.refresh_generated_code();
            self.export_status_message = Some("Project settings saved!".to_string());
        }
    }
}
