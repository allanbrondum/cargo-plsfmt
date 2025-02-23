use clap::{Parser, ValueEnum};
use std::fs;

#[derive(Debug, Clone, Copy, ValueEnum)]
#[clap(rename_all = "kebab-case")]
pub enum EmitMode {
    /// Emits to files.
    Files,
    /// Writes the output to stdout.
    Stdout,
}

#[derive(Parser)]
pub struct Args {
    #[arg(long = "emit", value_enum, default_value_t = EmitMode::Files)]
    emit_mode: EmitMode,

    files: Vec<String>,
}

fn main() {
    let args = Args::parse();

    for filename in &args.files {
        let content = fs::read_to_string(filename).unwrap();

        let formatted = plsfmt::format_file(&content);

        match args.emit_mode {
            EmitMode::Files => fs::write(filename, formatted).unwrap(),
            EmitMode::Stdout => {
                print!("{}", formatted);
            }
        }
    }
}
