# Unreleased Release Notes Draft

> Draft only. Do not publish or tag from this file without a final release decision.

See [`pre-release-checklist.md`](./pre-release-checklist.md) before turning this draft into a tagged release.

## Suggested Title

`vNEXT - chart coverage expansion and release-validation hardening`

## Suggested Summary

- Expand direct chart rendering coverage across bar/column, line, area, scatter, pie, and doughnut paths while keeping unsupported chart families on stable fallback behavior.
- Harden release-readiness with exactness-contract validation, installed-wheel Python smoke coverage, and WASM package/runtime smoke checks.
- Align the root README, evaluation guide, support-contract docs, and release-note workflow around the same exactness and packaging expectations.

## Rendering

- Direct clustered, stacked, and percent-stacked bar/column chart rendering now covers spacing controls (`gapWidth`, `overlap`) and first-pass data-label positioning.
- Simple line, area, scatter, pie, and doughnut charts now render directly in more cases, including explicit marker settings, point labels, and axis titles where supported.
- Unsupported chart families and complex variants continue to use stable preview or placeholder fallback paths instead of partially rendered output.

## Validation and Packaging

- CI now checks exactness-contract drift between docs and workflows before publishing evaluation artifacts.
- Tag-based release validation now replays Python wheel runtime smoke and WASM package/runtime smoke before creating release artifacts.
- The npm publish workflow now validates tag-to-version alignment, package metadata shaping, package-root imports, and runtime initialization before publish.
- Python package metadata now exposes homepage, repository, and issues URLs in installed wheel metadata.

## Evaluation Artifacts

Release validation now produces or verifies the following artifacts:

- `powerpoint-evidence-summary.json`
- `powerpoint-evidence-text-layout-gate.json`
- `exactness-contract-report.json`

The evaluation guide and contract checker now share the same Python 3.11+ floor used by CI and release workflows.

## Known Limits

- PowerPoint-reference checks remain required before promoting any capability to `exact`.
- Multi-series pie, 3D pie, and unsupported chart families still remain on preview-image or placeholder fallback paths.
- Notes, comments, media, and animation domains are still outside the current direct-rendering scope.

## Validation Notes

- `cargo test --workspace`
- installed-wheel Python runtime smoke
- WASM package/runtime smoke and package-root import checks
- exactness contract checker

Use [`pre-release-checklist.md`](./pre-release-checklist.md) to confirm the final release decision against the current tree.

## Publish Status

- This draft reflects the current `Unreleased` state after the `1.0.4` tag.
- No release or deployment action has been performed as part of this draft.
