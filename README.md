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
| Unsupported | SmartArt, OLE, Math rendered as labeled placeholders |

## Architecture

```
Cargo workspace
├── autoresearch/               # Autoresearch experiment loop
│   ├── program.md              # Master protocol
│   ├── run_loop.sh             # Experiment runner
│   ├── phases/                 # Phase-scoped programs
│   └── results.tsv             # Experiment audit log
├── crates/
│   ├── pptx2html-core/        # Core library (model, parser, resolver, renderer)
│   ├── pptx2html-cli/         # CLI binary (clap)
│   ├── pptx2html-py/          # PyO3 Python bindings (maturin)
│   └── pptx2html-wasm/        # WASM bindings (wasm-bindgen) + demo page
├── evaluate/                   # Fidelity evaluation (sacred — do not modify)
│   ├── evaluate_fidelity.py   # Composite scoring (SSIM + text + tests + perf)
│   ├── reference_render.py    # LibreOffice headless -> reference PNGs
│   ├── candidate_render.py    # Playwright HTML -> candidate PNGs
│   ├── create_golden_set.py   # Generate 50 golden PPTX test files
│   ├── golden_set/            # Golden PPTX files (generated)
│   └── golden_references/     # Reference PNG renders (generated)
└── Cargo.toml                 # Workspace root
```

See [ARCHITECTURE.md](ARCHITECTURE.md) for the full pipeline diagram and module responsibilities.

## Testing

```bash
# Run all tests
cargo test --workspace

# Run benchmarks
cargo bench --package pptx2html-core
```

145 tests across 4 crates, all passing:
- 59 unit tests: color resolution, HSL, modifiers, placeholder matching, style refs, SVG geometry
- 80 integration tests: PPTX generation / parsing / rendering verification + edge cases
- 6 CLI tests: slide selection parser

## Autoresearch (Experiment Loop)

Karpathy autoresearch 패턴을 적용한 자동 실험 루프 인프라입니다.
LLM이 코드를 수정하고, 빌드/테스트/평가를 거쳐 점수가 개선되면 유지, 아니면 되돌리는 무한 루프입니다.

```bash
# Phase 1: 테마 컬러 변환 정확도 개선
./autoresearch/run_loop.sh --phase 01_color_fidelity

# 최대 50회 반복으로 제한
./autoresearch/run_loop.sh --phase 02_performance --max-iterations 50
```

| Phase | 대상 |
|-------|------|
| `01_color_fidelity` | 테마 컬러 12종 모디파이어 변환 정확도 |
| `02_performance` | 렌더링 처리량 최적화 |
| `03_effect_rendering` | 그림자/glow CSS 변환 |
| `04_geometry_coverage` | 프리셋 도형 확장 (30 → 187) |

실험 결과는 `autoresearch/results.tsv`에 기록됩니다. 자세한 프로토콜은 `autoresearch/program.md`를 참조하세요.

### Evaluation Infrastructure

평가 인프라 설정 및 실행:

```bash
cd evaluate
python -m venv .venv && source .venv/bin/activate
pip install -r requirements.txt && playwright install chromium

# 1. 골든 테스트셋 생성 (50 PPTX, 10 categories)
python create_golden_set.py

# 2. LibreOffice 레퍼런스 렌더링
python reference_render.py --input golden_set/ --output golden_references/

# 3. 복합 점수 산출
python evaluate_fidelity.py --project-root ..
```

복합 점수: `0.40*SSIM + 0.25*TextMatch + 0.25*TestPassRate + 0.10*Performance`
자세한 사용법은 [`evaluate/README.md`](evaluate/README.md)를 참조하세요.

## Contributing

1. Fork the repository
2. Create a feature branch
3. Ensure `cargo test --workspace`, `cargo clippy --workspace -- -D warnings`, and `cargo fmt --all -- --check` all pass
4. Submit a pull request

See [ARCHITECTURE.md](ARCHITECTURE.md) for guidance on adding support for new PPTX features.

## License

MIT - see [LICENSE](LICENSE)
