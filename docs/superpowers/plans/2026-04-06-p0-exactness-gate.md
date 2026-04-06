# P0 Exactness Gate Implementation Plan

> **For agentic workers:** REQUIRED: Use superpowers:subagent-driven-development (if subagents available) or superpowers:executing-plans to implement this plan. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Harden the Text/Layout exactness gate first, then use targeted regressions to drive a minimal wrap-policy fix and final documentation sync.

**Architecture:** The work is split into four sequential slices: contract hardening, regression-first test coverage, minimal Rust renderer updates, and release/capability document synchronization. Each slice has its own verification boundary and commit so status claims are always backed by concrete evidence.

**Tech Stack:** Rust 2024, Cargo workspace tests, Python 3.11+ evaluation tooling, existing `evaluate/` contract and evidence CLIs.

---

## File map

- Modify: `evaluate/README.md`
- Modify: `evaluate/powerpoint_golden/README.md`
- Modify: `evaluate/check_exactness_contract.py`
- Modify: `evaluate/tests/test_check_exactness_contract.py`
- Modify: `crates/pptx2html-core/tests/edge_case_test.rs`
- Modify: `crates/pptx2html-core/tests/hierarchy_test.rs` only if inheritance is directly involved in the reproduced bug
- Modify: `crates/pptx2html-core/src/renderer/mod.rs`
- Modify: `crates/pptx2html-core/src/renderer/text_metrics.rs`
- Modify: `docs/architecture/REMAINING_WORK_PLAN.md`
- Modify: `docs/architecture/CAPABILITY_MATRIX.md`
- Modify: `docs/release-notes/pre-release-checklist.md`
- Modify: `docs/release-notes/unreleased-draft.md`
- Modify: `CHANGELOG.md`

## Guardrails

- Treat `evaluate/` as restricted. In this plan, only `evaluate/README.md`, `evaluate/powerpoint_golden/README.md`, `evaluate/check_exactness_contract.py`, and directly matching unit tests may change.
- Do not modify `evaluate/evaluate_fidelity.py`.
- Do not create a commit that leaves newly added regressions failing.
- If the current text/layout fixture bundle is insufficient, stop and surface that as a separate scope expansion instead of silently editing generator/evidence tooling.

## Chunk 1: Contract hardening

### Task 1: Make the exactness contract explicit

**Files:**
- Modify: `evaluate/README.md`
- Modify: `evaluate/powerpoint_golden/README.md`

- [ ] **Step 1: Update `evaluate/README.md` wording for the text/layout gate**

Add explicit language that the gate covers narrow-box wrapping, mixed-font or mixed-script segmentation fidelity, autofit behavior, and required artifact pairing for any exact promotion.

- [ ] **Step 2: Update `evaluate/powerpoint_golden/README.md` to state what text/layout promotions must cite**

Document that text/layout promotions must cite the capture batch metadata together with the required fixture bundle, not just generic PowerPoint output presence.

- [ ] **Step 3: Review the docs for consistency with `docs/architecture/CAPABILITY_MATRIX.md` and the release checklist**

No behavioral claims should imply that Text or Layout is already `exact`.

### Task 2: Tighten the contract checker

**Files:**
- Modify: `evaluate/check_exactness_contract.py`
- Modify: `evaluate/tests/test_check_exactness_contract.py`

- [ ] **Step 1: Write or update the failing Python tests**

Add assertions for any newly required exactness-contract snippets in `evaluate/README.md` and related docs.

- [ ] **Step 2: Run the focused tests to verify the red state if the new snippets are not yet present**

Run:

```bash
python3 -m unittest evaluate.tests.test_check_exactness_contract
```

Expected before implementation: failures if the new contract wording is not yet checked.

- [ ] **Step 3: Implement the minimal checker changes**

Extend `check_exactness_contract.py` only for targeted wording and artifact expectations already agreed in the docs.

- [ ] **Step 4: Re-run the focused tests**

Run:

```bash
python3 -m unittest evaluate.tests.test_check_exactness_contract
python3 evaluate/check_exactness_contract.py --repo-root .
```

Expected: PASS and `ok: true` for the current tree.

- [ ] **Step 5: Commit**

```bash
git add evaluate/README.md evaluate/powerpoint_golden/README.md evaluate/check_exactness_contract.py evaluate/tests/test_check_exactness_contract.py
git commit -m "docs: define text exactness gate acceptance criteria"
```

## Chunk 2: Regression-first renderer coverage

### Task 3: Add narrow-wrap and autofit regressions

**Files:**
- Modify: `crates/pptx2html-core/tests/edge_case_test.rs`
- Optional modify: `crates/pptx2html-core/tests/hierarchy_test.rs`

- [ ] **Step 1: Write the failing regressions in `edge_case_test.rs`**

Add focused tests for:

- narrow mixed-font text that should wrap normally,
- autofit cases where effective scaling should influence wrap policy,
- content that should still remain emergency-wrapped only when effectively unbreakable.

- [ ] **Step 2: If necessary, add one inheritance regression in `hierarchy_test.rs`**

Only add this if the reproduced issue occurs through inherited `bodyPr` or placeholder resolution rather than direct shape settings.

- [ ] **Step 3: Run focused Rust tests to confirm failure**

Run:

```bash
cargo test --package pptx2html-core --test edge_case_test
cargo test --package pptx2html-core --test hierarchy_test
```

Expected: targeted failure in the new regression coverage.

- [ ] **Step 4: Confirm that the current `basic_text_*` evidence bundle is sufficient for this scope**

If it is not sufficient, stop and treat fixture-bundle expansion as a separate scope decision. Do not modify `evaluate/create_golden_set.py` in this plan.

## Chunk 3: Minimal Rust fix

### Task 4: Tighten wrap-policy classification

**Files:**
- Modify: `crates/pptx2html-core/src/renderer/text_metrics.rs`
- Modify: `crates/pptx2html-core/src/renderer/mod.rs`

- [ ] **Step 1: Inspect the exact failing assertions and keep the fix bounded**

Do not touch run segmentation or font-family fallback logic unless the new regressions prove that is necessary.

- [ ] **Step 2: Implement the smallest helper or heuristic adjustment in `text_metrics.rs`**

Likely edit points:

- `classify_wrap_policy`
- `longest_token_width_px`
- `estimate_unbreakable_token_width_px`

- [ ] **Step 3: Update the renderer call site in `mod.rs` only if needed**

Keep the CSS contract stable:

- `.text-body.emergency-wrap`
- `.text-body.nowrap`
- inline `overflow-wrap: anywhere`
- inline `overflow: hidden` for font-scale cases

- [ ] **Step 4: Run the focused Rust tests**

Run:

```bash
cargo test --package pptx2html-core --test edge_case_test
cargo test --package pptx2html-core --test hierarchy_test
```

Expected: the new regressions pass.

- [ ] **Step 5: Run broader Rust verification**

Run:

```bash
cargo test --package pptx2html-core
cargo test --workspace
```

Expected: PASS.

- [ ] **Step 6: Commit**

```bash
git add crates/pptx2html-core/src/renderer/text_metrics.rs crates/pptx2html-core/src/renderer/mod.rs crates/pptx2html-core/tests/edge_case_test.rs crates/pptx2html-core/tests/hierarchy_test.rs
git commit -m "fix: tighten wrap policy for mixed-font and autofit text"
```

## Chunk 4: Documentation and release sync

### Task 5: Reflect the verified state in capability and release docs

**Files:**
- Modify: `docs/architecture/REMAINING_WORK_PLAN.md`
- Modify: `docs/architecture/CAPABILITY_MATRIX.md`
- Modify: `docs/release-notes/pre-release-checklist.md`
- Modify: `docs/release-notes/unreleased-draft.md`
- Modify: `CHANGELOG.md`

- [ ] **Step 1: Update remaining-work and capability documents**

Record the narrower remaining gap after the wrap/autofit fix, but keep Text and Layout at `approximate` unless the same revision has valid PowerPoint evidence for promotion.

- [ ] **Step 2: Update pre-release checklist and unreleased draft**

Make the release-facing docs match the current exactness gate and validation flow.

- [ ] **Step 3: Update `CHANGELOG.md` `Unreleased`**

Only include what actually shipped in the earlier slices.

- [ ] **Step 4: Run final verification commands**

Run:

```bash
python3 -m unittest evaluate.tests.test_check_exactness_contract evaluate.tests.test_powerpoint_evidence_cli
python3 evaluate/check_exactness_contract.py --repo-root .
cargo test --workspace
```

Expected: PASS.

- [ ] **Step 5: Commit**

```bash
git add docs/architecture/REMAINING_WORK_PLAN.md docs/architecture/CAPABILITY_MATRIX.md docs/release-notes/pre-release-checklist.md docs/release-notes/unreleased-draft.md CHANGELOG.md
git commit -m "docs: sync exactness status and release notes"
```

## Notes for execution

- Keep Chunk 3 single-owner because `renderer/mod.rs` and `text_metrics.rs` are tightly coupled.
- Do not create a separate commit that leaves new tests failing.
- If PowerPoint-native evidence is unavailable locally, stop short of `exact` promotion and document the external verification dependency clearly.
