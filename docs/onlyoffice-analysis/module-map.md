# ONLYOFFICE PPTXFormat Module Structure

> Analysis of the ONLYOFFICE Presentation SDK's PPTX parsing modules
> Source: `sdkjs/slide/Editor/Format/` and `sdkjs/common/DocFormat/`

---

## High-Level Architecture

ONLYOFFICE's PPTX handling is split across several layers:

```
┌─────────────────────────────────────────────────┐
│                    SDK API                        │
│  (CPresentation, CSlide, CShape, etc.)           │
├─────────────────────────────────────────────────┤
│               Format Layer                        │
│  (PPTX reader/writer, XML parsing)               │
├────────────────────┬────────────────────────────┤
│   Presentation     │    Drawing (DrawingML)      │
│   Specific (PML)   │    Shared across doc types  │
├────────────────────┴────────────────────────────┤
│              Common Utilities                     │
│  (Color, Math, XML helpers)                      │
└─────────────────────────────────────────────────┘
```

---

## Core Module Map

### Presentation-Specific Modules

| Module Path | Primary Responsibility |
|------------|----------------------|
| `slide/Editor/Format/Presentation.js` | CPresentation root object, slide management, theme references |
| `slide/Editor/Format/Slide.js` | CSlide object, shape tree, background, layout reference |
| `slide/Editor/Format/SlideLayout.js` | CSlideLayout, placeholder definitions, master reference |
| `slide/Editor/Format/SlideMaster.js` | CSlideMaster, clrMap, txStyles, theme reference |
| `slide/Editor/Format/Placeholder.js` | Placeholder resolution, type/idx matching logic |

### Shape & Drawing Modules

| Module Path | Primary Responsibility |
|------------|----------------------|
| `common/DocFormat/Format/Shape.js` | CShape base, spPr, txBody integration |
| `common/DocFormat/Format/GroupShape.js` | CGroupShape, child coordinate transforms |
| `common/DocFormat/Format/Image.js` | CImage/CPicture, blipFill handling |
| `common/DocFormat/Format/Connector.js` | CConnector, connection site resolution |
| `common/DocFormat/Format/GraphicFrame.js` | CGraphicFrame, table/chart container |
| `common/DocFormat/Format/ChartSpace.js` | Chart parsing and rendering model |
| `common/DocFormat/Format/Table.js` | CTable, grid/row/cell structure |

### Text Modules

| Module Path | Primary Responsibility |
|------------|----------------------|
| `common/DocFormat/Format/TextBody.js` | CTextBody, bodyPr, lstStyle |
| `common/DocFormat/Format/Paragraph.js` | CParagraph, pPr, bullet handling |
| `common/DocFormat/Format/Run.js` | CRun, rPr, text content |
| `common/DocFormat/Format/ParagraphProperties.js` | CParaProps, spacing, alignment, indent |
| `common/DocFormat/Format/RunProperties.js` | CRunProps, font, size, bold, italic, color |
| `common/DocFormat/Format/TextBodyProperties.js` | CBodyPr, insets, anchor, autofit, columns |
| `common/DocFormat/Format/ListStyle.js` | CLstStyle, level-based paragraph properties |

### Color & Fill Modules

| Module Path | Primary Responsibility |
|------------|----------------------|
| `common/DocFormat/Format/Fill.js` | CFill, unified fill interface (solid/gradient/blip/pattern/noFill) |
| `common/DocFormat/Format/SolidFill.js` | CSolidFill → single color |
| `common/DocFormat/Format/GradientFill.js` | CGradFill → gradient stops, linear/path |
| `common/DocFormat/Format/BlipFill.js` | CBlipFill → image fill, stretch/tile |
| `common/DocFormat/Format/PatternFill.js` | CPattFill → pattern preset |
| `common/DocFormat/Format/Color.js` | CColor, scheme resolution, modifier application |
| `common/DocFormat/Format/ColorModifiers.js` | Tint/shade/lumMod/satMod etc. computation |
| `common/DocFormat/Format/SchemeColor.js` | CSchemeClr, clrMap lookup |
| `common/DocFormat/Format/ThemeColor.js` | Theme color scheme access |

### Geometry Modules

| Module Path | Primary Responsibility |
|------------|----------------------|
| `common/DocFormat/Format/Geometry.js` | CGeometry base, custom geometry support |
| `common/DocFormat/Format/PresetGeometry.js` | Preset shape definitions, path generation |
| `common/DocFormat/Format/AdjustValue.js` | Adjustment handle value management |
| `common/DocFormat/Format/GeometryFormula.js` | Guide formula evaluation engine |
| `common/DocFormat/Format/Path.js` | Shape path (moveTo, lineTo, arcTo, etc.) |

### Theme & Style Modules

| Module Path | Primary Responsibility |
|------------|----------------------|
| `common/DocFormat/Format/Theme.js` | CTheme, color scheme, font scheme, format scheme |
| `common/DocFormat/Format/FontScheme.js` | Major/minor font families, script mappings |
| `common/DocFormat/Format/FormatScheme.js` | Fill/line/effect style lists |
| `common/DocFormat/Format/StyleRef.js` | Style reference resolution (fillRef, lnRef, etc.) |
| `common/DocFormat/Format/TableStyle.js` | Table style definitions |

### Line & Effect Modules

| Module Path | Primary Responsibility |
|------------|----------------------|
| `common/DocFormat/Format/Line.js` | CLn, width, dash, cap, join, head/tail end |
| `common/DocFormat/Format/EffectList.js` | CEffectLst, shadow/glow/reflection |
| `common/DocFormat/Format/OuterShadow.js` | Drop shadow parameters |
| `common/DocFormat/Format/Transform.js` | CXfrm, position/size/rotation/flip |

### PPTX Reader/Writer

| Module Path | Primary Responsibility |
|------------|----------------------|
| `slide/Editor/Format/PptxReader.js` | Main PPTX reading orchestrator |
| `slide/Editor/Format/PptxWriter.js` | Main PPTX writing orchestrator |
| `slide/Editor/Format/XmlReader.js` | XML element → model object parsing |
| `slide/Editor/Format/XmlWriter.js` | Model object → XML element serialization |
| `slide/Editor/Format/RelationshipManager.js` | .rels file management, rId resolution |
| `slide/Editor/Format/ContentTypes.js` | [Content_Types].xml management |

---

## Data Flow: PPTX → Internal Model

```
ZIP file
  │
  ├─ [Content_Types].xml → ContentTypes parser
  ├─ _rels/.rels → RelationshipManager
  │
  ├─ ppt/presentation.xml
  │   │
  │   └─ PptxReader.readPresentation()
  │       ├─ Parse sldSz → presentation dimensions
  │       ├─ Parse sldIdLst → slide references
  │       ├─ For each slide reference:
  │       │   ├─ Read slide .rels → get layout reference
  │       │   ├─ Read layout .rels → get master reference
  │       │   ├─ Read master .rels → get theme reference
  │       │   ├─ Parse theme → CTheme
  │       │   ├─ Parse master → CSlideMaster (clrMap, txStyles, shapes)
  │       │   ├─ Parse layout → CSlideLayout (placeholders, shapes)
  │       │   └─ Parse slide → CSlide (content shapes)
  │       └─ defaultTextStyle → global text defaults
  │
  ├─ ppt/theme/theme1.xml → Theme parser
  │   ├─ clrScheme → 12 theme colors
  │   ├─ fontScheme → major/minor fonts
  │   └─ fmtScheme → fill/line/effect/bgFill style lists
  │
  ├─ ppt/slideMasters/slideMaster1.xml → SlideMaster parser
  │   ├─ cSld → background + shape tree
  │   ├─ clrMap → scheme-to-theme mapping
  │   ├─ txStyles → title/body/other text styles
  │   └─ sldLayoutIdLst → layout references
  │
  ├─ ppt/slideLayouts/slideLayout{N}.xml → SlideLayout parser
  │   ├─ cSld → shape tree (placeholders)
  │   ├─ clrMapOvr → optional color map override
  │   └─ showMasterSp → master shape visibility
  │
  └─ ppt/slides/slide{N}.xml → Slide parser
      ├─ cSld → shape tree (content)
      ├─ clrMapOvr → optional color map override
      ├─ transition → slide transition
      └─ timing → animations
```

---

## Shape Parsing Call Chain

```
XmlReader.readShape(xmlNode)
  │
  ├─ readNvSpPr() → { cNvPr, cNvSpPr, nvPr }
  │   └─ nvPr.ph → placeholder info
  │
  ├─ readSpPr() → { xfrm, geom, fill, ln, effectLst }
  │   ├─ readXfrm() → { off, ext, rot, flipH, flipV }
  │   ├─ readGeometry() → prstGeom or custGeom
  │   ├─ readFill() → solidFill / gradFill / blipFill / pattFill / noFill / grpFill
  │   ├─ readLn() → { w, cap, cmpd, fill, dash, join, headEnd, tailEnd }
  │   └─ readEffectLst() → [ outerShdw, innerShdw, glow, ... ]
  │
  ├─ readTxBody() → { bodyPr, lstStyle, paragraphs[] }
  │   ├─ readBodyPr() → { vert, wrap, insets, anchor, autofit }
  │   ├─ readLstStyle() → lvl1pPr through lvl9pPr
  │   └─ readParagraph() → { pPr, runs[], endParaRPr }
  │       ├─ readPPr() → { lvl, algn, mar, indent, spc, bullet }
  │       └─ readRun() → { rPr, text }
  │           └─ readRPr() → { lang, sz, b, i, u, strike, fill, font, ... }
  │
  └─ readStyle() → { lnRef, fillRef, effectRef, fontRef }
```

---

## Key Design Patterns in ONLYOFFICE

### 1. Lazy Resolution

Properties are not resolved at parse time. Instead, the model stores raw values and resolves them at render time through the inheritance chain.

### 2. Unified Color Interface

All color types (srgb, scheme, sys, hsl, preset, scrgb) are wrapped in a `CUniColor` object that handles resolution uniformly.

### 3. Modifier Chain

Color modifiers are stored as an ordered list and applied sequentially during color resolution.

### 4. Geometry Formula Engine

Preset geometries are defined as formula-based path descriptions. The formula engine evaluates guide values based on shape dimensions and adjustment parameters.

### 5. Placeholder Shadowing

Each placeholder stores a reference to its "parent" placeholder (from layout/master), enabling property fallback without traversing the tree at every access.
