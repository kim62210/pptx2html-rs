# pptx2html-rs Autoresearch

## Overview
You are an AI coding agent working on pptx2html-rs, a pure Rust PPTX-to-HTML converter.
Your goal is to improve the visual fidelity of the HTML output by modifying specific Rust source files.

## Rules
1. ONLY modify files listed in the current phase's "Editable Files" section
2. NEVER modify anything in `evaluate/` — the evaluation function is sacred
3. NEVER modify `Cargo.toml` or add new dependencies
4. Run `cargo check` before committing — if it fails, revert immediately
5. All 145+ existing tests must pass — if any fail, revert
6. Prefer simple, clean changes. 20 lines of hacky code for 0.001 improvement = REJECT
7. Do not ask for confirmation — run experiments until manually stopped
8. Record every experiment in results.tsv

## Workflow
1. Read the current phase's program.md
2. Read the target source files and understand the current implementation
3. Propose an improvement hypothesis
4. Modify the code
5. Run: cargo check → cargo test → python evaluate/evaluate_fidelity.py
6. If score improves: git commit with experiment description
7. If score does not improve: git checkout -- . (revert all changes)
8. Log result in results.tsv
9. Go to step 2

## Evaluation
python evaluate/evaluate_fidelity.py --project-root .
The output line "FIDELITY_SCORE: X.XXX" is the only metric that matters.
Higher is better. Range: 0.0 to 1.0.
