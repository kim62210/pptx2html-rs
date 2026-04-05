# Remaining Work Plan

> Detailed development plan for the remaining OOXML implementation and fidelity work.

## Goal

Provide a sequenced backlog for the remaining pptx2html-rs work, grounded in the current capability contract and repository state.

## Source of Truth

- `docs/architecture/CAPABILITY_MATRIX.md`
- `SUPPORTED_FEATURES.md`
- `README.md`
- `CHANGELOG.md`
- `evaluate/README.md`

## Current Summary

- **Text**: rendered and significantly improved, but still `approximate` until measurement-driven fidelity and PowerPoint-reference verification are expanded.
- **Layout / inheritance**: most placeholder and bodyPr carry-over behavior is implemented, but exactness depends on broader verification and a few remaining metadata/template-style items.
- **Charts**: approximate direct rendering now covers clustered, stacked, and percent-stacked bar/column charts plus simple line and single-series pie charts, but richer labels, axes, and additional chart families remain.
- **Verification**: PowerPoint-first evaluation infrastructure exists, but the golden/reference workflow still needs to be turned into a routine release gate for exact claims.

---

## Priority 0 — Exactness Gate

These items directly block promotion from `approximate` to `exact`.

### P0.1 Text measurement and line-breaking fidelity

**Goal:** Reduce divergence between browser layout and PowerPoint text layout.

**Key gaps:**
- Replace browser-default behavior with more deterministic text measurement where feasible.
- Improve line breaking in narrow boxes, mixed-font runs, and autofit scenarios.
- Revisit `normAutofit`/`spAutoFit` precision once measurement hooks are in place.

**Files likely involved:**
- `crates/pptx2html-core/src/renderer/mod.rs`
- `crates/pptx2html-core/src/renderer/text_metrics.rs`
- `crates/pptx2html-core/tests/edge_case_test.rs`
- `crates/pptx2html-core/tests/hierarchy_test.rs`
- `evaluate/create_golden_set.py`

**Suggested slices:**
1. Add another deterministic line-breaking regression around narrow/mixed-font text.
2. Integrate a small measurement helper into autofit-related rendering decisions.
3. Expand golden fixtures to include mixed-font, narrow-box, and autoshrink reference cases.

### P0.2 PowerPoint-reference verification expansion

**Goal:** Make `exact` claims auditable.

**Key gaps:**
- Generate and maintain real PowerPoint reference renders for text/layout heavy fixtures.
- Add documented acceptance criteria for promoting a feature family to `exact`.
- Expand `evaluate/` coverage from infrastructure to routine verification practice.

**Files likely involved:**
- `evaluate/create_golden_set.py`
- `evaluate/README.md`
- `evaluate/powerpoint_golden/README.md`
- `docs/architecture/CAPABILITY_MATRIX.md`

**Suggested slices:**
1. Add more text/layout-heavy golden PPTX fixtures.
2. Document the minimum PowerPoint capture set required before an `exact` upgrade.
3. Record exact-promotion checklist items in capability docs.

---

## Priority 1 — Remaining direct rendering and fidelity work

### P1.1 Layout / inheritance closing work

**Goal:** Finish the last approximate gaps in placeholder/template carry-over.

**Remaining focus areas:**
- Template-style carry-over verification.
- Metadata placeholders (`dt`, `ftr`, `sldNum`) handling.
- Additional regression coverage for master vs layout vs slide override precedence.

**Files likely involved:**
- `crates/pptx2html-core/src/resolver/inheritance.rs`
- `crates/pptx2html-core/src/renderer/mod.rs`
- `crates/pptx2html-core/tests/hierarchy_test.rs`
- `docs/reference/slide-inheritance.md`

### P1.2 Colors and fills exact pass

**Goal:** Promote color/theme/styleRef resolution from approximate to exact.

**Remaining focus areas:**
- Stronger fidelity-test coverage for modifier stacks.
- Remaining gradient and pattern edge cases.
- More reference-driven validation for schemeClr/styleRef interactions.

**Files likely involved:**
- `crates/pptx2html-core/src/model/color.rs`
- `crates/pptx2html-core/src/resolver/style_ref.rs`
- `crates/pptx2html-core/tests/integration_test.rs`

### P1.3 Tables exact pass

**Goal:** Raise table support quality beyond core grid rendering.

**Remaining focus areas:**
- Advanced table styling.
- Table style references.
- Banding and first/last row/column styling.

**Files likely involved:**
- `crates/pptx2html-core/src/parser/slide_parser.rs`
- `crates/pptx2html-core/src/renderer/mod.rs`
- `crates/pptx2html-core/tests/integration_test.rs`

### P1.4 Images exact pass

**Goal:** Harden image contract and remaining rendering edge cases.

**Remaining focus areas:**
- External asset contract hardening.
- Remaining image effect/tile cases.
- SVG passthrough policy decision.

**Files likely involved:**
- `crates/pptx2html-core/src/parser/slide_parser.rs`
- `crates/pptx2html-core/src/lib.rs`
- `crates/pptx2html-core/tests/integration_test.rs`

---

## Priority 2 — New rendering domains

### P2.1 Charts direct rendering expansion

**Current state:** approximate / rendered

**Goal:** Broaden direct chart coverage while preserving deterministic fallback behavior for unsupported or structurally incompatible chart types.

**Remaining focus areas:**
- Polish the current direct renderer for bar/column/line/pie charts (axis titles, data labels, gap width, overlap, markers, legend/layout details).
- Add additional direct-rendered chart families such as doughnut, area, and scatter.
- Preserve preview-image or placeholder fallback for unsupported chart spaces, 3D chart variants, and incompatible series structures.

**Files likely involved:**
- `crates/pptx2html-core/src/model/slide.rs`
- `crates/pptx2html-core/src/parser/chart_parser.rs`
- `crates/pptx2html-core/src/parser/slide_parser.rs`
- `crates/pptx2html-core/src/renderer/mod.rs`
- `crates/pptx2html-core/tests/integration_test.rs`
- `README.md`
- `docs/architecture/CAPABILITY_MATRIX.md`

**Suggested slices:**
1. Add axis-title, data-label, and spacing/overlap polish for the current bar/column renderer.
2. Add one ring/filled family (`doughnut` or `area`) while keeping stable fallback for unsupported variants.
3. Add one axis-rich family (`scatter` or `area` with marker support) and extend fallback coverage for unsupported structures.

### P2.2 Notes / comments / media / animation fallback contracts

**Current state:** unparsed

**Goal:** Preserve these domains with explicit fallback semantics instead of silent omission.

**Likely work:**
- Notes/comment model capture.
- Media relationship preservation.
- Animation/timing metadata sideband.

---

## Priority 3 — Deferred or strategy-bound work

These remain intentionally lower priority unless product direction changes.

- SmartArt direct rendering
- OLE direct rendering
- Math direct rendering
- Pattern fills
- Advanced image effects
- Multiple text columns
- Curved connector routing
- Print CSS / slide navigation UX

---

## Recommended Execution Order

1. **Text measurement / line-breaking fidelity**
2. **PowerPoint-reference verification expansion for text/layout**
3. **Layout / inheritance closing work**
4. **Colors and fills exact pass**
5. **Tables exact pass**
6. **Images exact pass**
7. **Charts direct rendering expansion**
8. **Notes/comments/media/animation fallback contracts**

---

## Release-readiness Checklist

- [ ] `cargo check --workspace`
- [ ] `cargo test --workspace`
- [ ] `evaluate/create_golden_set.py` stays runnable
- [ ] PowerPoint-reference capture path documented for any `exact` promotion
- [ ] `CAPABILITY_MATRIX.md` and the repository-root `SUPPORTED_FEATURES.md` kept in sync with implementation state
