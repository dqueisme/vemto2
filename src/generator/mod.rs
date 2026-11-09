pub mod controller;
pub mod factory;
pub mod migration;
pub mod model;

use crate::models::Project;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GeneratedFile {
    pub relative_path: PathBuf,
    pub content: String,
}

pub struct CodeGenerator;

impl CodeGenerator {
    pub fn generate_all(project: &Project) -> Vec<GeneratedFile> {
        let mut files = Vec::new();

        // 1. Migrations
        let now = chrono::Utc::now();
        for (i, table) in project.tables.iter().enumerate() {
            let timestamp = (now + chrono::Duration::seconds(i as i64))
                .format("%Y_%m_%d_%H%M%S")
                .to_string();
            let (filename, content) = migration::MigrationGenerator::generate_create_table(table, &timestamp);
            files.push(GeneratedFile {
                relative_path: Path::new("database").join("migrations").join(filename),
                content,
            });
        }

        // 2. Models
        for table in &project.tables {
            let (filename, content) = model::ModelGenerator::generate(table, project);
            files.push(GeneratedFile {
                relative_path: Path::new("app").join("Models").join(filename),
                content,
            });
        }

        // 3. Factories
        for table in &project.tables {
            let (filename, content) = factory::FactoryGenerator::generate(table, project);
            files.push(GeneratedFile {
                relative_path: Path::new("database").join("factories").join(filename),
                content,
            });
        }

        // 4. Controllers
        for table in &project.tables {
            let (filename, content) = controller::ControllerGenerator::generate_crud(table, project);
            files.push(GeneratedFile {
                relative_path: Path::new("app").join("Http").join("Controllers").join(filename),
                content,
            });
        }

        files
    }

    pub fn write_to_directory(project: &Project, base_dir: &Path) -> std::io::Result<usize> {
        let files = Self::generate_all(project);
        let count = files.len();

        for file in files {
            let full_path = base_dir.join(file.relative_path);
            if let Some(parent) = full_path.parent() {
                std::fs::create_dir_all(parent)?;
            }
            std::fs::write(full_path, file.content)?;
        }

        Ok(count)
    }
}

#[cfg(test)]
mod tests;
