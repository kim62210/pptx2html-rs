# Color Resolution Chain

> How ECMA-376 resolves scheme colors through Theme, ClrMap, and Modifiers

---

## Overview

Color resolution in PPTX follows a multi-step chain:

```
SchemeClr value (e.g., "accent1")
    │
    ├── Direct scheme? (dk1, lt1, dk2, lt2, accent1-6, hlink, folHlink)
    │   └── Resolve directly from Theme ColorScheme
    │
    └── Mapped scheme? (bg1, bg2, tx1, tx2)
        └── Resolve via ClrMap → then Theme ColorScheme
    │
    ▼
Base RGB color
    │
    ▼
Apply modifiers in document order (tint, shade, lumMod, etc.)
    │
    ▼
Final RGBA color
```

---

## Step 1: Identify Color Type

```xml
<!-- Type A: Direct sRGB - no resolution needed -->
<a:srgbClr val="FF5733"/>

<!-- Type B: Scheme color - requires resolution -->
<a:schemeClr val="accent1"/>

<!-- Type C: System color - OS-dependent with fallback -->
<a:sysClr val="windowText" lastClr="000000"/>

<!-- Type D: HSL color - convert HSL to RGB -->
<a:hslClr hue="14400000" sat="100000" lum="50000"/>

<!-- Type E: Preset color - named color lookup -->
<a:prstClr val="red"/>

<!-- Type F: scRGB - linear RGB, convert to sRGB -->
<a:scrgbClr r="100000" g="0" b="0"/>
```

For Type B (scheme colors), proceed to Step 2.

---

## Step 2: ClrMap Resolution

The `p:clrMap` (on the slide master) maps abstract scheme names to theme color slots.

### Default ClrMap Mapping

```xml
<!-- From slideMaster -->
<p:clrMap bg1="lt1" tx1="dk1" bg2="lt2" tx2="dk2"
          accent1="accent1" accent2="accent2" accent3="accent3"
          accent4="accent4" accent5="accent5" accent6="accent6"
          hlink="hlink" folHlink="folHlink"/>
```

| Scheme Value | ClrMap Output | Theme Slot |
|-------------|--------------|------------|
| `bg1` | `lt1` | `a:lt1` |
| `tx1` | `dk1` | `a:dk1` |
| `bg2` | `lt2` | `a:lt2` |
| `tx2` | `dk2` | `a:dk2` |
| `accent1` | `accent1` | `a:accent1` |
| `accent2` | `accent2` | `a:accent2` |
| `accent3`-`accent6` | `accent3`-`accent6` | `a:accent3`-`a:accent6` |
| `hlink` | `hlink` | `a:hlink` |
| `folHlink` | `folHlink` | `a:folHlink` |
| `dk1` | (direct) | `a:dk1` |
| `lt1` | (direct) | `a:lt1` |
| `dk2` | (direct) | `a:dk2` |
| `lt2` | (direct) | `a:lt2` |

**Important:** `dk1`, `lt1`, `dk2`, `lt2`, `accent1`-`accent6`, `hlink`, `folHlink` bypass the ClrMap and resolve directly from the theme color scheme. Only `bg1`, `bg2`, `tx1`, `tx2` are mapped through ClrMap.

### ClrMap Override

Slides and layouts can override the master's ClrMap:

```xml
<!-- In slide or layout: use master's mapping -->
<p:clrMapOvr>
  <a:masterClrMapping/>
</p:clrMapOvr>

<!-- In slide or layout: override specific mappings -->
<p:clrMapOvr>
  <a:overrideClrMapping bg1="dk1" tx1="lt1" bg2="dk2" tx2="lt2"
                         accent1="accent1" accent2="accent2" accent3="accent3"
                         accent4="accent4" accent5="accent5" accent6="accent6"
                         hlink="hlink" folHlink="folHlink"/>
</p:clrMapOvr>
```

### Resolution Priority

```
Slide clrMapOvr (overrideClrMapping)
    ↓ (if masterClrMapping or absent)
Layout clrMapOvr (overrideClrMapping)
    ↓ (if masterClrMapping or absent)
Master clrMap
```

---

## Step 3: Theme ColorScheme Lookup

After ClrMap resolution, look up the actual RGB value from the theme:

```xml
<!-- ppt/theme/theme1.xml -->
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
```

### Example Resolution

```xml
<!-- Input -->
<a:schemeClr val="tx1"/>

<!-- Step 2: ClrMap says tx1 → dk1 -->
<!-- Step 3: Theme dk1 = sysClr windowText lastClr="000000" -->
<!-- Result: RGB(0, 0, 0) = black -->
```

```xml
<!-- Input -->
<a:schemeClr val="accent1"/>

<!-- Step 2: accent1 maps directly (no ClrMap transform) -->
<!-- Step 3: Theme accent1 = srgbClr "4472C4" -->
<!-- Result: RGB(68, 114, 196) -->
```

---

## Step 4: Apply Color Modifiers

Modifiers are applied **sequentially in document order**. Each modifier operates on the result of the previous one.

### Modifier Processing Order

```xml
<a:schemeClr val="accent1">
  <a:tint val="40000"/>       <!-- applied first -->
  <a:satMod val="120000"/>    <!-- applied second -->
  <a:lumMod val="80000"/>     <!-- applied third -->
  <a:alpha val="75000"/>      <!-- applied fourth -->
</a:schemeClr>
```

### Modifier Formulas

#### tint

Tint towards white. `val` range: 0-100000.

```
R' = 255 - (255 - R) * val / 100000
G' = 255 - (255 - G) * val / 100000
B' = 255 - (255 - B) * val / 100000
```

**Example:** accent1 (#4472C4) with tint=40000 (40%)
```
R' = 255 - (255 - 68) * 40000 / 100000 = 255 - 74.8 = 180
G' = 255 - (255 - 114) * 40000 / 100000 = 255 - 56.4 = 199
B' = 255 - (255 - 196) * 40000 / 100000 = 255 - 23.6 = 231
Result: RGB(180, 199, 231) = #B4C7E7
```

#### shade

Shade towards black. `val` range: 0-100000.

```
R' = R * val / 100000
G' = G * val / 100000
B' = B * val / 100000
```

**Example:** accent1 (#4472C4) with shade=50000 (50%)
```
R' = 68 * 50000 / 100000 = 34
G' = 114 * 50000 / 100000 = 57
B' = 196 * 50000 / 100000 = 98
Result: RGB(34, 57, 98) = #223962
```

#### lumMod / lumOff (Luminance Modify / Offset)

Operates in HSL color space.

```
1. Convert RGB to HSL
2. L' = L * lumMod / 100000 + lumOff / 100000
3. L' = clamp(L', 0.0, 1.0)
4. Convert HSL back to RGB
```

**Common patterns:**
- **Lighter 40%:** `lumMod=60000, lumOff=40000` — L = L*0.6 + 0.4
- **Lighter 60%:** `lumMod=40000, lumOff=60000` — L = L*0.4 + 0.6
- **Darker 25%:** `lumMod=75000` — L = L*0.75
- **Darker 50%:** `lumMod=50000` — L = L*0.50

#### satMod / satOff (Saturation Modify / Offset)

```
1. Convert RGB to HSL
2. S' = S * satMod / 100000 + satOff / 100000
3. S' = clamp(S', 0.0, 1.0)
4. Convert HSL back to RGB
```

#### alpha

Sets opacity. `val` range: 0-100000.

```
opacity = val / 100000
CSS: rgba(R, G, B, opacity)
```

#### inv (Inverse)

```
R' = R ^ 0xFF  (= 255 - R)
G' = G ^ 0xFF
B' = B ^ 0xFF
```

#### comp (Complementary)

```
1. Convert RGB to HSL
2. H' = (H + 180) mod 360
3. Convert HSL back to RGB
```

#### gray (Grayscale)

```
// Using luminance formula
gray = 0.299 * R + 0.587 * G + 0.114 * B
R' = G' = B' = gray
```

---

## Step 5: Special Color - phClr (Placeholder Color)

`phClr` is a special scheme color value that means "use the color from the context." It appears primarily in theme format schemes.

```xml
<!-- In theme fmtScheme fillStyleLst -->
<a:solidFill>
  <a:schemeClr val="phClr"/>
</a:solidFill>
```

When a shape references this fill via `a:fillRef`:

```xml
<a:fillRef idx="1">
  <a:schemeClr val="accent1"/>  <!-- THIS becomes the phClr value -->
</a:fillRef>
```

The `accent1` color from the `fillRef` replaces `phClr` in the theme fill definition.

---

## Complete Resolution Example

### Input

```xml
<!-- Shape on slide1.xml -->
<p:sp>
  <p:spPr>
    <a:solidFill>
      <a:schemeClr val="bg1">
        <a:lumMod val="85000"/>
        <a:lumOff val="15000"/>
      </a:schemeClr>
    </a:solidFill>
  </p:spPr>
</p:sp>
```

### Resolution Steps

```
1. schemeClr val="bg1"

2. ClrMap lookup: bg1 → lt1

3. Theme lookup: lt1 → sysClr val="window" lastClr="FFFFFF"
   Base color: RGB(255, 255, 255) = white

4. Convert to HSL: H=0, S=0.0, L=1.0

5. Apply lumMod=85000:
   L' = 1.0 * 85000/100000 = 0.85

6. Apply lumOff=15000:
   L' = 0.85 + 15000/100000 = 0.85 + 0.15 = 1.0
   (Note: this particular combination results in no change for white)

   But for a non-white color like RGB(128,128,128), H=0, S=0, L=0.502:
   L' = 0.502 * 0.85 + 0.15 = 0.4267 + 0.15 = 0.5767
   Result: RGB(163, 163, 163) = lighter gray

7. Convert HSL back to RGB
```

### Another Example: Dark Shade of Accent

```xml
<a:schemeClr val="accent1">
  <a:lumMod val="50000"/>
</a:schemeClr>
```

```
1. accent1 → Theme: #4472C4 = RGB(68, 114, 196)
2. Convert to HSL: H=216, S=0.546, L=0.518
3. lumMod=50000: L' = 0.518 * 0.5 = 0.259
4. Convert back: RGB(31, 52, 89) ≈ #1F3459
```

---

## Implementation Pseudocode

```rust
fn resolve_color(
    color_ref: &ColorRef,
    theme: &Theme,
    clr_map: &ClrMap,
) -> Rgba {
    let base_rgb = match color_ref.color_type {
        ColorType::SrgbClr(hex) => parse_hex(hex),
        ColorType::SchemeClr(name) => {
            let theme_slot = if is_mapped_scheme(&name) {
                clr_map.resolve(&name)  // bg1→lt1, tx1→dk1, etc.
            } else {
                name.clone()  // accent1, dk1, lt1, etc. pass through
            };
            theme.color_scheme.get(&theme_slot)
        },
        ColorType::SysClr { last_clr, .. } => parse_hex(last_clr),
        ColorType::HslClr { h, s, l } => hsl_to_rgb(h, s, l),
        ColorType::PrstClr(name) => preset_color_lookup(name),
        ColorType::ScrgbClr { r, g, b } => scrgb_to_srgb(r, g, b),
    };

    let mut rgba = Rgba::from_rgb(base_rgb);

    // Apply modifiers sequentially
    for modifier in &color_ref.modifiers {
        rgba = apply_modifier(rgba, modifier);
    }

    rgba
}

fn apply_modifier(color: Rgba, modifier: &ColorModifier) -> Rgba {
    match modifier {
        ColorModifier::Tint(val) => {
            let f = *val as f64 / 100000.0;
            Rgba {
                r: 255.0 - (255.0 - color.r as f64) * f,
                g: 255.0 - (255.0 - color.g as f64) * f,
                b: 255.0 - (255.0 - color.b as f64) * f,
                a: color.a,
            }
        },
        ColorModifier::Shade(val) => {
            let f = *val as f64 / 100000.0;
            Rgba {
                r: color.r as f64 * f,
                g: color.g as f64 * f,
                b: color.b as f64 * f,
                a: color.a,
            }
        },
        ColorModifier::LumMod(val) => {
            let (h, s, mut l) = rgb_to_hsl(color.r, color.g, color.b);
            l = (l * *val as f64 / 100000.0).clamp(0.0, 1.0);
            let (r, g, b) = hsl_to_rgb(h, s, l);
            Rgba { r, g, b, a: color.a }
        },
        ColorModifier::LumOff(val) => {
            let (h, s, mut l) = rgb_to_hsl(color.r, color.g, color.b);
            l = (l + *val as f64 / 100000.0).clamp(0.0, 1.0);
            let (r, g, b) = hsl_to_rgb(h, s, l);
            Rgba { r, g, b, a: color.a }
        },
        ColorModifier::Alpha(val) => {
            Rgba { a: *val as f64 / 100000.0, ..color }
        },
        // ... other modifiers
    }
}
```
