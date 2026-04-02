# PowerPoint Reference Goldens

This directory stores PowerPoint-rendered reference artifacts.

## Purpose

LibreOffice remains useful for broad regression detection, but PowerPoint is the primary fidelity oracle for features that claim `exact` support.

## Directory Contract

Each input deck gets its own subdirectory:

```text
evaluate/powerpoint_golden/
  manifest.json
  <deck-name>/
    Slide1.PNG
    Slide2.PNG
    ...
    metadata.json
```

## Generation Workflow

On a Windows machine with Microsoft PowerPoint installed:

```powershell
pwsh -File ./reference_render_powerpoint.ps1 `
  -InputDir ./golden_set `
  -OutputDir ./powerpoint_golden `
  -PowerPointChannel "Current Channel" `
  -WindowsVersion "Windows 11 23H2" `
  -OutputResolution "960x540" `
  -GoldenSetRevision <commit-sha>
```

The script opens each `.pptx` file in `evaluate/golden_set/`, exports each deck as slide images using PowerPoint's native rendering engine, then scaffolds `metadata.json` for every deck plus a root `manifest.json`.

## Rules

1. Treat these artifacts as the primary reference for any feature labeled `exact`.
2. Regenerate them in a pinned environment when the golden-set source files change.
3. Keep LibreOffice-generated references as a secondary regression signal, not the final fidelity authority.

## Capture Metadata Template

Record this metadata in `metadata.json` inside each deck directory. Required keys:

```json
{
  "powerpoint_version": "16.0.17726.20160",
  "powerpoint_channel": "Current Channel",
  "windows_version": "Windows 11 23H2",
  "export_command": "pwsh -File ./reference_render_powerpoint.ps1 -InputDir ./golden_set -OutputDir ./powerpoint_golden",
  "output_resolution": "960x540",
  "golden_set_revision": "abc1234",
  "capture_date": "2026-04-02"
}
```

The root `manifest.json` records the same batch metadata plus the exported deck list and expected slide counts. See `manifest.example.json` for the expected shape.

Validate the batch locally with:

```bash
python validate_powerpoint_golden.py --golden-set-dir golden_set --output-dir powerpoint_golden
```

Text/layout `exact` promotions should cite this metadata together with the matching fixture list from `evaluate/README.md`.
