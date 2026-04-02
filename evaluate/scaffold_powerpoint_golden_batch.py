from __future__ import annotations

import argparse
import json
from pathlib import Path

from pptx import Presentation

try:
    from evaluate.validate_powerpoint_golden import REQUIRED_METADATA_FIELDS
except ModuleNotFoundError:
    from validate_powerpoint_golden import REQUIRED_METADATA_FIELDS


class ScaffoldError(RuntimeError):
    pass


def scaffold_powerpoint_golden_batch(
    golden_set_dir: Path,
    output_dir: Path,
    metadata: dict[str, str],
) -> dict[str, object]:
    golden_set_dir = Path(golden_set_dir)
    output_dir = Path(output_dir)

    if not golden_set_dir.is_dir():
        raise ScaffoldError(f"Golden set directory does not exist: {golden_set_dir}")
    if not output_dir.is_dir():
        raise ScaffoldError(f"PowerPoint output directory does not exist: {output_dir}")

    missing_fields = [
        field for field in REQUIRED_METADATA_FIELDS if not metadata.get(field)
    ]
    if missing_fields:
        missing_list = ", ".join(missing_fields)
        raise ScaffoldError(f"Missing required metadata fields: {missing_list}")

    decks: list[dict[str, object]] = []
    total_slide_count = 0

    for deck_path in sorted(golden_set_dir.glob("*.pptx")):
        deck_name = deck_path.stem
        deck_output = output_dir / deck_name
        if not deck_output.is_dir():
            raise ScaffoldError(
                f"Missing PowerPoint output directory for deck '{deck_name}'"
            )

        slide_count = len(Presentation(deck_path).slides)
        deck_metadata = {
            **metadata,
            "deck_name": deck_name,
            "slide_count": slide_count,
        }
        (deck_output / "metadata.json").write_text(
            json.dumps(deck_metadata, indent=2, ensure_ascii=False) + "\n",
            encoding="utf-8",
        )

        decks.append(
            {
                "name": deck_name,
                "slide_count": slide_count,
                "output_dir": deck_name,
            }
        )
        total_slide_count += slide_count

    manifest = {
        **metadata,
        "deck_count": len(decks),
        "total_slide_count": total_slide_count,
        "decks": decks,
    }
    (output_dir / "manifest.json").write_text(
        json.dumps(manifest, indent=2, ensure_ascii=False) + "\n",
        encoding="utf-8",
    )

    return {
        "deck_count": len(decks),
        "slide_image_count": total_slide_count,
        "validated_decks": [deck["name"] for deck in decks],
    }


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--golden-set-dir", type=Path, default=Path("golden_set"))
    parser.add_argument("--output-dir", type=Path, default=Path("powerpoint_golden"))
    parser.add_argument("--powerpoint-version", required=True)
    parser.add_argument("--powerpoint-channel", required=True)
    parser.add_argument("--windows-version", required=True)
    parser.add_argument("--export-command", required=True)
    parser.add_argument("--output-resolution", required=True)
    parser.add_argument("--golden-set-revision", required=True)
    parser.add_argument("--capture-date", required=True)
    args = parser.parse_args()

    summary = scaffold_powerpoint_golden_batch(
        args.golden_set_dir,
        args.output_dir,
        metadata={
            "powerpoint_version": args.powerpoint_version,
            "powerpoint_channel": args.powerpoint_channel,
            "windows_version": args.windows_version,
            "export_command": args.export_command,
            "output_resolution": args.output_resolution,
            "golden_set_revision": args.golden_set_revision,
            "capture_date": args.capture_date,
        },
    )
    print(
        f"Scaffolded {summary['deck_count']} deck(s) and {summary['slide_image_count']} slide(s)"
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
