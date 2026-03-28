# ECMA-376 PresentationML (PML) Core Elements Reference

> Based on ECMA-376 5th Edition, Part 1 - PresentationML (namespace `p:`)
> XML namespace: `http://schemas.openxmlformats.org/presentationml/2006/main`

## Namespace Prefixes

| Prefix | Namespace URI | Description |
|--------|--------------|-------------|
| `p:` | `http://schemas.openxmlformats.org/presentationml/2006/main` | PresentationML |
| `a:` | `http://schemas.openxmlformats.org/drawingml/2006/main` | DrawingML |
| `r:` | `http://schemas.openxmlformats.org/officeDocument/2006/relationships` | Relationships |
| `p14:` | `http://schemas.microsoft.com/office/powerpoint/2010/main` | PowerPoint 2010 extensions |

---

## Presentation Structure

### p:presentation

Root element of `ppt/presentation.xml`. Contains global settings and slide references.

```xml
<p:presentation xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main"
                xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main"
                xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships"
                saveSubsetFonts="1">
  <p:sldMasterIdLst>
    <p:sldMasterId id="2147483648" r:id="rId1"/>
  </p:sldMasterIdLst>
  <p:sldIdLst>
    <p:sldId id="256" r:id="rId2"/>
    <p:sldId id="257" r:id="rId3"/>
  </p:sldIdLst>
  <p:sldSz cx="12192000" cy="6858000"/>  <!-- 16:9 default -->
  <p:notesSz cx="6858000" cy="9144000"/>
  <p:defaultTextStyle>
    <!-- default text formatting for all slides -->
  </p:defaultTextStyle>
</p:presentation>
```

**Key attributes:**
- `saveSubsetFonts` - embed subset of used font glyphs
- `autoCompressPictures` - compress images on save

### p:sldSz (Slide Size)

Defines the presentation slide dimensions in EMU.

| Preset | cx (EMU) | cy (EMU) | Aspect Ratio |
|--------|----------|----------|-------------|
| Standard | 9144000 | 6858000 | 4:3 |
| Widescreen | 12192000 | 6858000 | 16:9 |
| Widescreen (old) | 9144000 | 5143500 | 16:9 |
| A4 Paper | 9906000 | 6858000 | ~A4 |
| Custom | varies | varies | varies |

```xml
<p:sldSz cx="12192000" cy="6858000" type="screen16x9"/>
```

---

## Slide Elements

### p:sld (Slide)

Root element of each slide file (`ppt/slides/slide{N}.xml`).

```xml
<p:sld xmlns:p="..." xmlns:a="..." xmlns:r="...">
  <p:cSld name="Slide Title">
    <p:bg>
      <!-- optional background -->
    </p:bg>
    <p:spTree>
      <p:nvGrpSpPr>
        <p:cNvPr id="1" name=""/>
        <p:cNvGrpSpPr/>
        <p:nvPr/>
      </p:nvGrpSpPr>
      <p:grpSpPr>
        <a:xfrm>
          <a:off x="0" y="0"/>
          <a:ext cx="0" cy="0"/>
          <a:chOff x="0" y="0"/>
          <a:chExt cx="0" cy="0"/>
        </a:xfrm>
      </p:grpSpPr>
      <!-- shapes, pictures, groups, connectors go here -->
    </p:spTree>
  </p:cSld>
  <p:clrMapOvr>
    <a:masterClrMapping/>  <!-- or a:overrideClrMapping -->
  </p:clrMapOvr>
  <p:transition spd="med">
    <!-- optional transition -->
  </p:transition>
  <p:timing>
    <!-- optional animations -->
  </p:timing>
</p:sld>
```

### p:sldLayout (Slide Layout)

Root element of layout files (`ppt/slideLayouts/slideLayout{N}.xml`). Structure mirrors `p:sld`.

```xml
<p:sldLayout xmlns:p="..." type="twoObj" preserve="1">
  <p:cSld name="Two Content">
    <p:spTree>
      <!-- placeholder shapes defining the layout -->
    </p:spTree>
  </p:cSld>
  <p:clrMapOvr>
    <a:masterClrMapping/>
  </p:clrMapOvr>
</p:sldLayout>
```

**Layout type values:** `blank`, `chart`, `clipArt`, `cust`, `dgm`, `fourObj`, `obj`, `objAndTx`, `objOnly`, `objOverTx`, `objTx`, `picTx`, `secHead`, `tbl`, `title`, `titleOnly`, `twoColTx`, `twoObj`, `twoObjAndObj`, `twoObjAndTx`, `twoObjOverTx`, `twoTxTwoObj`, `txAndChart`, `txAndClipArt`, `txAndMedia`, `txAndObj`, `txAndTwoObj`, `txOverObj`, `vertTitleAndTx`, `vertTitleAndTxOverChart`, `vertTx`

### p:sldMaster (Slide Master)

Root element of master files (`ppt/slideMasters/slideMaster{N}.xml`).

```xml
<p:sldMaster xmlns:p="...">
  <p:cSld>
    <p:bg>
      <p:bgPr>
        <a:solidFill>
          <a:schemeClr val="bg1"/>
        </a:solidFill>
        <a:effectLst/>
      </p:bgPr>
    </p:bg>
    <p:spTree>
      <!-- master-level shapes and placeholders -->
    </p:spTree>
  </p:cSld>
  <p:clrMap bg1="lt1" tx1="dk1" bg2="lt2" tx2="dk2"
            accent1="accent1" accent2="accent2" accent3="accent3"
            accent4="accent4" accent5="accent5" accent6="accent6"
            hlink="hlink" folHlink="folHlink"/>
  <p:sldLayoutIdLst>
    <p:sldLayoutId id="2147483649" r:id="rId1"/>
  </p:sldLayoutIdLst>
  <p:txStyles>
    <p:titleStyle>
      <a:lvl1pPr algn="l" defTabSz="914400" rtl="0" eaLnBrk="1">
        <a:spcBef><a:spcPct val="0"/></a:spcBef>
        <a:defRPr sz="4400" kern="1200">
          <a:solidFill><a:schemeClr val="tx1"/></a:solidFill>
        </a:defRPr>
      </a:lvl1pPr>
    </p:titleStyle>
    <p:bodyStyle>
      <!-- lvl1pPr through lvl9pPr -->
    </p:bodyStyle>
    <p:otherStyle>
      <!-- default style for non-placeholder shapes -->
    </p:otherStyle>
  </p:txStyles>
</p:sldMaster>
```

---

## Shape Elements

### p:sp (Shape)

The primary shape element containing geometry and text.

```xml
<p:sp>
  <p:nvSpPr>
    <p:cNvPr id="4" name="Title 3"/>
    <p:cNvSpPr>
      <a:spLocks noGrp="1"/>
    </p:cNvSpPr>
    <p:nvPr>
      <p:ph type="title"/>  <!-- placeholder info -->
    </p:nvPr>
  </p:nvSpPr>
  <p:spPr>
    <a:xfrm>
      <a:off x="838200" y="365125"/>
      <a:ext cx="10515600" cy="1325563"/>
    </a:xfrm>
    <a:prstGeom prst="rect">
      <a:avLst/>
    </a:prstGeom>
    <a:solidFill>
      <a:srgbClr val="FF0000"/>
    </a:solidFill>
    <a:ln w="12700">
      <a:solidFill>
        <a:srgbClr val="000000"/>
      </a:solidFill>
    </a:ln>
  </p:spPr>
  <p:txBody>
    <a:bodyPr vert="horz" lIns="91440" tIns="45720" rIns="91440" bIns="45720"
              anchor="ctr" anchorCtr="0"/>
    <a:lstStyle/>
    <a:p>
      <a:pPr algn="ctr"/>
      <a:r>
        <a:rPr lang="en-US" sz="4400" b="1" dirty="0"/>
        <a:t>Title Text</a:t>
      </a:r>
    </a:p>
  </p:txBody>
</p:sp>
```

**Structure breakdown:**

| Element | Purpose |
|---------|---------|
| `p:nvSpPr` | Non-visual shape properties (id, name, placeholder) |
| `p:cNvPr` | Common non-visual properties (id, name, descr, hidden) |
| `p:cNvSpPr` | Shape-specific non-visual (locks, txBox flag) |
| `p:nvPr` | PresentationML-specific non-visual (placeholder info) |
| `p:spPr` | Shape properties (transform, geometry, fill, line) |
| `p:txBody` | Text body (body properties, paragraphs, runs) |
| `p:style` | Shape style reference (optional) |

### p:pic (Picture)

```xml
<p:pic>
  <p:nvPicPr>
    <p:cNvPr id="5" name="Picture 4" descr="Photo description"/>
    <p:cNvPicPr>
      <a:picLocks noChangeAspect="1"/>
    </p:cNvPicPr>
    <p:nvPr/>
  </p:nvPicPr>
  <p:blipFill>
    <a:blip r:embed="rId2">
      <a:extLst>
        <a:ext uri="{28A0092B-C50C-407E-A947-70E740481C1C}">
          <a14:useLocalDpi xmlns:a14="..." val="0"/>
        </a:ext>
      </a:extLst>
    </a:blip>
    <a:stretch>
      <a:fillRect/>
    </a:stretch>
  </p:blipFill>
  <p:spPr>
    <a:xfrm>
      <a:off x="1524000" y="1397000"/>
      <a:ext cx="6096000" cy="4064000"/>
    </a:xfrm>
    <a:prstGeom prst="rect">
      <a:avLst/>
    </a:prstGeom>
  </p:spPr>
</p:pic>
```

**Image reference:** The `r:embed` attribute on `a:blip` references a relationship ID that maps to the actual image file in `ppt/media/`.

### p:grpSp (Group Shape)

Groups multiple shapes with a shared transform.

```xml
<p:grpSp>
  <p:nvGrpSpPr>
    <p:cNvPr id="6" name="Group 5"/>
    <p:cNvGrpSpPr/>
    <p:nvPr/>
  </p:nvGrpSpPr>
  <p:grpSpPr>
    <a:xfrm>
      <a:off x="1000000" y="1000000"/>      <!-- group position on slide -->
      <a:ext cx="5000000" cy="3000000"/>      <!-- group size on slide -->
      <a:chOff x="0" y="0"/>                  <!-- child coordinate origin -->
      <a:chExt cx="10000000" cy="6000000"/>   <!-- child coordinate space -->
    </a:xfrm>
  </p:grpSpPr>
  <!-- child shapes use coordinates within chOff/chExt space -->
  <p:sp>...</p:sp>
  <p:sp>...</p:sp>
  <p:pic>...</p:pic>
</p:grpSp>
```

**Group transform logic:**
- Children use coordinates in the `chOff`/`chExt` coordinate space
- Scale factor: `scaleX = ext.cx / chExt.cx`, `scaleY = ext.cy / chExt.cy`
- Child position on slide: `slideX = off.x + (child.x - chOff.x) * scaleX`

### p:cxnSp (Connector Shape)

Connects two shapes with a line.

```xml
<p:cxnSp>
  <p:nvCxnSpPr>
    <p:cNvPr id="7" name="Connector 6"/>
    <p:cNvCxnSpPr>
      <a:stCxn id="3" idx="2"/>  <!-- start connected to shape id=3, connection site 2 -->
      <a:endCxn id="4" idx="0"/> <!-- end connected to shape id=4, connection site 0 -->
    </p:cNvCxnSpPr>
    <p:nvPr/>
  </p:nvCxnSpPr>
  <p:spPr>
    <a:xfrm>
      <a:off x="2000000" y="1500000"/>
      <a:ext cx="3000000" cy="2000000"/>
    </a:xfrm>
    <a:prstGeom prst="straightConnector1">
      <a:avLst/>
    </a:prstGeom>
    <a:ln w="19050">
      <a:solidFill>
        <a:srgbClr val="4472C4"/>
      </a:solidFill>
      <a:tailEnd type="triangle"/>
    </a:ln>
  </p:spPr>
</p:cxnSp>
```

**Connection site indices** (`idx`): refer to predefined connection points on the target shape geometry. For a rectangle: 0=top, 1=right, 2=bottom, 3=left.

### p:graphicFrame (Graphic Frame)

Container for tables, charts, diagrams, and OLE objects.

```xml
<p:graphicFrame>
  <p:nvGraphicFramePr>
    <p:cNvPr id="8" name="Table 7"/>
    <p:cNvGraphicFramePr>
      <a:graphicFrameLocks noGrp="1"/>
    </p:cNvGraphicFramePr>
    <p:nvPr/>
  </p:nvGraphicFramePr>
  <p:xfrm>
    <a:off x="838200" y="1825625"/>
    <a:ext cx="10515600" cy="4351338"/>
  </p:xfrm>
  <a:graphic>
    <a:graphicData uri="http://schemas.openxmlformats.org/drawingml/2006/table">
      <a:tbl>
        <!-- table content -->
      </a:tbl>
    </a:graphicData>
  </a:graphic>
</p:graphicFrame>
```

**graphicData URI values:**

| URI | Content Type |
|-----|-------------|
| `.../drawingml/2006/table` | Table |
| `.../drawingml/2006/chart` | Chart |
| `.../drawingml/2006/diagram` | SmartArt diagram |
| `.../presentationml/2006/ole` | OLE object |

---

## Text Elements

### p:txBody (Text Body)

Container for all text content within a shape.

```xml
<p:txBody>
  <a:bodyPr vert="horz" wrap="square"
            lIns="91440" tIns="45720" rIns="91440" bIns="45720"
            anchor="t" anchorCtr="0"
            rtlCol="0" fromWordArt="0">
    <a:normAutofit fontScale="90000" lnSpcReduction="10000"/>
  </a:bodyPr>
  <a:lstStyle/>
  <a:p>
    <a:pPr lvl="0" algn="l">
      <a:lnSpc><a:spcPct val="150000"/></a:lnSpc>
      <a:spcBef><a:spcPts val="600"/></a:spcBef>
      <a:buFont typeface="Arial"/>
      <a:buChar char="&#x2022;"/>
    </a:pPr>
    <a:r>
      <a:rPr lang="en-US" sz="1800" b="1" i="0" u="sng" dirty="0">
        <a:solidFill>
          <a:schemeClr val="tx1"/>
        </a:solidFill>
        <a:latin typeface="Calibri"/>
        <a:ea typeface="Malgun Gothic"/>
        <a:cs typeface="Arial"/>
      </a:rPr>
      <a:t>Bullet text content</a:t>
    </a:r>
    <a:br>
      <a:rPr lang="en-US" sz="1800"/>
    </a:br>
    <a:r>
      <a:rPr lang="en-US" sz="1800"/>
      <a:t>Second line after break</a:t>
    </a:r>
    <a:endParaRPr lang="en-US"/>
  </a:p>
</p:txBody>
```

**Text hierarchy:** `txBody` > `a:p` (paragraph) > `a:r` (run) / `a:br` (break) / `a:fld` (field)

### a:bodyPr Attributes

| Attribute | Values | Default | Description |
|-----------|--------|---------|-------------|
| `vert` | `horz`, `vert`, `vert270`, `wordArtVert`, `eaVert`, `mongolianVert` | `horz` | Text direction |
| `wrap` | `none`, `square` | `square` | Word wrap mode |
| `lIns` | EMU | 91440 (0.1in) | Left inset |
| `tIns` | EMU | 45720 (0.05in) | Top inset |
| `rIns` | EMU | 91440 (0.1in) | Right inset |
| `bIns` | EMU | 45720 (0.05in) | Bottom inset |
| `anchor` | `t`, `ctr`, `b`, `just`, `dist` | `t` | Vertical anchor |
| `anchorCtr` | boolean | `0` | Center text horizontally |
| `rtlCol` | boolean | `0` | Right-to-left columns |
| `numCol` | integer | 1 | Number of columns |
| `spcCol` | EMU | 0 | Space between columns |
| `rot` | 60000ths of degree | 0 | Text rotation |

**Auto-fit children (mutually exclusive):**

| Element | Description |
|---------|-------------|
| `a:noAutofit` | No auto-fit; text may overflow |
| `a:normAutofit` | Shrink text to fit (`fontScale`, `lnSpcReduction`) |
| `a:spAutoFit` | Resize shape to fit text |

---

## Background Elements

### p:bg (Slide Background)

```xml
<p:bg>
  <p:bgPr>
    <!-- Fill: one of solidFill, gradFill, blipFill, pattFill -->
    <a:solidFill>
      <a:schemeClr val="bg1"/>
    </a:solidFill>
    <a:effectLst/>
  </p:bgPr>
</p:bg>
```

Alternative — background style reference:

```xml
<p:bg>
  <p:bgRef idx="1001">
    <a:schemeClr val="bg1"/>
  </p:bgRef>
</p:bg>
```

---

## Placeholder Element

### p:ph (Placeholder)

Defined inside `p:nvPr` to mark a shape as a placeholder.

```xml
<p:nvPr>
  <p:ph type="body" idx="1" sz="quarter" orient="horz" hasCustomPrompt="0"/>
</p:nvPr>
```

| Attribute | Description |
|-----------|-------------|
| `type` | Placeholder type (see placeholder-types.md) |
| `idx` | Placeholder index for matching across slide/layout/master |
| `sz` | Size hint: `full`, `half`, `quarter` |
| `orient` | Orientation: `horz`, `vert` |
| `hasCustomPrompt` | Whether prompt text is customized |

**Matching rule:** A shape on a slide matches a layout/master placeholder when both `type` and `idx` match. If `type` is omitted, matching uses `idx` only. If both are omitted, no inheritance occurs.

---

## Style Reference

### p:style (Shape Style)

References theme-defined line, fill, effect, and font styles.

```xml
<p:style>
  <a:lnRef idx="2">
    <a:schemeClr val="accent1">
      <a:shade val="50000"/>
    </a:schemeClr>
  </a:lnRef>
  <a:fillRef idx="1">
    <a:schemeClr val="accent1"/>
  </a:fillRef>
  <a:effectRef idx="0">
    <a:schemeClr val="accent1"/>
  </a:effectRef>
  <a:fontRef idx="minor">
    <a:schemeClr val="lt1"/>
  </a:fontRef>
</p:style>
```

The `idx` values reference entries in the theme's `a:fmtScheme`. For `lnRef`/`fillRef`/`effectRef`, idx 1-3 correspond to subtle/moderate/intense variants. For `fontRef`, values are `major` or `minor`.

---

## Relationship Structure

### Content Types (`[Content_Types].xml`)

```xml
<Types xmlns="http://schemas.openxmlformats.org/package/2006/content-types">
  <Default Extension="rels" ContentType="application/vnd.openxmlformats-package.relationships+xml"/>
  <Default Extension="xml" ContentType="application/xml"/>
  <Default Extension="jpeg" ContentType="image/jpeg"/>
  <Default Extension="png" ContentType="image/png"/>
  <Override PartName="/ppt/presentation.xml"
            ContentType="application/vnd.openxmlformats-officedocument.presentationml.presentation.main+xml"/>
  <Override PartName="/ppt/slides/slide1.xml"
            ContentType="application/vnd.openxmlformats-officedocument.presentationml.slide+xml"/>
</Types>
```

### Relationships (`ppt/_rels/presentation.xml.rels`)

```xml
<Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships">
  <Relationship Id="rId1" Type=".../slideMaster" Target="slideMasters/slideMaster1.xml"/>
  <Relationship Id="rId2" Type=".../slide" Target="slides/slide1.xml"/>
  <Relationship Id="rId3" Type=".../theme" Target="theme/theme1.xml"/>
</Relationships>
```

### PPTX Package Layout

```
[Content_Types].xml
_rels/.rels
ppt/
  presentation.xml
  _rels/presentation.xml.rels
  slides/
    slide1.xml
    _rels/slide1.xml.rels
  slideLayouts/
    slideLayout1.xml
    _rels/slideLayout1.xml.rels
  slideMasters/
    slideMaster1.xml
    _rels/slideMaster1.xml.rels
  theme/
    theme1.xml
  media/
    image1.png
    image2.jpeg
  tableStyles.xml
  presProps.xml
  viewProps.xml
```
