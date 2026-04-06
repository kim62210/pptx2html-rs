# P0 Exactness Gate Design

## Goal

Establish a verifiable promotion gate for Text and Layout exactness, then use that gate to drive a minimal text-fidelity fix for narrow wrapping, mixed-font content, and autofit behavior.

## Why this work is first

The repository already documents broad feature coverage, but the remaining blocker for stronger support claims is evidence quality rather than raw surface area. The current state leaves Text and Layout at `approximate` because PowerPoint-reference verification and measurement-driven text behavior are not yet closed tightly enough.

This design intentionally fixes that in a strict order:

1. Define and harden the exactness contract.
2. Add regressions that expose the current narrow-wrap/autofit gap.
3. Make the smallest renderer change that satisfies those regressions.
4. Synchronize capability and release-facing documents with the verified behavior.

## Scope

### In scope

- Strengthen the text/layout exact-promotion contract in `evaluate/` docs and contract checks.
- Add or extend regressions around:
  - narrow text boxes,
  - mixed-font and mixed-script runs,
  - `normAutofit` / `spAutoFit` interactions,
  - inherited `bodyPr` wrap/autofit behavior where directly relevant.
- Make minimal changes in the Rust renderer to improve wrap-policy decisions.
- Sync capability and release docs to the verified repository state.

### Out of scope

- Promoting Text or Layout to `exact` without a valid PowerPoint evidence batch on the same revision.
- Broad refactors of the text rendering pipeline.
- New dependencies.
- Changing the evaluation scoring model in `evaluate/evaluate_fidelity.py`.
- Expanding unrelated rendering families beyond the narrow P0 target.

## Current architecture summary

The exactness contract path currently lives in:

- `evaluate/README.md`
- `evaluate/powerpoint_golden/README.md`
- `evaluate/check_exactness_contract.py`
- `evaluate/tests/test_check_exactness_contract.py`
- `evaluate/powerpoint_evidence.py`
- `evaluate/tests/test_powerpoint_evidence_cli.py`

The text wrap/autofit behavior currently flows through:

- `crates/pptx2html-core/src/renderer/mod.rs`
- `crates/pptx2html-core/src/renderer/text_metrics.rs`
- `crates/pptx2html-core/tests/edge_case_test.rs`
- `crates/pptx2html-core/tests/hierarchy_test.rs`

`renderer/mod.rs` decides the final CSS contract (`nowrap`, `overflow-wrap: anywhere`, and `emergency-wrap`) after consulting auto-fit and wrap policy helpers. `text_metrics.rs` owns the heuristic used to decide whether content is effectively unbreakable in the available width.

## Repository policy boundary

The repository root documentation describes `evaluate/` as a sensitive area. This design therefore treats `evaluate/` as **restricted, not generally editable**.

Allowed `evaluate/` changes in this work:

- exactness-contract wording in `evaluate/README.md`,
- exactness-contract enforcement in `evaluate/check_exactness_contract.py`,
- matching unit tests under `evaluate/tests/` when required to keep the contract checker aligned.

Disallowed `evaluate/` changes in this work unless a later, explicit scope expansion is approved:

- `evaluate/evaluate_fidelity.py`,
- scoring weights or scoring semantics,
- broad changes to the evaluation pipeline,
- opportunistic edits to unrelated evidence tooling.

If the current text/layout fixture bundle is insufficient, that should be treated as a separate follow-up decision rather than silently expanding this P0 slice.

## Design principles

### 1. Contract before implementation

The repository already has a text/layout exact-promotion gate, but it mostly guarantees artifact presence and workflow alignment. This work will make the contract clearer about what the gate is intended to protect:

- narrow-box wrapping must not fall back to emergency wrap too aggressively,
- mixed-font and mixed-script content must preserve segmentation/family intent,
- autofit behavior must be evaluated together with wrap decisions,
- exact promotions must cite both fixture coverage and PowerPoint-native evidence.

### 2. Test the interaction, not isolated symptoms

The existing tests already cover pieces of the problem separately. The main missing value is better interaction coverage where narrow width, mixed-font runs, and autofit influence each other. New regressions should therefore be written against those combined cases rather than introducing broad new fixture families prematurely.

### 3. Minimal renderer diff

The intended implementation change is not a pipeline redesign. The smallest likely edit surface is:

- the wrap-policy decision point in `renderer/mod.rs`, and
- token-width / unbreakable-width estimation in `text_metrics.rs`.

Mixed-font run segmentation should remain untouched unless the new regressions prove it is part of the bug.

### 4. Documentation must reflect verified state, not intent

Capability, changelog, and release-note documents must only claim what the repository can currently justify. If PowerPoint evidence is still external or incomplete, Text and Layout remain `approximate`, even if the Rust-side regressions are fixed.

## Proposed execution slices

### Slice A — Exactness gate contract hardening

Define explicit acceptance language for text/layout exactness and tighten the contract checker only as far as necessary to prevent documentation/workflow drift.

Expected files:

- `evaluate/README.md`
- `evaluate/powerpoint_golden/README.md`
- `evaluate/check_exactness_contract.py`
- `evaluate/tests/test_check_exactness_contract.py`

### Slice B — Regression-first text fidelity coverage

Add failing regressions that describe the narrow-wrap/autofit interaction precisely.

Expected files:

- `crates/pptx2html-core/tests/edge_case_test.rs`
- `crates/pptx2html-core/tests/hierarchy_test.rs` only if inheritance is directly implicated

This slice does **not** expand the golden-set generator or evidence tooling by default. If the current fixture bundle proves insufficient, stop and surface that as a separate scope decision.

### Slice C — Minimal Rust fix

Adjust the wrap-policy heuristic so effective width decisions better reflect autofit and content structure, while preserving the current CSS contract.

Expected files:

- `crates/pptx2html-core/src/renderer/mod.rs`
- `crates/pptx2html-core/src/renderer/text_metrics.rs`

### Slice D — Capability and release sync

Update repository-facing status documents so the documented state matches the validated implementation.

Expected files:

- `docs/architecture/REMAINING_WORK_PLAN.md`
- `docs/architecture/CAPABILITY_MATRIX.md`
- `docs/release-notes/pre-release-checklist.md`
- `docs/release-notes/unreleased-draft.md`
- `CHANGELOG.md`

## Error handling and rollback strategy

- If the contract change expands beyond targeted wording and artifact checks, stop and narrow it.
- If any proposed `evaluate/` edit goes beyond contract docs, contract checks, or directly matching unit tests, stop and treat it as a separate scope decision.
- If the new regressions require touching mixed-font segmentation logic, confirm that the simpler wrap-policy path is insufficient before widening scope.
- If the Rust fix causes unrelated text regressions, revert to the last green state and reduce the change to the smallest failing case.
- If PowerPoint evidence cannot be produced in the current environment, keep the gate and release docs in a “ready for external verification” state rather than claiming `exact`.

## Verification strategy

### Contract verification

- `python3 -m unittest evaluate.tests.test_check_exactness_contract`
- `python3 evaluate/check_exactness_contract.py --repo-root .`

### Renderer verification

- targeted `cargo test --package pptx2html-core` during regression work
- `cargo test --workspace` before any documentation claim is finalized

### Evidence tooling verification

- `python3 -m unittest evaluate.tests.test_powerpoint_evidence_cli`
- optional summary/gate CLI runs against the local golden-set/output directories when fixture or gate bundles change

## Commit model

Planned atomic commits:

1. `docs: define text exactness gate acceptance criteria`
2. `test: add text fidelity regressions for narrow wrap and autofit`
3. `fix: tighten wrap policy for mixed-font and autofit text`
4. `docs: sync exactness status and release notes`

Implementation and tests stay together. Documentation-only gate changes stay separate from Rust behavior changes.

## Success criteria

This design is successful when:

- the exactness gate is more explicit and contract-checked,
- new regressions capture the intended narrow-wrap/autofit behavior,
- the Rust fix passes those regressions with a minimal diff,
- repository status documents reflect the validated result without overstating support tier upgrades.
