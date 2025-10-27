use clap::Parser;
use std::fs::{self, File};
use std::io::{self, Write};
use std::path::Path;

/// Simple program to print and save file tree
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Input directory to print the file tree from
    #[arg(short = 'i', long = "input", num_args = 1.., value_name = "INPUT")]
    inputs: Vec<String>,

    /// Output file to save the file tree
    #[arg(short = 'o', long = "output", value_name = "OUTPUT")]
    output: String,
}

fn main() -> io::Result<()> {
    let args = Args::parse();
    let input_paths = args.inputs.iter().map(Path::new).collect::<Vec<_>>();
    let mut output_file = File::create(&args.output)?;
    for input_path in input_paths {
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

        println!("  {}", input_path.display());
        writeln!(output_file,"{}", input_path.display()).unwrap();
        let mut print_and_write = |path: &Path| {
            if let Some(p) = path.file_name().and_then(|n| n.to_str()) {
                println!("   - {}", p);
                writeln!(output_file, " - {}", p).unwrap();
            }
        };

        visit_dirs(input_path, &mut print_and_write)?;
    }

    Ok(())
}
