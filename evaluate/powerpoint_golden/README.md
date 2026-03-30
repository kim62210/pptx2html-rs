# PowerPoint Reference Goldens

This directory stores PowerPoint-rendered reference artifacts.

## Purpose

LibreOffice remains useful for broad regression detection, but PowerPoint is the primary fidelity oracle for features that claim `exact` support.

## Directory Contract

Each input deck gets its own subdirectory:

```text
evaluate/powerpoint_golden/
  <deck-name>/
    Slide1.PNG
    Slide2.PNG
    ...
```

## Generation Workflow

On a Windows machine with Microsoft PowerPoint installed:

```powershell
pwsh -File ./reference_render_powerpoint.ps1 -InputDir ./golden_set -OutputDir ./powerpoint_golden
```

The script opens each `.pptx` file in `evaluate/golden_set/` and exports each deck as slide images using PowerPoint's native rendering engine.

## Rules

1. Treat these artifacts as the primary reference for any feature labeled `exact`.
2. Regenerate them in a pinned environment when the golden-set source files change.
3. Keep LibreOffice-generated references as a secondary regression signal, not the final fidelity authority.
