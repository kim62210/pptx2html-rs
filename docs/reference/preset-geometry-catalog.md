# Preset Geometry Catalog

> Top 30+ preset geometries from ECMA-376 DrawingML (`a:prstGeom prst="..."`)
> Each preset defines a path, adjust handles, and connection sites.

---

## Overview

Preset geometries are referenced in shapes via:

```xml
<a:prstGeom prst="roundRect">
  <a:avLst>
    <a:gd name="adj" fmla="val 16667"/>
  </a:avLst>
</a:prstGeom>
```

The `a:avLst` contains adjustment values that parameterize the shape (e.g., corner radius, arrow width). If `a:avLst` is empty, default values are used. Adjustment values are specified as integers in the shape's coordinate system (typically 0-100000 range, representing percentages of the shape dimension).

---

## Basic Shapes

### rect (Rectangle)

```
┌─────────────┐
│             │
│             │
│             │
└─────────────┘
```

- **Adjustments:** None
- **Path:** Simple rectangle from (0,0) to (w,h)
- **Usage frequency:** Most common shape in presentations

```xml
<a:prstGeom prst="rect"><a:avLst/></a:prstGeom>
```

### roundRect (Rounded Rectangle)

```
╭─────────────╮
│             │
│             │
│             │
╰─────────────╯
```

- **Adjustments:** `adj` = corner radius (default: 16667 = 1/6 of min(w,h))
- **Formula:** `radius = min(w,h) * adj / 100000`

```xml
<a:prstGeom prst="roundRect">
  <a:avLst>
    <a:gd name="adj" fmla="val 16667"/>
  </a:avLst>
</a:prstGeom>
```

**CSS equivalent:**
```css
border-radius: calc(min(width, height) * adj / 100000);
```

### ellipse (Ellipse/Circle)

```
    ╭───────╮
   ╱         ╲
  │           │
   ╲         ╱
    ╰───────╯
```

- **Adjustments:** None
- **Path:** Ellipse inscribed in the bounding box

**CSS equivalent:**
```css
border-radius: 50%;
```

### triangle (Isosceles Triangle)

```
      ╱╲
     ╱  ╲
    ╱    ╲
   ╱      ╲
  ╱________╲
```

- **Adjustments:** `adj` = apex horizontal position (default: 50000 = center)
- **Formula:** `apex_x = w * adj / 100000`

**SVG path (default):**
```svg
M 0 h  L w/2 0  L w h  Z
```

### rtTriangle (Right Triangle)

```
  │╲
  │  ╲
  │    ╲
  │      ╲
  │________╲
```

- **Adjustments:** None
- **Path:** Right angle at bottom-left

### diamond (Diamond/Rhombus)

```
      ╱╲
    ╱    ╲
  ╱        ╲
    ╲    ╱
      ╲╱
```

- **Adjustments:** None
- **Path:** Four midpoints of bounding box edges

### parallelogram

```
    ╱────────────╱
   ╱            ╱
  ╱────────────╱
```

- **Adjustments:** `adj` = skew amount (default: 25000)

### trapezoid

```
  ╱──────────────╲
 ╱                ╲
╱____________________╲
```

- **Adjustments:** `adj` = top edge inset (default: 25000)

---

## Arrow Shapes

### rightArrow

```
  ┌──────────┐
  │          ├──────►
  └──────────┘
```

- **Adjustments:**
  - `adj1` = shaft height ratio (default: 50000)
  - `adj2` = head width (default: 50000)

### leftArrow

Mirror of rightArrow.

### upArrow

```
      ▲
     ╱ ╲
    ╱   ╲
   ┌─────┐
   │     │
   │     │
   └─────┘
```

- **Adjustments:** `adj1` (shaft width), `adj2` (head height)

### downArrow

Mirror of upArrow.

### leftRightArrow

```
       ┌──────────┐
  ◄────┤          ├────►
       └──────────┘
```

- **Adjustments:** `adj1` (shaft height), `adj2` (head width)

### upDownArrow

Vertical version of leftRightArrow.

### bentArrow

```
         ┌──►
         │
    ┌────┘
    │
```

- **Adjustments:** `adj1` (vertical shaft width), `adj2` (bend radius), `adj3` (head size), `adj4` (head angle)

### stripedRightArrow

```
  ║ ┌──────────┐
  ║ │          ├──────►
  ║ └──────────┘
```

- Striped lines before the arrow shaft

### notchedRightArrow

```
  ┌──────────┐
  ◁          ├──────►
  └──────────┘
```

- Notched (concave) tail end

### chevron

```
  ┌─────────╲
  │          ►
  └─────────╱
```

- **Adjustments:** `adj` = point depth (default: 50000)

### homePlate (Pentagon/Home Plate)

```
  ┌───────────╲
  │            ►
  └───────────╱
```

- **Adjustments:** `adj` = point depth (default: 50000)

---

## Callout & Star Shapes

### wedgeRoundRectCallout

```
  ╭─────────────╮
  │  Callout     │
  │  text here   │
  ╰──────┬──────╯
         ╲
          ╲
           ╲ (tail point)
```

- **Adjustments:** `adj1`, `adj2` = tail anchor position relative to shape center

### wedgeRectCallout

Same as above but with sharp corners.

### wedgeEllipseCallout

Elliptical callout with tail.

### cloudCallout

Cloud-shaped callout bubble.

### star4 (4-Pointed Star)

```
      │
   ╲  │  ╱
    ╲ │ ╱
  ───  ───
    ╱ │ ╲
   ╱  │  ╲
      │
```

- **Adjustments:** `adj` = inner point distance (default: 12500)

### star5 (5-Pointed Star)

- **Adjustments:** `adj`, `hf`, `vf`

### star6, star8, star10, star12, star16, star24, star32

Variants with different point counts. Each has an `adj` value controlling the inner/outer radius ratio.

---

## Block Shapes

### cube

```
    ╱────────╱│
   ╱        ╱ │
  ╱────────╱  │
  │        │  │
  │        │ ╱
  │________│╱
```

- **Adjustments:** `adj` = depth/perspective (default: 25000)

### can (Cylinder)

```
    ╭──────╮
   ╱        ╲
  │          │
  │          │
  │          │
   ╲        ╱
    ╰──────╯
```

- **Adjustments:** `adj` = cap height ratio (default: 25000)

### bevel

```
  ┌────────────────┐
  │ ╱────────────╲ │
  ││              ││
  ││              ││
  │ ╲────────────╱ │
  └────────────────┘
```

- **Adjustments:** `adj` = bevel depth (default: 12500)

### foldedCorner

```
  ┌──────────────┐
  │              │
  │              │
  │           ╱──┘
  └──────────╱
```

- **Adjustments:** `adj` = fold size (default: 16667)

---

## Flow & Process Shapes

### flowChartProcess

```
  ┌─────────────┐
  │             │
  └─────────────┘
```

Standard rectangle (same as `rect` but classified as flowchart).

### flowChartDecision

```
      ╱╲
    ╱    ╲
  ╱        ╲
    ╲    ╱
      ╲╱
```

Diamond shape (same as `diamond` but classified as flowchart).

### flowChartTerminator

```
  ╭─────────────╮
  │             │
  ╰─────────────╯
```

Rounded rectangle with maximum corner radius.

### flowChartDocument

```
  ┌──────────────┐
  │              │
  │              │
  └─~─~─~─~─~─~─┘
```

Rectangle with wavy bottom edge.

### flowChartInputOutput

Parallelogram (same as `parallelogram`).

### flowChartPredefinedProcess

```
  ┌─┬─────────┬─┐
  │ │         │ │
  │ │         │ │
  └─┴─────────┴─┘
```

Rectangle with vertical lines near edges.

---

## Connector Shapes

### straightConnector1

Simple straight line from start to end.

### bentConnector2, bentConnector3, bentConnector4, bentConnector5

Connector with 1, 2, 3, or 4 bends (right-angle routing).

### curvedConnector2, curvedConnector3, curvedConnector4, curvedConnector5

Connector with curved routing.

---

## Special Shapes

### line

Diagonal line from top-left to bottom-right of bounding box.

### actionButtonBlank, actionButtonHome, actionButtonHelp, etc.

Rectangular buttons with predefined icons (navigation action buttons).

### mathPlus, mathMinus, mathMultiply, mathDivide, mathEqual, mathNotEqual

Mathematical operator shapes.

### heart

```
  ╱╲     ╱╲
 ╱  ╲   ╱  ╲
╱    ╲ ╱    ╲
╲           ╱
 ╲         ╱
  ╲       ╱
   ╲     ╱
    ╲   ╱
     ╲ ╱
```

### lightningBolt

Zigzag lightning bolt shape.

### smileyFace

```
  ╭──────────╮
  │ ●      ● │
  │          │
  │  ╲____╱  │
  ╰──────────╯
```

- **Adjustments:** `adj` = mouth curvature (positive=smile, negative=frown)

---

## Adjustment Value Ranges

| Shape | Adjustment | Default | Min | Max | Description |
|-------|-----------|---------|-----|-----|-------------|
| roundRect | adj | 16667 | 0 | 50000 | Corner radius ratio |
| triangle | adj | 50000 | 0 | 100000 | Apex X position |
| parallelogram | adj | 25000 | 0 | 100000 | Skew amount |
| trapezoid | adj | 25000 | 0 | 50000 | Top inset |
| rightArrow | adj1 | 50000 | 0 | 100000 | Shaft height |
| rightArrow | adj2 | 50000 | 0 | 100000 | Head length |
| chevron | adj | 50000 | 0 | 100000 | Point depth |
| star4 | adj | 12500 | 0 | 50000 | Inner radius |
| foldedCorner | adj | 16667 | 0 | 50000 | Fold size |
| cube | adj | 25000 | 0 | 100000 | 3D depth |
| can | adj | 25000 | 0 | 50000 | Cap height |
| bevel | adj | 12500 | 0 | 50000 | Bevel depth |

---

## CSS/SVG Rendering Strategies

### Simple Shapes (CSS-only)

| Shape | CSS Strategy |
|-------|-------------|
| rect | No `border-radius` |
| roundRect | `border-radius` with calculated value |
| ellipse | `border-radius: 50%` |

### Complex Shapes (SVG required)

For shapes that cannot be represented with CSS `border-radius` alone, generate an SVG `<path>` or `<clipPath>`:

```html
<!-- Triangle via SVG clip-path -->
<div style="clip-path: polygon(50% 0%, 0% 100%, 100% 100%);">
  Content
</div>

<!-- Or using inline SVG -->
<svg viewBox="0 0 100 100">
  <polygon points="50,0 0,100 100,100" fill="currentColor"/>
</svg>
```

### Recommended Implementation Priority

1. **Tier 1 (CSS-only):** rect, roundRect, ellipse
2. **Tier 2 (Simple SVG):** triangle, rtTriangle, diamond, parallelogram, trapezoid
3. **Tier 3 (Complex SVG):** arrows, stars, callouts, flowchart shapes
4. **Tier 4 (Deferred):** 3D shapes, action buttons, connector routing

---

## Full Preset Name List (187 presets in ECMA-376)

The spec defines 187 preset geometries. Here are the most commonly encountered ones grouped by category:

**Basic:** `rect`, `roundRect`, `ellipse`, `triangle`, `rtTriangle`, `diamond`, `parallelogram`, `trapezoid`, `pentagon`, `hexagon`, `heptagon`, `octagon`, `decagon`, `dodecagon`

**Rectangles:** `round1Rect`, `round2SameRect`, `round2DiagRect`, `snip1Rect`, `snip2SameRect`, `snip2DiagRect`, `snipRoundRect`

**Arrows:** `rightArrow`, `leftArrow`, `upArrow`, `downArrow`, `leftRightArrow`, `upDownArrow`, `bentArrow`, `uturnArrow`, `leftUpArrow`, `bentUpArrow`, `curvedRightArrow`, `curvedLeftArrow`, `curvedUpArrow`, `curvedDownArrow`, `stripedRightArrow`, `notchedRightArrow`, `chevron`, `homePlate`, `quadArrow`, `circularArrow`

**Stars:** `star4`, `star5`, `star6`, `star8`, `star10`, `star12`, `star16`, `star24`, `star32`, `irregularSeal1`, `irregularSeal2`

**Callouts:** `wedgeRectCallout`, `wedgeRoundRectCallout`, `wedgeEllipseCallout`, `cloudCallout`, `borderCallout1`, `borderCallout2`, `borderCallout3`, `accentCallout1`, `accentCallout2`, `accentCallout3`, `callout1`, `callout2`, `callout3`, `accentBorderCallout1`, `accentBorderCallout2`, `accentBorderCallout3`

**Block:** `cube`, `can`, `bevel`, `foldedCorner`, `frame`, `halfFrame`, `corner`, `diagStripe`, `plaque`, `donut`, `noSmoking`, `blockArc`, `pie`, `pieWedge`, `gear6`, `gear9`, `funnel`

**Flowchart:** `flowChartProcess`, `flowChartDecision`, `flowChartInputOutput`, `flowChartPredefinedProcess`, `flowChartInternalStorage`, `flowChartDocument`, `flowChartMultidocument`, `flowChartTerminator`, `flowChartPreparation`, `flowChartManualInput`, `flowChartManualOperation`, `flowChartConnector`, `flowChartOffpageConnector`, `flowChartPunchedCard`, `flowChartPunchedTape`, `flowChartSummingJunction`, `flowChartOr`, `flowChartCollate`, `flowChartSort`, `flowChartExtract`, `flowChartMerge`, `flowChartOnlineStorage`, `flowChartDelay`, `flowChartMagneticTape`, `flowChartMagneticDisk`, `flowChartMagneticDrum`, `flowChartDisplay`, `flowChartAlternateProcess`, `flowChartOfflineStorage`

**Special:** `line`, `arc`, `bracketPair`, `bracePair`, `plaqueTabs`, `chartPlus`, `chartStar`, `chartX`, `heart`, `lightningBolt`, `sun`, `moon`, `cloud`, `smileyFace`, `ribbon`, `ribbon2`, `wave`, `doubleWave`, `plus`, `cross`, `mathPlus`, `mathMinus`, `mathMultiply`, `mathDivide`, `mathEqual`, `mathNotEqual`
