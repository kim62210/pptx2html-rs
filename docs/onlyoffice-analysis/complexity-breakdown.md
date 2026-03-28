# ONLYOFFICE PPTXFormat Complexity Breakdown

> Module line counts, complexity indicators, and implementation priority analysis

---

## Module Size Estimates

Based on analysis of the ONLYOFFICE `sdkjs` repository structure. Line counts are approximate and include comments/whitespace.

### Core Presentation Modules

| Module | Est. Lines | Complexity | Notes |
|--------|-----------|------------|-------|
| `Presentation.js` | ~2,500 | Medium | Slide management, global state |
| `Slide.js` | ~1,800 | Medium | Shape tree, background, layout ref |
| `SlideLayout.js` | ~800 | Low | Placeholder container |
| `SlideMaster.js` | ~1,200 | Medium | clrMap, txStyles, theme linkage |
| `Placeholder.js` | ~600 | Medium | Matching algorithm, type handling |

### Shape Modules

| Module | Est. Lines | Complexity | Notes |
|--------|-----------|------------|-------|
| `Shape.js` | ~3,500 | High | Central shape object, many property accessors |
| `GroupShape.js` | ~1,200 | Medium | Child transform computation |
| `Image.js` | ~800 | Low-Medium | Blip handling, crop |
| `Connector.js` | ~1,500 | High | Route calculation, site resolution |
| `GraphicFrame.js` | ~600 | Low | Container delegation |
| `Table.js` | ~3,000 | High | Grid layout, merge handling, style bands |

### Text Modules

| Module | Est. Lines | Complexity | Notes |
|--------|-----------|------------|-------|
| `TextBody.js` | ~1,000 | Medium | Body property management |
| `Paragraph.js` | ~2,500 | High | Bullet resolution, spacing calculation |
| `Run.js` | ~800 | Medium | Run property merge |
| `ParagraphProperties.js` | ~1,500 | Medium-High | Many properties, inheritance merge |
| `RunProperties.js` | ~2,000 | High | Font resolution, color, many attributes |
| `TextBodyProperties.js` | ~600 | Low | Insets, autofit, columns |
| `ListStyle.js` | ~400 | Low | Level property container |

### Color & Fill Modules

| Module | Est. Lines | Complexity | Notes |
|--------|-----------|------------|-------|
| `Color.js` | ~2,000 | **Very High** | Core color resolution, HSL/RGB conversion |
| `ColorModifiers.js` | ~1,500 | **Very High** | Modifier chain, math-heavy |
| `SchemeColor.js` | ~600 | High | ClrMap traversal |
| `Fill.js` | ~800 | Medium | Unified fill interface |
| `SolidFill.js` | ~200 | Low | Single color wrapper |
| `GradientFill.js` | ~1,000 | High | Stop interpolation, angle calculation |
| `BlipFill.js` | ~800 | Medium | Stretch/tile modes, crop |
| `PatternFill.js` | ~400 | Medium | Pattern rendering |

### Geometry Modules

| Module | Est. Lines | Complexity | Notes |
|--------|-----------|------------|-------|
| `PresetGeometry.js` | ~8,000+ | **Very High** | 187 preset shape definitions |
| `GeometryFormula.js` | ~1,200 | High | Formula evaluation engine |
| `Geometry.js` | ~600 | Medium | Custom geometry support |
| `Path.js` | ~800 | Medium | SVG-like path commands |
| `AdjustValue.js` | ~300 | Low | Adjustment handle storage |

### Theme & Style Modules

| Module | Est. Lines | Complexity | Notes |
|--------|-----------|------------|-------|
| `Theme.js` | ~1,500 | Medium | Color/font/format scheme container |
| `FormatScheme.js` | ~600 | Medium | Style list management |
| `FontScheme.js` | ~400 | Low | Major/minor font container |
| `StyleRef.js` | ~500 | Medium | phClr substitution logic |
| `TableStyle.js` | ~1,500 | High | Band/row/col conditional styles |

### Reader/Writer

| Module | Est. Lines | Complexity | Notes |
|--------|-----------|------------|-------|
| `PptxReader.js` | ~5,000+ | **Very High** | Main orchestrator, many branches |
| `XmlReader.js` | ~8,000+ | **Very High** | Element-by-element XML parsing |
| `PptxWriter.js` | ~4,000+ | High | Serialization |
| `XmlWriter.js` | ~6,000+ | High | Model → XML conversion |
| `RelationshipManager.js` | ~600 | Medium | rId tracking |

---

## Complexity Analysis by Feature Area

### Difficulty Ranking (1-5)

| Feature Area | Difficulty | Total Lines | Reason |
|-------------|-----------|-------------|--------|
| ZIP/XML parsing | 2 | ~2,000 | Well-structured, library-assisted |
| Slide structure | 2 | ~3,000 | Straightforward hierarchy |
| Basic shapes (rect, ellipse) | 1 | ~500 | CSS border-radius |
| Text rendering (basic) | 3 | ~4,000 | Many properties, inheritance |
| Text rendering (bullets, levels) | 4 | ~2,000 | Complex state management |
| Solid fill colors | 2 | ~1,000 | Direct resolution |
| **Scheme color resolution** | **5** | ~4,000 | **ClrMap + theme + modifiers** |
| **Color modifiers** | **5** | ~2,000 | **HSL conversion, sequential apply** |
| Gradient fills | 4 | ~1,500 | Stop interpolation, angles |
| Image fills | 3 | ~1,000 | Crop, stretch/tile |
| Line/outline | 2 | ~800 | Straightforward CSS mapping |
| **Preset geometries** | **5** | ~10,000 | **187 shapes, formula engine** |
| Custom geometries | 4 | ~2,000 | Path parsing, guide evaluation |
| **Placeholder inheritance** | **4** | ~3,000 | **Multi-level property cascade** |
| **Text style inheritance** | **5** | ~3,000 | **9-level cascade, per-property** |
| Group transforms | 3 | ~1,500 | Recursive coordinate mapping |
| Tables | 4 | ~4,000 | Grid layout, merge, style bands |
| Effects (shadow, etc.) | 3 | ~1,500 | CSS filter/box-shadow mapping |
| Connectors | 4 | ~2,000 | Route calculation |
| Charts | 5 | ~10,000+ | Deferred - very complex |
| Animations/transitions | 5 | ~5,000+ | Deferred - CSS animation |

---

## Estimated Total Implementation Effort

### Phase 1: Core (MVP)

| Component | Est. Lines (Rust) | Person-Days |
|-----------|------------------|-------------|
| ZIP extraction | 200 | 1 |
| XML parsing infrastructure | 500 | 2 |
| Presentation/Slide/Layout/Master model | 800 | 3 |
| Relationship resolution | 300 | 1 |
| Theme parsing | 600 | 2 |
| Basic shape model (rect, text) | 400 | 2 |
| Solid color (sRGB only) | 200 | 1 |
| Basic text (single run, no bullets) | 600 | 3 |
| HTML renderer (basic) | 800 | 3 |
| **Phase 1 Total** | **~4,400** | **~18 days** |

### Phase 2: Color & Text

| Component | Est. Lines (Rust) | Person-Days |
|-----------|------------------|-------------|
| Scheme color resolution | 400 | 3 |
| ClrMap chain | 200 | 1 |
| Color modifiers (full set) | 600 | 4 |
| HSL/RGB conversion | 200 | 1 |
| Text inheritance cascade | 500 | 4 |
| Bullet/numbering | 400 | 3 |
| Paragraph spacing/indent | 300 | 2 |
| Font resolution (theme refs) | 200 | 1 |
| **Phase 2 Total** | **~2,800** | **~19 days** |

### Phase 3: Geometry & Fill

| Component | Est. Lines (Rust) | Person-Days |
|-----------|------------------|-------------|
| Preset geometry (top 30) | 2,000 | 8 |
| Geometry formula engine | 500 | 3 |
| Gradient fill | 400 | 3 |
| Image fill (blipFill) | 300 | 2 |
| Pattern fill | 200 | 1 |
| Line/outline rendering | 300 | 2 |
| Effects (shadow) | 300 | 2 |
| **Phase 3 Total** | **~4,000** | **~21 days** |

### Phase 4: Advanced

| Component | Est. Lines (Rust) | Person-Days |
|-----------|------------------|-------------|
| Tables | 800 | 5 |
| Group transforms | 300 | 2 |
| Connectors | 400 | 3 |
| Remaining preset geometries | 3,000 | 10 |
| Custom geometries | 500 | 3 |
| Image handling (embed/base64) | 200 | 1 |
| **Phase 4 Total** | **~5,200** | **~24 days** |

---

## Hotspot Analysis

### Most Complex Functions (by cyclomatic complexity)

| Function Area | Est. Cyclomatic Complexity | Risk Level |
|--------------|---------------------------|-----------|
| Color modifier application | 40+ | Critical |
| Text property cascade resolution | 35+ | Critical |
| Preset geometry path generation | 30+ per shape | High |
| Placeholder matching & inheritance | 25+ | High |
| Gradient fill rendering | 20+ | Medium |
| Table cell merge resolution | 20+ | Medium |
| XML element dispatch (readElement) | 100+ (switch) | High (but mechanical) |

### Critical Path Dependencies

```
Theme parsing ──────────────────┐
                                ▼
Slide/Layout/Master parsing ──► Color resolution ──► HTML rendering
                                ▲
ClrMap resolution ──────────────┘
```

Color resolution is on the critical path for nearly all visual output. Implementing it correctly and early is essential.

---

## Code Reuse Opportunities

### What we can reference but NOT copy (AGPL license)

- Algorithm structures for color modifier math
- Placeholder matching logic patterns
- Preset geometry adjustment formulas
- Text inheritance cascade order

### What we must implement independently

- All Rust code (clean-room implementation)
- XML parsing using quick-xml (different API than ONLYOFFICE's custom parser)
- HTML/CSS output generation
- SVG path generation for preset geometries
