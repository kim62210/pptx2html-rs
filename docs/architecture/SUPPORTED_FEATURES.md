# ECMA-376 Feature Support Checklist

> Feature implementation status for pptx-to-html
> Status: done | wip | planned | deferred

---

## Legend

| Status | Meaning |
|--------|---------|
| done | Fully implemented and tested |
| wip | Work in progress |
| planned | Planned for implementation |
| deferred | Out of current scope |

---

## Package & Structure

| Feature | Status | Notes |
|---------|--------|-------|
| ZIP archive extraction | planned | `zip` crate |
| [Content_Types].xml parsing | planned | Content type registry |
| Relationship (.rels) parsing | planned | rId resolution |
| Presentation.xml parsing | planned | Slide list, dimensions |
| Slide ordering (sldIdLst) | planned | Preserve slide order |
| Multiple slide masters | planned | |
| Multiple themes | planned | Rare in practice |
| Notes slides | deferred | Not needed for HTML output |
| Handout master | deferred | Print-only feature |
| Custom XML parts | deferred | Application-specific |

---

## Slide Structure

| Feature | Status | Notes |
|---------|--------|-------|
| Slide dimensions (sldSz) | planned | 4:3, 16:9, custom |
| Slide background (solid) | planned | |
| Slide background (gradient) | planned | |
| Slide background (image) | planned | |
| Slide background (pattern) | deferred | Low priority |
| Background style reference | planned | bgRef → theme |
| Slide layout linking | planned | slide → layout .rels |
| Slide master linking | planned | layout → master .rels |
| showMasterSp flag | planned | Master shape visibility |
| clrMapOvr (slide) | planned | Color map override |
| clrMapOvr (layout) | planned | |
| Slide transitions | deferred | CSS animation complexity |
| Slide timing/animations | deferred | Very complex |

---

## Shape Types

| Feature | Status | Notes |
|---------|--------|-------|
| p:sp (shape) | planned | Core shape element |
| p:pic (picture) | planned | Image shapes |
| p:grpSp (group) | planned | Group with child transforms |
| p:cxnSp (connector) | planned | Line connectors |
| p:graphicFrame (table) | planned | Table content |
| p:graphicFrame (chart) | deferred | Very complex, separate effort |
| p:graphicFrame (SmartArt) | deferred | Requires diagram layout engine |
| p:graphicFrame (OLE) | deferred | Not renderable in HTML |
| mc:AlternateContent | planned | Fallback handling |

---

## Shape Properties

| Feature | Status | Notes |
|---------|--------|-------|
| Position (a:off x/y) | planned | EMU → px conversion |
| Size (a:ext cx/cy) | planned | EMU → px conversion |
| Rotation (xfrm rot) | planned | CSS transform: rotate() |
| Flip horizontal | planned | CSS transform: scaleX(-1) |
| Flip vertical | planned | CSS transform: scaleY(-1) |
| Shape name/id (cNvPr) | planned | Accessibility, debugging |
| Alt text (descr) | planned | img alt attribute |
| Hidden shapes | planned | display: none |
| Shape locks | deferred | Editing-only feature |
| Hyperlinks (hlinkClick) | planned | a href on shapes |

---

## Geometry

| Feature | Status | Notes |
|---------|--------|-------|
| rect | planned | CSS only |
| roundRect | planned | CSS border-radius |
| ellipse | planned | CSS border-radius: 50% |
| triangle | planned | SVG/clip-path |
| diamond | planned | SVG/clip-path |
| rightArrow | planned | SVG path |
| Other preset geometries (187 total) | planned | Phased implementation |
| Adjustment values (avLst) | planned | Parameterized shapes |
| Custom geometry (custGeom) | planned | SVG path generation |
| Geometry formula engine | planned | Guide evaluation |
| Connection sites | deferred | Connector routing only |

---

## Fill

| Feature | Status | Notes |
|---------|--------|-------|
| a:noFill | planned | transparent |
| a:solidFill (srgbClr) | planned | Direct RGB |
| a:solidFill (schemeClr) | planned | Theme color resolution |
| a:solidFill (sysClr) | planned | lastClr fallback |
| a:solidFill (prstClr) | planned | Named color lookup |
| a:solidFill (hslClr) | planned | HSL → RGB conversion |
| a:solidFill (scrgbClr) | planned | Linear → sRGB gamma |
| a:gradFill (linear) | planned | CSS linear-gradient |
| a:gradFill (path/radial) | planned | CSS radial-gradient |
| a:gradFill (tileRect) | deferred | Complex tiling |
| a:blipFill (stretch) | planned | background-image + background-size |
| a:blipFill (tile) | planned | background-repeat |
| a:blipFill (srcRect crop) | planned | object-position + clip |
| a:pattFill | deferred | SVG pattern generation |
| a:grpFill | planned | Inherit from group |
| Fill from style reference | planned | fillRef → theme fmtScheme |

---

## Line/Outline

| Feature | Status | Notes |
|---------|--------|-------|
| Line width | planned | border-width |
| Line color (solid) | planned | border-color |
| Line dash pattern | planned | border-style / SVG stroke-dasharray |
| Line cap (flat/round/square) | planned | SVG stroke-linecap |
| Line join (round/bevel/miter) | planned | SVG stroke-linejoin |
| Compound line (double, etc.) | deferred | Complex CSS |
| Arrow head end | planned | SVG marker |
| Arrow tail end | planned | SVG marker |
| No line | planned | border: none |
| Line from style reference | planned | lnRef → theme |

---

## Color

| Feature | Status | Notes |
|---------|--------|-------|
| srgbClr (direct RGB) | planned | |
| schemeClr resolution | planned | ClrMap → Theme chain |
| sysClr (system color) | planned | lastClr fallback |
| prstClr (preset/named) | planned | 149 named colors |
| hslClr | planned | HSL conversion |
| scrgbClr | planned | sRGB gamma correction |
| ClrMap (master) | planned | Scheme → theme mapping |
| ClrMap override (slide/layout) | planned | |
| Modifier: tint | planned | |
| Modifier: shade | planned | |
| Modifier: lumMod | planned | HSL luminance multiply |
| Modifier: lumOff | planned | HSL luminance offset |
| Modifier: satMod | planned | HSL saturation multiply |
| Modifier: satOff | planned | HSL saturation offset |
| Modifier: alpha | planned | CSS opacity/rgba |
| Modifier: alphaOff | planned | |
| Modifier: alphaMod | planned | |
| Modifier: hueMod | planned | |
| Modifier: hueOff | planned | |
| Modifier: inv | planned | Inverse (XOR 0xFF) |
| Modifier: comp | planned | Complementary hue |
| Modifier: gray | planned | Grayscale |
| Modifier: red/green/blue | deferred | Rarely used |
| phClr substitution | planned | Style reference context |

---

## Text

| Feature | Status | Notes |
|---------|--------|-------|
| Text body (txBody) | planned | |
| Paragraphs (a:p) | planned | |
| Text runs (a:r) | planned | |
| Line breaks (a:br) | planned | |
| Fields (a:fld) - slide number | planned | |
| Fields (a:fld) - date/time | planned | |
| End paragraph properties | planned | |
| **Paragraph Properties** | | |
| Alignment (algn) | planned | text-align |
| Indent level (lvl) | planned | Bullet level |
| Left margin (marL) | planned | margin-left / padding-left |
| Right margin (marR) | planned | |
| First line indent | planned | text-indent |
| Line spacing (lnSpc) | planned | line-height |
| Space before (spcBef) | planned | margin-top / padding-top |
| Space after (spcAft) | planned | margin-bottom |
| Right-to-left | planned | direction: rtl |
| Default tab size | deferred | |
| Tab stops | deferred | |
| **Bullet/Numbering** | | |
| Character bullet (buChar) | planned | CSS ::before or list-style |
| Auto-number bullet (buAutoNum) | planned | CSS counter |
| Image bullet (buBlip) | deferred | |
| No bullet (buNone) | planned | |
| Bullet color (buClr) | planned | |
| Bullet size (buSzPct) | planned | |
| Bullet font (buFont) | planned | |
| **Run Properties** | | |
| Font size (sz) | planned | font-size |
| Bold (b) | planned | font-weight |
| Italic (i) | planned | font-style |
| Underline (u) | planned | text-decoration |
| Strikethrough (strike) | planned | text-decoration |
| Character spacing (spc) | planned | letter-spacing |
| Baseline offset (super/sub) | planned | vertical-align + font-size |
| Capitalization (cap) | planned | text-transform |
| Kerning | deferred | font-kerning |
| Text color (solidFill) | planned | color |
| Latin font | planned | font-family |
| East Asian font | planned | font-family fallback |
| Complex script font | planned | font-family fallback |
| Symbol font | deferred | |
| Hyperlink (hlinkClick) | planned | a href |
| Text effects (shadow, etc.) | deferred | CSS text-shadow |

---

## Text Body Properties

| Feature | Status | Notes |
|---------|--------|-------|
| Text direction (vert) | planned | writing-mode |
| Word wrap (wrap) | planned | word-wrap |
| Text insets (lIns, tIns, rIns, bIns) | planned | padding |
| Vertical anchor (anchor) | planned | flexbox align-items |
| Horizontal centering (anchorCtr) | planned | |
| No autofit | planned | overflow: hidden |
| Normal autofit (shrink text) | planned | Font scale algorithm |
| Shape autofit (resize shape) | planned | |
| Text rotation | planned | CSS transform |
| Multiple columns | deferred | CSS column-count |

---

## Placeholder Inheritance

| Feature | Status | Notes |
|---------|--------|-------|
| Placeholder type detection | planned | p:ph type attribute |
| Placeholder idx matching | planned | Cross-level matching |
| Transform inheritance | planned | Position/size cascade |
| Fill inheritance | planned | |
| Line inheritance | planned | |
| Text style inheritance (title) | planned | titleStyle cascade |
| Text style inheritance (body) | planned | bodyStyle cascade |
| Text style inheritance (other) | planned | otherStyle cascade |
| Per-property independence | planned | Each prop cascades separately |
| Metadata placeholders (dt, ftr, sldNum) | planned | |

---

## Theme

| Feature | Status | Notes |
|---------|--------|-------|
| Color scheme (12 colors) | planned | |
| Font scheme (major/minor) | planned | |
| Script-specific fonts | planned | Jpan, Hang, Hans, etc. |
| Format scheme (fill styles) | planned | fillStyleLst |
| Format scheme (line styles) | planned | lnStyleLst |
| Format scheme (effect styles) | planned | effectStyleLst |
| Format scheme (bg fill styles) | planned | bgFillStyleLst |
| Style reference resolution | planned | fillRef, lnRef, effectRef, fontRef |
| Theme font references (+mj-lt) | planned | |
| Extra color scheme list | deferred | Rarely used |

---

## Tables

| Feature | Status | Notes |
|---------|--------|-------|
| Basic table grid | planned | HTML table |
| Cell text content | planned | |
| Cell borders | planned | |
| Cell fill | planned | |
| Cell padding/margins | planned | |
| Column widths | planned | |
| Row heights | planned | |
| Horizontal merge (gridSpan) | planned | colspan |
| Vertical merge (rowSpan) | planned | rowspan |
| Table style reference | deferred | |
| Band row/column styling | deferred | |
| First/last row/column styling | deferred | |

---

## Effects

| Feature | Status | Notes |
|---------|--------|-------|
| Outer shadow (outerShdw) | planned | CSS box-shadow |
| Inner shadow (innerShdw) | planned | CSS box-shadow inset |
| Glow | deferred | CSS filter or SVG |
| Reflection | deferred | Complex CSS |
| Soft edge | deferred | CSS filter: blur |
| Blur | deferred | CSS filter: blur |
| 3D effects (scene3d, sp3d) | deferred | Not feasible in CSS |

---

## Images

| Feature | Status | Notes |
|---------|--------|-------|
| Embedded images (r:embed) | planned | Extract from ZIP |
| Base64 inline encoding | planned | data: URI |
| External image references | planned | file path output |
| Image crop (srcRect) | planned | object-fit + clip |
| Image stretch | planned | object-fit: fill |
| Image tile | deferred | background-repeat |
| Image effects (grayscale) | deferred | CSS filter |
| Image effects (duotone) | deferred | Complex filter |
| Image transparency (alphaModFix) | planned | CSS opacity |
| SVG images | deferred | Passthrough |

---

## Group Shapes

| Feature | Status | Notes |
|---------|--------|-------|
| Group bounding box | planned | Container div |
| Child coordinate transform | planned | Scale + translate |
| Nested groups (recursive) | planned | |
| Group rotation | planned | CSS transform |
| Group flip | planned | CSS transform |
| Group fill (grpFill) | planned | Inherited fill |

---

## Connectors

| Feature | Status | Notes |
|---------|--------|-------|
| Straight connector | planned | SVG line |
| Bent connector (elbow) | planned | SVG polyline |
| Curved connector | deferred | SVG cubic bezier |
| Connection site resolution | deferred | Shape geometry dependent |
| Connector arrowheads | planned | SVG marker |
| Connector line style | planned | |

---

## Output Format

| Feature | Status | Notes |
|---------|--------|-------|
| Single self-contained HTML | planned | Default output |
| Inline CSS styles | planned | No external stylesheet |
| Base64 embedded images | planned | Portable output |
| Separate image files | planned | Optional mode |
| Responsive scaling | planned | CSS transform: scale() |
| Print-friendly CSS | deferred | @media print |
| Slide navigation | deferred | JavaScript optional |
| Speaker notes | deferred | |
| Accessibility (alt text, roles) | planned | ARIA attributes |

---

## Implementation Priority Roadmap

### Phase 1: Core MVP

Target: Render simple slides with titles and text

- ZIP/XML infrastructure
- Slide structure (single master/layout/slide)
- Basic shapes (rect, no geometry)
- Solid fill (sRGB only)
- Basic text (single level, no bullets)
- HTML output skeleton

### Phase 2: Color & Text Fidelity

Target: Accurate color and text rendering

- Full color resolution chain (scheme → theme → modifiers)
- ClrMap and overrides
- Text inheritance cascade
- Bullets and numbering
- Font resolution (theme references)
- Paragraph spacing and alignment

### Phase 3: Visual Completeness

Target: Handle most visual elements

- Top 30 preset geometries
- Gradient fills
- Image fills and embedded images
- Line/outline styles
- Drop shadows
- Tables (basic)

### Phase 4: Advanced Features

Target: Handle complex presentations

- Remaining preset geometries
- Custom geometries
- Group transforms
- Connectors
- Table styling
- Pattern fills
- Multiple masters/themes
