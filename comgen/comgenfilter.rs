use clap::Parser;
use std::fs;
use std::io;
use std::path::PathBuf;
use serde_json::Value;

/// Command-line tool to copy filtered files from one directory to another.
#[derive(Parser, Debug)]
#[command(name = "comgenfilter")]
#[command(about = "Copy files from input to output directory, filtering by glob pattern", long_about = None)]
struct Args {
    /// Input directory to copy files from
    #[arg(short = 'i', long = "input", value_name = "INPUT")]
    input: PathBuf,

    /// Output directory to copy files to
    #[arg(short = 'o', long = "output", value_name = "OUTPUT")]
    output: PathBuf,

    /// Group name
    #[arg(short = 'g', long = "group", value_name = "GROUP")]
    group: String,
}

fn main() -> io::Result<()> {
    let args = Args::parse();

    // Read "files.json" from input directory

    let files_json = args.input.join("files.json");

    let input_str = fs::read_to_string(files_json.as_os_str())
        .expect(format!("Failed to read input file: {:?}", &files_json).as_str());

    let json_data: Value = serde_yaml::from_str(&input_str)
        .expect(format!("Failed to parse files.json: {:?}", files_json.display()).as_str());
    
    fs::create_dir_all(args.output.as_path()).expect("Failed to create output directories");

    //print!("Loaded JSON data: {:?}\n", json_data);
    if let Some(xxx) = json_data["files"].as_array() {
        for item in xxx {
            let file = item["file"].as_str().expect(format!("Failed to parse file element in files.json: {:?}", files_json.display()).as_str());
            let group = item["group"].as_str().expect(format!("Failed to parse group element in files.json: {:?}", files_json.display()).as_str());
            if group == args.group {
                let src_path = args.input.join(file);
                let dest_path = args.output.join(file);
                //println!("Copying file: {:?} to {:?}", src_path, dest_path);
                fs::copy(&src_path, &dest_path).expect("Failed to copy file");
            }
        }
    }
    Ok(())
}
