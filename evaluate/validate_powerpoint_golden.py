from __future__ import annotations

import argparse
import json
from pathlib import Path

from pptx import Presentation


REQUIRED_METADATA_FIELDS = (
    "powerpoint_version",
    "powerpoint_channel",
    "windows_version",
    "export_command",
    "output_resolution",
    "golden_set_revision",
    "capture_date",
)


class ValidationError(RuntimeError):
    pass


def validate_powerpoint_golden_batch(
    golden_set_dir: Path, output_dir: Path
) -> dict[str, object]:
    golden_set_dir = Path(golden_set_dir)
    output_dir = Path(output_dir)

    if not golden_set_dir.is_dir():
        raise ValidationError(f"Golden set directory does not exist: {golden_set_dir}")
    if not output_dir.is_dir():
        raise ValidationError(
            f"PowerPoint output directory does not exist: {output_dir}"
        )

    validated_decks: list[str] = []
    slide_image_count = 0

    for deck_path in sorted(golden_set_dir.glob("*.pptx")):
        deck_name = deck_path.stem
        deck_output = output_dir / deck_name
        if not deck_output.is_dir():
            raise ValidationError(
                f"Missing PowerPoint output directory for deck '{deck_name}'"
            )

        metadata_path = deck_output / "metadata.json"
        metadata = _read_metadata(metadata_path)
        missing_fields = [
            field for field in REQUIRED_METADATA_FIELDS if not metadata.get(field)
        ]
        if missing_fields:
            missing_list = ", ".join(missing_fields)
            raise ValidationError(
                f"metadata.json for '{deck_name}' is missing required fields: {missing_list}"
            )

        expected_slide_count = len(Presentation(deck_path).slides)
        for slide_number in range(1, expected_slide_count + 1):
            slide_path = deck_output / f"Slide{slide_number}.PNG"
            if not slide_path.is_file():
                raise ValidationError(
                    f"Missing exported slide image: {slide_path.name} for deck '{deck_name}'"
                )

        extra_pngs = sorted(
            path.name
            for path in deck_output.glob("Slide*.PNG")
            if not _is_expected_slide_name(path.name, expected_slide_count)
        )
        if extra_pngs:
            raise ValidationError(
                f"Unexpected slide exports for deck '{deck_name}': {', '.join(extra_pngs)}"
            )

        validated_decks.append(deck_name)
        slide_image_count += expected_slide_count

    return {
        "deck_count": len(validated_decks),
        "slide_image_count": slide_image_count,
        "validated_decks": validated_decks,
    }


def _read_metadata(path: Path) -> dict[str, object]:
    if not path.is_file():
        raise ValidationError(f"Missing metadata.json: {path}")
    return json.loads(path.read_text(encoding="utf-8"))


def _is_expected_slide_name(name: str, expected_slide_count: int) -> bool:
    if not name.startswith("Slide") or not name.endswith(".PNG"):
        return False
    slide_number = name.removeprefix("Slide").removesuffix(".PNG")
    if not slide_number.isdigit():
        return False
    return 1 <= int(slide_number) <= expected_slide_count


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--golden-set-dir", type=Path, default=Path("golden_set"))
    parser.add_argument("--output-dir", type=Path, default=Path("powerpoint_golden"))
    args = parser.parse_args()

    summary = validate_powerpoint_golden_batch(args.golden_set_dir, args.output_dir)
    print(
        f"Validated {summary['deck_count']} deck(s) and {summary['slide_image_count']} slide image(s)"
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
