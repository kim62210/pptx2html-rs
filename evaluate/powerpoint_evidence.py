from __future__ import annotations

import argparse
import json

try:
    from evaluate.scaffold_powerpoint_golden_batch import (
        scaffold_powerpoint_golden_batch,
    )
    from evaluate.summarize_powerpoint_golden import summarize_powerpoint_golden_batch
    from evaluate.validate_powerpoint_golden import validate_powerpoint_golden_batch
except ModuleNotFoundError:
    from scaffold_powerpoint_golden_batch import scaffold_powerpoint_golden_batch
    from summarize_powerpoint_golden import summarize_powerpoint_golden_batch
    from validate_powerpoint_golden import validate_powerpoint_golden_batch


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser()
    subparsers = parser.add_subparsers(dest="command", required=True)

    def add_common_paths(subparser: argparse.ArgumentParser) -> None:
        subparser.add_argument("--golden-set-dir", required=True)
        subparser.add_argument("--output-dir", required=True)

    summary_parser = subparsers.add_parser("summary")
    add_common_paths(summary_parser)

    validate_parser = subparsers.add_parser("validate")
    add_common_paths(validate_parser)

    ready_parser = subparsers.add_parser("ready")
    add_common_paths(ready_parser)

    scaffold_parser = subparsers.add_parser("scaffold")
    add_common_paths(scaffold_parser)
    scaffold_parser.add_argument("--powerpoint-version", required=True)
    scaffold_parser.add_argument("--powerpoint-channel", required=True)
    scaffold_parser.add_argument("--windows-version", required=True)
    scaffold_parser.add_argument("--export-command", required=True)
    scaffold_parser.add_argument("--output-resolution", required=True)
    scaffold_parser.add_argument("--golden-set-revision", required=True)
    scaffold_parser.add_argument("--capture-date", required=True)

    args = parser.parse_args(argv)

    if args.command == "scaffold":
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
        print(json.dumps(summary, indent=2, ensure_ascii=False))
        return 0

    if args.command == "validate":
        summary = validate_powerpoint_golden_batch(args.golden_set_dir, args.output_dir)
        print(json.dumps(summary, indent=2, ensure_ascii=False))
        return 0

    summary = summarize_powerpoint_golden_batch(args.golden_set_dir, args.output_dir)
    print(json.dumps(summary, indent=2, ensure_ascii=False))
    if args.command == "ready":
        return 0 if summary["evidence_ready_for_exact_promotion"] else 1
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
