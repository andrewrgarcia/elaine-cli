use std::fs;
use std::path::{Path, PathBuf};
use colored::*;
use crate::project::Project;

pub fn projects_dir() -> PathBuf {
    Path::new(".elaine").join("projects")
}

pub fn project_path(project_id: &str) -> PathBuf {
    projects_dir().join(format!("{}.yaml", project_id))
}

pub fn project_exists(project_id: &str) -> bool {
    project_path(project_id).exists()
}

pub fn load_project(project_id: &str) -> Project {
    let path = project_path(project_id);

    let contents = fs::read_to_string(&path)
        .unwrap_or_else(|_| panic!("❌ Failed to read project {}", project_id));

    serde_yaml::from_str(&contents)
        .unwrap_or_else(|_| panic!("❌ Failed to parse project {}", project_id))
}

pub fn save_project(project: &Project) {
    let path = project_path(&project.id);

    let contents = serde_yaml::to_string(project)
        .expect("❌ Failed to serialize project");

    fs::write(&path, contents)
        .expect("❌ Failed to write project file");
}

pub fn create_project_if_missing(project_id: &str) -> Project {
    if project_exists(project_id) {
        load_project(project_id)
    } else {
        let project = Project::new(project_id);
        save_project(&project);
        println!(
            "{}",
            format!("📁 Created project '{}'", project_id)
                .bright_green()
                .bold()
        );
        project
    }
}
