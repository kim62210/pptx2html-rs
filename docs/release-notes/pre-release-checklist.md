# Pre-Release Checklist

Use this checklist before creating a release tag or publishing release artifacts.

See [`README.md`](./README.md) for the intended release-note workflow and how this checklist relates to `unreleased-draft.md`.

## 1. Workspace Health

- [ ] `cargo check --workspace`
- [ ] `cargo test --workspace`
- [ ] Local working tree is clean

## 2. Packaging Validation

- [ ] Python wheel builds successfully via `maturin`
- [ ] Installed-wheel smoke tests pass for the published `pptx2html` API surface
- [ ] WASM package preparation passes via `scripts/prepare_wasm_release_package.sh <version>`
- [ ] WASM package contract, package-root import smoke, and runtime smoke all pass
- [ ] `publish-npm.yml` succeeds in `workflow_dispatch` dry-run mode for the intended version line before the real tag is pushed — or, if the current operator lacks dispatch/admin rights, the equivalent local `npm publish --dry-run` path is verified from `crates/pptx2html-wasm/pkg`

## 3. Evaluation Artifacts

- [ ] `python3 evaluate/check_exactness_contract.py --repo-root .` returns `ok: true`
- [ ] `powerpoint-evidence-summary.json` is present or reproducible from the current tree
- [ ] `powerpoint-evidence-text-layout-gate.json` is present or reproducible from the current tree
- [ ] `exactness-contract-report.json` is present or reproducible from the current tree

## 4. Exactness Claims

- [ ] No capability is labeled `exact` without a documented PowerPoint-reference verification path
- [ ] Text/layout promotions cite the text/layout gate defined in `evaluate/README.md`
- [ ] The text/layout gate wording in `evaluate/README.md` still matches the current narrow-wrap, mixed-font, and autofit behavior covered by tests
- [ ] The text-layout fixture bundle documented in `evaluate/README.md` still matches `evaluate/powerpoint_evidence.py`
- [ ] `CAPABILITY_MATRIX.md` and the repository-root `SUPPORTED_FEATURES.md` still describe the same current state

## 5. Release Notes and Docs

- [ ] `CHANGELOG.md` `Unreleased` section reflects the actual shipped scope
- [ ] `docs/release-notes/unreleased-draft.md` is updated or intentionally superseded
- [ ] Root `README.md` reflects any new validation or packaging expectations introduced since the last release

## 6. Tagging Decision

- [ ] `bash scripts/read_release_version.sh <tag>` succeeds for the intended release tag
- [ ] Release artifacts are ready, but no deployment/publish step is triggered until a human approves the tag/publish action
