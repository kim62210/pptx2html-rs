# Unreleased Release Notes Draft

> Draft only. Do not publish or tag from this file without a final release decision.

## Highlights

- Expanded direct chart rendering coverage across bar/column, line, area, scatter, pie, and doughnut paths, while keeping complex chart families on stable fallback behavior.
- Hardened release-readiness checks with exactness-contract validation, installed-wheel Python smoke tests, and WASM package/runtime smoke coverage.
- Improved release documentation so the root README, changelog, evaluation guide, and support matrix now describe the same validation and exactness expectations.

## Rendering

- Direct clustered, stacked, and percent-stacked bar/column chart rendering now covers spacing controls (`gapWidth`, `overlap`) and first-pass data-label positioning.
- Simple line, area, scatter, pie, and doughnut charts now render directly in more cases, including explicit marker settings, point labels, and axis titles where supported.
- Unsupported chart families and complex variants continue to use stable preview or placeholder fallback paths instead of partially rendered output.

## Validation and Packaging

- CI now checks exactness-contract drift between docs and workflows before publishing evaluation artifacts.
- Tag-based release validation now replays Python wheel runtime smoke and WASM package/runtime smoke before creating release artifacts.
- The npm publish workflow now validates tag-to-version alignment, package metadata shaping, package-root imports, and runtime initialization before publish.
- Python package metadata now exposes homepage, repository, and issues URLs in the installed wheel metadata.

## Evaluation

- Release artifacts now include:
  - `powerpoint-evidence-summary.json`
  - `powerpoint-evidence-text-layout-gate.json`
  - `exactness-contract-report.json`
- The evaluation guide and contract checker now share the same Python 3.11+ floor used by CI and release workflows.
- PowerPoint-reference checks remain required before promoting any capability to `exact`.

## Developer Notes

- This draft reflects the current `Unreleased` state after the `1.0.4` tag.
- No release or deployment action has been performed as part of this draft.
