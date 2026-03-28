use std::path::PathBuf;

use clap::Parser;

/// PPTX to HTML converter — preserves original layout
#[derive(Parser)]
#[command(name = "pptx2html", version, about)]
struct Cli {
    /// Input PPTX file path
    input: PathBuf,

    /// Output HTML file path (default: input filename.html)
    #[arg(short, long)]
    output: Option<PathBuf>,
}

fn main() {
    env_logger::init();
    let cli = Cli::parse();

    let output = cli.output.unwrap_or_else(|| {
        cli.input.with_extension("html")
    });

    match pptx2html_core::convert_file(&cli.input) {
        Ok(html) => {
            if let Err(e) = std::fs::write(&output, &html) {
                eprintln!("Failed to write output file: {e}");
                std::process::exit(1);
            }
            println!("Conversion complete: {} → {}", cli.input.display(), output.display());
        }
        Err(e) => {
            eprintln!("Conversion failed: {e}");
            std::process::exit(1);
        }
    }
}
