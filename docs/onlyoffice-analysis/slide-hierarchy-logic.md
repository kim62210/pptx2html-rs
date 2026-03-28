# Slide Hierarchy Logic (ONLYOFFICE Analysis)

> How ONLYOFFICE builds and traverses the Slide → Layout → Master → Theme hierarchy

---

## Hierarchy Construction

### Phase 1: Parse Presentation Structure

```
1. Open PPTX ZIP archive
2. Parse [Content_Types].xml → content type registry
3. Parse _rels/.rels → find presentation.xml path
4. Parse ppt/presentation.xml:
   a. sldSz → presentation dimensions
   b. sldMasterIdLst → master slide references (rId)
   c. sldIdLst → slide references (rId, order)
   d. notesMasterIdLst → notes master (if present)
   e. defaultTextStyle → global text defaults
```

### Phase 2: Build Master Chain

For each slide master:

```
1. Parse ppt/_rels/presentation.xml.rels → resolve master rId to file path
2. Parse slideMaster{N}.xml:
   a. cSld.bg → master background
   b. cSld.spTree → master shapes (non-placeholder + placeholder templates)
   c. clrMap → color map (bg1→lt1, tx1→dk1, etc.)
   d. txStyles → titleStyle, bodyStyle, otherStyle
   e. sldLayoutIdLst → layout references
3. Parse slideMaster{N}.xml.rels → get theme reference
4. Parse theme{N}.xml:
   a. clrScheme → 12 theme colors
   b. fontScheme → major/minor fonts with script variants
   c. fmtScheme → fill/line/effect/bgFill style lists
```

### Phase 3: Build Layout Chain

For each layout referenced by a master:

```
1. Parse slideLayout{N}.xml:
   a. type → layout type identifier
   b. cSld.spTree → placeholder shapes (position/style templates)
   c. clrMapOvr → color map override (or masterClrMapping)
   d. showMasterSp → whether to show master shapes
2. Parse slideLayout{N}.xml.rels → get master reference (back-link)
3. Link layout.master → resolved master object
```

### Phase 4: Build Slide Chain

For each slide:

```
1. Parse slide{N}.xml:
   a. cSld.bg → slide background (or inherit)
   b. cSld.spTree → content shapes
   c. clrMapOvr → color map override
   d. showMasterSp → whether to show master shapes
   e. transition → slide transition
   f. timing → animations
2. Parse slide{N}.xml.rels:
   a. Get layout reference → link slide.layout
   b. Get image/media references
   c. Get notes slide reference (if present)
3. Through layout, inherit:
   slide.master = slide.layout.master
   slide.theme = slide.master.theme
```

---

## Object Reference Graph

```
CPresentation
├─ dimensions: (cx, cy)
├─ defaultTextStyle: CListStyle
├─ masters[]: CSlideMaster[]
│  ├─ CSlideMaster
│  │  ├─ theme: CTheme
│  │  │  ├─ clrScheme: { dk1, lt1, dk2, lt2, accent1-6, hlink, folHlink }
│  │  │  ├─ fontScheme: { major: FontCollection, minor: FontCollection }
│  │  │  └─ fmtScheme: { fillStyleLst, lnStyleLst, effectStyleLst, bgFillStyleLst }
│  │  ├─ clrMap: { bg1→lt1, tx1→dk1, bg2→lt2, tx2→dk2, ... }
│  │  ├─ txStyles: { titleStyle, bodyStyle, otherStyle }
│  │  ├─ background: CBg
│  │  ├─ shapes[]: CShape[] (master-level shapes)
│  │  └─ layouts[]: CSlideLayout[]
│  │     ├─ CSlideLayout
│  │     │  ├─ master: → CSlideMaster (back-reference)
│  │     │  ├─ type: string
│  │     │  ├─ clrMapOvr: Option<ClrMapOverride>
│  │     │  ├─ showMasterSp: bool
│  │     │  ├─ background: Option<CBg>
│  │     │  └─ shapes[]: CShape[] (placeholder templates)
│  │     └─ ...
│  └─ ...
└─ slides[]: CSlide[]
   ├─ CSlide
   │  ├─ layout: → CSlideLayout (reference)
   │  ├─ clrMapOvr: Option<ClrMapOverride>
   │  ├─ showMasterSp: bool
   │  ├─ background: Option<CBg>
   │  └─ shapes[]: CShape[] (content)
   └─ ...
```

---

## Placeholder Resolution Algorithm

### Step 1: Build Placeholder Index

When a layout or master is parsed, ONLYOFFICE builds a lookup map:

```
Layout placeholder index:
  (type="title", idx=None) → LayoutShape#2
  (type="body", idx=1)     → LayoutShape#3
  (type="body", idx=2)     → LayoutShape#4
  (type="dt", idx=10)      → LayoutShape#5
  (type="ftr", idx=11)     → LayoutShape#6
  (type="sldNum", idx=12)  → LayoutShape#7

Master placeholder index:
  (type="title", idx=None) → MasterShape#2
  (type="body", idx=1)     → MasterShape#3
  (type="dt", idx=10)      → MasterShape#4
  (type="ftr", idx=11)     → MasterShape#5
  (type="sldNum", idx=12)  → MasterShape#6
```

### Step 2: Match Slide Placeholders

For each placeholder shape on a slide:

```rust
fn find_parent_placeholder(
    slide_ph: &Placeholder,
    layout_shapes: &[Shape],
    master_shapes: &[Shape],
) -> (Option<&Shape>, Option<&Shape>) {
    let layout_match = find_match(slide_ph, layout_shapes);
    let master_match = find_match(slide_ph, master_shapes);
    (layout_match, master_match)
}

fn find_match(ph: &Placeholder, shapes: &[Shape]) -> Option<&Shape> {
    // Priority 1: Match both type and idx
    if let (Some(ph_type), Some(ph_idx)) = (&ph.r#type, &ph.idx) {
        for shape in shapes {
            if let Some(ref target_ph) = shape.placeholder {
                if target_ph.r#type.as_deref() == Some(ph_type)
                    && target_ph.idx == Some(*ph_idx)
                {
                    return Some(shape);
                }
            }
        }
    }

    // Priority 2: Match type only (for title, ctrTitle, subTitle, dt, ftr, sldNum)
    if let Some(ph_type) = &ph.r#type {
        for shape in shapes {
            if let Some(ref target_ph) = shape.placeholder {
                if target_ph.r#type.as_deref() == Some(ph_type) {
                    return Some(shape);
                }
            }
        }
    }

    // Priority 3: Match idx only (for body placeholders with no type)
    if let Some(ph_idx) = &ph.idx {
        for shape in shapes {
            if let Some(ref target_ph) = shape.placeholder {
                if target_ph.idx == Some(*ph_idx)
                    && target_ph.r#type.is_none()
                {
                    return Some(shape);
                }
            }
        }
    }

    None
}
```

### Step 3: Property Cascade

For each property, the cascade checks in order:

```
function getEffectiveTransform(slideShape, layoutMatch, masterMatch):
    if slideShape.spPr.xfrm is defined:
        return slideShape.spPr.xfrm
    if layoutMatch and layoutMatch.spPr.xfrm is defined:
        return layoutMatch.spPr.xfrm
    if masterMatch and masterMatch.spPr.xfrm is defined:
        return masterMatch.spPr.xfrm
    return DEFAULT_TRANSFORM
```

---

## Text Style Cascade

### For Placeholder Shapes

The text style cascade is the most complex part of the hierarchy:

```
Level: a paragraph's indent level (a:pPr lvl="N", 0-indexed)

For a text run in a paragraph at level N:

1. Run's own a:rPr
2. Paragraph's a:pPr/a:defRPr
3. Shape's txBody/a:lstStyle/a:lvl{N+1}pPr/a:defRPr
4. Slide placeholder match: (no separate text style)
5. Layout placeholder's txBody/a:lstStyle/a:lvl{N+1}pPr/a:defRPr
6. Master placeholder's txBody/a:lstStyle/a:lvl{N+1}pPr/a:defRPr
7. Master txStyles:
   - For title/ctrTitle placeholders → p:titleStyle/a:lvl{N+1}pPr
   - For body/subTitle/obj placeholders → p:bodyStyle/a:lvl{N+1}pPr
   - For other/non-placeholder → p:otherStyle/a:lvl{N+1}pPr
8. Presentation defaultTextStyle/a:lvl{N+1}pPr
9. Hardcoded defaults
```

### For Non-Placeholder Shapes

```
1. Run's own a:rPr
2. Paragraph's a:pPr/a:defRPr
3. Shape's txBody/a:lstStyle/a:lvl{N+1}pPr/a:defRPr
4. Master otherStyle/a:lvl{N+1}pPr
5. Presentation defaultTextStyle/a:lvl{N+1}pPr
6. Hardcoded defaults
```

### Per-Property Independence

Each text property cascades independently:

```
Effective fontSize = run.rPr.sz ?? para.defRPr.sz ?? lstStyle.lvlNpPr.sz ?? ...
Effective bold     = run.rPr.b ?? para.defRPr.b ?? lstStyle.lvlNpPr.b ?? ...
Effective font     = run.rPr.latin ?? para.defRPr.latin ?? lstStyle.lvlNpPr.latin ?? ...
Effective color    = run.rPr.solidFill ?? para.defRPr.solidFill ?? ...
```

---

## Color Map Resolution Chain

```
function getEffectiveClrMap(slide):
    // Check slide override
    if slide.clrMapOvr:
        if slide.clrMapOvr is overrideClrMapping:
            return slide.clrMapOvr  // full replacement
        // else masterClrMapping → fall through

    // Check layout override
    layout = slide.layout
    if layout.clrMapOvr:
        if layout.clrMapOvr is overrideClrMapping:
            return layout.clrMapOvr
        // else masterClrMapping → fall through

    // Use master's clrMap
    return slide.layout.master.clrMap
```

---

## Background Resolution

```
function getEffectiveBackground(slide):
    if slide.background is defined:
        return slide.background

    layout = slide.layout
    if layout.background is defined:
        return layout.background

    master = layout.master
    if master.background is defined:
        return master.background

    return DEFAULT_WHITE_BACKGROUND
```

---

## Master Shape Visibility

### showMasterSp Flag

This flag controls whether the slide master's non-placeholder shapes are rendered:

```
function shouldShowMasterShapes(slide):
    // Default is true if attribute is absent
    return slide.showMasterSp ?? true

function shouldShowLayoutMasterShapes(layout):
    return layout.showMasterSp ?? true
```

### Rendering Order for a Slide

```
function getVisibleShapes(slide):
    result = []

    // 1. Master background (always applied unless overridden)
    background = getEffectiveBackground(slide)

    // 2. Master non-placeholder shapes (if enabled)
    if shouldShowMasterShapes(slide) && shouldShowLayoutMasterShapes(slide.layout):
        for shape in slide.layout.master.shapes:
            if not shape.isPlaceholder():
                result.push(shape)

    // 3. Layout non-placeholder shapes
    for shape in slide.layout.shapes:
        if not shape.isPlaceholder():
            result.push(shape)

    // 4. Slide shapes (all - including placeholder content)
    for shape in slide.shapes:
        result.push(shape)

    return result
```

---

## Theme Resolution

### Font Theme References

When a font typeface starts with `+`, it references the theme:

```
function resolveThemeFont(typeface, theme, script):
    if typeface starts with "+mj-":
        collection = theme.fontScheme.majorFont
    elif typeface starts with "+mn-":
        collection = theme.fontScheme.minorFont
    else:
        return typeface  // literal font name

    suffix = typeface[4..]  // "lt", "ea", "cs"

    switch suffix:
        case "lt": return collection.latin.typeface
        case "ea":
            // Check for script-specific override
            if script and collection.fonts[script]:
                return collection.fonts[script].typeface
            return collection.ea.typeface
        case "cs": return collection.cs.typeface
```

### Style Reference Resolution

```
function resolveStyleRef(styleRef, theme, contextColor):
    // styleRef has: idx (integer), color (schemeClr or other)
    idx = styleRef.idx

    switch styleRef.type:
        case "fillRef":
            if idx >= 1001:
                fill = theme.fmtScheme.bgFillStyleLst[idx - 1001]
            else:
                fill = theme.fmtScheme.fillStyleLst[idx - 1]
            // Replace phClr with styleRef's color
            replacePHClr(fill, styleRef.color)
            return fill

        case "lnRef":
            line = theme.fmtScheme.lnStyleLst[idx - 1]
            replacePHClr(line, styleRef.color)
            return line

        case "effectRef":
            effect = theme.fmtScheme.effectStyleLst[idx - 1]
            replacePHClr(effect, styleRef.color)
            return effect

        case "fontRef":
            // idx is "major" or "minor"
            return theme.fontScheme[idx]
```

---

## Implementation Strategy

### Recommended Data Model (Rust)

```rust
struct Presentation {
    slide_size: (i64, i64),
    default_text_style: Option<ListStyle>,
    slides: Vec<Slide>,
    // Masters and layouts are owned here, slides reference by index
    masters: Vec<SlideMaster>,
    layouts: Vec<SlideLayout>,
    themes: Vec<Theme>,
}

struct Slide {
    layout_idx: usize,  // index into Presentation.layouts
    background: Option<Background>,
    clr_map_ovr: Option<ClrMapOverride>,
    show_master_sp: bool,
    shapes: Vec<Shape>,
}

struct SlideLayout {
    master_idx: usize,  // index into Presentation.masters
    layout_type: String,
    background: Option<Background>,
    clr_map_ovr: Option<ClrMapOverride>,
    show_master_sp: bool,
    shapes: Vec<Shape>,
}

struct SlideMaster {
    theme_idx: usize,  // index into Presentation.themes
    background: Option<Background>,
    clr_map: ClrMap,
    tx_styles: TxStyles,
    shapes: Vec<Shape>,
}

struct Theme {
    clr_scheme: ColorScheme,
    font_scheme: FontScheme,
    fmt_scheme: FormatScheme,
}
```

### Index-Based References

Using indices instead of reference-counted pointers avoids Rust lifetime complexity and enables easy serialization. The presentation owns all masters, layouts, and themes; slides, layouts, and masters reference them by index.

```
Slide[0].layout_idx = 2 → Presentation.layouts[2]
Layout[2].master_idx = 0 → Presentation.masters[0]
Master[0].theme_idx = 0 → Presentation.themes[0]
```
