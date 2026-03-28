# pptx2html-rs Project Instructions

## Project Overview
Pure Rust PPTX to HTML converter based on the ECMA-376 open standard.
Target: Tier 1+2 coverage (~95% of real-world slides) in ~9,000-10,000 lines of Rust, MIT licensed.

## Language
- All code comments, commit messages, README, docs, and PR descriptions MUST be in **English**
- User-facing CLI output may include Korean for local dev convenience

## Tech Stack
- Rust 2024 edition
- `quick-xml` (SAX streaming parser), `zip`, `base64`, `thiserror`, `clap`
- No runtime dependencies beyond these

## Architecture
```
PPTX (ZIP) → parser/ (SAX XML) → model/ (Rust structs) → renderer/ (HTML/CSS)
```
- `src/model/` — Data model: Color, Shape, Slide, Presentation, Theme, ClrMap
- `src/parser/` — OOXML XML parsing with state-machine SAX approach
- `src/renderer/` — HTML/CSS generation with theme-aware color resolution

## Coding Rules
- `log` crate for output, never `println!` in library code
- Explicit imports only (no wildcard `use crate::*` except in slide_parser)
- Early return pattern preferred
- No `unsafe`, no `unwrap()` in library code (ok in tests)
- No lint suppression comments (`#[allow(...)]` only for documented reasons)
- Color resolution always goes through `Color::resolve()` with theme context

## Error Handling
- All errors via `PptxError` enum with `thiserror`
- Always specify exception type, no bare `.unwrap()` in lib
- Use `?` operator for propagation

## Testing
- Unit tests in `src/model/color.rs` (color resolution, HSL, modifiers)
- Integration tests in `tests/integration_test.rs` using `MinimalPptx` builder
- `MinimalPptx` creates valid PPTX ZIP archives in memory for testing
- Run: `cargo test`

## Git Conventions
- Branch: `main` only (no feature branches)
- Commit messages in English, conventional commits format:
  ```
  feat/fix/refactor/docs/test: summary

  - detail 1
  - detail 2
  ```
- Never commit `.env` or secrets
- Verify `cargo test` passes before commit
- Update README.md after significant changes

## Reference Architecture
- ONLYOFFICE core (`PPTXFormat/`) used as reference for algorithm understanding only
- Never translate or copy AGPL code — implement independently from ECMA-376 spec
- Reference docs in `docs/` directory

## Key Design Decisions
- Self-contained HTML output (images base64 inlined)
- EMU coordinate system (914400 EMU = 1 inch = 96px at 96 DPI)
- SAX streaming parser (not DOM) for memory efficiency
- Color resolution chain: ColorKind → ClrMap mapping → Theme lookup → Modifiers
- HSL uses standard ranges (H: 0-360, S: 0-1, L: 0-1)

## 6-Month Roadmap
| Month | Focus | Status |
|-------|-------|--------|
| 1 | Theme colors + Fill/Border + bodyPr | Done |
| 2 | Slide Master/Layout inheritance + Paragraph props | Next |
| 3 | Table + Bullets + Group shapes | Planned |
| 4 | Advanced text + Preset shape SVG + Image crop | Planned |
| 5 | PyO3 bindings + WASM + Performance | Planned |
| 6 | Polish + Real-world testing + Release v0.5.0 | Planned |
