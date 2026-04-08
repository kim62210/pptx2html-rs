# Unreleased Release Notes Draft

> Draft only. Do not publish or tag from this file without a final release decision.

See [`README.md`](./README.md) for the release-note workflow in this directory.
See [`pre-release-checklist.md`](./pre-release-checklist.md) before turning this draft into a tagged release.

## Suggested Title

`vNEXT - chart coverage expansion, text wrap fidelity, and release-validation hardening`

## Suggested Summary

- Expand direct chart rendering coverage across bar/column, line, area, scatter, single-series bubble (area semantics only), single-series radar, pie, doughnut, and flat-rendered single-series pie3D paths while letting unsupported chart parts use preview-image fallback before placeholders.
- Tighten text-wrap fidelity so unbreakable narrow-box tokens that span adjacent text runs still trigger the emergency-wrap path when needed.
- Respect paragraph-level default font sizes in the same narrow-wrap/autofit path so measurement uses the rendered text size even when runs omit `sz`.
- Respect inherited `txStyles` / `defaultTextStyle` font sizes in the same narrow-wrap/autofit path so placeholder text uses the same effective size for rendering and wrap classification.
- Keep `spAutoFit` text bodies on the grow-to-fit path for long unbreakable tokens instead of forcing emergency word breaking.
- Preserve inherited `lnSpcReduction` when a child `normAutofit` override only changes `fontScale`, so partial overrides keep the same shrink semantics.
- Treat NBSP-separated text as non-breaking during wrap classification so emergency-wrap decisions match browser layout more closely.
- Treat soft hyphen as a discretionary break opportunity during wrap classification so normal hyphenation does not fall back to emergency wrapping.
- Treat fullwidth and ideographic forms as East Asian breakable units during wrap classification so they do not collapse into one Latin-style token.
- Treat mixed East Asian/Latin script boundaries as natural break opportunities during wrap classification so narrow autofit text does not fall back to emergency wrapping unnecessarily.
- Keep CJK non-starter punctuation attached to the preceding glyph during wrap classification so punctuation clusters do not undercount line width.
- Treat slash-separated text as having ordinary visible break opportunities during wrap classification so it does not fall back to emergency wrapping.
- Treat hyphen-separated text as having ordinary visible break opportunities during wrap classification so it does not fall back to emergency wrapping.
- Keep CJK opening punctuation attached to the following glyph during wrap classification so opener+glyph clusters do not undercount line width.
- Keep CJK closing angle-bracket punctuation attached to the preceding glyph during wrap classification so closer+glyph clusters do not undercount line width.
- Keep CJK white square bracket pairs on the same East Asian punctuation cluster during wrap classification so bracketed glyphs do not undercount line width.
- Keep CJK tortoise-shell bracket pairs on the same East Asian punctuation cluster during wrap classification so bracketed glyphs do not undercount line width.
- Keep CJK lenticular bracket pairs on the same East Asian punctuation cluster during wrap classification so bracketed glyphs do not undercount line width.
- Harden release-readiness with exactness-contract validation, installed-wheel Python smoke coverage, and WASM package/runtime smoke checks.
- Align the root README, evaluation guide, support-contract docs, and release-note workflow around the same exactness and packaging expectations.

## Rendering

- Direct clustered, stacked, and percent-stacked bar/column chart rendering now covers spacing controls (`gapWidth`, `overlap`) and first-pass data-label positioning.
- Simple line, area, scatter, pie, and doughnut charts now render directly in more cases, including explicit marker settings, point labels, and axis titles where supported.
- Narrow-box text now detects unbreakable tokens even when the token spans adjacent runs with different fonts, so emergency wrapping is triggered from the combined token width instead of per-run fragments.
- Narrow-box autofit text now also respects paragraph-level default run sizes when measuring those combined tokens.
- Narrow-box autofit text now also respects inherited text-style font sizes from placeholder/default style chains when measuring those combined tokens.
- `spAutoFit` text bodies now keep their growth-oriented sizing semantics even when the content is a long unbreakable token.
- Child `normAutofit` overrides now keep inherited line-spacing reduction instead of dropping it when they only override `fontScale`.
- NBSP-separated text now enters the same emergency-wrap path as other non-breaking tokens when it no longer fits the box.
- Soft-hyphenated text now stays on the normal break path instead of being treated as one long unbreakable token.
- Fullwidth and ideographic-width forms now stay on the natural East Asian break path instead of being measured like one Latin-style token.
- Mixed East Asian/Latin text like `漢ABC漢DEF` now stays on the natural script-boundary break path instead of opting into emergency wrapping in narrow autofit layouts.
- CJK punctuation clusters like `漢、` now stay on the same non-breaking cluster for emergency-wrap decisions.
- Slash-separated text like `Alpha/Beta/Gamma` now stays on the normal break path instead of being treated as one long unbreakable token.
- Hyphen-separated text like `Alpha-Beta-Gamma` now stays on the normal break path instead of being treated as one long unbreakable token.
- CJK opening punctuation clusters like `（漢` now stay on the same non-breaking cluster for emergency-wrap decisions.
- CJK closing angle-bracket clusters like `漢》` now stay on the same non-breaking cluster for emergency-wrap decisions.
- White square bracket clusters like `〚漢〛` now stay on the same East Asian punctuation cluster for emergency-wrap decisions.
- Tortoise-shell bracket clusters like `〔漢〕` now stay on the same East Asian punctuation cluster for emergency-wrap decisions.
- Lenticular bracket clusters like `〘漢〙` now stay on the same East Asian punctuation cluster for emergency-wrap decisions.
- Unsupported chart families and complex variants continue to use stable preview or placeholder fallback paths instead of partially rendered output.

## Validation and Packaging

- CI now checks exactness-contract drift between docs and workflows before publishing evaluation artifacts.
- The text/layout exactness gate now spells out narrow-wrap, mixed-font, and autofit expectations alongside the required evidence bundle.
- Exactness contract checks now also fail fast when the text-layout fixture bundle in `evaluate/README.md` drifts from `evaluate/powerpoint_evidence.py`.
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
- Multi-series pie and unsupported chart families still remain on preview-image or placeholder fallback paths; simple single-series 3D pie charts now flatten through the direct pie renderer.
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
