import tempfile
import unittest
from pathlib import Path

from evaluate.check_exactness_contract import check_exactness_contract


TEXT_LAYOUT_GATE_FIXTURES = [
    "basic_text_08_narrow_box_autofit",
    "basic_text_09_mixed_font_paragraph",
    "basic_text_10_bodypr_fidelity",
    "basic_text_11_wrap_gate_sentence",
    "basic_text_12_wrap_gate_unbreakable",
    "basic_text_13_autofit_modes",
    "basic_text_14_complex_script_fonts",
    "basic_text_15_mixed_script_single_run",
    "basic_text_16_cjk_autofit_wrap_gate",
    "basic_text_17_indic_complex_script_fonts",
    "basic_text_18_emoji_cluster_segments",
]


def text_layout_gate_fixture_lines() -> str:
    return "".join(f"- `{name}.pptx`\n" for name in TEXT_LAYOUT_GATE_FIXTURES)


class CheckExactnessContractTests(unittest.TestCase):
    def test_reports_success_when_docs_and_workflows_are_aligned(self) -> None:
        with tempfile.TemporaryDirectory() as tmpdir:
            root = Path(tmpdir)
            self._write_file(
                root / "README.md",
                "require a PowerPoint-reference check before labeling a feature `exact`\n",
            )
            self._write_file(
                root / "docs/architecture/CAPABILITY_MATRIX.md",
                "evaluate/README.md\nevaluate/powerpoint_golden/README.md\n",
            )
            self._write_file(
                root / "evaluate/README.md",
                (
                    "`powerpoint-evidence-summary.json`\n"
                    "`powerpoint-evidence-text-layout-gate.json`\n"
                    + "### Text/Layout exact-promotion gate\n"
                    + "1. **Fixture coverage** from `create_golden_set.py` for all of these families:\n"
                    + text_layout_gate_fixture_lines()
                    + "- narrow-box wrapping should stay on normal wrapping paths unless content remains effectively unbreakable after ordinary break opportunities are considered\n"
                    + "- mixed-font and mixed-script segmentation should preserve intended run-level font resolution through the text/layout gate\n"
                    + "- mixed East Asian/Latin script boundaries should stay on natural wrap paths before emergency wrapping is considered\n"
                    + "- `normAutofit` / `spAutoFit` behavior should be evaluated together with wrapping decisions before exact promotion\n"
                    + "- Python 3.11+\n"
                ),
            )
            self._write_file(
                root / "evaluate/powerpoint_golden/README.md",
                "text/layout promotions must cite the capture batch metadata together with the matching fixture bundle from `evaluate/README.md`\n",
            )
            self._write_file(
                root / ".github/workflows/ci.yml",
                'python-version: "3.11"\n'
                "python evaluate/powerpoint_evidence.py summary --output-json artifacts/evaluate/powerpoint-evidence-summary.json\n"
                "python evaluate/powerpoint_evidence.py gate --family text-layout --output-json artifacts/evaluate/powerpoint-evidence-text-layout-gate.json || true\n",
            )
            self._write_file(
                root / ".github/workflows/release.yml",
                'python-version: "3.11"\n'
                "python evaluate/powerpoint_evidence.py summary --output-json artifacts/evaluate/powerpoint-evidence-summary.json\n"
                "python evaluate/powerpoint_evidence.py gate --family text-layout --output-json artifacts/evaluate/powerpoint-evidence-text-layout-gate.json || true\n",
            )

            payload = check_exactness_contract(root)

            self.assertTrue(payload["ok"])
            self.assertEqual(payload["missing_checks"], [])

    def test_reports_missing_contract_snippets(self) -> None:
        with tempfile.TemporaryDirectory() as tmpdir:
            root = Path(tmpdir)
            self._write_file(root / "README.md", "")
            self._write_file(root / "docs/architecture/CAPABILITY_MATRIX.md", "")
            self._write_file(root / "evaluate/README.md", "")
            self._write_file(root / ".github/workflows/ci.yml", "")
            self._write_file(root / ".github/workflows/release.yml", "")

            payload = check_exactness_contract(root)

            self.assertFalse(payload["ok"])
            self.assertIn(
                "README.md: exact promotion requires PowerPoint-reference check",
                payload["missing_checks"],
            )
            self.assertIn(
                "docs/architecture/CAPABILITY_MATRIX.md: cites evaluate/README.md and evaluate/powerpoint_golden/README.md",
                payload["missing_checks"],
            )
            self.assertIn(
                "evaluate/README.md: documents summary and text-layout gate artifacts",
                payload["missing_checks"],
            )
            self.assertIn(
                "evaluate/README.md: Python version matches CI/release evaluate workflows",
                payload["missing_checks"],
            )
            self.assertIn(
                "evaluate/README.md: text-layout fixture bundle matches powerpoint_evidence.py",
                payload["missing_checks"],
            )
            self.assertIn(
                "evaluate/README.md: documents text-layout gate behavior expectations",
                payload["missing_checks"],
            )
            self.assertIn(
                "evaluate/powerpoint_golden/README.md: requires capture metadata and matching fixture bundle for text/layout promotions",
                payload["missing_checks"],
            )
            self.assertIn(
                ".github/workflows/ci.yml: emits summary and text-layout gate JSON artifacts",
                payload["missing_checks"],
            )
            self.assertIn(
                ".github/workflows/release.yml: emits summary and text-layout gate JSON artifacts",
                payload["missing_checks"],
            )

    def test_reports_python_version_drift_between_evaluate_docs_and_workflows(
        self,
    ) -> None:
        with tempfile.TemporaryDirectory() as tmpdir:
            root = Path(tmpdir)
            self._write_file(
                root / "README.md",
                "require a PowerPoint-reference check before labeling a feature `exact`\n",
            )
            self._write_file(
                root / "docs/architecture/CAPABILITY_MATRIX.md",
                "evaluate/README.md\nevaluate/powerpoint_golden/README.md\n",
            )
            self._write_file(
                root / "evaluate/README.md",
                (
                    "`powerpoint-evidence-summary.json`\n"
                    "`powerpoint-evidence-text-layout-gate.json`\n"
                    + "### Text/Layout exact-promotion gate\n"
                    + "1. **Fixture coverage** from `create_golden_set.py` for all of these families:\n"
                    + text_layout_gate_fixture_lines()
                    + "- narrow-box wrapping should stay on normal wrapping paths unless content remains effectively unbreakable after ordinary break opportunities are considered\n"
                    + "- mixed-font and mixed-script segmentation should preserve intended run-level font resolution through the text/layout gate\n"
                    + "- mixed East Asian/Latin script boundaries should stay on natural wrap paths before emergency wrapping is considered\n"
                    + "- `normAutofit` / `spAutoFit` behavior should be evaluated together with wrapping decisions before exact promotion\n"
                    + "- Python 3.12+\n"
                ),
            )
            self._write_file(
                root / "evaluate/powerpoint_golden/README.md",
                "text/layout promotions must cite the capture batch metadata together with the matching fixture bundle from `evaluate/README.md`\n",
            )
            self._write_file(
                root / ".github/workflows/ci.yml",
                'python-version: "3.11"\n'
                "python evaluate/powerpoint_evidence.py summary --output-json artifacts/evaluate/powerpoint-evidence-summary.json\n"
                "python evaluate/powerpoint_evidence.py gate --family text-layout --output-json artifacts/evaluate/powerpoint-evidence-text-layout-gate.json || true\n",
            )
            self._write_file(
                root / ".github/workflows/release.yml",
                'python-version: "3.11"\n'
                "python evaluate/powerpoint_evidence.py summary --output-json artifacts/evaluate/powerpoint-evidence-summary.json\n"
                "python evaluate/powerpoint_evidence.py gate --family text-layout --output-json artifacts/evaluate/powerpoint-evidence-text-layout-gate.json || true\n",
            )

            payload = check_exactness_contract(root)

            self.assertFalse(payload["ok"])
            self.assertIn(
                "evaluate/README.md: Python version matches CI/release evaluate workflows",
                payload["missing_checks"],
            )

    def test_reports_text_layout_fixture_bundle_drift(self) -> None:
        with tempfile.TemporaryDirectory() as tmpdir:
            root = Path(tmpdir)
            self._write_file(
                root / "README.md",
                "require a PowerPoint-reference check before labeling a feature `exact`\n",
            )
            self._write_file(
                root / "docs/architecture/CAPABILITY_MATRIX.md",
                "evaluate/README.md\nevaluate/powerpoint_golden/README.md\n",
            )
            drifted_fixture_lines = text_layout_gate_fixture_lines().replace(
                "- `basic_text_18_emoji_cluster_segments.pptx`\n",
                "",
            )
            self._write_file(
                root / "evaluate/README.md",
                "`powerpoint-evidence-summary.json`\n"
                "`powerpoint-evidence-text-layout-gate.json`\n"
                + "### Text/Layout exact-promotion gate\n"
                + "1. **Fixture coverage** from `create_golden_set.py` for all of these families:\n"
                + drifted_fixture_lines
                + "- narrow-box wrapping should stay on normal wrapping paths unless content remains effectively unbreakable after ordinary break opportunities are considered\n"
                + "- mixed-font and mixed-script segmentation should preserve intended run-level font resolution through the text/layout gate\n"
                + "- mixed East Asian/Latin script boundaries should stay on natural wrap paths before emergency wrapping is considered\n"
                + "- `normAutofit` / `spAutoFit` behavior should be evaluated together with wrapping decisions before exact promotion\n"
                + "- Python 3.11+\n",
            )
            self._write_file(
                root / "evaluate/powerpoint_golden/README.md",
                "text/layout promotions must cite the capture batch metadata together with the matching fixture bundle from `evaluate/README.md`\n",
            )
            self._write_file(
                root / ".github/workflows/ci.yml",
                'python-version: "3.11"\n'
                "python evaluate/powerpoint_evidence.py summary --output-json artifacts/evaluate/powerpoint-evidence-summary.json\n"
                "python evaluate/powerpoint_evidence.py gate --family text-layout --output-json artifacts/evaluate/powerpoint-evidence-text-layout-gate.json || true\n",
            )
            self._write_file(
                root / ".github/workflows/release.yml",
                'python-version: "3.11"\n'
                "python evaluate/powerpoint_evidence.py summary --output-json artifacts/evaluate/powerpoint-evidence-summary.json\n"
                "python evaluate/powerpoint_evidence.py gate --family text-layout --output-json artifacts/evaluate/powerpoint-evidence-text-layout-gate.json || true\n",
            )

            payload = check_exactness_contract(root)

            self.assertFalse(payload["ok"])
            self.assertIn(
                "evaluate/README.md: text-layout fixture bundle matches powerpoint_evidence.py",
                payload["missing_checks"],
            )

    def test_reports_missing_mixed_script_wrap_behavior_expectation(self) -> None:
        with tempfile.TemporaryDirectory() as tmpdir:
            root = Path(tmpdir)
            self._write_file(
                root / "README.md",
                "require a PowerPoint-reference check before labeling a feature `exact`\n",
            )
            self._write_file(
                root / "docs/architecture/CAPABILITY_MATRIX.md",
                "evaluate/README.md\nevaluate/powerpoint_golden/README.md\n",
            )
            self._write_file(
                root / "evaluate/README.md",
                (
                    "`powerpoint-evidence-summary.json`\n"
                    "`powerpoint-evidence-text-layout-gate.json`\n"
                    + "### Text/Layout exact-promotion gate\n"
                    + "1. **Fixture coverage** from `create_golden_set.py` for all of these families:\n"
                    + text_layout_gate_fixture_lines()
                    + "- narrow-box wrapping should stay on normal wrapping paths unless content remains effectively unbreakable after ordinary break opportunities are considered\n"
                    + "- mixed-font and mixed-script segmentation should preserve intended run-level font resolution through the text/layout gate\n"
                    + "- `normAutofit` / `spAutoFit` behavior should be evaluated together with wrapping decisions before exact promotion\n"
                    + "- Python 3.11+\n"
                ),
            )
            self._write_file(
                root / "evaluate/powerpoint_golden/README.md",
                "text/layout promotions must cite the capture batch metadata together with the matching fixture bundle from `evaluate/README.md`\n",
            )
            self._write_file(
                root / ".github/workflows/ci.yml",
                'python-version: "3.11"\n'
                "python evaluate/powerpoint_evidence.py summary --output-json artifacts/evaluate/powerpoint-evidence-summary.json\n"
                "python evaluate/powerpoint_evidence.py gate --family text-layout --output-json artifacts/evaluate/powerpoint-evidence-text-layout-gate.json || true\n",
            )
            self._write_file(
                root / ".github/workflows/release.yml",
                'python-version: "3.11"\n'
                "python evaluate/powerpoint_evidence.py summary --output-json artifacts/evaluate/powerpoint-evidence-summary.json\n"
                "python evaluate/powerpoint_evidence.py gate --family text-layout --output-json artifacts/evaluate/powerpoint-evidence-text-layout-gate.json || true\n",
            )

            payload = check_exactness_contract(root)

            self.assertFalse(payload["ok"])
            self.assertIn(
                "evaluate/README.md: documents text-layout gate behavior expectations",
                payload["missing_checks"],
            )

    def _write_file(self, path: Path, content: str) -> None:
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_text(content, encoding="utf-8")


if __name__ == "__main__":
    unittest.main()
