# Changelog

All notable changes to this project will be documented in this file.

## [0.5.0] - 2026-03-28

### Added
- PPTX to HTML conversion with high-fidelity layout preservation
- 30 preset shape SVG rendering with adjust value support
- Slide master / layout inheritance chain with placeholder matching
- Table support (cell fill, borders, col/row span, merge)
- Group shape support with nested coordinate remapping
- Image embedding (base64 data URI) and external reference modes
- Image cropping via CSS clip-path
- Background image fill support
- Chart detection with preview image fallback rendering
- Theme color resolution with full 12-color scheme
- 12 color modifiers: tint, shade, alpha, lumMod/Off, satMod/Off, hueMod/Off, comp, inv, gray
- ClrMap and ClrMapOverride support per slide/layout
- Text styling: bold, italic, underline, strikethrough, superscript, subscript
- Font resolution: theme font references (+mj-lt, +mn-lt, +mj-ea, +mn-ea)
- Bullet and auto-numbering support with font, size, and color
- Vertical text rendering (vert, vert270, mongolianVert)
- Text shadow and highlight support
- Line spacing, space before/after, indent, margin
- Hyperlink support
- TxStyles inheritance (titleStyle, bodyStyle, otherStyle)
- defaultTextStyle inheritance
- FmtScheme style references (fillRef, lnRef, fontRef)
- PyO3 Python bindings with `convert()`/`info()` API
- WASM target with drag-and-drop demo page
- CLI with slide selection, multi-file output, info command
- Criterion performance benchmarks
- Graceful degradation for unsupported content (SmartArt, OLE, Math)
- Password-protected PPTX detection with clear error message
- Conversion progress logging via `log` crate
- GitHub Actions CI/CD workflows

### Architecture
- Cargo workspace: core library, CLI, Python bindings, WASM target
- SAX streaming parser for memory-efficient XML processing
- Index-based hierarchy references (no Rc/Arc)
- EMU coordinate system (914400 EMU = 1 inch = 96px)
