use clap::Parser;
use minijinja::{context, Environment};
use serde_json::Value;

mod fileio;
use fileio::Config;

/// Renders a namespace using the given template and writes it to a file in the output directory.
fn generate_file(
    ns: &serde_json::Value,
    tpl: &minijinja::Template,
    suffix: &str,
    output_dir: &str,
) {
    let rendered = tpl
        .render(context! { ns => ns })
        .expect("Failed to render template");
    println!(
        "Generate File: {}/{}{}",
        output_dir,
        ns["name"].as_str().unwrap(),
        suffix
    );
    fileio::write(output_dir, ns["name"].as_str().unwrap(), suffix, &rendered);
}

/// Simple code generator using MiniJinja and JSON/YAML input
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
#[command(name = "comgen")]
#[command(about = "Code generator using MiniJinja and JSON/YAML input", long_about = None)]
struct Args {
    /// Template file name (must exist in embedded templates)
    #[arg(short = 'n', long = "namespace", value_name = "NAMESPACE")]
    ns_filter: Option<String>,
    /// Template file name (must exist in embedded templates)
    #[arg(short = 't', long = "template", value_name = "TEMPLATE")]
    template: Option<String>,
    /// Input file (JSON, YAML, or YML)
    #[arg(short = 'i', long = "input", value_name = "FILE")]
    input: String,
    /// Output folder
    #[arg(short = 'o', long = "output", value_name = "FOLDER")]
    output: String,
}

fn main() {
    let args = Args::parse();
    // Use default template if not provided
    let input_path = &args.input;
    let template_folder = &args.template;

    let config: Config = fileio::load_config(template_folder);

    // Read input (JSON or YAML)
    let json_data: Value = fileio::load_input(input_path);

    // Setup Jinja environment
    let mut env = Environment::new();
    env.set_trim_blocks(true);

    // Create a vector to hold template tuples of (template_name, template_content)
    let mut template_content: Vec<(String, String)> = Vec::new();
    for tpl in config.templates.iter() {
        let tpl_str = fileio::load_template(template_folder, &tpl.file);
        template_content.push((tpl.name.clone(), tpl_str.clone()));
    }

    // add templates to environment
    for (name, tpl_str) in template_content.iter() {
        env.add_template(name.as_str(), tpl_str)
            .expect(format!("Failed to add template: {:?}", tpl_str).as_str());
    }
    // get templates from the environment
    let mut templates: Vec<(String, minijinja::Template<'_, '_>)> = Vec::new();
    for tpl in config.templates.iter() {
        if let Some(suffix) = &tpl.suffix {
            let template = env
                .get_template(tpl.name.as_str())
                .expect("Template not found in environment");
            templates.push((suffix.clone(), template));
        }
    }

    // iterate over namespaces in json_data
    for ns in json_data["namespaces"].as_array().unwrap() {
        let namespace = ns["name"].as_str().unwrap();
        // if ns_filter is set, skip non-matching namespaces
        if let Some(filter) = &args.ns_filter {
            if filter != namespace {
                println!("Skipping Namespace: {}", namespace);
                continue;
            }
        }
        println!("Generate files for Namespace: {}", namespace);
        // generate files for each template
        for (suffix, tpl) in templates.iter() {
            generate_file(ns, tpl, suffix, &args.output);
        }
    }
}
