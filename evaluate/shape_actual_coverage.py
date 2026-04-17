#!/usr/bin/env python3
"""Benchmark shape crops against reference renders using actual foreground agreement.

Coverage is measured against reference foreground pixels in the rendered images:
- ref_cov12/ref_cov24: share of reference foreground pixels that overlap the
  candidate foreground and match within the RGB threshold.
- cand_cov12/cand_cov24: same agreement pixels measured against candidate
  foreground size.
"""

from __future__ import annotations

import argparse
import json
from pathlib import Path
from typing import Any

import numpy as np
from PIL import Image
from skimage.metrics import structural_similarity as ssim


def composite_white(path: Path) -> Image.Image:
    src = Image.open(path).convert("RGBA")
    bg = Image.new("RGBA", src.size, (255, 255, 255, 255))
    return Image.alpha_composite(bg, src).convert("RGB")


def crop_entry(img: Image.Image, crop: dict[str, float]) -> Image.Image:
    px_per_in_x = img.width / 10.0
    px_per_in_y = img.height / 7.5
    x0 = int(round(crop["left"] * px_per_in_x))
    y0 = int(round(crop["top"] * px_per_in_y))
    x1 = int(round((crop["left"] + crop["width"]) * px_per_in_x))
    y1 = int(round((crop["top"] + crop["height"]) * px_per_in_y))
    return img.crop((x0, y0, x1, y1))


def evaluate_entry(
    entry: dict[str, Any], ref_base: Path, cand_base: Path, fg_threshold: int
) -> dict[str, Any]:
    slide = entry["slide_index"]
    ref_crop = crop_entry(
        composite_white(ref_base / f"slide_{slide}.png"), entry["crop_in"]
    )
    cand_crop = crop_entry(
        composite_white(cand_base / f"slide_{slide}.png"), entry["crop_in"]
    )
    if cand_crop.size != ref_crop.size:
        cand_crop = cand_crop.resize(ref_crop.size, Image.LANCZOS)

    ref_arr = np.array(ref_crop).astype(np.int16)
    cand_arr = np.array(cand_crop).astype(np.int16)
    ref_fg = np.max(255 - ref_arr, axis=2) > fg_threshold
    cand_fg = np.max(255 - cand_arr, axis=2) > fg_threshold
    inter = ref_fg & cand_fg
    union = ref_fg | cand_fg
    rgb_diff = np.max(np.abs(ref_arr - cand_arr), axis=2)
    agree12 = inter & (rgb_diff <= 12)
    agree24 = inter & (rgb_diff <= 24)

    ref_fg_count = int(ref_fg.sum())
    cand_fg_count = int(cand_fg.sum())

    return {
        "shape_name": entry["shape_name"],
        "slide_index": entry["slide_index"],
        "slot_index": entry["slot_index"],
        "ref_cov12": float(agree12.sum() / ref_fg_count * 100.0)
        if ref_fg_count
        else 100.0,
        "ref_cov24": float(agree24.sum() / ref_fg_count * 100.0)
        if ref_fg_count
        else 100.0,
        "cand_cov12": float(agree12.sum() / cand_fg_count * 100.0)
        if cand_fg_count
        else 100.0,
        "cand_cov24": float(agree24.sum() / cand_fg_count * 100.0)
        if cand_fg_count
        else 100.0,
        "fg_ssim": float(
            ssim(
                np.array(ref_crop.convert("L")),
                np.array(cand_crop.convert("L")),
                data_range=255,
            )
        ),
        "mask_iou": float(inter.sum() / union.sum()) if union.any() else 1.0,
    }


def build_summary(rows: list[dict[str, Any]]) -> dict[str, float]:
    return {
        "shape_count": len(rows),
        "ref_cov12_mean": float(np.mean([r["ref_cov12"] for r in rows])),
        "ref_cov24_mean": float(np.mean([r["ref_cov24"] for r in rows])),
        "cand_cov12_mean": float(np.mean([r["cand_cov12"] for r in rows])),
        "cand_cov24_mean": float(np.mean([r["cand_cov24"] for r in rows])),
        "fg_ssim_mean": float(np.mean([r["fg_ssim"] for r in rows])),
        "mask_iou_mean": float(np.mean([r["mask_iou"] for r in rows])),
        "fail_ref_cov12_lt_50": float(
            np.mean([r["ref_cov12"] < 50.0 for r in rows]) * 100.0
        ),
    }


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--manifest", type=Path, required=True)
    parser.add_argument("--reference-dir", type=Path, required=True)
    parser.add_argument("--candidate-dir", type=Path, required=True)
    parser.add_argument("--output", type=Path, required=True)
    parser.add_argument("--fg-threshold", type=int, default=8)
    parser.add_argument("--top-n", type=int, default=20)
    parser.add_argument(
        "--print-shapes",
        nargs="*",
        default=[],
        help="Optional shape names to print after the summary.",
    )
    return parser.parse_args()


def main() -> None:
    args = parse_args()
    manifest = json.loads(args.manifest.read_text())
    entries = manifest["entries"] if isinstance(manifest, dict) else manifest
    rows = [
        evaluate_entry(entry, args.reference_dir, args.candidate_dir, args.fg_threshold)
        for entry in entries
    ]
    rows.sort(key=lambda row: row["ref_cov12"])
    summary = build_summary(rows)
    result = {
        "summary": summary,
        "worst_shapes": rows[: args.top_n],
        "all_shapes": rows,
    }
    args.output.write_text(json.dumps(result, indent=2))
    print(json.dumps(summary, indent=2))
    selected = args.print_shapes or []
    if selected:
        row_map = {row["shape_name"]: row for row in rows}
        for shape_name in selected:
            if shape_name not in row_map:
                print(f"{shape_name}: <missing>")
                continue
            row = row_map[shape_name]
            print(
                shape_name,
                f"ref_cov12={row['ref_cov12']:.2f}",
                f"ref_cov24={row['ref_cov24']:.2f}",
                f"cand_cov12={row['cand_cov12']:.2f}",
                f"ssim={row['fg_ssim']:.4f}",
                f"iou={row['mask_iou']:.4f}",
            )


if __name__ == "__main__":
    main()
