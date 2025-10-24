use clap::Parser;
use std::fs::{self, File};
use std::io::{self, Write};
use std::path::Path;

/// Simple program to print and save file tree
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Target name for the debug output
    #[arg(short = 'n', long = "name", value_name = "NAME")]
    name: String,

    /// Input directory to print the file tree from
    #[arg(short = 'i', long = "input", value_name = "INPUT")]
    input: String,

    /// Output file to save the file tree
    #[arg(short = 'o', long = "output", value_name = "OUTPUT")]
    output: String,
}

fn main() -> io::Result<()> {
    let args = Args::parse();
    let input_path = Path::new(&args.input);
    
    fn visit_dirs(dir: &Path, cb: &mut dyn FnMut(&Path)) -> io::Result<()> {
        if dir.is_dir() {
            for entry in fs::read_dir(dir)? {
                let entry = entry?;
                let path = entry.path();
                cb(&path);
                if path.is_dir() {
                    visit_dirs(&path, cb)?;
                }
            }
        }
        Ok(())
    }
    
    let mut output_file = File::create(&args.output)?;
    writeln!(output_file, "File tree for target: {}", args.name)?;
    println!("File tree for target: {} on {}", args.name, input_path.display());
    
    let mut print_and_write = |path: &Path| {
        if let Some(p) = path.file_name().and_then(|n| n.to_str()) {
            println!(" - {}", p);
            writeln!(output_file, " - {}", p).unwrap();
        }
    };

    visit_dirs(input_path, &mut print_and_write)?;
    Ok(())
}
