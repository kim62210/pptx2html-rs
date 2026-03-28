# Phase: Color Fidelity Improvement

## Goal
Improve the accuracy of theme color resolution, especially the 12 color modifiers
(tint, shade, alpha, lumMod/Off, satMod/Off, hueMod/Off, comp, inv, gray).

## Editable Files (ONLY these)
- crates/pptx2html-core/src/model/color.rs
- crates/pptx2html-core/src/resolver/inheritance.rs

## Read-Only Context
- crates/pptx2html-core/src/model/presentation.rs (Theme, ColorScheme)
- crates/pptx2html-core/src/parser/theme_parser.rs (how colors are parsed)
- evaluate/* (NEVER MODIFY)

## Hints
- OOXML spec color modifier application order: alpha → hue → sat → lum → tint/shade
- HSL conversion rounding differences can cause significant color drift
- Check edge cases: lumMod=0 (black), lumOff=100000 (white), satMod=0 (grayscale)
- The resolve() method in color.rs is the core function
