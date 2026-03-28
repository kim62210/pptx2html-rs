# pptx-to-html Architecture

> Current project architecture overview: ZIP → XML → Model → HTML pipeline

---

## Overview

`pptx-to-html` is a Rust CLI tool that converts PPTX presentation files to high-fidelity HTML with preserved layout and styling. The conversion follows a 4-stage pipeline.

```
┌──────────┐    ┌──────────┐    ┌───────────┐    ┌──────────┐
│  PPTX    │    │   XML    │    │  Internal │    │  HTML    │
│  (ZIP)   │───►│  Parser  │───►│   Model   │───►│ Renderer │
│          │    │          │    │           │    │          │
└──────────┘    └──────────┘    └───────────┘    └──────────┘
  Stage 1         Stage 2         Stage 3          Stage 4
  Extract         Parse           Transform        Render
```

---

## Project Structure

```
pptx-to-html/
├── Cargo.toml              # Project manifest
├── src/
│   ├── main.rs             # CLI entry point (clap)
│   ├── lib.rs              # Library root, public API
│   ├── error.rs            # Error types (thiserror)
│   ├── model/              # Internal data model
│   │   ├── mod.rs          # Model module root
│   │   ├── presentation.rs # Presentation, slide size
│   │   ├── slide.rs        # Slide, shapes, placeholders
│   │   ├── style.rs        # Text/paragraph styles
│   │   ├── color.rs        # Color types, modifiers
│   │   └── geometry.rs     # Shape geometry, transforms
│   ├── parser/             # PPTX/XML parsing
│   │   ├── mod.rs          # Parser module root
│   │   ├── slide_parser.rs # Slide XML parsing
│   │   ├── theme_parser.rs # Theme XML parsing
│   │   ├── relationships.rs# .rels file parsing
│   │   └── xml_utils.rs    # XML helper utilities
│   └── renderer/           # HTML output generation
│       └── mod.rs          # HTML renderer
├── tests/                  # Integration tests
└── docs/                   # Technical documentation
```

---

## Stage 1: ZIP Extraction

**Crate:** `zip` v2

The PPTX file format is a ZIP archive containing XML files and media resources.

```
Input: PPTX file path
Output: In-memory access to archive entries

Key files extracted:
- [Content_Types].xml          → content type registry
- _rels/.rels                  → root relationships
- ppt/presentation.xml         → presentation structure
- ppt/_rels/presentation.xml.rels → presentation relationships
- ppt/slides/slide{N}.xml     → slide content
- ppt/slides/_rels/slide{N}.xml.rels → slide relationships
- ppt/slideLayouts/slideLayout{N}.xml → layout definitions
- ppt/slideMasters/slideMaster{N}.xml → master definitions
- ppt/theme/theme{N}.xml      → theme definitions
- ppt/media/*                  → embedded images/media
```

### Implementation Notes

- Files are read on-demand from the ZIP archive, not all extracted upfront
- Media files (images) are extracted and either base64-encoded inline or saved to an output directory
- The `zip` crate provides random access to entries by name

---

## Stage 2: XML Parsing

**Crate:** `quick-xml` v0.37

XML parsing converts raw XML elements into structured Rust types.

### Parser Components

#### Relationship Parser (`relationships.rs`)

Parses `.rels` files to build reference maps:

```
rId1 → slideMasters/slideMaster1.xml
rId2 → slides/slide1.xml
rId3 → theme/theme1.xml
```

Used to navigate the slide → layout → master → theme chain.

#### Theme Parser (`theme_parser.rs`)

Parses `ppt/theme/theme{N}.xml`:

```
Theme
├── ColorScheme: 12 named colors (dk1, lt1, dk2, lt2, accent1-6, hlink, folHlink)
├── FontScheme: major/minor font families with script variants
└── FormatScheme: fill/line/effect style lists (3 levels each)
```

#### Slide Parser (`slide_parser.rs`)

Parses slide, layout, and master XML files. The structure is similar across all three; the parser handles:

```
cSld (common slide data)
├── bg (background)
└── spTree (shape tree)
    ├── p:sp (shape)
    ├── p:pic (picture)
    ├── p:grpSp (group)
    ├── p:cxnSp (connector)
    └── p:graphicFrame (table/chart/diagram)
```

#### XML Utilities (`xml_utils.rs`)

Helper functions for common XML parsing patterns:
- Attribute extraction with type conversion
- Namespace-aware element matching
- Optional element/attribute handling

---

## Stage 3: Internal Model

The model layer defines Rust structs that represent the presentation data in a renderer-independent format.

### Core Types

#### Presentation (`presentation.rs`)

```rust
struct Presentation {
    slide_width: i64,   // EMU
    slide_height: i64,  // EMU
    slides: Vec<Slide>,
    // + masters, layouts, themes
}
```

#### Slide (`slide.rs`)

```rust
struct Slide {
    shapes: Vec<Shape>,
    background: Option<Background>,
    // + layout/master references, clrMapOvr
}

enum Shape {
    Shape(SpShape),
    Picture(PicShape),
    Group(GroupShape),
    Connector(ConnectorShape),
    GraphicFrame(GraphicFrameShape),
}
```

#### Style (`style.rs`)

```rust
struct TextStyle {
    font_size: Option<i32>,     // hundredths of pt
    bold: Option<bool>,
    italic: Option<bool>,
    underline: Option<UnderlineType>,
    color: Option<ColorRef>,
    font_family: Option<String>,
    // ...
}
```

#### Color (`color.rs`)

```rust
enum ColorRef {
    SrgbClr { val: String, modifiers: Vec<ColorModifier> },
    SchemeClr { val: String, modifiers: Vec<ColorModifier> },
    SysClr { val: String, last_clr: String, modifiers: Vec<ColorModifier> },
    HslClr { hue: i32, sat: i32, lum: i32, modifiers: Vec<ColorModifier> },
    PrstClr { val: String, modifiers: Vec<ColorModifier> },
    ScrgbClr { r: i32, g: i32, b: i32, modifiers: Vec<ColorModifier> },
}

enum ColorModifier {
    Tint(u32),
    Shade(u32),
    LumMod(u32),
    LumOff(u32),
    SatMod(u32),
    SatOff(u32),
    Alpha(u32),
    Inv,
    Comp,
    Gray,
    // ...
}
```

#### Geometry (`geometry.rs`)

```rust
enum Geometry {
    Preset { name: String, adjustments: Vec<AdjustValue> },
    Custom { paths: Vec<GeometryPath>, guides: Vec<Guide> },
}
```

---

## Stage 4: HTML Rendering

**Output:** Self-contained HTML file with inline CSS

The renderer traverses the model and generates HTML/CSS that preserves the original layout.

### Rendering Strategy

#### Slide Container

Each slide is rendered as a positioned container:

```html
<div class="slide" style="width: 1280px; height: 720px; position: relative; overflow: hidden;">
  <!-- shapes rendered as absolutely positioned children -->
</div>
```

#### Shape Rendering

Shapes are rendered as absolutely positioned `<div>` elements:

```html
<div class="shape" style="
  position: absolute;
  left: 88px;
  top: 38px;
  width: 1104px;
  height: 139px;
  background-color: #FF0000;
  border: 1px solid #000000;
  border-radius: 8px;
  transform: rotate(45deg);
">
  <div class="text-body" style="padding: 7.2pt 3.6pt;">
    <p style="text-align: center;">
      <span style="font-size: 44pt; font-weight: bold;">Title Text</span>
    </p>
  </div>
</div>
```

#### Image Rendering

Images are embedded as base64 data URIs or referenced from an output directory:

```html
<div class="picture" style="position: absolute; left: 160px; top: 147px; width: 640px; height: 427px;">
  <img src="data:image/png;base64,..." style="width: 100%; height: 100%; object-fit: fill;" alt="Description"/>
</div>
```

#### Text Rendering

Text is rendered with inline styles matching the PPTX formatting:

```html
<div class="text-body" style="padding: 9.6px 4.8px; display: flex; flex-direction: column; justify-content: center;">
  <p style="margin: 0; padding-bottom: 6pt; line-height: 1.5; text-align: left;">
    <span style="font-size: 18pt; color: #000000; font-family: 'Calibri', sans-serif;">
      Bullet text content
    </span>
  </p>
</div>
```

---

## Dependency Graph

```
main.rs
  └─ lib.rs (convert function)
      ├─ parser/mod.rs (orchestrate parsing)
      │  ├─ relationships.rs (resolve .rels)
      │  ├─ theme_parser.rs (parse theme XML)
      │  ├─ slide_parser.rs (parse slide/layout/master XML)
      │  └─ xml_utils.rs (XML helpers)
      ├─ model/ (data structures)
      │  ├─ presentation.rs
      │  ├─ slide.rs
      │  ├─ style.rs
      │  ├─ color.rs
      │  └─ geometry.rs
      └─ renderer/mod.rs (generate HTML)
```

### External Dependencies

| Crate | Version | Purpose |
|-------|---------|---------|
| `quick-xml` | 0.37 | XML parsing (with serde support) |
| `zip` | 2 | ZIP archive reading |
| `base64` | 0.22 | Image base64 encoding |
| `thiserror` | 2 | Error type derivation |
| `log` | 0.4 | Logging facade |
| `env_logger` | 0.11 | Logger implementation |
| `clap` | 4 | CLI argument parsing |

---

## Data Flow Detail

```
main.rs: parse CLI args → call lib::convert(input, output)

lib::convert():
  1. Open ZIP archive
  2. Parse relationships → build reference map
  3. Parse theme(s) → Theme objects
  4. Parse master(s) → SlideMaster objects (linked to themes)
  5. Parse layout(s) → SlideLayout objects (linked to masters)
  6. Parse slide(s) → Slide objects (linked to layouts)
  7. Build Presentation model
  8. For each slide:
     a. Resolve inheritance (background, shapes, text styles)
     b. Resolve colors (scheme → theme → modifiers → RGB)
     c. Render to HTML div
  9. Wrap slides in HTML document
  10. Write output file
```

---

## Design Decisions

### Why Rust?

- Memory safety without garbage collection overhead
- Strong type system catches OOXML parsing edge cases at compile time
- Single binary deployment (no runtime dependencies)
- Competitive performance for batch processing

### Why quick-xml?

- Zero-copy parsing (borrows from input buffer)
- Event-driven API matches OOXML's deeply nested structure
- Serde integration for simpler elements
- Active maintenance and good performance benchmarks

### Why not DOM parsing?

OOXML files can be large (complex presentations with many shapes). Event-driven (SAX-style) parsing with quick-xml avoids loading the entire XML tree into memory.

### Absolute positioning vs. flow layout

PPTX uses absolute positioning (EMU coordinates) for all shapes. HTML rendering preserves this with `position: absolute` rather than attempting to convert to flow layout, ensuring pixel-accurate output.

### Self-contained HTML

The default output is a single HTML file with all styles inline and images base64-encoded. This ensures portability. An optional mode could output images as separate files with relative references for reduced file size.
