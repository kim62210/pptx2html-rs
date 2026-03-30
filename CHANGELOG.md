# Changelog

All notable changes to this project will be documented in this file.

## [1.0.0] - 2026-03-30

### npm / WASM
- Publish WASM package to npm as `pptx2html-wasm`
- Add `convert_with_options()` — full ConversionOptions support (embedImages, includeHidden, slideIndices)
- Add `convert_with_metadata()` — returns typed ConversionResult with HTML + unresolved elements
- Add `convert_with_options_metadata()` — combined options + metadata API
- Add `get_presentation_info()` — typed PresentationInfo object (replaces JSON string `get_info()`)
- Add GitHub Actions workflow for automated npm publishing on version tags
- Add WASM build verification to CI pipeline

### Open Source
- Add CONTRIBUTING.md with development setup and code style guide
- Add CODE_OF_CONDUCT.md (Contributor Covenant)
- Add GitHub issue templates (bug report, feature request) and PR template
- Add keywords and categories to all crate Cargo.toml metadata

### Performance
- Eliminate intermediate String allocations in renderer (~28% faster rendering)
- Optimize CSS style string building with direct write!() (~21% additional, ~43% cumulative)

### Rendering — Shapes & Geometry
- Expand preset shape geometries from 30 to 187 (full OOXML ECMA-376 coverage)
- Implement custom geometry (`<a:custGeom>`) DrawingML path → SVG conversion
  - Supports moveTo, lnTo, cubicBezTo, quadBezTo, arcTo, close commands
  - DrawingML arc → SVG arc mathematical transformation
- Add shape shadow (`<a:outerShdw>`) and glow (`<a:glow>`) → CSS box-shadow rendering
- Implement auto-fit fontScale and lnSpcReduction for text body sizing
- Add connector geometry paths (straightConnector1, bentConnector5)
- Default 0.75pt stroke for connectors without explicit border

### Rendering — Images
- Fix relative path resolution for image relationship targets (`../media/` → correct ZIP path)
- Handle `<a:blip>` elements with child nodes (Start event, not just Empty)
- Parse background images from master and layout slides (`<a:blipFill>` in `<p:bgPr>`)
- Load image data for shape-level blipFill (image-filled rectangles)
- Fix image crop CSS: replace extreme percentage scaling with pixel-based offsets

### Rendering — Colors & Fills
- Correct OOXML color modifier application order per ECMA-376 spec (alpha→hue→sat→lum→tint/shade)
- Fix HSL tint/shade formula to match OOXML definition
- Distinguish explicit `<a:noFill>` from unspecified fill (prevent theme fillRef overriding transparency)
- Resolve empty and unresolvable theme font references (filter `+mn-ea` → actual typeface)

### Rendering — Layout
- Fix group shape children coordinate transform (chOff/chExt → group bounding box scaling)
- Guard `<a:off>`/`<a:ext>` parsing to `<a:xfrm>` context only (prevent extLst overwriting shape size)
- Fix shape position resolution: treat (0,0) as valid position, not "unset"
- Filter master placeholder shapes through layout matching per OOXML spec
- Change `.shape` overflow from `hidden` to `visible` (prevent text clipping)
- Add word-break/overflow-wrap to text body for proper wrapping
- Remove CSS border duplication on SVG shapes (use SVG stroke only)

### Infrastructure
- Add autoresearch experiment loop (program.md, run_loop.sh, 4 phase programs)
- Add evaluation infrastructure (SSIM fidelity scorer, golden set generator, reference/candidate renderers)
- Add pptx2html-enhance LLM post-processing package (SmartArt/Math/Effects handlers)
- Add ConversionResult with unresolved_elements metadata sideband
- Python bindings: `convert_with_metadata()` API

### Tests
- Total tests: 195+ (was 145 in v0.5.0)
- 16 color modifier edge case tests
- 8 custom geometry integration tests
- 7 shadow/glow effect tests
- 7 auto-fit rendering tests
- Hierarchy/position/background fill tests

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
