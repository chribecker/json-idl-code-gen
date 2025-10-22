// Utility functions for template handling

#![allow(dead_code)]

use rust_embed::RustEmbed;
use serde_derive::Deserialize;
use serde_json::Value;
use std::fs;
use std::path::Path;

#[derive(Debug, Deserialize)]
pub struct Template {
    pub file: String,
    pub name: String,
    pub suffix: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub templates: Vec<Template>,
}

/*
 * Embedded templates using rust-embed
 */
#[derive(RustEmbed)]
#[folder = "templates/"]
struct EmbeddedTemplates;

/// Load from external file if template folder is provided, otherwise load from embedded assets.
pub fn load_template(template_folder: &Option<String>, name: &str) -> String {
    if let Some(folder) = template_folder {
        load_external(folder, name)
    } else {
        load_embedded(name)
    }
}

/// Loads an embedded template by name.
pub fn load_embedded(name: &str) -> String {
    let data = EmbeddedTemplates::get(name)
        .expect(format!("Template not found in embedded assets: {:?}", name).as_str());
    std::str::from_utf8(data.data.as_ref())
        .expect("Template is not valid UTF-8")
        .to_string()
}

/// Loads an external template from a specified directory.
pub fn load_external(folder: &str, name: &str) -> String {
    let path = Path::new(folder).join(name);
    fs::read_to_string(path).expect("Failed to read template file")
}

pub fn load_config(template_folder: &Option<String>) -> Config {
    let config_str = load_template(template_folder, "config.yaml");
    let config: Config = serde_yaml::from_str(&config_str).expect("Failed to parse config.yaml");
    config
}

pub fn load_input(input_path: &str) -> Value {
    let input_str = fs::read_to_string(input_path).expect("Failed to read input file");

    let ext = Path::new(input_path)
        .extension()
        .and_then(|s| s.to_str())
        .unwrap_or("")
        .to_ascii_lowercase();

    print!("Input file extension: {}\n", ext);

    let json_data: Value = match ext.as_str() {
        "json" => serde_json::from_str(&input_str).expect("Failed to parse JSON"),
        "yaml" | "yml" => serde_yaml::from_str(&input_str).expect("Failed to parse YAML"),
        _ => {
            eprintln!(
                "Unsupported input file extension: {}. Use .json, .yaml, or .yml",
                ext
            );
            std::process::exit(1);
        }
    };
    json_data
}

/// Writes content to a file in the specified directory with the given name and extension.
pub fn write(dir: &str, name: &str, suffix: &str, content: &str) {
    fs::create_dir_all(dir).expect(format!("Failed to create output directory: {}", dir).as_str());

    let file_path = std::path::Path::new(dir).join(format!("{}{}", name, suffix));
    let content = trim_last_linebreak(content.to_string());
    fs::write(&file_path, content)
        .expect(format!("Failed to write output file: {:?}", file_path).as_str());
}

fn trim_last_linebreak(mut s: String) -> String {
    if s.ends_with('\n') {
        s.pop();
        if s.ends_with('\r') {
            s.pop();
        }
    }
    s
}
