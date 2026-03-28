# ECMA-376 DrawingML (DML) Core Elements Reference

> Based on ECMA-376 5th Edition, Part 1 - DrawingML (namespace `a:`)
> XML namespace: `http://schemas.openxmlformats.org/drawingml/2006/main`

---

## Fill Elements

### a:solidFill

Applies a uniform color fill.

```xml
<a:solidFill>
  <a:srgbClr val="4472C4"/>
</a:solidFill>
```

With modifiers:

```xml
<a:solidFill>
  <a:schemeClr val="accent1">
    <a:lumMod val="75000"/>
    <a:lumOff val="25000"/>
  </a:schemeClr>
</a:solidFill>
```

### a:gradFill

Linear or path (radial/shape) gradient fill.

```xml
<a:gradFill flip="none" rotWithShape="1">
  <a:gsLst>
    <a:gs pos="0">
      <a:srgbClr val="1F4E79"/>
    </a:gs>
    <a:gs pos="50000">
      <a:srgbClr val="2E75B6"/>
    </a:gs>
    <a:gs pos="100000">
      <a:srgbClr val="9DC3E6"/>
    </a:gs>
  </a:gsLst>
  <a:lin ang="5400000" scaled="1"/>  <!-- 90 degrees (top to bottom) -->
</a:gradFill>
```

**Gradient stop positions:** `pos` ranges from `0` (start) to `100000` (end), in 1/1000th percent.

**Linear gradient angles:** Measured in 60,000ths of a degree.

| Angle Value | Direction |
|-------------|-----------|
| 0 | Left to right |
| 5400000 | Top to bottom |
| 10800000 | Right to left |
| 16200000 | Bottom to top |
| 2700000 | Top-left to bottom-right |

Path gradient (radial):

```xml
<a:gradFill>
  <a:gsLst>
    <a:gs pos="0"><a:srgbClr val="FFFFFF"/></a:gs>
    <a:gs pos="100000"><a:srgbClr val="4472C4"/></a:gs>
  </a:gsLst>
  <a:path path="circle">
    <a:fillToRect l="50000" t="50000" r="50000" b="50000"/>
  </a:path>
  <a:tileRect/>
</a:gradFill>
```

**Path types:** `circle`, `rect`, `shape`

### a:blipFill

Image/texture fill.

```xml
<a:blipFill dpi="0" rotWithShape="1">
  <a:blip r:embed="rId2" cstate="print">
    <a:duotone>
      <a:schemeClr val="accent1">
        <a:shade val="45000"/>
      </a:schemeClr>
      <a:srgbClr val="FFFFFF"/>
    </a:duotone>
  </a:blip>
  <a:srcRect l="10000" t="10000" r="10000" b="10000"/>  <!-- crop -->
  <a:stretch>
    <a:fillRect/>
  </a:stretch>
</a:blipFill>
```

**Blip children (image effects):**

| Element | Description |
|---------|-------------|
| `a:alphaBiLevel` | Alpha bi-level threshold |
| `a:alphaCeiling` | Alpha ceiling |
| `a:alphaFloor` | Alpha floor |
| `a:alphaInv` | Alpha inverse |
| `a:alphaMod` | Alpha modulate |
| `a:alphaModFix` | Alpha modulate fixed (amt="75000" = 75% opacity) |
| `a:alphaRepl` | Alpha replace |
| `a:biLevel` | Bi-level (black/white) |
| `a:clrChange` | Color change (transparency key) |
| `a:clrRepl` | Color replace |
| `a:duotone` | Duotone effect |
| `a:grayscl` | Grayscale |
| `a:hsl` | Hue/Saturation/Luminance adjust |
| `a:lum` | Luminance adjust |
| `a:tint` | Tint |

### a:pattFill

Pattern fill with foreground and background colors.

```xml
<a:pattFill prst="pct20">
  <a:fgClr>
    <a:srgbClr val="000000"/>
  </a:fgClr>
  <a:bgClr>
    <a:srgbClr val="FFFFFF"/>
  </a:bgClr>
</a:pattFill>
```

**Common pattern presets:** `pct5`, `pct10`, `pct20`, `pct25`, `pct30`, `pct40`, `pct50`, `pct60`, `pct70`, `pct75`, `pct80`, `pct90`, `horz`, `vert`, `ltHorz`, `ltVert`, `dkHorz`, `dkVert`, `narHorz`, `narVert`, `dashHorz`, `dashVert`, `cross`, `dnDiag`, `upDiag`, `ltDnDiag`, `ltUpDiag`, `dkDnDiag`, `dkUpDiag`, `wdDnDiag`, `wdUpDiag`, `dashDnDiag`, `dashUpDiag`, `diagCross`, `smCheck`, `lgCheck`, `smGrid`, `lgGrid`, `dotGrid`, `smConfetti`, `lgConfetti`, `horzBrick`, `diagBrick`, `solidDmnd`, `openDmnd`, `dotDmnd`, `plaid`, `sphere`, `weave`, `divot`, `shingle`, `wave`, `trellis`, `zigZag`

### a:noFill

Explicitly specifies no fill.

```xml
<a:noFill/>
```

### a:grpFill

Inherits the fill from the parent group.

```xml
<a:grpFill/>
```

---

## Line Element

### a:ln (Line/Outline)

```xml
<a:ln w="19050" cap="flat" cmpd="sng" algn="ctr">
  <a:solidFill>
    <a:srgbClr val="000000"/>
  </a:solidFill>
  <a:prstDash val="dash"/>
  <a:round/>  <!-- or a:bevel or a:miter -->
  <a:headEnd type="none"/>
  <a:tailEnd type="triangle" w="med" len="med"/>
</a:ln>
```

| Attribute | Values | Description |
|-----------|--------|-------------|
| `w` | EMU | Line width (12700 = 1pt) |
| `cap` | `flat`, `rnd`, `sq` | Line cap style |
| `cmpd` | `sng`, `dbl`, `thickThin`, `thinThick`, `tri` | Compound line type |
| `algn` | `ctr`, `in` | Line alignment relative to shape edge |

**Dash presets (`a:prstDash`):**

| val | Pattern |
|-----|---------|
| `solid` | Solid line |
| `dot` | Dotted |
| `dash` | Dashed |
| `lgDash` | Long dashed |
| `dashDot` | Dash-dot |
| `lgDashDot` | Long dash-dot |
| `lgDashDotDot` | Long dash-dot-dot |
| `sysDot` | System dot |
| `sysDash` | System dash |
| `sysDashDot` | System dash-dot |
| `sysDashDotDot` | System dash-dot-dot |

**Arrow/end types (`a:headEnd`/`a:tailEnd`):**

| type | Description |
|------|-------------|
| `none` | No arrowhead |
| `triangle` | Filled triangle |
| `stealth` | Stealth arrowhead |
| `diamond` | Filled diamond |
| `oval` | Filled oval |
| `arrow` | Open arrow |

Width/length values: `sm`, `med`, `lg`

**Join types (mutually exclusive children):**

| Element | Description |
|---------|-------------|
| `a:round` | Rounded join |
| `a:bevel` | Beveled join |
| `a:miter` | Mitered join (with optional `lim` attribute) |

---

## Color Elements

### Color Specification Types

```xml
<!-- Direct sRGB -->
<a:srgbClr val="FF5733"/>

<!-- Scheme color (resolved via clrMap + theme) -->
<a:schemeClr val="accent1"/>

<!-- System color -->
<a:sysClr val="windowText" lastClr="000000"/>

<!-- HSL color -->
<a:hslClr hue="14400000" sat="100000" lum="50000"/>

<!-- Preset color -->
<a:prstClr val="red"/>

<!-- scRGB (linear RGB, 0-100000 range) -->
<a:scrgbClr r="100000" g="0" b="0"/>
```

### Scheme Color Values

| val | Description | Typical Mapping |
|-----|-------------|----------------|
| `bg1` | Background 1 | lt1 (light) |
| `bg2` | Background 2 | lt2 |
| `tx1` | Text 1 | dk1 (dark) |
| `tx2` | Text 2 | dk2 |
| `accent1`-`accent6` | Accent colors | Direct theme accent |
| `hlink` | Hyperlink | Theme hlink |
| `folHlink` | Followed hyperlink | Theme folHlink |
| `dk1` | Dark 1 | Direct theme dk1 |
| `dk2` | Dark 2 | Direct theme dk2 |
| `lt1` | Light 1 | Direct theme lt1 |
| `lt2` | Light 2 | Direct theme lt2 |
| `phClr` | Placeholder color | Context-dependent |

### Color Modifiers

Applied as child elements of any color, processed sequentially in document order.

```xml
<a:schemeClr val="accent1">
  <a:tint val="40000"/>       <!-- 40% tint towards white -->
  <a:shade val="75000"/>      <!-- 75% shade towards black -->
  <a:satMod val="120000"/>    <!-- 120% saturation -->
  <a:lumMod val="80000"/>     <!-- 80% luminance multiply -->
  <a:lumOff val="20000"/>     <!-- +20% luminance offset -->
  <a:alpha val="80000"/>      <!-- 80% opacity -->
</a:schemeClr>
```

**Full modifier list:**

| Element | Range | Formula |
|---------|-------|---------|
| `a:tint` | 0-100000 | `result = 255 - (255 - C) * val / 100000` |
| `a:shade` | 0-100000 | `result = C * val / 100000` |
| `a:satMod` | percentage | Multiply saturation by val/100000 |
| `a:satOff` | percentage | Add val/100000 to saturation |
| `a:lumMod` | percentage | Multiply luminance by val/100000 |
| `a:lumOff` | percentage | Add val/100000 to luminance |
| `a:alpha` | 0-100000 | `opacity = val / 1000` (percent) |
| `a:alphaOff` | percentage | Add to alpha |
| `a:alphaMod` | percentage | Multiply alpha |
| `a:hueMod` | percentage | Multiply hue |
| `a:hueOff` | 60000ths deg | Add to hue |
| `a:comp` | (none) | Complementary color (hue + 180) |
| `a:inv` | (none) | Inverse: `result = C ^ 0xFF` |
| `a:gray` | (none) | Convert to grayscale |
| `a:red` | 0-100000 | Set red channel |
| `a:redMod` | percentage | Multiply red channel |
| `a:redOff` | percentage | Offset red channel |
| `a:green` | 0-100000 | Set green channel |
| `a:greenMod` | percentage | Multiply green |
| `a:greenOff` | percentage | Offset green |
| `a:blue` | 0-100000 | Set blue channel |
| `a:blueMod` | percentage | Multiply blue |
| `a:blueOff` | percentage | Offset blue |

---

## Text Properties

### a:rPr (Run Properties)

```xml
<a:rPr lang="en-US" altLang="ko-KR" sz="1800" b="1" i="0" u="sng"
        strike="noStrike" kern="1200" cap="none" spc="0" dirty="0"
        err="0" smtClean="0" baseline="0">
  <a:solidFill>
    <a:schemeClr val="tx1"/>
  </a:solidFill>
  <a:latin typeface="Calibri" panose="020F0502020204030204" pitchFamily="34" charset="0"/>
  <a:ea typeface="Malgun Gothic"/>
  <a:cs typeface="Arial"/>
  <a:sym typeface="Wingdings"/>
  <a:hlinkClick r:id="rId1"/>
  <a:effectLst>
    <a:outerShdw blurRad="38100" dist="19050" dir="2700000" algn="tl" rotWithShape="0">
      <a:srgbClr val="000000">
        <a:alpha val="40000"/>
      </a:srgbClr>
    </a:outerShdw>
  </a:effectLst>
</a:rPr>
```

| Attribute | Type | Description |
|-----------|------|-------------|
| `lang` | string | Language (BCP-47) |
| `altLang` | string | Alternate language |
| `sz` | hundredths pt | Font size (1800 = 18pt) |
| `b` | boolean | Bold |
| `i` | boolean | Italic |
| `u` | enum | Underline: `none`, `sng`, `dbl`, `heavy`, `dotted`, `dottedHeavy`, `dash`, `dashHeavy`, `dashLong`, `dashLongHeavy`, `dotDash`, `dotDashHeavy`, `dotDotDash`, `dotDotDashHeavy`, `wavy`, `wavyHeavy`, `wavyDbl`, `words` |
| `strike` | enum | `noStrike`, `sngStrike`, `dblStrike` |
| `kern` | hundredths pt | Kerning threshold (0 = no kerning) |
| `cap` | enum | `none`, `small`, `all` |
| `spc` | hundredths pt | Character spacing (can be negative) |
| `baseline` | percentage | Baseline offset: `30000`=superscript, `-25000`=subscript |
| `dirty` | boolean | Needs spell-check |

**Font children:**

| Element | Description |
|---------|-------------|
| `a:latin` | Latin/Western font |
| `a:ea` | East Asian font |
| `a:cs` | Complex script (Arabic, Hebrew, Thai) |
| `a:sym` | Symbol font |

Font `typeface` can be a theme reference: `+mj-lt` (major Latin), `+mn-lt` (minor Latin), `+mj-ea` (major EA), `+mn-ea` (minor EA), etc.

### a:pPr (Paragraph Properties)

```xml
<a:pPr lvl="0" algn="l" defTabSz="914400" rtl="0" fontAlgn="auto"
        marL="342900" marR="0" indent="-342900">
  <a:lnSpc>
    <a:spcPct val="150000"/>  <!-- 150% line spacing -->
  </a:lnSpc>
  <a:spcBef>
    <a:spcPts val="600"/>  <!-- 6pt before -->
  </a:spcBef>
  <a:spcAft>
    <a:spcPts val="0"/>
  </a:spcAft>
  <a:buClr>
    <a:schemeClr val="accent1"/>
  </a:buClr>
  <a:buSzPct val="100000"/>
  <a:buFont typeface="Arial"/>
  <a:buChar char="&#x2022;"/>
  <a:tabLst>
    <a:tab pos="914400" algn="l"/>
  </a:tabLst>
  <a:defRPr sz="1800"/>
</a:pPr>
```

| Attribute | Type | Description |
|-----------|------|-------------|
| `lvl` | 0-8 | Indent level (for bullet lists) |
| `algn` | enum | `l` (left), `ctr`, `r`, `just`, `justLow`, `dist`, `thaiDist` |
| `marL` | EMU | Left margin |
| `marR` | EMU | Right margin |
| `indent` | EMU | First line indent (negative = hanging) |
| `defTabSz` | EMU | Default tab size |
| `rtl` | boolean | Right-to-left |
| `fontAlgn` | enum | `auto`, `t`, `ctr`, `base`, `b` |

**Spacing children:**

| Element | Sub-element | Description |
|---------|-------------|-------------|
| `a:lnSpc` | `a:spcPct` or `a:spcPts` | Line spacing |
| `a:spcBef` | `a:spcPct` or `a:spcPts` | Space before paragraph |
| `a:spcAft` | `a:spcPct` or `a:spcPts` | Space after paragraph |

`a:spcPct` values are in 1/1000th percent (e.g., `100000` = 100% = single spacing).
`a:spcPts` values are in hundredths of a point (e.g., `600` = 6pt).

**Bullet types (mutually exclusive):**

| Element | Description |
|---------|-------------|
| `a:buNone` | No bullet |
| `a:buChar` | Character bullet |
| `a:buAutoNum` | Auto-numbered bullet |
| `a:buBlip` | Image bullet |

`a:buAutoNum` types: `alphaLcParenBoth`, `alphaLcParenR`, `alphaLcPeriod`, `alphaUcParenBoth`, `alphaUcParenR`, `alphaUcPeriod`, `arabicParenBoth`, `arabicParenR`, `arabicPeriod`, `arabicPlain`, `romanLcParenBoth`, `romanLcParenR`, `romanLcPeriod`, `romanUcParenBoth`, `romanUcParenR`, `romanUcPeriod`, etc.

---

## Geometry Elements

### a:xfrm (2D Transform)

```xml
<a:xfrm rot="5400000" flipH="0" flipV="1">
  <a:off x="914400" y="914400"/>    <!-- position in EMU -->
  <a:ext cx="3048000" cy="2286000"/> <!-- size in EMU -->
</a:xfrm>
```

| Attribute | Type | Description |
|-----------|------|-------------|
| `rot` | 60000ths deg | Rotation (5400000 = 90 degrees) |
| `flipH` | boolean | Horizontal flip |
| `flipV` | boolean | Vertical flip |

### a:prstGeom (Preset Geometry)

```xml
<a:prstGeom prst="roundRect">
  <a:avLst>
    <a:gd name="adj" fmla="val 16667"/>  <!-- corner radius adjustment -->
  </a:avLst>
</a:prstGeom>
```

See `preset-geometry-catalog.md` for the full catalog of preset shapes.

### a:custGeom (Custom Geometry)

```xml
<a:custGeom>
  <a:avLst/>
  <a:gdLst>
    <a:gd name="x1" fmla="*/ w 1 2"/>  <!-- x1 = width / 2 -->
  </a:gdLst>
  <a:ahLst/>
  <a:cxnLst/>
  <a:rect l="0" t="0" r="r" b="b"/>
  <a:pathLst>
    <a:path w="21600" h="21600">
      <a:moveTo><a:pt x="0" y="21600"/></a:moveTo>
      <a:lnTo><a:pt x="10800" y="0"/></a:lnTo>
      <a:lnTo><a:pt x="21600" y="21600"/></a:lnTo>
      <a:close/>
    </a:path>
  </a:pathLst>
</a:custGeom>
```

**Path commands:**

| Element | Description |
|---------|-------------|
| `a:moveTo` | Move to point |
| `a:lnTo` | Line to point |
| `a:arcTo` | Arc (wR, hR, stAng, swAng) |
| `a:cubicBezTo` | Cubic Bezier (3 control points) |
| `a:quadBezTo` | Quadratic Bezier (2 control points) |
| `a:close` | Close path |

**Guide formulas (`a:gd fmla`):**

| Formula | Description |
|---------|-------------|
| `val N` | Literal value |
| `*/ x y z` | `x * y / z` |
| `+- x y z` | `x + y - z` |
| `+/ x y z` | `(x + y) / z` |
| `?: x y z` | `x > 0 ? y : z` |
| `sin x y` | `x * sin(y)` |
| `cos x y` | `x * cos(y)` |
| `tan x y` | `x * tan(y)` |
| `at2 x y` | `atan2(y, x)` |
| `sqrt x` | `sqrt(x)` |
| `abs x` | `abs(x)` |
| `max x y` | `max(x, y)` |
| `min x y` | `min(x, y)` |
| `mod x y z` | `sqrt(x^2 + y^2 + z^2)` |
| `pin x y z` | `clamp(x, y, z)` |
| `cat2 x y z` | `x * cos(atan2(z, y))` |
| `sat2 x y z` | `x * sin(atan2(z, y))` |

**Built-in variables:** `w` (width), `h` (height), `r` (right=w), `b` (bottom=h), `l` (left=0), `t` (top=0), `hc` (h center=w/2), `vc` (v center=h/2), `wd2`-`wd10` (width/N), `hd2`-`hd10` (height/N), `ss` (short side), `ls` (long side), `ssd2`-`ssd8` (short side/N)

---

## Effect Elements

### a:effectLst (Effect List)

Applied in rendering order:

```xml
<a:effectLst>
  <a:outerShdw blurRad="50800" dist="38100" dir="2700000"
               algn="tl" rotWithShape="0">
    <a:srgbClr val="000000">
      <a:alpha val="43000"/>
    </a:srgbClr>
  </a:outerShdw>
</a:effectLst>
```

**Effect elements:**

| Element | Description |
|---------|-------------|
| `a:blur` | Gaussian blur |
| `a:fillOverlay` | Fill overlay blend |
| `a:glow` | Outer glow |
| `a:innerShdw` | Inner shadow |
| `a:outerShdw` | Outer (drop) shadow |
| `a:prstShdw` | Preset shadow |
| `a:reflection` | Reflection |
| `a:softEdge` | Soft edge (feathering) |

### a:outerShdw Attributes

| Attribute | Type | Description |
|-----------|------|-------------|
| `blurRad` | EMU | Blur radius |
| `dist` | EMU | Distance from shape |
| `dir` | 60000ths deg | Direction angle |
| `sx`, `sy` | percentage | Shadow scale |
| `kx`, `ky` | 60000ths deg | Skew |
| `algn` | enum | Alignment: `tl`, `t`, `tr`, `l`, `ctr`, `r`, `bl`, `b`, `br` |
| `rotWithShape` | boolean | Rotate shadow with shape |

---

## Theme Elements

### a:theme

Root element of `ppt/theme/theme1.xml`.

```xml
<a:theme xmlns:a="..." name="Office Theme">
  <a:themeElements>
    <a:clrScheme name="Office">
      <a:dk1><a:sysClr val="windowText" lastClr="000000"/></a:dk1>
      <a:lt1><a:sysClr val="window" lastClr="FFFFFF"/></a:lt1>
      <a:dk2><a:srgbClr val="44546A"/></a:dk2>
      <a:lt2><a:srgbClr val="E7E6E6"/></a:lt2>
      <a:accent1><a:srgbClr val="4472C4"/></a:accent1>
      <a:accent2><a:srgbClr val="ED7D31"/></a:accent2>
      <a:accent3><a:srgbClr val="A5A5A5"/></a:accent3>
      <a:accent4><a:srgbClr val="FFC000"/></a:accent4>
      <a:accent5><a:srgbClr val="5B9BD5"/></a:accent5>
      <a:accent6><a:srgbClr val="70AD47"/></a:accent6>
      <a:hlink><a:srgbClr val="0563C1"/></a:hlink>
      <a:folHlink><a:srgbClr val="954F72"/></a:folHlink>
    </a:clrScheme>
    <a:fontScheme name="Office">
      <a:majorFont>
        <a:latin typeface="Calibri Light" panose="020F0302020204030204"/>
        <a:ea typeface=""/>
        <a:cs typeface=""/>
        <a:font script="Jpan" typeface="Yu Gothic Light"/>
        <a:font script="Hang" typeface="Malgun Gothic"/>
      </a:majorFont>
      <a:minorFont>
        <a:latin typeface="Calibri"/>
        <a:ea typeface=""/>
        <a:cs typeface=""/>
      </a:minorFont>
    </a:fontScheme>
    <a:fmtScheme name="Office">
      <a:fillStyleLst>
        <a:solidFill><a:schemeClr val="phClr"/></a:solidFill>
        <a:gradFill rotWithShape="1"><!-- subtle --></a:gradFill>
        <a:gradFill rotWithShape="1"><!-- intense --></a:gradFill>
      </a:fillStyleLst>
      <a:lnStyleLst>
        <a:ln w="6350" cap="flat" cmpd="sng" algn="ctr"><!-- subtle --></a:ln>
        <a:ln w="12700" cap="flat" cmpd="sng" algn="ctr"><!-- moderate --></a:ln>
        <a:ln w="19050" cap="flat" cmpd="sng" algn="ctr"><!-- intense --></a:ln>
      </a:lnStyleLst>
      <a:effectStyleLst>
        <a:effectStyle><a:effectLst/></a:effectStyle>
        <a:effectStyle><a:effectLst/></a:effectStyle>
        <a:effectStyle><a:effectLst/></a:effectStyle>
      </a:effectStyleLst>
      <a:bgFillStyleLst>
        <a:solidFill><a:schemeClr val="phClr"/></a:solidFill>
        <a:solidFill><a:schemeClr val="phClr"/></a:solidFill>
        <a:gradFill rotWithShape="1"><!-- background gradient --></a:gradFill>
      </a:bgFillStyleLst>
    </a:fmtScheme>
  </a:themeElements>
  <a:objectDefaults/>
  <a:extraClrSchemeLst/>
</a:theme>
```

### Format Scheme Index Mapping

| Style Reference idx | fillStyleLst | lnStyleLst | effectStyleLst |
|---------------------|-------------|------------|---------------|
| 1 | Subtle (idx 0) | Subtle (idx 0) | Subtle (idx 0) |
| 2 | Moderate (idx 1) | Moderate (idx 1) | Moderate (idx 1) |
| 3 | Intense (idx 2) | Intense (idx 2) | Intense (idx 2) |
| 1001+ | bgFillStyleLst[idx-1001] | - | - |

---

## Table Elements

### a:tbl

```xml
<a:tbl>
  <a:tblPr firstRow="1" lastRow="0" bandRow="1" bandCol="0">
    <a:tableStyleId>{5C22544A-7EE6-4342-B048-85BDC9FD1C3A}</a:tableStyleId>
  </a:tblPr>
  <a:tblGrid>
    <a:gridCol w="3505200"/>
    <a:gridCol w="3505200"/>
    <a:gridCol w="3505200"/>
  </a:tblGrid>
  <a:tr h="370840">
    <a:tc>
      <a:txBody>
        <a:bodyPr/>
        <a:lstStyle/>
        <a:p><a:r><a:rPr lang="en-US" dirty="0"/><a:t>Cell</a:t></a:r></a:p>
      </a:txBody>
      <a:tcPr marL="68580" marR="68580" marT="0" marB="0" anchor="ctr">
        <a:lnL w="12700" cap="flat" cmpd="sng" algn="ctr">
          <a:solidFill><a:schemeClr val="tx1"/></a:solidFill>
        </a:lnL>
        <a:lnR w="12700"><a:solidFill><a:schemeClr val="tx1"/></a:solidFill></a:lnR>
        <a:lnT w="12700"><a:solidFill><a:schemeClr val="tx1"/></a:solidFill></a:lnT>
        <a:lnB w="12700"><a:solidFill><a:schemeClr val="tx1"/></a:solidFill></a:lnB>
        <a:solidFill><a:schemeClr val="accent1"/></a:solidFill>
      </a:tcPr>
    </a:tc>
    <a:tc gridSpan="2"><!-- merged cell spanning 2 columns --></a:tc>
    <a:tc hMerge="1"/><!-- hidden cell consumed by merge -->
  </a:tr>
</a:tbl>
```

**Cell merge attributes:**

| Attribute | Description |
|-----------|-------------|
| `gridSpan` | Number of columns this cell spans |
| `rowSpan` | Number of rows this cell spans |
| `hMerge` | Cell is horizontally merged (hidden) |
| `vMerge` | Cell is vertically merged (hidden) |
