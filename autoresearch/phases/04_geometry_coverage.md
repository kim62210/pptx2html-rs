# Phase: Preset Geometry Coverage Expansion

## Goal
Add SVG path generation for more preset shapes beyond the current 30.
ECMA-376 defines 187 preset shapes — add the most commonly used ones.

## Editable Files (ONLY these)
- crates/pptx2html-core/src/renderer/geometry.rs

## Read-Only Context
- ECMA-376 Part 1, §20.1.10 (DrawingML Preset Shape Geometries)
- evaluate/* (NEVER MODIFY)

## Hints
- Each preset has a name (e.g., "flowChartProcess") and a set of path commands
- The generate_preset_path() function maps preset name → SVG path string
- Adjust values (avLst) modify the shape proportionally
- Priority shapes: flowchart shapes, block arrows, equation shapes, action buttons
