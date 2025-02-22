use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::fs::File;
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};

#[derive(Serialize, Deserialize)]
struct Template {
    name: String,
    folders: Vec<String>,
    files: std::collections::HashMap<String, String>,
}

fn get_template_dir() -> PathBuf {
    if let Ok(path) = env::var("TPS_PATH") {
        return PathBuf::from(path);
    }
    #[cfg(target_os = "windows")]
    let default_path = dirs::data_local_dir().unwrap_or_else(|| PathBuf::from("C:/Users/Public"));
    #[cfg(not(target_os = "windows"))]
    let default_path = dirs::config_dir().unwrap_or_else(|| PathBuf::from("/tmp"));

    let path = default_path.join("projectTemplates");
    fs::create_dir_all(&path).expect("Failed to create template directory");
    path
}

fn read_template(template_file: &str) -> io::Result<Template> {
    let template_dir = get_template_dir();
    let template_path = template_dir.join(template_file);

    let mut file = File::open(&template_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let template: Template =
        serde_json::from_str(&contents).expect("Failed to parse template JSON");
    Ok(template)
}

fn create_project(template: Template, project_name: &str) {
    fs::create_dir_all(project_name).expect("Failed to create project root directory");

    for folder in &template.folders {
        let path = format!("{}/{}", project_name, folder);
        fs::create_dir_all(&path)
            .unwrap_or_else(|err| panic!("Failed to create folder {}: {}", path, err));
    }

    for (file_path, content) in template.files {
        let path = format!("{}/{}", project_name, file_path);
        if let Some(parent) = Path::new(&path).parent() {
            fs::create_dir_all(parent).unwrap_or_else(|err| {
                panic!("Failed to create parent directory for {}: {}", path, err)
            });
        }
        let final_content = content.replace("{{project_name}}", project_name);
        let mut file = File::create(&path)
            .unwrap_or_else(|err| panic!("Failed to create file {}: {}", path, err));
        file.write_all(final_content.as_bytes())
            .unwrap_or_else(|err| panic!("Failed to write file {}: {}", path, err));
    }
    println!("Project '{}' created successfully!", project_name);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: tps <template_file> <project_name>");
        return;
    }
    let template_file = &args[1];
    let project_name = &args[2];

    match read_template(template_file) {
        Ok(template) => create_project(template, project_name),
        Err(err) => eprintln!("Error loading template: {}", err),
    }
}
