# pptx2html-rs

Convert PPTX slides to pixel-perfect HTML in pure Rust.

Built on the ECMA-376 open standard — no Microsoft dependencies, no C/C++ bindings, just Rust.

## Install

```bash
# CLI
cargo install --path crates/pptx2html-cli

# Python (requires maturin)
cd crates/pptx2html-py && maturin develop

# WASM (requires wasm-pack)
cd crates/pptx2html-wasm && wasm-pack build --target web
```

## Usage

### CLI

```bash
# Basic conversion
pptx2html input.pptx -o output.html

# Default output: input.html
pptx2html input.pptx

# Select specific slides
pptx2html input.pptx --slides 1,3,5-8

# Per-slide output files
pptx2html input.pptx --format multi -o output_dir/

# External images (not embedded)
pptx2html input.pptx --no-embed

# Include hidden slides
pptx2html input.pptx --include-hidden

# Print presentation info as JSON
pptx2html input.pptx --info
```

### Rust Library

```rust
use std::path::Path;
use pptx2html_core::{convert_file, convert_file_with_options, ConversionOptions, get_info};

// Simple conversion
let html = convert_file(Path::new("presentation.pptx"))?;

// From bytes
let html = pptx2html_core::convert_bytes(&pptx_data)?;

// With options
let opts = ConversionOptions {
    embed_images: false,
    include_hidden: true,
    slide_indices: Some(vec![1, 3, 5]),
    ..Default::default()
};
let html = convert_file_with_options(Path::new("presentation.pptx"), &opts)?;

// Get metadata
let info = get_info(Path::new("presentation.pptx"))?;
println!("Slides: {}, Size: {}x{}", info.slide_count, info.width_px, info.height_px);
```

### Python

```python
import pptx2html

# Simple conversion
html = pptx2html.convert_file("presentation.pptx")

# From bytes
html = pptx2html.convert_bytes(pptx_data)

# With options
html = pptx2html.convert(
    "presentation.pptx",
    embed_images=False,
    include_hidden=True,
    slides=[1, 3, 5],
)

# Get metadata
info = pptx2html.get_info("presentation.pptx")
print(f"Slides: {info.slide_count}, Size: {info.width_px}x{info.height_px}")
```

### WASM / Browser

```html
<script type="module">
import init, { convert, get_info } from './pkg/pptx2html_wasm.js';

await init();

const response = await fetch('presentation.pptx');
const data = new Uint8Array(await response.arrayBuffer());

const html = convert(data);
document.getElementById('output').srcdoc = html;

const info = JSON.parse(get_info(data));
console.log(`Slides: ${info.slide_count}`);
</script>
```

A drag-and-drop demo page is included at `crates/pptx2html-wasm/demo/index.html`.

## Supported Features

| Feature | Status |
|---------|--------|
| Slide size / position / rotation | ✅ |
| Theme color resolution (Theme → ClrMap → Modifiers) | ✅ |
| Color modifiers (tint, shade, lumMod/Off, satMod/Off, alpha, etc.) | ✅ |
| SolidFill (RGB / theme / system / preset) | ✅ |
| GradientFill (linear) | ✅ |
| NoFill | ✅ |
| Border / Line (width, color, dash style) | ✅ |
| Text (font, size, bold, italic, underline, strikethrough) | ✅ |
| Text color (RGB / theme + modifiers) | ✅ |
| Superscript / subscript | ✅ |
| Letter spacing | ✅ |
| bodyPr (vertical alignment, internal margins) | ✅ |
| Images (base64 inline + external) | ✅ |
| Hyperlinks | ✅ |
| Preset shapes (rect, ellipse, roundRect, etc.) | ✅ |
| Slide Master / Layout inheritance | ✅ |
| Placeholder content inheritance | ✅ |
| Shape style refs (fillRef / lnRef / fontRef) | ✅ |
| Paragraph spacing (lnSpc / spcBef / spcAft) | ✅ |
| defaultTextStyle | ✅ |
| Text style inheritance (txStyles → defaultTextStyle) | ✅ |
| Font theme refs (+mj-lt / +mn-lt resolution) | ✅ |
| fontRef (major/minor → font-family) | ✅ |
| Tables | ✅ |
| Bullets (multi-level) | ✅ |
| Group shapes | ✅ |
| Preset shape SVG (30 shapes with adjust values) | ✅ |
| Image cropping (srcRect) | ✅ |
| Image MIME auto-detection | ✅ |
| Background image fill | ✅ |
| Chart fallback rendering | ✅ |
| Text breaks (\<a:br\>) | ✅ |
| Vertical text (vert, vert270, wordArtVert) | ✅ |
| Text highlight | ✅ |
| Text shadow (outerShdw) | ✅ |
| Slide filtering (range / indices) | ✅ |
| PyO3 Python bindings | ✅ |
| WASM target | ✅ |

## Architecture

```
Cargo workspace
├── crates/
│   ├── pptx2html-core/        # Core library
│   │   ├── src/
│   │   │   ├── model/         # Data model (color, geometry, hierarchy, slide, style)
│   │   │   ├── parser/        # OOXML SAX parser (7-stage pipeline)
│   │   │   ├── resolver/      # Property inheritance cascade
│   │   │   └── renderer/      # HTML/CSS generation + preset shape SVG
│   │   ├── tests/             # Integration tests
│   │   └── benches/           # Criterion benchmarks
│   ├── pptx2html-cli/         # CLI binary (clap)
│   ├── pptx2html-py/          # PyO3 Python bindings (maturin)
│   └── pptx2html-wasm/        # WASM bindings (wasm-bindgen)
│       └── demo/              # Browser demo page
└── Cargo.toml                 # Workspace root
```

## Testing

```bash
# Run all tests
cargo test --workspace

# Run benchmarks
cargo bench --package pptx2html-core
```

- 59 unit tests: color resolution, HSL, modifiers, placeholder matching, inheritance, style refs, geometry SVG
- 60 integration tests: PPTX generation → parsing → rendering verification
- 6 CLI tests: slide selection parser
- 125 tests total, all passing

## License

MIT
