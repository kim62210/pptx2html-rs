# pptx2html-rs Project Instructions

## Project Overview
Pure Rust PPTX to HTML converter based on the ECMA-376 open standard.
Cargo workspace with core library, CLI, Python bindings, and WASM target.

## Language
- All code comments, commit messages, README, docs, and PR descriptions MUST be in **English**
- User-facing CLI output in English

## Tech Stack
- Rust 2024 edition, Cargo workspace
- `quick-xml` (SAX streaming parser), `zip`, `base64`, `thiserror`, `clap`
- `pyo3` for Python bindings, `wasm-bindgen` for WASM target
- `criterion` for benchmarks

## Architecture
```
Cargo workspace
├── crates/pptx2html-core/     # Core library (model, parser, resolver, renderer)
├── crates/pptx2html-cli/      # CLI binary
├── crates/pptx2html-py/       # PyO3 Python bindings (maturin)
└── crates/pptx2html-wasm/     # WASM bindings (wasm-bindgen)
```

### Pipeline
```
PPTX (ZIP) → parser/ (SAX XML) → model/ (Rust structs) → resolver/ (inheritance) → renderer/ (HTML/CSS)
```

### Slide Hierarchy
```
Slide → SlideLayout → SlideMaster → Theme
  │         │              │           │
  │         │              ├── ClrMap   ├── ColorScheme
  │         │              ├── TxStyles ├── FontScheme
  │         ├── ClrMapOvr  ├── Shapes   └── FmtScheme
  ├── ClrMapOvr            └── Background
  ├── Shapes (placeholders)
  └── Background
```

### Property Inheritance Order
```
slide shape → layout placeholder → master placeholder → txStyles → defaultTextStyle → hardcoded default
```

## Coding Rules
- `log` crate for output, never `println!` in library code
- Explicit imports only (no wildcard `use crate::*` except in slide_parser)
- Early return pattern preferred
- No `unsafe`, no `unwrap()` in library code (ok in tests)
- No lint suppression comments (`#[allow(...)]` only for documented reasons)
- Color resolution always goes through `Color::resolve()` with theme context
- Index-based references between hierarchy levels (e.g., `layout_idx: usize`)

## Error Handling
- All errors via `PptxError` enum with `thiserror`
- Always specify exception type, no bare `.unwrap()` in lib
- Use `?` operator for propagation

## Testing
- Unit tests in source files (e.g., `src/model/color.rs`)
- Integration tests in `crates/pptx2html-core/tests/` using `MinimalPptx` builder
- CLI tests in `crates/pptx2html-cli/src/main.rs`
- `MinimalPptx` creates valid PPTX ZIP archives in memory for testing
- Run: `cargo test --workspace`
- Benchmarks: `cargo bench --package pptx2html-core`
- Verify all tests pass before each commit

## Git Conventions
- Branch: `main` only (no feature branches for the repo itself)
- Worktrees for parallel development, cherry-pick into main
- **Central rule:** every commit title MUST start with a conventional prefix such as `feat:`, `fix:`, `bug:`, `refactor:`, `docs:`, or `test:`
- Do not create prefix-less commit titles; rewrite local commits before handing off if a prefix is missing
- Commit messages in English, conventional commits format:
  ```
  feat/fix/bug/refactor/docs/test: concise summary

  - detail 1
  - detail 2
  ```
- Small, focused commits (one logical unit per commit)
- Never commit `.env` or secrets
- Update README.md after milestone commits

## Reference Architecture
- ONLYOFFICE core (`PPTXFormat/`) used as reference for algorithm understanding only
- Never translate or copy AGPL code — implement independently from ECMA-376 spec
- Reference docs in `docs/` directory

## Key Design Decisions
- Self-contained HTML output (images base64 inlined) by default, external image mode optional
- EMU coordinate system (914400 EMU = 1 inch = 96px at 96 DPI)
- SAX streaming parser (not DOM) for memory efficiency
- Color resolution chain: ColorKind → ClrMap mapping → Theme lookup → Modifiers
- HSL uses standard ranges (H: 0-360, S: 0-1, L: 0-1)
- Hierarchy uses index-based references (Vec + usize), not Rc/Arc
- Backward-compatible `Option<Theme>` accessor maintained via `Presentation::primary_theme()`
- ConversionOptions for slide filtering and image embedding control

## 6-Month Roadmap
| Month | Focus | Status |
|-------|-------|--------|
| 1 | Theme colors + Fill/Border + bodyPr | Done |
| 2 | Slide Master/Layout inheritance + Paragraph props | Done |
| 3 | Table + Bullets + Group shapes | Done |
| 4 | Advanced text + Preset shape SVG + Image crop | Done |
| 5 | PyO3 bindings + WASM + Performance + CLI | Done |
| 6 | Polish + Real-world testing + Release v0.5.0 | Done |
