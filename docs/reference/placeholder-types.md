# Placeholder Types Reference

> ECMA-376 `p:ph` element `type` attribute values and their behavior

---

## Overview

Placeholders are special shapes that participate in the slide-layout-master inheritance chain. They are identified by the `p:ph` element inside `p:nvPr`.

```xml
<p:nvSpPr>
  <p:cNvPr id="2" name="Title 1"/>
  <p:cNvSpPr>
    <a:spLocks noGrp="1"/>
  </p:cNvSpPr>
  <p:nvPr>
    <p:ph type="title" idx="0"/>
  </p:nvPr>
</p:nvSpPr>
```

---

## Placeholder Type Values

### Content Placeholders

| type | Name | Description | Typical idx | Text Style Source |
|------|------|-------------|-------------|------------------|
| `title` | Title | Slide title | 0 | `p:titleStyle` |
| `ctrTitle` | Center Title | Centered title (title slide) | 0 | `p:titleStyle` |
| `subTitle` | Subtitle | Subtitle text (title slide) | 1 | `p:bodyStyle` |
| `body` | Body | Main content area | 1+ | `p:bodyStyle` |
| `obj` | Object | Generic object placeholder | varies | `p:bodyStyle` |
| `tbl` | Table | Table placeholder | varies | `p:bodyStyle` |
| `chart` | Chart | Chart placeholder | varies | `p:bodyStyle` |
| `dgm` | Diagram | SmartArt diagram placeholder | varies | `p:bodyStyle` |
| `media` | Media | Audio/video placeholder | varies | `p:bodyStyle` |
| `clipArt` | Clip Art | Clip art placeholder (legacy) | varies | `p:bodyStyle` |
| `pic` | Picture | Picture placeholder | varies | `p:bodyStyle` |

### Metadata Placeholders

| type | Name | Description | Typical idx | Typical Position |
|------|------|-------------|-------------|-----------------|
| `dt` | Date/Time | Date and time field | 10 | Bottom-left |
| `ftr` | Footer | Footer text | 11 | Bottom-center |
| `sldNum` | Slide Number | Slide number field | 12 | Bottom-right |
| `hdr` | Header | Header (notes/handout only) | varies | Top area |

### Special Placeholders

| type | Name | Description |
|------|------|-------------|
| `sldImg` | Slide Image | Thumbnail of slide (notes page) |
| (omitted) | Body (implicit) | When `type` is omitted, treated as `body` |

---

## Detailed Placeholder Descriptions

### title

The primary title of a slide. Each slide typically has at most one title placeholder.

```xml
<p:nvPr>
  <p:ph type="title"/>
</p:nvPr>
```

- Text style inherited from master `p:titleStyle`
- Default font: Major theme font (`+mj-lt`)
- Default size: 44pt (4400 hundredths)
- Default alignment: Left
- Typically positioned at the top of the slide

### ctrTitle

Center-aligned title, used on "Title Slide" layout type.

```xml
<p:nvPr>
  <p:ph type="ctrTitle"/>
</p:nvPr>
```

- Text style inherited from master `p:titleStyle`
- Default alignment: Center
- Typically vertically centered on the slide
- Larger font size than regular title (often 44-60pt)

**Matching note:** `ctrTitle` and `title` are distinct types. A slide using a "Title Slide" layout will have `ctrTitle` (not `title`), and the layout placeholder must also be `ctrTitle`.

### subTitle

Subtitle, typically paired with `ctrTitle` on title slides.

```xml
<p:nvPr>
  <p:ph type="subTitle" idx="1"/>
</p:nvPr>
```

- Text style inherited from master `p:bodyStyle`
- Default font: Minor theme font (`+mn-lt`)
- Default alignment: Center
- Positioned below the center title

### body

Main content placeholder. Can have multiple instances per slide, distinguished by `idx`.

```xml
<!-- First body placeholder -->
<p:nvPr>
  <p:ph type="body" idx="1"/>
</p:nvPr>

<!-- Second body placeholder (e.g., "Two Content" layout) -->
<p:nvPr>
  <p:ph type="body" idx="2"/>
</p:nvPr>
```

- Text style from master `p:bodyStyle`
- Supports bulleted lists with indent levels (lvl 0-8)
- Each level has its own font size, bullet character, and indentation

#### Implicit Body

When `type` is omitted on `p:ph`, the placeholder is treated as `body`:

```xml
<!-- These are equivalent for matching purposes -->
<p:ph idx="1"/>
<p:ph type="body" idx="1"/>
```

### obj (Object)

Generic object placeholder that can contain any content type.

```xml
<p:nvPr>
  <p:ph type="obj" idx="1" sz="half"/>
</p:nvPr>
```

- In PowerPoint, the user can click icons to insert: table, chart, SmartArt, picture, media, or text
- For HTML conversion: treat as text content (the actual content determines the rendering)
- Text style from `p:bodyStyle`

### tbl, chart, dgm, media, clipArt, pic

Content-type-specific placeholders. These indicate what type of content is expected but otherwise behave similarly to `body`/`obj` for text styling.

For pictures:

```xml
<p:nvPr>
  <p:ph type="pic" idx="2"/>
</p:nvPr>
```

In practice, the actual content (a table, chart, picture, etc.) replaces the placeholder prompt, so the `type` mainly affects:
- The icon/prompt displayed in PowerPoint's editing UI
- The inheritance behavior (all inherit from `p:bodyStyle` for text)

---

## Metadata Placeholder Details

### dt (Date/Time)

```xml
<p:nvPr>
  <p:ph type="dt" sz="half" idx="10"/>
</p:nvPr>
```

Can contain either:

**Fixed date text:**
```xml
<p:txBody>
  <a:bodyPr/>
  <a:lstStyle/>
  <a:p>
    <a:fld id="{...}" type="datetimeFigureOut">
      <a:rPr lang="en-US"/>
      <a:t>3/27/2026</a:t>
    </a:fld>
    <a:endParaRPr lang="en-US"/>
  </a:p>
</p:txBody>
```

**Or auto-updating date field:**
```xml
<a:fld id="{...}" type="datetime1">
  <a:rPr lang="en-US"/>
  <a:t>3/27/2026</a:t>
</a:fld>
```

**Date/time field types:**

| type | Format Example |
|------|---------------|
| `datetime1` | 3/27/2026 |
| `datetime2` | Thursday, March 27, 2026 |
| `datetime3` | 27 March 2026 |
| `datetime4` | March 27, 2026 |
| `datetime5` | 27-Mar-26 |
| `datetime6` | March 26 |
| `datetime7` | Mar-26 |
| `datetime8` | 3/27/2026 12:00 AM |
| `datetime9` | 3/27/2026 12:00:00 AM |
| `datetime10` | 12:00 |
| `datetime11` | 12:00:00 |
| `datetime12` | 12:00 AM |
| `datetime13` | 12:00:00 AM |

### ftr (Footer)

```xml
<p:nvPr>
  <p:ph type="ftr" sz="quarter" idx="11"/>
</p:nvPr>
```

- Contains static text defined in presentation properties
- Text from `p:presentation/p:hf` or slide-level override
- Typically small font, centered at bottom

### sldNum (Slide Number)

```xml
<p:nvPr>
  <p:ph type="sldNum" sz="quarter" idx="12"/>
</p:nvPr>
```

Contains a field element with the slide number:

```xml
<a:p>
  <a:fld id="{...}" type="slidenum">
    <a:rPr lang="en-US"/>
    <a:t>&#x2039;#&#x203A;</a:t>  <!-- placeholder text -->
  </a:fld>
  <a:endParaRPr lang="en-US"/>
</a:p>
```

### hdr (Header)

```xml
<p:nvPr>
  <p:ph type="hdr" sz="quarter" idx="0"/>
</p:nvPr>
```

- Only appears on Notes Pages and Handout Master
- Not used on regular slides

### sldImg (Slide Image)

```xml
<p:nvPr>
  <p:ph type="sldImg"/>
</p:nvPr>
```

- Only appears on Notes Pages
- Contains a thumbnail of the associated slide
- For HTML conversion: can be omitted or rendered as a reference

---

## Placeholder Matching Algorithm

### Step 1: Build Match Table

For each placeholder on the slide, find the corresponding placeholder on the layout and master.

```
function find_matching_placeholder(slide_ph, layout_shapes, master_shapes):
    // Try layout first
    for layout_shape in layout_shapes:
        if layout_shape has p:ph:
            if matches(slide_ph, layout_shape.ph):
                return layout_shape

    // Fallback to master
    for master_shape in master_shapes:
        if master_shape has p:ph:
            if matches(slide_ph, master_shape.ph):
                return master_shape

    return None

function matches(ph_a, ph_b):
    if ph_a.type is set AND ph_b.type is set:
        if ph_a.type != ph_b.type:
            return false
    if ph_a.idx is set AND ph_b.idx is set:
        return ph_a.idx == ph_b.idx
    if ph_a.type is set AND ph_b.type is set:
        return ph_a.type == ph_b.type
    return false
```

### Step 2: Inherit Properties

Once matched, inherit properties not defined at the slide level from the layout/master placeholder:

```
function resolve_placeholder_properties(slide_sp, layout_sp, master_sp):
    effective = {}

    // Transform (position/size)
    effective.xfrm = slide_sp.xfrm ?? layout_sp.xfrm ?? master_sp.xfrm

    // Fill
    effective.fill = slide_sp.fill ?? layout_sp.fill ?? master_sp.fill

    // Line
    effective.ln = slide_sp.ln ?? layout_sp.ln ?? master_sp.ln

    // Text body properties
    effective.bodyPr = merge(slide_sp.bodyPr, layout_sp.bodyPr, master_sp.bodyPr)

    // Text content (never inherited - each level has its own)
    effective.paragraphs = slide_sp.paragraphs  // always from slide

    return effective
```

---

## Size Hint (sz attribute)

The `sz` attribute provides a size hint for the placeholder:

| Value | Description | Typical Usage |
|-------|-------------|---------------|
| `full` | Full slide area | Single content area |
| `half` | Half slide area | "Two Content" layout |
| `quarter` | Quarter area | Metadata placeholders (dt, ftr, sldNum) |

This is advisory only; actual size is determined by the `a:xfrm` transform.

---

## Orientation (orient attribute)

| Value | Description |
|-------|-------------|
| `horz` | Horizontal (default) |
| `vert` | Vertical text direction |

---

## Common Layout Configurations

### Title Slide

```
┌────────────────────────────────────────┐
│                                        │
│                                        │
│         ctrTitle (idx=0)               │
│                                        │
│         subTitle (idx=1)               │
│                                        │
│  dt(10)      ftr(11)      sldNum(12)   │
└────────────────────────────────────────┘
```

### Title and Content

```
┌────────────────────────────────────────┐
│  title (idx=0)                         │
├────────────────────────────────────────┤
│                                        │
│  body (idx=1)                          │
│                                        │
│                                        │
│  dt(10)      ftr(11)      sldNum(12)   │
└────────────────────────────────────────┘
```

### Two Content

```
┌────────────────────────────────────────┐
│  title (idx=0)                         │
├───────────────────┬────────────────────┤
│                   │                    │
│  body (idx=1)     │  body (idx=2)      │
│                   │                    │
│                   │                    │
│  dt(10)      ftr(11)      sldNum(12)   │
└────────────────────────────────────────┘
```

### Section Header

```
┌────────────────────────────────────────┐
│                                        │
│                                        │
│                                        │
│  title (idx=0)                         │
│  body (idx=1) - description text       │
│                                        │
│  dt(10)      ftr(11)      sldNum(12)   │
└────────────────────────────────────────┘
```

### Blank

```
┌────────────────────────────────────────┐
│                                        │
│           (no placeholders)            │
│                                        │
│                                        │
│                                        │
│                                        │
│  dt(10)      ftr(11)      sldNum(12)   │
└────────────────────────────────────────┘
```

---

## Implementation Notes

### Placeholder Detection

A shape is a placeholder if and only if it has a `p:ph` element inside `p:nvPr`:

```rust
fn is_placeholder(shape: &Shape) -> bool {
    shape.nv_sp_pr.nv_pr.ph.is_some()
}
```

### Text Style Routing

```rust
fn get_text_style_source(ph_type: Option<&str>) -> TextStyleSource {
    match ph_type {
        Some("title") | Some("ctrTitle") => TextStyleSource::TitleStyle,
        Some("subTitle") | Some("body") | Some("obj") |
        Some("tbl") | Some("chart") | Some("dgm") |
        Some("media") | Some("clipArt") | Some("pic") |
        None => TextStyleSource::BodyStyle,
        Some("dt") | Some("ftr") | Some("sldNum") | Some("hdr") => TextStyleSource::OtherStyle,
        _ => TextStyleSource::OtherStyle,
    }
}
```

### Visibility

Metadata placeholders (dt, ftr, sldNum) may be hidden based on presentation-level settings:

```xml
<!-- In presProps.xml or presentation.xml -->
<p:hf sldNum="1" hdr="0" ftr="1" dt="1"/>
```

| Attribute | Value | Meaning |
|-----------|-------|---------|
| `sldNum="1"` | true | Show slide numbers |
| `ftr="1"` | true | Show footer |
| `dt="1"` | true | Show date/time |
| `hdr="0"` | false | Hide header |
