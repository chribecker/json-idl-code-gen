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
    // verbose
    #[arg(short = 'v', long = "verbose")]
    verbose: Option<bool>,
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
    let mut templates: Vec<(String, Option<String>, minijinja::Template<'_, '_>)> = Vec::new();
    for tpl in config.templates.iter() {
        if let Some(suffix) = &tpl.suffix {
            let template = env
                .get_template(tpl.name.as_str())
                .expect("Template not found in environment");
            templates.push((suffix.clone(), tpl.group.clone(), template));
        }
    }

    // create a json output file which contains the info about generated files
    let mut output_info = serde_json::json!({
        "files": [],
    });

    let output_info_vec = output_info["files"].as_array_mut().unwrap();

    for ns in json_data["namespaces"].as_array().unwrap() {
        let namespace = ns["name"].as_str().unwrap();
        // if ns_filter is set, skip non-matching namespaces
        if let Some(filter) = &args.ns_filter {
            if filter.as_str() != namespace {
                println!(
                    "Skipping Namespace: '{}' filtered out by '{}'",
                    namespace,
                    filter.as_str()
                );
                continue;
            }
        }
        
        if args.verbose.is_some() {
            println!("Generate files for Namespace: '{}'", namespace);
        }

        // generate files for each template
        for (suffix, group, tpl) in templates.iter() {
            if group.is_some() {
                output_info_vec.push(serde_json::json!({
                    "file": format!("{}{}", ns["name"].as_str().unwrap(), suffix),
                    "namespace": format!("{}",  namespace),
                    "template": format!("{}",  tpl.name()),
                    "group": group
                }));
            }
            generate_file(ns, tpl, suffix, &args.output);

            if args.verbose.is_some() {
                println!(
                    "Generate File: {}/{}{}",
                    &args.output,
                    ns["name"].as_str().unwrap(),
                    suffix
                );
            }
        }
    }
    let pretty = serde_json::to_string_pretty(&output_info).unwrap();
    fileio::write(&args.output, "files", ".json", &pretty);
}
