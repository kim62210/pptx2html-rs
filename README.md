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
| PyO3 Python bindings | 🔜 |
| WASM target | 🔜 |

## Architecture

```
PPTX (ZIP) → XML Parsing → Model → Resolver (inheritance) → HTML Rendering

src/
├── model/              # Data model
│   ├── color.rs        # Theme-aware color system + modifiers + HSL
│   ├── hierarchy.rs    # SlideMaster, SlideLayout, TxStyles, PlaceholderInfo
│   ├── presentation.rs # Presentation, Theme, ClrMap, FmtScheme
│   ├── slide.rs        # Slide, Shape, TextBody
│   └── style.rs        # Fill, Border, TextStyle
├── parser/             # OOXML SAX parser
│   ├── mod.rs          # PptxParser (7-stage pipeline)
│   ├── slide_parser.rs # Slide XML parsing
│   ├── master_parser.rs # SlideMaster XML parsing
│   ├── layout_parser.rs # SlideLayout XML parsing
│   └── theme_parser.rs # Theme + FmtScheme parsing
├── resolver/           # Property inheritance cascade
│   ├── placeholder.rs  # Placeholder matching (type+idx)
│   ├── inheritance.rs  # Background, fill, position, ClrMap cascade
│   └── style_ref.rs    # fillRef/lnRef/fontRef resolution
└── renderer/           # HTML/CSS generation
    ├── mod.rs          # HtmlRenderer with resolver integration
    └── geometry.rs     # Preset shape SVG path generation (30 shapes)
```

## Testing

```bash
cargo test
```

- 59 unit tests: color resolution, HSL, modifiers, placeholder matching, inheritance, style refs, geometry SVG
- 60 integration tests: PPTX generation → parsing → rendering verification (hierarchy + integration)
- 119 tests total, all passing

## License

MIT
