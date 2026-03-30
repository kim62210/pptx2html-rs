# Capability Matrix

This document is the source of truth for implementation maturity and fidelity expectations.

## Support Tiers

| Tier | Meaning |
|------|---------|
| `exact` | Intended to match the supported PowerPoint behavior in a controlled evaluation environment |
| `approximate` | Rendered directly, but known to diverge in layout, metrics, or visual details |
| `fallback` | Not fully rendered; emitted as deterministic fallback HTML/metadata |
| `unparsed` | Not yet parsed or not preserved well enough for reliable downstream handling |

## Capability Stages

| Stage | Meaning |
|-------|---------|
| `parsed` | OOXML is captured into the internal model |
| `resolved` | Inheritance/theme/style resolution is applied |
| `rendered` | Direct HTML/CSS/SVG output exists |
| `fidelity-tested` | Compared against a reference workflow in a pinned environment |

## Current High-Level Matrix

| Family | Current Tier | Highest Stage | Target Tier | Owner Chunk | Notes |
|--------|--------------|---------------|-------------|-------------|-------|
| Shapes | `approximate` | `rendered` | `exact` | Chunk 2 | Broad preset/custom SVG coverage exists; PowerPoint-reference validation still needs expansion |
| Text | `approximate` | `rendered` | `exact` | Chunk 2 | Text layout works, but font metrics, line breaking, and autofit still need a dedicated fidelity pass |
| Colors and fills | `approximate` | `rendered` | `exact` | Chunk 2 | Theme/styleRef/color modifier stack is implemented, but needs stronger fidelity-test coverage |
| Tables | `approximate` | `rendered` | `exact` | Chunk 2 | Core table rendering exists; advanced table styling remains limited |
| Images | `approximate` | `rendered` | `exact` | Chunk 1 | Crop/render paths exist; external asset contract is still being hardened |
| Layout and inheritance | `approximate` | `resolved` | `exact` | Chunk 1 | Placeholder matching and ClrMap work, but layout `lstStyle` and template-style carry-over still need closing work |
| Charts | `fallback` | `parsed` | `approximate` | Chunk 3 | Detection exists, but direct chart rendering is not implemented yet |
| SmartArt / OLE / Math | `fallback` | `rendered` | `fallback` | Chunk 3 | Deterministic unresolved placeholders + metadata sideband are emitted |
| Notes / comments / media / animation | `unparsed` | - | `fallback` | Chunk 3 | Domain-specific parsing and fallback contracts still need to be introduced |

## Operating Rules

1. A feature must not be marked `exact` until it has a PowerPoint-reference verification path.
2. Unsupported domains must never silently disappear; they must land in `fallback` or `unparsed` with stable metadata.
3. `SUPPORTED_FEATURES.md` remains the detailed element inventory, but this matrix defines the authoritative support contract.
