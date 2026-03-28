# Color Resolution Logic (ONLYOFFICE Analysis)

> Detailed analysis of how ONLYOFFICE resolves colors from OOXML scheme references to final RGB values.
> This documents the algorithm for clean-room reimplementation.

---

## Resolution Pipeline

```
Input: ColorRef (schemeClr/srgbClr/sysClr/hslClr/prstClr/scrgbClr + modifiers[])
  │
  ▼
Step 1: Resolve base color to RGB
  │
  ▼
Step 2: Apply modifiers sequentially
  │
  ▼
Output: RGBA(r, g, b, a) where each channel is 0-255 and alpha is 0.0-1.0
```

---

## Step 1: SchemeClr Resolution via ClrMap

### The ClrMap Lookup

SchemeClr values like `bg1`, `tx1`, `bg2`, `tx2` are **abstract** names that must be resolved through the color map (`p:clrMap` on the slide master).

```
SchemeClr "bg1" → ClrMap lookup → "lt1" → Theme ColorScheme a:lt1 → RGB
SchemeClr "tx1" → ClrMap lookup → "dk1" → Theme ColorScheme a:dk1 → RGB
SchemeClr "bg2" → ClrMap lookup → "lt2" → Theme ColorScheme a:lt2 → RGB
SchemeClr "tx2" → ClrMap lookup → "dk2" → Theme ColorScheme a:dk2 → RGB
```

Direct scheme names (`accent1`-`accent6`, `dk1`, `lt1`, `dk2`, `lt2`, `hlink`, `folHlink`) resolve directly to the theme color scheme without ClrMap transformation.

### ONLYOFFICE Implementation Pattern

```javascript
// Pseudocode derived from ONLYOFFICE algorithm structure
function resolveSchemeColor(schemeName, clrMap, themeColorScheme) {
    // Map abstract names through clrMap
    let themeSlot;
    switch (schemeName) {
        case "bg1": themeSlot = clrMap.bg1; break;  // typically "lt1"
        case "tx1": themeSlot = clrMap.tx1; break;  // typically "dk1"
        case "bg2": themeSlot = clrMap.bg2; break;  // typically "lt2"
        case "tx2": themeSlot = clrMap.tx2; break;  // typically "dk2"
        case "accent1": case "accent2": case "accent3":
        case "accent4": case "accent5": case "accent6":
        case "hlink": case "folHlink":
        case "dk1": case "lt1": case "dk2": case "lt2":
            themeSlot = schemeName;  // direct mapping
            break;
    }

    return themeColorScheme[themeSlot];  // returns RGB
}
```

### ClrMap Override Chain

```
1. Check slide.clrMapOvr → if overrideClrMapping, use it
2. Else check layout.clrMapOvr → if overrideClrMapping, use it
3. Else use master.clrMap (always present)
```

---

## Step 2: Color Modifier Application

Modifiers are applied **in document order** (top to bottom in XML). Each modifier transforms the current color state.

### Tint

Tint moves the color towards white.

```
Formula: C' = 255 - (255 - C) * val / 100000

Where:
  C = current channel value (R, G, or B), range 0-255
  val = tint value, range 0-100000 (representing 0-100%)
```

**Implementation:**
```rust
fn apply_tint(r: u8, g: u8, b: u8, val: u32) -> (u8, u8, u8) {
    let f = val as f64 / 100000.0;
    let r2 = (255.0 - (255.0 - r as f64) * f) as u8;
    let g2 = (255.0 - (255.0 - g as f64) * f) as u8;
    let b2 = (255.0 - (255.0 - b as f64) * f) as u8;
    (r2, g2, b2)
}
```

**Edge cases:**
- `val=0`: result is white (255, 255, 255)
- `val=100000`: result is unchanged
- `val=50000`: result is midpoint between color and white

### Shade

Shade moves the color towards black.

```
Formula: C' = C * val / 100000
```

**Implementation:**
```rust
fn apply_shade(r: u8, g: u8, b: u8, val: u32) -> (u8, u8, u8) {
    let f = val as f64 / 100000.0;
    let r2 = (r as f64 * f) as u8;
    let g2 = (g as f64 * f) as u8;
    let b2 = (b as f64 * f) as u8;
    (r2, g2, b2)
}
```

### LumMod / LumOff (Luminance Modify / Offset)

These operate in HSL color space. ONLYOFFICE uses a non-standard HSL range.

**Critical detail: ONLYOFFICE uses 0-240 range for HSL components, not the standard 0-360 (hue) / 0-100 (S,L).**

```
1. Convert RGB (0-255) to HSL (H: 0-240, S: 0-240, L: 0-240)
2. L' = L * lumMod / 100000
3. L' = L' + 240 * lumOff / 100000  (lumOff adds to the 0-240 scale)
4. L' = clamp(L', 0, 240)
5. Convert HSL back to RGB
```

**Implementation:**
```rust
fn apply_lum_mod_off(r: u8, g: u8, b: u8, lum_mod: Option<u32>, lum_off: Option<u32>) -> (u8, u8, u8) {
    let (h, s, mut l) = rgb_to_hsl_240(r, g, b);

    if let Some(mod_val) = lum_mod {
        l = l * mod_val as f64 / 100000.0;
    }
    if let Some(off_val) = lum_off {
        l = l + 240.0 * off_val as f64 / 100000.0;
    }
    l = l.clamp(0.0, 240.0);

    hsl_240_to_rgb(h, s, l)
}
```

### SatMod / SatOff (Saturation Modify / Offset)

Same pattern as lumMod/lumOff but operating on the S component.

```
1. Convert RGB to HSL (0-240 range)
2. S' = S * satMod / 100000
3. S' = S' + 240 * satOff / 100000
4. S' = clamp(S', 0, 240)
5. Convert back to RGB
```

### Alpha

Sets opacity.

```
Formula: alpha = val * 255 / 100000

Where:
  val = alpha value, range 0-100000
  result alpha channel: 0-255 (or 0.0-1.0 as float)
```

**Note:** In ONLYOFFICE's internal representation, alpha is stored as 0-255. For CSS output, convert to 0.0-1.0.

```rust
fn apply_alpha(val: u32) -> f64 {
    val as f64 / 100000.0  // 0.0 to 1.0 for CSS
}
```

### Inv (Inverse)

```
Formula: C' = C ^ 0xFF  (equivalent to 255 - C)
```

**Implementation:**
```rust
fn apply_inv(r: u8, g: u8, b: u8) -> (u8, u8, u8) {
    (r ^ 0xFF, g ^ 0xFF, b ^ 0xFF)
}
```

### HueMod / HueOff

Operates on the H component of HSL.

```
hueMod: H' = H * val / 100000
hueOff: H' = H + 240 * val / 21600000  (val is in 60000ths of degree)
         (240 units = 360 degrees, so 1 degree = 240/360 = 2/3 unit)
```

### Comp (Complementary)

```
H' = (H + 120) mod 240  (in 0-240 range, 120 = 180 degrees)
```

### Gray (Grayscale)

```
gray = (R * 299 + G * 587 + B * 114) / 1000  (ITU-R BT.601)
R' = G' = B' = gray
```

---

## HSL Conversion (0-240 Range)

### RGB to HSL

This is the Windows-style HSL (used by the Win32 API `ColorRGBToHLS`), using 0-240 range.

```
Input: R, G, B in range 0-255
Output: H, S, L in range 0-240

Max = max(R, G, B)
Min = min(R, G, B)
Delta = Max - Min

// Luminance
L = (Max + Min) * 240 / (2 * 255)

// Saturation
if Delta == 0:
    S = 0
    H = 0 (undefined, achromatic)
else:
    if L <= 120:  // L <= 0.5 in 0-240 scale
        S = Delta * 240 / (Max + Min)
    else:
        S = Delta * 240 / (2 * 255 - Max - Min)

    // Hue
    if Max == R:
        H = (G - B) * 40 / Delta        // 40 = 240/6
    elif Max == G:
        H = 80 + (B - R) * 40 / Delta   // 80 = 240/3
    else:  // Max == B
        H = 160 + (R - G) * 40 / Delta  // 160 = 2*240/3

    if H < 0:
        H = H + 240
```

### HSL to RGB

```
Input: H, S, L in range 0-240
Output: R, G, B in range 0-255

if S == 0:
    R = G = B = L * 255 / 240
else:
    if L <= 120:
        temp2 = L * (240 + S) / 240
    else:
        temp2 = L + S - L * S / 240

    temp1 = 2 * L - temp2

    // Convert hue to RGB channel
    R = hue_to_rgb(temp1, temp2, H + 80)   // +80 = +1/3 of 240
    G = hue_to_rgb(temp1, temp2, H)
    B = hue_to_rgb(temp1, temp2, H - 80)   // -80 = -1/3 of 240

function hue_to_rgb(t1, t2, hue):
    if hue < 0: hue += 240
    if hue > 240: hue -= 240

    if hue < 40:      // < 1/6
        return t1 + (t2 - t1) * hue / 40
    elif hue < 120:    // < 1/2
        return t2
    elif hue < 160:    // < 2/3
        return t1 + (t2 - t1) * (160 - hue) / 40
    else:
        return t1
```

**All intermediate values should use integer arithmetic scaled by 240 to match ONLYOFFICE's behavior.** Using floating-point may produce slightly different rounding results.

---

## scRGB to sRGB Conversion

scRGB (linear RGB) values in OOXML use a 0-100000 range. Conversion to sRGB applies gamma correction.

```
Input: scRGB component value (0-100000)
Output: sRGB component value (0-255)

// Normalize to 0.0-1.0
val = scrgb_val / 100000.0

// Apply sRGB gamma
if val <= 0.0031308:
    srgb = val * 12.92
else:
    srgb = 1.055 * pow(val, 1.0 / 2.4) - 0.055

// Scale to 0-255
result = round(srgb * 255)
```

**Implementation:**
```rust
fn scrgb_to_srgb_component(val: u32) -> u8 {
    let linear = val as f64 / 100000.0;
    let gamma = if linear <= 0.0031308 {
        linear * 12.92
    } else {
        1.055 * linear.powf(1.0 / 2.4) - 0.055
    };
    (gamma * 255.0).round().clamp(0.0, 255.0) as u8
}
```

---

## Preset Color Table

The `a:prstClr` element uses named colors. Common ones:

| Name | RGB | Name | RGB |
|------|-----|------|-----|
| `black` | 000000 | `white` | FFFFFF |
| `red` | FF0000 | `green` | 008000 |
| `blue` | 0000FF | `yellow` | FFFF00 |
| `cyan` | 00FFFF | `magenta` | FF00FF |
| `darkBlue` | 00008B | `darkRed` | 8B0000 |
| `darkGreen` | 006400 | `darkCyan` | 008B8B |
| `darkMagenta` | 8B008B | `darkYellow` | 808000 |
| `gray` | 808080 | `darkGray` | A9A9A9 |
| `lightGray` | D3D3D3 | `orange` | FFA500 |
| `coral` | FF7F50 | `cornflowerBlue` | 6495ED |
| `crimson` | DC143C | `gold` | FFD700 |
| `indigo` | 4B0082 | `ivory` | FFFFF0 |
| `khaki` | F0E68C | `lavender` | E6E6FA |
| `limeGreen` | 32CD32 | `navy` | 000080 |
| `olive` | 808000 | `orchid` | DA70D6 |
| `peru` | CD853F | `pink` | FFC0CB |
| `plum` | DDA0DD | `salmon` | FA8072 |
| `sienna` | A0522D | `silver` | C0C0C0 |
| `skyBlue` | 87CEEB | `slateBlue` | 6A5ACD |
| `steelBlue` | 4682B4 | `tan` | D2B48C |
| `teal` | 008080 | `tomato` | FF6347 |
| `turquoise` | 40E0D0 | `violet` | EE82EE |
| `wheat` | F5DEB3 | | |

The full table includes 149 named colors matching CSS/X11 color names.

---

## System Color Resolution

`a:sysClr` references OS-dependent colors. The `lastClr` attribute provides a fallback:

```xml
<a:sysClr val="windowText" lastClr="000000"/>
```

For cross-platform rendering, always use `lastClr` as the resolved value.

| val | Typical lastClr | Description |
|-----|----------------|-------------|
| `windowText` | 000000 | Window text (black) |
| `window` | FFFFFF | Window background (white) |
| `highlight` | 0078D7 | Selection highlight |
| `highlightText` | FFFFFF | Selection text |
| `btnFace` | F0F0F0 | Button face |
| `btnText` | 000000 | Button text |
| `grayText` | 6D6D6D | Disabled text |

---

## Complete Modifier Processing Example

### Input

```xml
<a:schemeClr val="accent1">
  <a:tint val="40000"/>
  <a:satMod val="350000"/>
</a:schemeClr>
```

### Processing

```
1. Resolve accent1 from theme: #4472C4 = RGB(68, 114, 196)

2. Apply tint val=40000:
   R' = 255 - (255 - 68) * 40000 / 100000 = 255 - 74.8 = 180
   G' = 255 - (255 - 114) * 40000 / 100000 = 255 - 56.4 = 199
   B' = 255 - (255 - 196) * 40000 / 100000 = 255 - 23.6 = 231
   After tint: RGB(180, 199, 231)

3. Apply satMod val=350000 (350% saturation):
   Convert RGB(180, 199, 231) to HSL-240:
     Max=231, Min=180, Delta=51
     L = (231 + 180) * 240 / 510 = 193
     S = 51 * 240 / (510 - 411) = 51 * 240 / 99 = 124
     H = 80 + (231 - 180) * 40 / 51 = 80 + 40 = 120

   S' = 124 * 350000 / 100000 = 434 → clamp to 240

   Convert HSL(120, 240, 193) back to RGB:
   Result: approximately RGB(146, 182, 240) — a more saturated blue

4. Final: RGB(146, 182, 240) with alpha 1.0
```

---

## Key Implementation Gotchas

1. **HSL range is 0-240, NOT 0-360/0-100** - This is the single most common source of bugs
2. **Modifiers are sequential** - Order matters; `tint` then `shade` differs from `shade` then `tint`
3. **Integer vs float** - ONLYOFFICE uses integer arithmetic for HSL. Use integer math for exact parity
4. **scRGB gamma** - The sRGB transfer function has a linear segment below 0.0031308
5. **phClr substitution** - Must happen before modifier application
6. **lumMod + lumOff are separate modifiers** - Each is a separate XML element applied in sequence, not a combined operation
7. **Alpha is a modifier** - It appears in the modifier chain like any other modifier, not as a separate channel
