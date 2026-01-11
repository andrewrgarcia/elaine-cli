use std::fs;
use std::path::PathBuf;
use colored::*;

use crate::project::Project;
use crate::state::elaine_dir;
use crate::utils::id::make_sid;

pub fn projects_dir() -> PathBuf {
    elaine_dir().join("projects")
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

    let mut project: Project = serde_yaml::from_str(&contents)
        .unwrap_or_else(|_| panic!("❌ Failed to parse project {}", project_id));

    // 🔥 LAZY SID MIGRATION (THIS WAS MISSING)
    if project.sid.len() < 16 {
        project.sid = make_sid();
        save_project(&project);
    }

    project
}

pub fn load_all_projects() -> Vec<Project> {
    let mut out = Vec::new();

    if let Ok(entries) = fs::read_dir(projects_dir()) {
        for e in entries.flatten() {
            if let Ok(s) = fs::read_to_string(e.path()) {
                if let Ok(mut p) = serde_yaml::from_str::<Project>(&s) {
                    // 🔥 migrate here too
                    if p.sid.len() < 16 {
                        p.sid = make_sid();
                        let _ = fs::write(
                            e.path(),
                            serde_yaml::to_string(&p).unwrap(),
                        );
                    }
                    out.push(p);
                }
            }
        }
    }

    out
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
        let project = Project::new(project_id, make_sid());
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
