import contextlib
import io
import json
import tempfile
import unittest
from pathlib import Path

from pptx import Presentation

from evaluate.powerpoint_evidence import main


class PowerPointEvidenceCliTests(unittest.TestCase):
    def test_ready_command_returns_nonzero_when_batch_is_incomplete(self) -> None:
        with tempfile.TemporaryDirectory() as tmpdir:
            root = Path(tmpdir)
            golden_set = root / "golden_set"
            output_dir = root / "powerpoint_golden"
            golden_set.mkdir()
            output_dir.mkdir()
            self._create_pptx(golden_set / "sample.pptx", slide_count=1)

            stdout = io.StringIO()
            with contextlib.redirect_stdout(stdout):
                exit_code = main(
                    [
                        "ready",
                        "--golden-set-dir",
                        str(golden_set),
                        "--output-dir",
                        str(output_dir),
                    ]
                )

            self.assertEqual(exit_code, 1)
            payload = json.loads(stdout.getvalue())
            self.assertFalse(payload["evidence_ready_for_exact_promotion"])

    def test_ready_command_returns_zero_when_batch_is_complete(self) -> None:
        with tempfile.TemporaryDirectory() as tmpdir:
            root = Path(tmpdir)
            golden_set = root / "golden_set"
            output_dir = root / "powerpoint_golden"
            golden_set.mkdir()
            output_dir.mkdir()

            deck_path = golden_set / "sample.pptx"
            self._create_pptx(deck_path, slide_count=1)
            deck_output = output_dir / "sample"
            deck_output.mkdir()
            (deck_output / "Slide1.PNG").write_bytes(b"png1")

            stdout = io.StringIO()
            with contextlib.redirect_stdout(stdout):
                scaffold_exit = main(
                    [
                        "scaffold",
                        "--golden-set-dir",
                        str(golden_set),
                        "--output-dir",
                        str(output_dir),
                        "--powerpoint-version",
                        "16.0.17726.20160",
                        "--powerpoint-channel",
                        "Current Channel",
                        "--windows-version",
                        "Windows 11 23H2",
                        "--export-command",
                        "pwsh -File ./reference_render_powerpoint.ps1 -InputDir ./golden_set -OutputDir ./powerpoint_golden",
                        "--output-resolution",
                        "960x540",
                        "--golden-set-revision",
                        "abc1234",
                        "--capture-date",
                        "2026-04-02",
                    ]
                )
            self.assertEqual(scaffold_exit, 0)

            stdout = io.StringIO()
            with contextlib.redirect_stdout(stdout):
                exit_code = main(
                    [
                        "ready",
                        "--golden-set-dir",
                        str(golden_set),
                        "--output-dir",
                        str(output_dir),
                    ]
                )

            self.assertEqual(exit_code, 0)
            payload = json.loads(stdout.getvalue())
            self.assertTrue(payload["evidence_ready_for_exact_promotion"])

    def test_summary_command_emits_json(self) -> None:
        with tempfile.TemporaryDirectory() as tmpdir:
            root = Path(tmpdir)
            golden_set = root / "golden_set"
            output_dir = root / "powerpoint_golden"
            golden_set.mkdir()
            output_dir.mkdir()
            self._create_pptx(golden_set / "sample.pptx", slide_count=1)

            stdout = io.StringIO()
            with contextlib.redirect_stdout(stdout):
                exit_code = main(
                    [
                        "summary",
                        "--golden-set-dir",
                        str(golden_set),
                        "--output-dir",
                        str(output_dir),
                    ]
                )

            self.assertEqual(exit_code, 0)
            payload = json.loads(stdout.getvalue())
            self.assertEqual(payload["golden_deck_count"], 1)
            self.assertEqual(payload["missing_decks"], ["sample"])

    def _create_pptx(self, path: Path, slide_count: int) -> None:
        presentation = Presentation()
        blank_layout = presentation.slide_layouts[6]
        while len(presentation.slides) < slide_count:
            presentation.slides.add_slide(blank_layout)
        presentation.save(path)


if __name__ == "__main__":
    unittest.main()
