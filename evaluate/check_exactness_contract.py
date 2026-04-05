from __future__ import annotations

import argparse
import json
from pathlib import Path


def check_exactness_contract(repo_root: str | Path) -> dict[str, object]:
    root = Path(repo_root)

    checks = [
        (
            "README.md: exact promotion requires PowerPoint-reference check",
            root / "README.md",
            ["require a PowerPoint-reference check before labeling a feature `exact`"],
        ),
        (
            "docs/architecture/CAPABILITY_MATRIX.md: cites evaluate/README.md and evaluate/powerpoint_golden/README.md",
            root / "docs/architecture/CAPABILITY_MATRIX.md",
            ["evaluate/README.md", "evaluate/powerpoint_golden/README.md"],
        ),
        (
            "evaluate/README.md: documents summary and text-layout gate artifacts",
            root / "evaluate/README.md",
            [
                "`powerpoint-evidence-summary.json`",
                "`powerpoint-evidence-text-layout-gate.json`",
            ],
        ),
        (
            ".github/workflows/ci.yml: emits summary and text-layout gate JSON artifacts",
            root / ".github/workflows/ci.yml",
            [
                "powerpoint-evidence-summary.json",
                "powerpoint-evidence-text-layout-gate.json",
                "python evaluate/powerpoint_evidence.py summary",
                "python evaluate/powerpoint_evidence.py gate --family text-layout",
            ],
        ),
        (
            ".github/workflows/release.yml: emits summary and text-layout gate JSON artifacts",
            root / ".github/workflows/release.yml",
            [
                "powerpoint-evidence-summary.json",
                "powerpoint-evidence-text-layout-gate.json",
                "python evaluate/powerpoint_evidence.py summary",
                "python evaluate/powerpoint_evidence.py gate --family text-layout",
            ],
        ),
    ]

    missing_checks: list[str] = []
    checked_files: list[str] = []

    for label, path, snippets in checks:
        checked_files.append(str(path.relative_to(root)))
        if not path.is_file():
            missing_checks.append(label)
            continue

        content = path.read_text(encoding="utf-8")
        if any(snippet not in content for snippet in snippets):
            missing_checks.append(label)

    return {
        "ok": not missing_checks,
        "checked_files": checked_files,
        "missing_checks": missing_checks,
    }


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--repo-root", required=True)
    parser.add_argument("--output-json", type=Path)
    args = parser.parse_args(argv)

    payload = check_exactness_contract(args.repo_root)
    text = json.dumps(payload, indent=2, ensure_ascii=False)
    print(text)
    if args.output_json is not None:
        args.output_json.write_text(text + "\n", encoding="utf-8")

    return 0 if payload["ok"] else 1


if __name__ == "__main__":
    raise SystemExit(main())
