use std::path::PathBuf;

use clap::Parser;
use log::info;

use pptx2html_core::ConversionOptions;

/// PPTX to HTML converter — preserves original layout
#[derive(Parser)]
#[command(name = "pptx2html", version, about)]
struct Cli {
    /// Input PPTX file path
    input: PathBuf,

    /// Output HTML file path (default: input filename.html)
    #[arg(short, long)]
    output: Option<PathBuf>,

    /// Slide selection (e.g. "1,3,5-8")
    #[arg(long)]
    slides: Option<String>,

    /// Output format: single HTML file or per-slide files
    #[arg(long, value_parser = ["single", "multi"], default_value = "single")]
    format: String,

    /// Do not embed images — extract to images/ directory
    #[arg(long)]
    no_embed: bool,

    /// Print presentation metadata as JSON and exit
    #[arg(long)]
    info: bool,

    /// Include hidden slides
    #[arg(long)]
    include_hidden: bool,
}

/// Parse a slide selection string like "1,3,5-8" into a sorted list of 1-based indices
fn parse_slide_selection(s: &str) -> Result<Vec<usize>, String> {
    let mut indices = Vec::new();
    for part in s.split(',') {
        let part = part.trim();
        if part.contains('-') {
            let mut parts = part.splitn(2, '-');
            let start: usize = parts
                .next()
                .ok_or_else(|| format!("invalid range: {part}"))?
                .trim()
                .parse()
                .map_err(|_| format!("invalid number in range: {part}"))?;
            let end: usize = parts
                .next()
                .ok_or_else(|| format!("invalid range: {part}"))?
                .trim()
                .parse()
                .map_err(|_| format!("invalid number in range: {part}"))?;
            if start > end {
                return Err(format!("invalid range {start}-{end}: start > end"));
            }
            for i in start..=end {
                indices.push(i);
            }
        } else {
            let idx: usize = part
                .parse()
                .map_err(|_| format!("invalid slide number: {part}"))?;
            indices.push(idx);
        }
    }
    indices.sort_unstable();
    indices.dedup();
    Ok(indices)
}

fn main() {
    env_logger::init();
    let cli = Cli::parse();

    // --info: print metadata and exit
    if cli.info {
        match pptx2html_core::get_info(&cli.input) {
            Ok(info) => {
                let title = match &info.title {
                    Some(t) => format!("\"{}\"", t.replace('\\', "\\\\").replace('"', "\\\"")),
                    None => "null".to_string(),
                };
                println!(
                    r#"{{"slide_count":{},"width_px":{:.1},"height_px":{:.1},"title":{}}}"#,
                    info.slide_count, info.width_px, info.height_px, title
                );
            }
            Err(e) => {
                eprintln!("Failed to read presentation: {e}");
                std::process::exit(1);
            }
        }
        return;
    }

    // Build conversion options
    let slide_indices = if let Some(ref sel) = cli.slides {
        match parse_slide_selection(sel) {
            Ok(indices) => Some(indices),
            Err(e) => {
                eprintln!("Invalid --slides value: {e}");
                std::process::exit(1);
            }
        }
    } else {
        None
    };

    let opts = ConversionOptions {
        embed_images: !cli.no_embed,
        include_hidden: cli.include_hidden,
        slide_range: None,
        slide_indices: slide_indices.clone(),
    };

    if cli.format == "multi" {
        // Multi-file output: one HTML per slide
        let output_dir = cli.output.unwrap_or_else(|| cli.input.with_extension(""));
        if let Err(e) = std::fs::create_dir_all(&output_dir) {
            eprintln!("Failed to create output directory: {e}");
            std::process::exit(1);
        }

        // Determine which slides to render
        let info = match pptx2html_core::get_info(&cli.input) {
            Ok(info) => info,
            Err(e) => {
                eprintln!("Failed to read presentation: {e}");
                std::process::exit(1);
            }
        };

        let indices_to_render: Vec<usize> = match &slide_indices {
            Some(indices) => indices.clone(),
            None => (1..=info.slide_count).collect(),
        };

        for &idx in &indices_to_render {
            let per_slide_opts = ConversionOptions {
                embed_images: !cli.no_embed,
                include_hidden: cli.include_hidden,
                slide_range: None,
                slide_indices: Some(vec![idx]),
            };
            match pptx2html_core::convert_file_with_options(&cli.input, &per_slide_opts) {
                Ok(html) => {
                    let path = output_dir.join(format!("slide-{idx}.html"));
                    if let Err(e) = std::fs::write(&path, &html) {
                        eprintln!("Failed to write {}: {e}", path.display());
                        std::process::exit(1);
                    }
                    info!("Written: {}", path.display());
                }
                Err(e) => {
                    eprintln!("Failed to convert slide {idx}: {e}");
                    std::process::exit(1);
                }
            }
        }
        println!(
            "Conversion complete: {} slides → {}",
            indices_to_render.len(),
            output_dir.display()
        );
    } else {
        // Single-file output
        let output = cli
            .output
            .unwrap_or_else(|| cli.input.with_extension("html"));

        match pptx2html_core::convert_file_with_options(&cli.input, &opts) {
            Ok(html) => {
                if let Err(e) = std::fs::write(&output, &html) {
                    eprintln!("Failed to write output file: {e}");
                    std::process::exit(1);
                }
                println!(
                    "Conversion complete: {} \u{2192} {}",
                    cli.input.display(),
                    output.display()
                );
            }
            Err(e) => {
                eprintln!("Conversion failed: {e}");
                std::process::exit(1);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_single_slides() {
        assert_eq!(parse_slide_selection("1,3,5").unwrap(), vec![1, 3, 5]);
    }

    #[test]
    fn test_parse_slide_range() {
        assert_eq!(parse_slide_selection("2-5").unwrap(), vec![2, 3, 4, 5]);
    }

    #[test]
    fn test_parse_mixed_selection() {
        assert_eq!(
            parse_slide_selection("1,3-5,8").unwrap(),
            vec![1, 3, 4, 5, 8]
        );
    }

    #[test]
    fn test_parse_dedup() {
        assert_eq!(parse_slide_selection("1,1,2,2-3").unwrap(), vec![1, 2, 3]);
    }

    #[test]
    fn test_parse_invalid_range() {
        assert!(parse_slide_selection("5-2").is_err());
    }

    #[test]
    fn test_parse_invalid_number() {
        assert!(parse_slide_selection("abc").is_err());
    }
}
