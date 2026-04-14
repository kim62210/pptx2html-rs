# Unreleased Release Notes Draft

> Draft only. Do not publish or tag from this file without a final release decision.

See [`README.md`](./README.md) for the release-note workflow in this directory.
See [`pre-release-checklist.md`](./pre-release-checklist.md) before turning this draft into a tagged release.

## Suggested Title

`v1.1.0 - exact-layout whole-slide scale across Rust, CLI, Python, and WASM`

## Suggested Summary

- Add exact-layout whole-slide scale so rendered output can grow like an image while keeping the original slide ratios, coordinates, and text flow intact.
- Surface the same scale control consistently through the Rust `ConversionOptions`, the CLI `--scale` flag, the Python `scale=` keyword arguments, and the WASM `convert_with_options(..., scale)` APIs.
- Ship a browser demo flow that re-renders from the original PPTX bytes when the zoom changes, so previewed dimensions and exported HTML stay aligned.
- Keep the public GitHub Pages demo on the stable `v1.0.5` line until the `1.1.0` release is tagged.

## Rendering

- The renderer now wraps each slide in a scaled shell and applies a uniform CSS transform to the slide canvas, so zooming does not recompute per-element coordinates.
- Because the scale is applied to the whole slide surface, text keeps the same layout decisions and objects keep the same relative position and aspect ratio.
- Invalid or non-positive scale inputs fall back to `1.0`, keeping the public APIs predictable.

## API Surface

- Rust: `ConversionOptions { scale, .. }`
- CLI: `pptx2html input.pptx --scale 2.0`
- Python: `pptx2html.convert(..., scale=2.0)` and metadata variants
- WASM: `convert_with_options(data, embedImages, includeHidden, slideIndices, scale)` and metadata variants

## Demo and Docs

- The branch-local WASM demo now exposes slider + numeric inputs for image-like whole-slide zoom.
- The root README and WASM package README document the scale parameter and its no-reflow semantics.
- The feature branch demo is labeled as a `v1.1.0` preview while the published GitHub Pages site remains the stable `v1.0.5` demo.

## Validation and Packaging

- All package manifests are aligned to the `1.1.0` version line so future tag validation stays deterministic.
- `scripts/read_release_version.sh` remains the single version gate for release and npm publication workflows.
- Existing CI/release packaging continues to validate the Rust workspace, Python wheel, and WASM package contract against the aligned manifest version.
- GitHub Actions workflow dependencies are pinned to the current Node 24-compatible majors so CI, release, npm publish, and Pages deploy lanes do not carry forward the earlier Node 20 deprecation warnings.
- The feature branch is allowed to run the shared CI workflow directly, so release-prep verification does not depend on opening a pull request first.

## Known Limits

- Scale is uniform whole-slide zoom only; it does not independently resize or reposition individual elements.
- Public GitHub Pages remains on the stable release demo until `1.1.0` is actually released.
- Notes, comments, media, and animation domains remain outside the current direct-rendering scope.

## Validation Notes

- `cargo fmt --all`
- `cargo check --workspace`
- `cargo clippy --workspace -- -D warnings`
- `cargo test --workspace`
- `bash scripts/read_release_version.sh v1.1.0`
- `bash scripts/prepare_wasm_release_package.sh 1.1.0`
- `publish-npm.yml` workflow-dispatch dry run on `feature/slide-scale-output` when repository permissions allow it, otherwise local `npm publish --dry-run` from `crates/pptx2html-wasm/pkg`

Use [`pre-release-checklist.md`](./pre-release-checklist.md) to confirm the final release decision against the current tree.

## Publish Status

- This draft reflects the `feature/slide-scale-output` branch state after the `v1.0.5` release.
- No release or deployment action has been performed as part of this draft.
