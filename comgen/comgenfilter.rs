use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use clap::{Parser};
use glob::Pattern;

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

    /// Glob pattern to filter files (e.g. "*.json")
    #[arg(short = 'f', long = "filter", value_name = "FILTER")]
    filter: String,
}

fn copy_filtered_files(input: &Path, output: &Path, pattern: &Pattern) -> io::Result<()> {
    if !output.exists() {
        fs::create_dir_all(output)?;
    }

    for entry in fs::read_dir(input)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() {
            if let Some(file_name) = path.file_name().and_then(|n| n.to_str()) {
                if pattern.matches(file_name) {
                    let dest = output.join(file_name);
                    fs::copy(&path, &dest)?;
                }
            }
        }
    }
    Ok(())
}

fn main() -> io::Result<()> {
    let args = Args::parse();

    let pattern = Pattern::new(&args.filter)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e))?;

    copy_filtered_files(&args.input, &args.output, &pattern)?;

    Ok(())
}
