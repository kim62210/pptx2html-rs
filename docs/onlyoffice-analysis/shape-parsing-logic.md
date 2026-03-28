# Shape Parsing Logic (ONLYOFFICE Analysis)

> How ONLYOFFICE parses shape elements from PPTX XML into its internal model

---

## Shape Type Dispatch

When parsing a slide's `p:spTree`, ONLYOFFICE dispatches based on element name:

```
p:spTree children:
  ├─ p:sp       → parseShape()       → CShape
  ├─ p:pic      → parsePicture()     → CImage
  ├─ p:grpSp    → parseGroupShape()  → CGroupShape
  ├─ p:cxnSp    → parseConnector()   → CConnector
  ├─ p:graphicFrame → parseGraphicFrame() → CGraphicFrame
  │   ├─ a:tbl  → parseTable()       → CTable
  │   ├─ chart  → parseChart()       → CChartSpace
  │   └─ dgm    → parseDiagram()     → CDiagram
  └─ mc:AlternateContent → parseAlternateContent()
```

---

## Shape (p:sp) Parsing

### Full Parse Structure

```
p:sp
├─ p:nvSpPr (non-visual shape properties)
│  ├─ p:cNvPr
│  │  ├─ @id          → shape unique ID (integer)
│  │  ├─ @name        → shape name (string)
│  │  ├─ @descr       → alt text description
│  │  ├─ @hidden      → visibility flag
│  │  └─ a:hlinkClick → hyperlink on click
│  ├─ p:cNvSpPr
│  │  ├─ @txBox       → is this a text box? (boolean)
│  │  └─ a:spLocks    → editing locks
│  │     ├─ @noGrp    → cannot group
│  │     ├─ @noRot    → cannot rotate
│  │     ├─ @noMove   → cannot move
│  │     └─ @noResize → cannot resize
│  └─ p:nvPr
│     ├─ p:ph         → placeholder info
│     │  ├─ @type     → placeholder type
│     │  ├─ @idx      → placeholder index
│     │  ├─ @sz       → size hint
│     │  └─ @orient   → orientation
│     ├─ p:custDataLst → custom data
│     └─ a:audioFile / a:videoFile → media references
│
├─ p:spPr (shape properties)
│  ├─ a:xfrm         → transform (position, size, rotation, flip)
│  │  ├─ @rot        → rotation in 60000ths of degree
│  │  ├─ @flipH      → horizontal flip
│  │  ├─ @flipV      → vertical flip
│  │  ├─ a:off       → position (x, y in EMU)
│  │  └─ a:ext       → size (cx, cy in EMU)
│  ├─ a:prstGeom     → preset geometry
│  │  ├─ @prst       → preset name
│  │  └─ a:avLst     → adjustment values
│  ├─ a:custGeom     → custom geometry (alternative to prstGeom)
│  │  ├─ a:avLst     → adjustment values
│  │  ├─ a:gdLst     → guide definitions
│  │  ├─ a:ahLst     → adjust handle list
│  │  ├─ a:cxnLst    → connection site list
│  │  ├─ a:rect      → text rectangle
│  │  └─ a:pathLst   → path definitions
│  ├─ FILL CHOICE:   → (exactly one)
│  │  ├─ a:solidFill → solid color fill
│  │  ├─ a:gradFill  → gradient fill
│  │  ├─ a:blipFill  → image fill
│  │  ├─ a:pattFill  → pattern fill
│  │  ├─ a:noFill    → no fill
│  │  └─ a:grpFill   → group fill (inherit from group parent)
│  ├─ a:ln           → outline/line
│  │  ├─ @w          → width in EMU
│  │  ├─ @cap        → cap style
│  │  ├─ @cmpd       → compound type
│  │  ├─ FILL        → line fill (solid/grad/noFill)
│  │  ├─ a:prstDash  → dash pattern
│  │  ├─ JOIN        → a:round / a:bevel / a:miter
│  │  ├─ a:headEnd   → start arrowhead
│  │  └─ a:tailEnd   → end arrowhead
│  ├─ a:effectLst    → effect list
│  │  ├─ a:outerShdw → drop shadow
│  │  ├─ a:innerShdw → inner shadow
│  │  ├─ a:glow      → outer glow
│  │  ├─ a:reflection→ reflection
│  │  └─ a:softEdge  → soft edge
│  └─ a:scene3d / a:sp3d → 3D properties (rarely used)
│
├─ p:txBody (text body)
│  ├─ a:bodyPr       → text body properties
│  │  ├─ @vert       → text direction
│  │  ├─ @wrap       → word wrap
│  │  ├─ @lIns       → left inset
│  │  ├─ @tIns       → top inset
│  │  ├─ @rIns       → right inset
│  │  ├─ @bIns       → bottom inset
│  │  ├─ @anchor     → vertical anchor
│  │  ├─ @anchorCtr  → horizontal centering
│  │  ├─ @rot        → text rotation
│  │  ├─ @numCol     → column count
│  │  ├─ @spcCol     → column spacing
│  │  └─ AUTOFIT     → a:noAutofit / a:normAutofit / a:spAutoFit
│  ├─ a:lstStyle     → list style (level-based overrides)
│  │  └─ a:lvl{1-9}pPr → per-level paragraph properties
│  └─ a:p[]          → paragraphs (1 or more)
│     ├─ a:pPr       → paragraph properties
│     │  ├─ @lvl     → indent level (0-8)
│     │  ├─ @algn    → alignment
│     │  ├─ @marL    → left margin
│     │  ├─ @marR    → right margin
│     │  ├─ @indent  → first line indent
│     │  ├─ @rtl     → right-to-left
│     │  ├─ a:lnSpc  → line spacing
│     │  ├─ a:spcBef → space before
│     │  ├─ a:spcAft → space after
│     │  ├─ BULLET   → a:buNone / a:buChar / a:buAutoNum / a:buBlip
│     │  ├─ a:buClr  → bullet color
│     │  ├─ a:buSzPct→ bullet size percentage
│     │  ├─ a:buFont → bullet font
│     │  └─ a:defRPr → default run properties for this paragraph
│     ├─ a:r[]       → text runs
│     │  ├─ a:rPr    → run properties
│     │  │  ├─ @lang     → language
│     │  │  ├─ @sz       → font size (hundredths pt)
│     │  │  ├─ @b        → bold
│     │  │  ├─ @i        → italic
│     │  │  ├─ @u        → underline
│     │  │  ├─ @strike   → strikethrough
│     │  │  ├─ @cap      → capitalization
│     │  │  ├─ @spc      → character spacing
│     │  │  ├─ @baseline → super/subscript offset
│     │  │  ├─ FILL      → text color (solidFill etc.)
│     │  │  ├─ a:latin   → Latin font
│     │  │  ├─ a:ea      → East Asian font
│     │  │  ├─ a:cs      → Complex script font
│     │  │  ├─ a:sym     → Symbol font
│     │  │  ├─ a:hlinkClick → hyperlink
│     │  │  └─ a:effectLst  → text effects
│     │  └─ a:t      → text content (string)
│     ├─ a:br        → line break (with optional rPr)
│     ├─ a:fld       → field (date, slide number, etc.)
│     │  ├─ @id      → field ID
│     │  ├─ @type    → field type
│     │  ├─ a:rPr    → field run properties
│     │  └─ a:t      → field text value
│     └─ a:endParaRPr→ end-of-paragraph run properties
│
└─ p:style (shape style - optional)
   ├─ a:lnRef      → line style reference
   │  ├─ @idx      → format scheme index (1-3)
   │  └─ COLOR     → override color (replaces phClr)
   ├─ a:fillRef    → fill style reference
   │  ├─ @idx      → format scheme index (1-3 or 1001+)
   │  └─ COLOR     → override color
   ├─ a:effectRef  → effect style reference
   │  ├─ @idx      → format scheme index (1-3)
   │  └─ COLOR     → override color
   └─ a:fontRef    → font style reference
      ├─ @idx      → "major" or "minor"
      └─ COLOR     → font color
```

---

## Property Resolution Priority

For each visual property, ONLYOFFICE resolves through this priority chain:

### Fill Resolution

```
1. Shape's own spPr fill (solidFill/gradFill/blipFill/pattFill/noFill)
   ↓ (if absent)
2. Shape's p:style/a:fillRef → theme fmtScheme fillStyleLst[idx]
   (with phClr substituted by fillRef's color child)
   ↓ (if absent)
3. For placeholders: layout placeholder's fill
   ↓ (if absent)
4. For placeholders: master placeholder's fill
   ↓ (if absent)
5. No fill (transparent)
```

### Line Resolution

```
1. Shape's own spPr a:ln
   ↓ (if absent)
2. Shape's p:style/a:lnRef → theme fmtScheme lnStyleLst[idx]
   ↓ (if absent)
3. For placeholders: inherited from layout/master
   ↓ (if absent)
4. No line
```

### Transform Resolution

```
1. Shape's own spPr a:xfrm
   ↓ (if absent)
2. For placeholders: layout placeholder's xfrm
   ↓ (if absent)
3. For placeholders: master placeholder's xfrm
   ↓ (if absent)
4. Default: (0, 0, 0, 0) — invisible
```

### Geometry Resolution

```
1. Shape's own spPr geometry (prstGeom or custGeom)
   ↓ (if absent)
2. For placeholders: layout placeholder's geometry
   ↓ (if absent)
3. For placeholders: master placeholder's geometry
   ↓ (if absent)
4. Default: rect
```

---

## Picture (p:pic) Parsing

Pictures share most structure with shapes, but use `p:blipFill` instead of geometry:

```
p:pic
├─ p:nvPicPr
│  ├─ p:cNvPr (same as shape)
│  ├─ p:cNvPicPr
│  │  └─ a:picLocks
│  │     └─ @noChangeAspect
│  └─ p:nvPr (placeholder info)
├─ p:blipFill
│  ├─ a:blip
│  │  ├─ @r:embed → relationship ID to image file
│  │  ├─ @r:link  → external image link (rare)
│  │  ├─ @cstate  → compression state
│  │  └─ EFFECTS  → image effects (alphaMod, grayscl, duotone, etc.)
│  ├─ a:srcRect   → crop rectangle (l, t, r, b as percentages)
│  └─ FILL MODE   → a:stretch / a:tile
│     ├─ a:stretch → a:fillRect (inset adjustments)
│     └─ a:tile    → @tx, @ty, @sx, @sy, @flip, @algn
└─ p:spPr (same as shape, but geometry often just rect)
```

### Image File Resolution

```
1. Read r:embed value from a:blip (e.g., "rId2")
2. Look up in slide's .rels file: rId2 → "../media/image1.png"
3. Resolve path: ppt/media/image1.png
4. Extract from ZIP archive
5. For HTML: base64-encode or save to output directory
```

---

## Group Shape (p:grpSp) Parsing

```
p:grpSp
├─ p:nvGrpSpPr
│  ├─ p:cNvPr
│  ├─ p:cNvGrpSpPr
│  └─ p:nvPr
├─ p:grpSpPr
│  └─ a:xfrm
│     ├─ a:off   → group position on parent
│     ├─ a:ext   → group size on parent
│     ├─ a:chOff → child space origin
│     └─ a:chExt → child space dimensions
└─ CHILDREN (any of):
   ├─ p:sp
   ├─ p:pic
   ├─ p:grpSp (nested groups)
   ├─ p:cxnSp
   └─ p:graphicFrame
```

### Child Coordinate Transform

Each child's position is in the group's internal coordinate space (`chOff`/`chExt`), which maps to the group's actual space (`off`/`ext`):

```rust
fn transform_child_to_parent(
    child_off: (i64, i64),
    child_ext: (i64, i64),
    grp_off: (i64, i64),
    grp_ext: (i64, i64),
    grp_ch_off: (i64, i64),
    grp_ch_ext: (i64, i64),
) -> ((i64, i64), (i64, i64)) {
    let scale_x = grp_ext.0 as f64 / grp_ch_ext.0 as f64;
    let scale_y = grp_ext.1 as f64 / grp_ch_ext.1 as f64;

    let parent_off = (
        grp_off.0 + ((child_off.0 - grp_ch_off.0) as f64 * scale_x) as i64,
        grp_off.1 + ((child_off.1 - grp_ch_off.1) as f64 * scale_y) as i64,
    );
    let parent_ext = (
        (child_ext.0 as f64 * scale_x) as i64,
        (child_ext.1 as f64 * scale_y) as i64,
    );

    (parent_off, parent_ext)
}
```

---

## Connector (p:cxnSp) Parsing

```
p:cxnSp
├─ p:nvCxnSpPr
│  ├─ p:cNvPr
│  ├─ p:cNvCxnSpPr
│  │  ├─ a:stCxn → start connection
│  │  │  ├─ @id  → connected shape ID
│  │  │  └─ @idx → connection site index
│  │  └─ a:endCxn → end connection
│  │     ├─ @id
│  │     └─ @idx
│  └─ p:nvPr
└─ p:spPr
   ├─ a:xfrm   → bounding box of connector
   ├─ a:prstGeom → connector type (straightConnector1, bentConnector3, etc.)
   └─ a:ln     → line style (width, color, dash, arrows)
```

### Connection Site Mapping

Default connection sites for basic shapes:

```
Rectangle: 0=top-center, 1=right-center, 2=bottom-center, 3=left-center
Ellipse: 0=top, 1=right, 2=bottom, 3=left
Triangle: 0=top, 1=bottom-right, 2=bottom-left
```

For presets with custom connection sites, the sites are defined in the geometry's `a:cxnLst`.

---

## Graphic Frame (p:graphicFrame) Parsing

```
p:graphicFrame
├─ p:nvGraphicFramePr
│  ├─ p:cNvPr
│  ├─ p:cNvGraphicFramePr
│  │  └─ a:graphicFrameLocks
│  └─ p:nvPr
│     └─ p:ph (if placeholder)
├─ p:xfrm → position and size
└─ a:graphic
   └─ a:graphicData
      ├─ @uri → content type URI
      └─ CONTENT (based on URI):
         ├─ a:tbl (table)
         ├─ c:chartSpace (chart)
         ├─ dgm:relIds (diagram/SmartArt)
         └─ mc:AlternateContent
```

---

## Common Parsing Patterns

### Null/Missing Element Handling

ONLYOFFICE uses a consistent pattern: if an XML element is missing, the corresponding model property is `null`/`undefined`, triggering inheritance lookup.

```rust
// Rust equivalent pattern
struct ShapeProperties {
    xfrm: Option<Transform>,      // None = inherit from parent
    fill: Option<Fill>,           // None = inherit
    line: Option<Line>,           // None = inherit
    geometry: Option<Geometry>,   // None = inherit (default rect)
    effect_list: Option<EffectList>,
}
```

### Attribute Parsing with Defaults

```rust
// Pattern for parsing optional integer attributes with defaults
fn parse_emu_attr(elem: &Element, name: &str) -> Option<i64> {
    elem.attribute(name).map(|v| v.parse::<i64>().unwrap_or(0))
}

fn parse_bool_attr(elem: &Element, name: &str) -> Option<bool> {
    elem.attribute(name).map(|v| v == "1" || v == "true")
}

fn parse_percentage_attr(elem: &Element, name: &str) -> Option<u32> {
    elem.attribute(name).map(|v| v.parse::<u32>().unwrap_or(100000))
}
```

### Fill Dispatch

```rust
fn parse_fill(children: &[XmlNode]) -> Option<Fill> {
    for child in children {
        match child.name() {
            "solidFill" => return Some(Fill::Solid(parse_solid_fill(child))),
            "gradFill" => return Some(Fill::Gradient(parse_gradient_fill(child))),
            "blipFill" => return Some(Fill::Blip(parse_blip_fill(child))),
            "pattFill" => return Some(Fill::Pattern(parse_pattern_fill(child))),
            "noFill" => return Some(Fill::None),
            "grpFill" => return Some(Fill::Group),
            _ => continue,
        }
    }
    None  // no fill specified = inherit
}
```
