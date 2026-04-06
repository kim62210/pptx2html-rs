from __future__ import annotations

import argparse
import json
import re
from pathlib import Path

try:
    from evaluate.powerpoint_evidence import EXACT_PROMOTION_FAMILIES
except ModuleNotFoundError:
    from powerpoint_evidence import EXACT_PROMOTION_FAMILIES


def extract_text_layout_gate_fixtures(readme_content: str) -> list[str]:
    section_match = re.search(
        r"### Text/Layout exact-promotion gate(?P<section>.*?)(?:\n### |\Z)",
        readme_content,
        flags=re.DOTALL,
    )
    if section_match is None:
        return []

    return re.findall(r"- `([^`]+)\.pptx`", section_match.group("section"))


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
            "evaluate/README.md: documents text-layout gate behavior expectations",
            root / "evaluate/README.md",
            [
                "narrow-box wrapping should stay on normal wrapping paths unless content remains effectively unbreakable after ordinary break opportunities are considered",
                "mixed-font and mixed-script segmentation should preserve intended run-level font resolution through the text/layout gate",
                "`normAutofit` / `spAutoFit` behavior should be evaluated together with wrapping decisions before exact promotion",
            ],
        ),
        (
            "evaluate/README.md: Python version matches CI/release evaluate workflows",
            root / "evaluate/README.md",
            ["Python 3.11+"],
        ),
        (
            "evaluate/powerpoint_golden/README.md: requires capture metadata and matching fixture bundle for text/layout promotions",
            root / "evaluate/powerpoint_golden/README.md",
            [
                "capture batch metadata together with the matching fixture bundle from `evaluate/README.md`",
            ],
        ),
        (
            ".github/workflows/ci.yml: emits summary and text-layout gate JSON artifacts",
            root / ".github/workflows/ci.yml",
            [
                'python-version: "3.11"',
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
                'python-version: "3.11"',
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

    evaluate_readme = root / "evaluate/README.md"
    if evaluate_readme.is_file():
        checked_files.append(str(evaluate_readme.relative_to(root)))
        documented_fixtures = extract_text_layout_gate_fixtures(
            evaluate_readme.read_text(encoding="utf-8")
        )
        expected_fixtures = EXACT_PROMOTION_FAMILIES.get("text-layout", [])
        if sorted(documented_fixtures) != sorted(expected_fixtures):
            missing_checks.append(
                "evaluate/README.md: text-layout fixture bundle matches powerpoint_evidence.py"
            )
    else:
        missing_checks.append(
            "evaluate/README.md: text-layout fixture bundle matches powerpoint_evidence.py"
        )

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
