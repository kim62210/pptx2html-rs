# Evaluation Infrastructure

Objective scoring pipeline for the pptx2html-rs autoresearch loop.
Measures conversion fidelity across four metrics and produces a single
composite score that determines keep/revert decisions.

## Composite Score

```
fidelity_score = 0.40 * ssim + 0.25 * text_match + 0.25 * test_pass + 0.10 * perf
```

| Weight | Metric         | Description                             |
|--------|----------------|-----------------------------------------|
| 0.40   | SSIM           | Structural similarity vs LibreOffice    |
| 0.25   | Text Match     | Token-level Jaccard on extracted text    |
| 0.25   | Test Pass Rate | `cargo test --workspace` pass ratio     |
| 0.10   | Performance    | Slides/sec normalized to 50 sps baseline|

## Prerequisites

- Python 3.12+
- LibreOffice (for reference rendering)
- Poppler (`pdftoimage` — `brew install poppler` on macOS)
- Chromium (installed automatically by Playwright)
- Rust toolchain with `cargo`

## Setup

```bash
cd evaluate
python -m venv .venv
source .venv/bin/activate
pip install -r requirements.txt
playwright install chromium
```

## Usage

### 1. Generate golden PPTX test set

```bash
python create_golden_set.py
# -> evaluate/golden_set/*.pptx  (50 files, 10 categories x 5)
```

Filter by category:

```bash
python create_golden_set.py --categories basic_text shapes tables
```

### 2. Render reference PNGs (LibreOffice)

```bash
python reference_render.py --input golden_set/ --output golden_references/
```

### 3. Run fidelity evaluation

```bash
python evaluate_fidelity.py --project-root /path/to/pptx2html-rs
```

Options:

```bash
# Evaluate specific phase only
python evaluate_fidelity.py --project-root . --phase theme_colors

# Verbose per-slide scores
python evaluate_fidelity.py --project-root . --verbose

# JSON output for automation
python evaluate_fidelity.py --project-root . --output-json result.json
```

### 4. Render candidate screenshots (standalone)

```bash
python candidate_render.py --html-dir output/ --output candidates/
```

## Directory Structure

```
evaluate/
├── evaluate_fidelity.py       # Immutable scoring function (DO NOT MODIFY)
├── reference_render.py        # LibreOffice headless -> PNG
├── candidate_render.py        # Playwright HTML -> PNG
├── create_golden_set.py       # Generate golden PPTX test files
├── requirements.txt           # Python dependencies
├── README.md                  # This file
├── golden_set/                # Golden PPTX files (generated)
│   └── .gitkeep
└── golden_references/         # LibreOffice reference PNGs (generated)
    └── .gitkeep
```

## Golden Set Categories

| Category     | Count | Tests                                    |
|--------------|-------|------------------------------------------|
| basic_text   | 5     | Bold, italic, sizes, colors, alignment   |
| shapes       | 5     | Rectangles, ellipses, arrows, stars      |
| theme_colors | 5     | 12 theme colors, tint, shade, dark bg    |
| tables       | 5     | Headers, merge, colors, alignment, large |
| images       | 5     | Centered, tiled, overlay, bordered       |
| gradients    | 5     | Two-color, three-color, oval, dark bg    |
| groups       | 5     | Overlapping, rotated, concentric, z-order|
| layouts      | 5     | Title, content, two-column, section      |
| bullets      | 5     | Simple, nested, bold labels, colored     |
| mixed        | 5     | Dashboard, comparison, architecture      |

## Autoresearch Integration

This evaluation infrastructure is the "immutable scoring function" in the
autoresearch pattern. The LLM agent:

1. Makes a code change to pptx2html-rs
2. Runs `evaluate_fidelity.py` to get a score
3. If score improved -> keep the change
4. If score regressed -> revert the change

The `evaluate_fidelity.py` file must never be modified by the LLM agent.
Only humans may change the scoring weights or metric definitions.
