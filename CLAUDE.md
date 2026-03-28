# pptx2html-rs Project Instructions

## Project Overview
Pure Rust PPTX to HTML converter based on the ECMA-376 open standard.
Target: Tier 1+2 coverage (~95% of real-world slides) in ~9,000-10,000 lines of Rust, MIT licensed.

## Language
- All code comments, commit messages, README, docs, and PR descriptions MUST be in **English**
- User-facing CLI output in English

## Tech Stack
- Rust 2024 edition
- `quick-xml` (SAX streaming parser), `zip`, `base64`, `thiserror`, `clap`
- No runtime dependencies beyond these

## Architecture
```
PPTX (ZIP) → parser/ (SAX XML) → model/ (Rust structs) → resolver/ (inheritance) → renderer/ (HTML/CSS)
```
- `src/model/` — Data model: Color, Shape, Slide, Presentation, Theme, ClrMap, hierarchy types
- `src/parser/` — OOXML XML parsing with state-machine SAX approach
- `src/resolver/` — Placeholder matching + property inheritance cascade
- `src/renderer/` — HTML/CSS generation with theme-aware color resolution

### Slide Hierarchy (Month 2+)
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
- Integration tests in `tests/` using `MinimalPptx` builder
- `MinimalPptx` creates valid PPTX ZIP archives in memory for testing
- Run: `cargo test`
- Verify all tests pass before each commit

## Git Conventions
- Branch: `main` only (no feature branches for the repo itself)
- Worktrees for parallel development, cherry-pick into main
- Commit messages in English, conventional commits format:
  ```
  feat/fix/refactor/docs/test: concise summary

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
- Self-contained HTML output (images base64 inlined)
- EMU coordinate system (914400 EMU = 1 inch = 96px at 96 DPI)
- SAX streaming parser (not DOM) for memory efficiency
- Color resolution chain: ColorKind → ClrMap mapping → Theme lookup → Modifiers
- HSL uses standard ranges (H: 0-360, S: 0-1, L: 0-1)
- Hierarchy uses index-based references (Vec + usize), not Rc/Arc
- Backward-compatible `Option<Theme>` accessor maintained via `Presentation::primary_theme()`

## 6-Month Roadmap
| Month | Focus | Status |
|-------|-------|--------|
| 1 | Theme colors + Fill/Border + bodyPr | Done |
| 2 | Slide Master/Layout inheritance + Paragraph props | In Progress |
| 3 | Table + Bullets + Group shapes | Planned |
| 4 | Advanced text + Preset shape SVG + Image crop | Planned |
| 5 | PyO3 bindings + WASM + Performance | Planned |
| 6 | Polish + Real-world testing + Release v0.5.0 | Planned |

## Month 2 Commit Plan
```
Phase A (sequential): Commit 1 (model) → Commit 2 (master/layout parser)
Phase B (parallel):   Commit 3 (resolver/placeholder) ‖ Commit 4 (paragraph props)
Phase C (sequential): Commit 5 (shape style refs)
Phase D (sequential): Commit 6 (integration tests)
```
