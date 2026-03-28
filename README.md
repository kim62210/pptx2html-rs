# pptx2html-rs

Convert PPTX slides to pixel-perfect HTML in pure Rust.

Built on the ECMA-376 open standard — no Microsoft dependencies, no C/C++ bindings, just Rust.

## Features

- High-fidelity layout preservation using absolute positioning
- Theme color resolution with 12 color modifiers (tint, shade, lumMod, etc.)
- Slide master / layout inheritance chain with placeholder matching
- 30 preset shape SVG rendering with adjust value support
- Table, group shape, and connector support
- Image embedding (base64) or external references, with cropping
- Text styling: bold, italic, underline, bullets, vertical text, shadows
- Graceful placeholders for unsupported content (SmartArt, OLE, Math)
- Self-contained HTML output (single file, no external dependencies)

## Install

```bash
# Rust library
cargo add pptx2html-core

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

// Conversion with metadata (SmartArt/OLE/Math sideband)
let result = pptx2html_core::convert_file_with_metadata(Path::new("presentation.pptx"))?;
println!("HTML length: {}", result.html.len());
for elem in &result.unresolved_elements {
    println!("Unresolved: {:?} at slide {}", elem.element_type, elem.slide_index);
}
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

# Conversion with metadata (SmartArt/OLE/Math sideband)
result = pptx2html.convert_with_metadata("presentation.pptx")
print(f"HTML: {len(result.html)} chars, Unresolved: {len(result.unresolved_elements)}")
for elem in result.unresolved_elements:
    print(f"  {elem.element_type} at slide {elem.slide_index}: {elem.placeholder_id}")
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

See [SUPPORTED_FEATURES.md](SUPPORTED_FEATURES.md) for the full ECMA-376 element mapping.

| Category | Highlights |
|----------|-----------|
| Shapes | 30 preset shapes (rect, ellipse, arrows, stars, callouts, etc.) with SVG rendering |
| Text | Bold, italic, underline, strikethrough, super/subscript, vertical text, shadows, highlights |
| Colors | RGB, theme, system, preset with 12 modifiers (tint, shade, lumMod, satMod, etc.) |
| Fills | Solid, gradient, image, noFill; style references (fillRef/lnRef) |
| Tables | Cell fill, borders, column/row spans, vertical merge |
| Images | Base64 embedding, external refs, cropping, MIME auto-detection |
| Layout | Master/layout inheritance, ClrMap overrides, placeholder matching, TxStyles |
| Bullets | Character and auto-numbered bullets with font, size, color |
| Charts | Detection with preview image fallback |
| Unsupported | SmartArt, OLE, Math — structured placeholders with metadata sideband (raw XML, type, position) |
| LLM Enhance | Post-processing layer: SmartArt→HTML/CSS, OMML→MathML, DrawingML→CSS via LLM (pptx2html-enhance) |

## Architecture

### Pipeline

```
PPTX → pptx2html-rs (Rust) → HTML + Metadata
                                    │
                                    ├─→ Direct HTML output (existing, zero dependencies)
                                    └─→ pptx2html-enhance (Python, LLM) → Enhanced HTML
                                              │
                                              ├── SmartArt XML  → HTML/CSS layout
                                              ├── OMML equations → MathML
                                              └── DrawingML effects → CSS (shadow, glow, blur)
```

The Rust core converts PPTX to HTML with high fidelity. Elements it cannot fully render (SmartArt, Math, OLE) are emitted as structured placeholders with a metadata sideband containing the original XML. The optional Python `pptx2html-enhance` package uses LLM providers to transform these placeholders into semantic HTML.

### Project Layout

```
├── autoresearch/               # Autoresearch experiment loop
│   ├── program.md              # Master protocol
│   ├── run_loop.sh             # Experiment runner
│   ├── phases/                 # Phase-scoped programs (4 phases)
│   └── results.tsv             # Experiment audit log
├── crates/
│   ├── pptx2html-core/        # Core library (model, parser, resolver, renderer)
│   ├── pptx2html-cli/         # CLI binary (clap)
│   ├── pptx2html-py/          # PyO3 Python bindings (maturin)
│   └── pptx2html-wasm/        # WASM bindings (wasm-bindgen) + demo page
├── evaluate/                   # Fidelity evaluation (sacred — do not modify)
│   ├── evaluate_fidelity.py   # Composite scoring (SSIM + text + tests + perf)
│   ├── reference_render.py    # LibreOffice headless → reference PNGs
│   ├── candidate_render.py    # Playwright HTML → candidate PNGs
│   ├── create_golden_set.py   # Generate 50 golden PPTX test files
│   ├── golden_set/            # Golden PPTX files (generated)
│   └── golden_references/     # Reference PNG renders (generated)
├── pptx2html-enhance/         # LLM post-processing for unresolved elements (Python)
│   ├── src/pptx2html_enhance/ # Enhancer, handlers (SmartArt/Math/Effects), providers
│   └── tests/                 # 32 tests with mock LLM provider
└── Cargo.toml                 # Workspace root
```

See [ARCHITECTURE.md](ARCHITECTURE.md) for the full pipeline diagram and module responsibilities.

## Testing

```bash
# Rust tests (154 tests)
cargo test --workspace

# Python tests (32 tests)
cd pptx2html-enhance && .venv/bin/python -m pytest tests/ -v

# Benchmarks
cargo bench --package pptx2html-core
```

186 tests total, all passing:
- **Rust (154):** 59 unit tests (color, HSL, modifiers, placeholders, style refs, SVG geometry) + 87 integration tests (PPTX generation/parsing/rendering + edge cases + metadata sideband) + 6 CLI tests + 2 doc-tests
- **Python (32):** Enhancer pipeline, SmartArt/Math/Effects handlers, HTML patching (mock LLM provider)

## Autoresearch

Automated experiment loop inspired by the [Karpathy autoresearch](https://x.com/karpathy/status/1886192184808149383) pattern. An LLM agent modifies source code, runs build/test/evaluation, and keeps the change only if the fidelity score improves — otherwise it reverts.

```bash
# Run a specific phase
./autoresearch/run_loop.sh --phase 01_color_fidelity

# Limit iterations
./autoresearch/run_loop.sh --phase 02_performance --max-iterations 50
```

| Phase | Target |
|-------|--------|
| `01_color_fidelity` | Theme color modifier accuracy (12 modifier types) |
| `02_performance` | Rendering throughput optimization |
| `03_effect_rendering` | Shadow/glow DrawingML → CSS conversion |
| `04_geometry_coverage` | Preset shape expansion (30 → 187) |

Results are logged to `autoresearch/results.tsv`. See `autoresearch/program.md` for the full protocol.

## Evaluation

SSIM-based fidelity scoring system that compares Rust-rendered HTML against LibreOffice reference renders.

```bash
cd evaluate
python -m venv .venv && source .venv/bin/activate
pip install -r requirements.txt && playwright install chromium

# 1. Generate golden test set (50 PPTX files, 10 categories)
python create_golden_set.py

# 2. Render references via LibreOffice headless
python reference_render.py --input golden_set/ --output golden_references/

# 3. Compute composite fidelity score
python evaluate_fidelity.py --project-root ..
```

Composite score: `0.40*SSIM + 0.25*TextMatch + 0.25*TestPassRate + 0.10*Performance`

See [`evaluate/README.md`](evaluate/README.md) for details.

## pptx2html-enhance (LLM Post-Processing)

Optional Python package that uses LLM providers to enhance the Rust converter's output. Replaces structured placeholders (SmartArt, Math, OLE) with semantic HTML generated by an LLM.

### Install

```bash
pip install ./pptx2html-enhance[anthropic]   # or [openai] or [all]
```

### Quick Usage

```python
import pptx2html
from pptx2html_enhance import enhance

# Step 1: Convert with metadata sideband
result = pptx2html.convert_with_metadata("presentation.pptx")

# Step 2: Enhance placeholders via LLM
enhanced_html = await enhance(
    result.html,
    [e.__dict__ for e in result.unresolved_elements],
    provider="anthropic",       # or "openai"
    timeout=30.0,
    max_concurrent=5,
)
```

### Supported Element Types

| Type | Handler | Strategy |
|------|---------|----------|
| SmartArt | `SmartArtHandler` | LLM converts raw DrawingML XML to HTML/CSS layout |
| Math (OMML) | `MathHandler` | Rule-based for simple formulas (fractions, scripts, roots); LLM fallback for complex equations |
| Effects | `EffectsHandler` | Rule-based: outer shadow → `box-shadow`, glow → `box-shadow`, soft edge → `filter: blur()` |

## Contributing

1. Fork the repository
2. Create a feature branch
3. Ensure `cargo test --workspace`, `cargo clippy --workspace -- -D warnings`, and `cargo fmt --all -- --check` all pass
4. Submit a pull request

See [ARCHITECTURE.md](ARCHITECTURE.md) for guidance on adding support for new PPTX features.

## License

MIT - see [LICENSE](LICENSE)
