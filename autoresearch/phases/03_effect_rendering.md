# Phase: Effect Rendering (Shadow, Glow)

## Goal
Implement CSS rendering for shape-level effects (outerShadow, glow).
Currently these effects are ignored in the renderer.

## Editable Files (ONLY these)
- crates/pptx2html-core/src/renderer/mod.rs
- crates/pptx2html-core/src/model/style.rs (if new structs needed)

## Read-Only Context
- crates/pptx2html-core/src/parser/slide_parser.rs (how effects are parsed)
- ECMA-376 Part 1, §20.1.8 (DrawingML Effects)
- evaluate/* (NEVER MODIFY)

## Hints
- outerShdw → CSS box-shadow (blur, offset-x, offset-y, color with alpha)
- glow → CSS box-shadow with spread but no offset
- Effect XML: <a:effectLst><a:outerShdw blurRad="50800" dist="38100" dir="2700000">
- blurRad/dist units are EMU (divide by 12700 for pt)
- dir is in 60000ths of a degree (2700000 = 45°)
