# pptx2html-rs

Convert PPTX slides to pixel-perfect HTML in pure Rust.

Built on the ECMA-376 open standard — no Microsoft dependencies, no C/C++ bindings, just Rust.

## Install

```bash
cargo install --path .
```

## Usage

### CLI

```bash
pptx2html input.pptx -o output.html

# Default output: input.html
pptx2html input.pptx
```

### Library

```rust
use std::path::Path;

let html = pptx2html_rs::convert_file(Path::new("presentation.pptx"))?;

// Or from bytes
let html = pptx2html_rs::convert_bytes(&pptx_data)?;
```

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
| Images (base64 inline) | ✅ |
| Hyperlinks | ✅ |
| Preset shapes (rect, ellipse, roundRect, etc.) | ✅ |
| Slide Master / Layout inheritance | 🔜 |
| Placeholder content inheritance | 🔜 |
| Tables | 🔜 |
| Bullets (multi-level) | 🔜 |
| Group shapes | 🔜 |
| Preset shape SVG paths | 🔜 |
| PyO3 Python bindings | 🔜 |
| WASM target | 🔜 |

## Architecture

```
PPTX (ZIP) → XML Parsing → Model → HTML Rendering

src/
├── model/              # Data model (Color, Shape, Slide, Presentation)
│   ├── color.rs        # Theme-aware color system + modifiers + HSL
│   ├── presentation.rs # Presentation, Theme, ClrMap
│   ├── slide.rs        # Slide, Shape, TextBody
│   └── style.rs        # Fill, Border, TextStyle
├── parser/             # OOXML SAX parser
│   ├── mod.rs          # PptxParser (ZIP → Model)
│   ├── slide_parser.rs # Slide XML parsing
│   └── theme_parser.rs # Theme XML parsing
└── renderer/           # HTML/CSS generation
    └── mod.rs          # HtmlRenderer
```

## Testing

```bash
cargo test
```

- 15 unit tests: color resolution, HSL conversion, modifier application
- 19 integration tests: PPTX generation → parsing → HTML verification

## License

MIT
