# EMU (English Metric Unit) Coordinate System

> ECMA-376 uses EMU as its fundamental unit of measurement for all positioning and sizing.
> 1 EMU = 1/914400 inch = 1/360000 cm

---

## Core Conversion Constants

| From | To EMU | Formula |
|------|--------|---------|
| 1 inch | 914400 EMU | `EMU = inches * 914400` |
| 1 centimeter | 360000 EMU | `EMU = cm * 360000` |
| 1 millimeter | 36000 EMU | `EMU = mm * 36000` |
| 1 point (pt) | 12700 EMU | `EMU = pt * 12700` |
| 1 pixel (96 dpi) | 9525 EMU | `EMU = px * 9525` |
| 1 pixel (72 dpi) | 12700 EMU | `EMU = px * 12700` |

### Inverse Conversions

| From EMU | To | Formula |
|----------|-----|---------|
| EMU | inches | `inches = EMU / 914400` |
| EMU | cm | `cm = EMU / 360000` |
| EMU | mm | `mm = EMU / 36000` |
| EMU | pt | `pt = EMU / 12700` |
| EMU | px (96 dpi) | `px = EMU / 9525` |
| EMU | px (72 dpi) | `px = EMU / 12700` |

---

## Why EMU?

EMU was chosen as the unit system for OOXML because:

1. **Integer arithmetic** - 914400 has many factors, enabling exact subdivision without floating point
2. **Cross-unit precision** - Converts cleanly to both imperial and metric systems
3. **Sub-pixel accuracy** - At 96dpi, 1 EMU = 1/9525 pixel (more than sufficient precision)

### Factor decomposition of 914400

```
914400 = 2^5 * 3^2 * 5^2 * 127
       = 32 * 9 * 25 * 127
```

Key: the factor `127` makes the inch-to-point conversion exact (1pt = 1/72 inch, and 914400/72 = 12700 = 100 * 127).

---

## Coordinate System

### Origin and Axes

```
(0,0) ─────────────────────── +x
  │
  │    Slide content area
  │
  │
  +y
```

- **Origin (0, 0):** Top-left corner
- **X axis:** Increases rightward
- **Y axis:** Increases downward
- All coordinates and dimensions are non-negative EMU integers (exception: `indent` in paragraph properties can be negative)

### Shape Positioning

A shape's position is defined by `a:xfrm`:

```xml
<a:xfrm rot="0" flipH="0" flipV="0">
  <a:off x="914400" y="914400"/>      <!-- top-left corner position -->
  <a:ext cx="3048000" cy="2286000"/>   <!-- width and height -->
</a:xfrm>
```

The bounding box of the shape:
- Left edge: `x`
- Top edge: `y`
- Right edge: `x + cx`
- Bottom edge: `y + cy`
- Center: `(x + cx/2, y + cy/2)`

---

## Common Slide Sizes in EMU

### Standard Sizes

| Size Name | Width (cx) | Height (cy) | Width (in) | Height (in) |
|-----------|-----------|-------------|-----------|-------------|
| Standard 4:3 | 9144000 | 6858000 | 10.0 | 7.5 |
| Widescreen 16:9 | 12192000 | 6858000 | 13.333 | 7.5 |
| Widescreen 16:10 | 12192000 | 7620000 | 13.333 | 8.333 |
| A4 Portrait | 6858000 | 9906000 | 7.5 | 10.833 |
| A4 Landscape | 9906000 | 6858000 | 10.833 | 7.5 |
| Letter | 9144000 | 6858000 | 10.0 | 7.5 |
| Custom 16:9 (old) | 9144000 | 5143500 | 10.0 | 5.625 |

### Converting to CSS Pixels

For HTML rendering, the typical conversion uses 96dpi:

```
px = EMU / 9525
```

**Example: Widescreen 16:9 slide**
```
width  = 12192000 / 9525 = 1280 px
height = 6858000 / 9525  = 720 px
```

**Example: Standard 4:3 slide**
```
width  = 9144000 / 9525 = 960 px
height = 6858000 / 9525 = 720 px
```

---

## Rotation Units

ECMA-376 uses **60,000ths of a degree** for rotation angles.

| Degrees | EMU Rotation Value |
|---------|-------------------|
| 0 | 0 |
| 45 | 2700000 |
| 90 | 5400000 |
| 180 | 10800000 |
| 270 | 16200000 |
| 360 | 21600000 |

### Conversion

```
degrees = emu_rotation / 60000
emu_rotation = degrees * 60000
```

### Rotation Direction

- **Positive values:** Clockwise rotation
- Rotation is applied around the **center** of the shape's bounding box
- When both rotation and flip are specified, the order is: flip first, then rotate

---

## Text Size Units

Font sizes in DrawingML use **hundredths of a point**.

| Display Size | a:rPr `sz` Value |
|-------------|------------------|
| 8pt | 800 |
| 10pt | 1000 |
| 11pt | 1100 |
| 12pt | 1200 |
| 14pt | 1400 |
| 18pt | 1800 |
| 24pt | 2400 |
| 28pt | 2800 |
| 32pt | 3200 |
| 36pt | 3600 |
| 44pt | 4400 |

### Conversion

```
pt = sz / 100
CSS_px = sz / 100  (when using pt units in CSS)
CSS_px = sz / 100 * 96 / 72  (when using px units in CSS at 96dpi = sz * 4/300)
```

---

## Spacing and Margin Units

### Text Insets (bodyPr)

Default text insets in `a:bodyPr` are in EMU:

| Attribute | Default (EMU) | Default (in) | Default (pt) |
|-----------|--------------|-------------|-------------|
| `lIns` | 91440 | 0.1 | 7.2 |
| `tIns` | 45720 | 0.05 | 3.6 |
| `rIns` | 91440 | 0.1 | 7.2 |
| `bIns` | 45720 | 0.05 | 3.6 |

### Line Width

Line widths (`a:ln w`) are in EMU:

| Visual | EMU | pt |
|--------|-----|-----|
| Hairline | 6350 | 0.5 |
| Thin | 12700 | 1.0 |
| Medium | 19050 | 1.5 |
| Thick | 25400 | 2.0 |
| Extra thick | 38100 | 3.0 |
| Heavy | 76200 | 6.0 |

### Paragraph Spacing

`a:spcPts` values are in **hundredths of a point**:

| Display | spcPts value |
|---------|-------------|
| 6pt | 600 |
| 12pt | 1200 |
| 18pt | 1800 |

`a:spcPct` values are in **thousandths of a percent**:

| Display | spcPct value |
|---------|-------------|
| Single (100%) | 100000 |
| 1.15 lines | 115000 |
| 1.5 lines | 150000 |
| Double (200%) | 200000 |

---

## Percentage Units

ECMA-376 uses **thousandths of a percent** (1/100000 = 0.001%) for many percentage values:

| Display | Value |
|---------|-------|
| 0% | 0 |
| 25% | 25000 |
| 50% | 50000 |
| 75% | 75000 |
| 100% | 100000 |
| 120% | 120000 |
| 200% | 200000 |

This applies to: color modifiers (tint, shade, satMod, lumMod, etc.), gradient stop positions, line spacing percentages, and scale factors.

---

## Group Shape Coordinate Transform

When shapes are inside a group (`p:grpSp`), their coordinates are in the group's child coordinate space, not the slide coordinate space.

```xml
<p:grpSpPr>
  <a:xfrm>
    <a:off x="1000000" y="1000000"/>      <!-- group's position on slide -->
    <a:ext cx="4000000" cy="3000000"/>      <!-- group's size on slide -->
    <a:chOff x="0" y="0"/>                  <!-- child space origin -->
    <a:chExt cx="8000000" cy="6000000"/>    <!-- child space dimensions -->
  </a:xfrm>
</p:grpSpPr>
```

### Transform Formula

To convert a child shape's position to slide coordinates:

```
scaleX = ext.cx / chExt.cx
scaleY = ext.cy / chExt.cy

slideX = off.x + (childOff.x - chOff.x) * scaleX
slideY = off.y + (childOff.y - chOff.y) * scaleY
slideW = childExt.cx * scaleX
slideH = childExt.cy * scaleY
```

**Example:**
```
Group: off=(1000000, 1000000), ext=(4000000, 3000000), chOff=(0,0), chExt=(8000000, 6000000)
Child: off=(2000000, 1500000), ext=(4000000, 3000000)

scaleX = 4000000 / 8000000 = 0.5
scaleY = 3000000 / 6000000 = 0.5

slideX = 1000000 + (2000000 - 0) * 0.5 = 2000000
slideY = 1000000 + (1500000 - 0) * 0.5 = 1750000
slideW = 4000000 * 0.5 = 2000000
slideH = 3000000 * 0.5 = 1500000
```

### Nested Groups

For nested groups, apply the transform recursively from the innermost group outward:

```
finalX = outerOff.x + (innerSlideX - outerChOff.x) * outerScaleX
```

---

## Implementation Notes for Rust

### Recommended Type

```rust
/// EMU (English Metric Unit) value.
/// 1 inch = 914400 EMU, 1 pt = 12700 EMU, 1 cm = 360000 EMU
type Emu = i64;

/// Convert EMU to CSS pixels (96 dpi)
fn emu_to_px(emu: Emu) -> f64 {
    emu as f64 / 9525.0
}

/// Convert EMU to points
fn emu_to_pt(emu: Emu) -> f64 {
    emu as f64 / 12700.0
}

/// Convert hundredths-of-point to CSS pt
fn hundredths_pt_to_pt(val: i32) -> f64 {
    val as f64 / 100.0
}

/// Convert 60,000ths-of-degree to CSS degrees
fn emu_angle_to_degrees(angle: i32) -> f64 {
    angle as f64 / 60000.0
}

/// Convert thousandths-of-percent to fraction (0.0-1.0+)
fn emu_pct_to_fraction(val: i32) -> f64 {
    val as f64 / 100000.0
}
```

Use `i64` for EMU values to handle large coordinates and avoid overflow during intermediate calculations (e.g., group transform multiplication). The maximum EMU value in practice is around 51206400 (56 inches), well within i32 range, but intermediate products during transform calculations can exceed i32.
