# Slide Inheritance Rules

> Slide -> Layout -> Master -> Theme property inheritance chain in ECMA-376

---

## Inheritance Hierarchy

```
┌─────────────────┐
│     Theme        │  Font schemes, color schemes, format schemes
│  (theme1.xml)    │
└────────┬─────────┘
         │
┌────────▼─────────┐
│   Slide Master    │  Default styles, backgrounds, common shapes
│ (slideMaster.xml) │  clrMap, txStyles (title/body/other)
└────────┬─────────┘
         │
┌────────▼─────────┐
│   Slide Layout    │  Placeholder positions, local overrides
│ (slideLayout.xml) │  clrMapOvr, showMasterSp
└────────┬─────────┘
         │
┌────────▼─────────┐
│      Slide        │  Actual content, final overrides
│   (slide.xml)     │  clrMapOvr, showMasterSp
└──────────────────┘
```

### Relationship Chain (via .rels)

```
slide1.xml.rels  → references → slideLayout3.xml
slideLayout3.xml.rels → references → slideMaster1.xml
slideMaster1.xml.rels → references → theme1.xml
```

---

## Property Inheritance Rules

### General Principle

Properties are inherited from the nearest ancestor that defines them. A property defined at the slide level overrides the same property from layout, which overrides master, which overrides theme.

```
Effective value = Slide ?? Layout ?? Master ?? Theme ?? Default
```

Where `??` means "if not defined, fall through to next level."

### What Inherits

| Property | Inherits? | Notes |
|----------|-----------|-------|
| Background | Yes | Slide > Layout > Master |
| Shape fill | Partial | Through placeholder matching only |
| Text formatting | Yes | Via txStyles and placeholder matching |
| Color map | Yes | clrMapOvr can override at each level |
| Shapes/content | No | Each level has its own shapes |
| Transitions | No | Slide-specific |
| Animations | No | Slide-specific |

---

## Background Inheritance

### Resolution Order

1. **Slide `p:bg`** - If present, use it
2. **Layout `p:bg`** - If present and slide doesn't define one
3. **Master `p:bg`** - Default background

```xml
<!-- Slide with own background -->
<p:sld>
  <p:cSld>
    <p:bg>
      <p:bgPr>
        <a:solidFill><a:srgbClr val="FF0000"/></a:solidFill>
      </p:bgPr>
    </p:bg>
    <p:spTree>...</p:spTree>
  </p:cSld>
</p:sld>

<!-- Slide inheriting background (no p:bg element) -->
<p:sld>
  <p:cSld>
    <p:spTree>...</p:spTree>
  </p:cSld>
</p:sld>
```

---

## Placeholder Inheritance

### Matching Mechanism

Placeholders are matched across levels using the `type` and `idx` attributes on `p:ph`.

```xml
<!-- Master defines a title placeholder -->
<p:sp>
  <p:nvSpPr>
    <p:cNvPr id="2" name="Title Placeholder 1"/>
    <p:cNvSpPr><a:spLocks noGrp="1"/></p:cNvSpPr>
    <p:nvPr><p:ph type="title"/></p:nvPr>
  </p:nvSpPr>
  <p:spPr>
    <a:xfrm>
      <a:off x="838200" y="365125"/>
      <a:ext cx="10515600" cy="1325563"/>
    </a:xfrm>
  </p:spPr>
  <p:txBody>
    <a:bodyPr vert="horz" anchor="ctr"/>
    <a:lstStyle/>
    <a:p><a:r><a:rPr lang="en-US" sz="4400"/><a:t>Click to edit Master title style</a:t></a:r></a:p>
  </p:txBody>
</p:sp>

<!-- Layout refines position (type="title" matches) -->
<p:sp>
  <p:nvSpPr>
    <p:cNvPr id="2" name="Title 1"/>
    <p:cNvSpPr><a:spLocks noGrp="1"/></p:cNvSpPr>
    <p:nvPr><p:ph type="title"/></p:nvPr>
  </p:nvSpPr>
  <p:spPr>
    <a:xfrm>
      <a:off x="838200" y="365125"/>
      <a:ext cx="10515600" cy="1325563"/>
    </a:xfrm>
  </p:spPr>
  <!-- inherits text style from master if not specified -->
</p:sp>

<!-- Slide uses placeholder (type="title" matches) -->
<p:sp>
  <p:nvSpPr>
    <p:cNvPr id="2" name="Title 1"/>
    <p:cNvSpPr><a:spLocks noGrp="1"/></p:cNvSpPr>
    <p:nvPr><p:ph type="title"/></p:nvPr>
  </p:nvSpPr>
  <p:spPr/>  <!-- inherits position from layout -->
  <p:txBody>
    <a:bodyPr/>
    <a:lstStyle/>
    <a:p><a:r><a:rPr lang="en-US" dirty="0"/><a:t>Actual Title</a:t></a:r></a:p>
  </p:txBody>
</p:sp>
```

### Matching Rules

| Scenario | Match Key |
|----------|-----------|
| Both `type` and `idx` present | Match on both `type` AND `idx` |
| Only `type` present (`idx` omitted) | Match on `type` alone |
| Only `idx` present (`type` omitted) | Match on `idx` alone (body placeholder assumed) |
| Neither present | No matching; standalone shape |

### Special Cases

- `type="title"` and `type="ctrTitle"` - Only one title placeholder per slide; `idx` is typically 0 or omitted
- `type="body"` - Can have multiple; distinguished by `idx` values (1, 2, 3...)
- `type="dt"`, `type="ftr"`, `type="sldNum"` - At most one each per slide; `idx` usually 10, 11, 12

---

## Text Style Inheritance

### txStyles (Master Level)

The slide master defines three text style collections in `p:txStyles`:

```xml
<p:txStyles>
  <p:titleStyle>
    <a:lvl1pPr algn="l">
      <a:defRPr sz="4400" b="0" kern="1200">
        <a:solidFill><a:schemeClr val="tx1"/></a:solidFill>
        <a:latin typeface="+mj-lt"/>
        <a:ea typeface="+mj-ea"/>
        <a:cs typeface="+mj-cs"/>
      </a:defRPr>
    </a:lvl1pPr>
  </p:titleStyle>
  <p:bodyStyle>
    <a:lvl1pPr marL="228600" indent="-228600" algn="l">
      <a:buFont typeface="Arial"/>
      <a:buChar char="&#x2022;"/>
      <a:defRPr sz="2800" kern="1200">
        <a:solidFill><a:schemeClr val="tx1"/></a:solidFill>
        <a:latin typeface="+mn-lt"/>
      </a:defRPr>
    </a:lvl1pPr>
    <a:lvl2pPr marL="685800" indent="-228600" algn="l">
      <a:buFont typeface="Arial"/>
      <a:buChar char="&#x2013;"/>
      <a:defRPr sz="2400" kern="1200">
        <a:solidFill><a:schemeClr val="tx1"/></a:solidFill>
      </a:defRPr>
    </a:lvl2pPr>
    <!-- lvl3pPr through lvl9pPr -->
  </p:bodyStyle>
  <p:otherStyle>
    <a:defPPr>
      <a:defRPr lang="en-US"/>
    </a:defPPr>
    <a:lvl1pPr marL="0" algn="l">
      <a:defRPr sz="1800" kern="1200">
        <a:solidFill><a:schemeClr val="tx1"/></a:solidFill>
      </a:defRPr>
    </a:lvl1pPr>
    <!-- used for non-placeholder shapes -->
  </p:otherStyle>
</p:txStyles>
```

### Text Property Resolution Order

For a text run inside a placeholder shape:

```
1. Run properties (a:rPr) on the text run itself
2. Paragraph default run properties (a:pPr/a:defRPr)
3. Paragraph-level lstStyle in the shape's txBody (a:lstStyle/a:lvl{N}pPr)
4. Slide placeholder shape's spPr/txBody properties
5. Layout placeholder shape's lstStyle/spPr/txBody properties
6. Master placeholder shape's lstStyle/spPr/txBody properties
7. Master txStyles (titleStyle for title ph, bodyStyle for body ph, otherStyle for others)
8. Theme default text style (p:presentation/p:defaultTextStyle)
9. Built-in defaults
```

For a **non-placeholder** shape:

```
1. Run properties (a:rPr)
2. Paragraph default run properties (a:pPr/a:defRPr)
3. Shape's lstStyle
4. Master otherStyle
5. Theme default text style
6. Built-in defaults
```

### Individual Property Resolution

Each text property is resolved independently. A run can inherit `sz` (font size) from one level and `b` (bold) from another.

```xml
<!-- Master titleStyle defines: sz=4400, b=0, fill=tx1, font=+mj-lt -->
<!-- Layout placeholder adds: (nothing new) -->
<!-- Slide run specifies: b=1 -->

<!-- Effective: sz=4400 (from master), b=1 (from slide), fill=tx1 (from master), font=+mj-lt (from master) -->
```

---

## Shape Property Inheritance

### spPr (Shape Properties)

For placeholder shapes, individual `spPr` children inherit independently:

```
Effective xfrm = slide.xfrm ?? layout.xfrm ?? master.xfrm
Effective fill = slide.fill ?? layout.fill ?? master.fill ?? style.fill ?? noFill
Effective ln   = slide.ln ?? layout.ln ?? master.ln ?? style.ln ?? noLine
Effective geom = slide.geom ?? layout.geom ?? master.geom ?? rect
```

**Empty `p:spPr/` means "inherit everything":**

```xml
<!-- Slide: inherits all shape properties from layout/master -->
<p:spPr/>

<!-- Slide: overrides only position, inherits fill/line from layout/master -->
<p:spPr>
  <a:xfrm>
    <a:off x="1000000" y="500000"/>
    <a:ext cx="5000000" cy="3000000"/>
  </a:xfrm>
</p:spPr>

<!-- Slide: overrides fill, inherits position from layout/master -->
<p:spPr>
  <a:solidFill>
    <a:srgbClr val="FF0000"/>
  </a:solidFill>
</p:spPr>
```

---

## Master Shape Visibility

### showMasterSp Attribute

Controls whether the master's non-placeholder shapes are visible on this slide/layout.

```xml
<!-- Layout: show master shapes (default=true) -->
<p:sldLayout showMasterSp="1" type="title">

<!-- Layout: hide master shapes -->
<p:sldLayout showMasterSp="0" type="blank">

<!-- Slide: hide master shapes for this specific slide -->
<p:sld showMasterSp="0">
```

**Default:** `showMasterSp="1"` (show master shapes)

### Rendering Order

When `showMasterSp="1"`:

```
1. Master background
2. Master non-placeholder shapes (behind slide content)
3. Layout non-placeholder shapes
4. Slide shapes (including placeholder content)
```

When `showMasterSp="0"`:

```
1. Inherited or slide background
2. Layout non-placeholder shapes (if slide shows layout shapes)
3. Slide shapes
```

---

## Color Map Inheritance

```
Slide clrMapOvr
    ↓ (if masterClrMapping or absent)
Layout clrMapOvr
    ↓ (if masterClrMapping or absent)
Master clrMap (always present)
```

Each level can either:
- **Override:** `<a:overrideClrMapping .../>` - Completely replaces the color map
- **Inherit:** `<a:masterClrMapping/>` - Uses the inherited mapping
- **Absent:** No `p:clrMapOvr` element - Same as inherit

---

## Theme Inheritance

### Font Resolution

```xml
<!-- In a:rPr -->
<a:latin typeface="+mj-lt"/>  <!-- Major Latin → theme majorFont/latin -->
<a:latin typeface="+mn-lt"/>  <!-- Minor Latin → theme minorFont/latin -->
<a:ea typeface="+mj-ea"/>     <!-- Major East Asian → theme majorFont/ea or script-specific -->
<a:ea typeface="+mn-ea"/>     <!-- Minor East Asian → theme minorFont/ea or script-specific -->
```

Theme font references (starting with `+`) are resolved from the theme's `a:fontScheme`:

| Reference | Theme Path |
|-----------|-----------|
| `+mj-lt` | `a:majorFont/a:latin/@typeface` |
| `+mn-lt` | `a:minorFont/a:latin/@typeface` |
| `+mj-ea` | `a:majorFont/a:ea/@typeface` (or script-specific `a:font`) |
| `+mn-ea` | `a:minorFont/a:ea/@typeface` (or script-specific `a:font`) |
| `+mj-cs` | `a:majorFont/a:cs/@typeface` |
| `+mn-cs` | `a:minorFont/a:cs/@typeface` |

### Style Reference Resolution

Shape style references (`p:style`) resolve through the theme's `a:fmtScheme`:

```xml
<!-- Shape -->
<p:style>
  <a:fillRef idx="1"><a:schemeClr val="accent1"/></a:fillRef>
</p:style>

<!-- Resolution: idx=1 → fmtScheme/fillStyleLst[0] -->
<!-- The schemeClr "accent1" replaces phClr in the fill definition -->
```

---

## Complete Inheritance Example

### Setup

**Master** (`slideMaster1.xml`):
- Background: solid white
- Title placeholder at (838200, 365125) with size (10515600, 1325563)
- Title style: 44pt, Calibri Light, color tx1
- Body style: 28pt, Calibri, color tx1, bullet "&#x2022;"

**Layout** (`slideLayout1.xml` - Title Slide):
- No background override (inherits white from master)
- Title placeholder repositioned to (831850, 1709738) with size (10515600, 2852737)
- Layout overrides title to 48pt centered
- Subtitle placeholder at (831850, 4589463) with size (10515600, 1655762)

**Slide** (`slide1.xml`):
- No background (inherits from layout/master)
- Title placeholder with text "Hello World" (inherits position from layout)
- Title run has `b="1"` (bold override)

### Effective Result

```
Background: white (from master)
Title position: (831850, 1709738) — from layout
Title text: "Hello World" — from slide
Title font: 48pt Calibri Light, bold, color=dk1 — size from layout, bold from slide, rest from master
```

---

## Implementation Checklist

- [ ] Parse relationship chain: slide → layout → master → theme
- [ ] Build placeholder match table (type+idx → shape reference)
- [ ] Implement property-level fallback for spPr children
- [ ] Implement text style cascade (run → paragraph → lstStyle → txStyles → defaultTextStyle)
- [ ] Handle showMasterSp flag for shape visibility
- [ ] Resolve clrMapOvr chain for color mapping
- [ ] Resolve theme font references (+mj-lt, +mn-lt, etc.)
- [ ] Resolve style references (fillRef, lnRef, effectRef, fontRef)
