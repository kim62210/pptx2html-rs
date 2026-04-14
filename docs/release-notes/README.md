# Release Notes Workflow

This directory holds the human-maintained release-note inputs used before a tag or publish decision.

## Files

- `unreleased-draft.md` — the current draft release body for the next version.
- `pre-release-checklist.md` — the operator-facing checklist to run before tagging or publishing.

## Intended Flow

1. Update `CHANGELOG.md` `Unreleased` with the shipped scope.
2. Update `unreleased-draft.md` so it can be copied into a GitHub release body with minimal editing.
3. Run `pre-release-checklist.md` against the current tree, including the npm workflow-dispatch dry run for the intended version line.
4. Only after human approval, create the release tag and let `.github/workflows/release.yml` attach the validated artifacts.

## Important Note

The current release workflow still uses `generate_release_notes: true` for GitHub's automatic notes.
That means this directory is the human-curated source for release wording, but it is **not** consumed automatically by the workflow.
If you want the polished draft to appear in GitHub Releases, copy or adapt `unreleased-draft.md` when preparing the final release.
