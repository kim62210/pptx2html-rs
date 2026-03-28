#!/usr/bin/env python3
"""
Render golden PPTX files to reference PNG images via LibreOffice headless.

Pipeline: PPTX -> LibreOffice headless -> PDF -> pdf2image -> slide PNGs

Output structure:
    golden_references/{pptx_stem}/slide_0.png
    golden_references/{pptx_stem}/slide_1.png
    ...
"""

from __future__ import annotations

import argparse
import logging
import shutil
import subprocess
import tempfile
from pathlib import Path

from pdf2image import convert_from_path
from PIL import Image

logger = logging.getLogger(__name__)

# Default DPI for PDF -> PNG conversion
DEFAULT_DPI = 150


def find_libreoffice() -> Path | None:
    """Locate the LibreOffice binary on the system."""
    candidates = [
        Path("/Applications/LibreOffice.app/Contents/MacOS/soffice"),
        Path("/usr/bin/libreoffice"),
        Path("/usr/bin/soffice"),
        Path("/usr/local/bin/libreoffice"),
        Path("/snap/bin/libreoffice"),
    ]

    for candidate in candidates:
        if candidate.exists():
            return candidate

    # Try PATH lookup
    result = shutil.which("libreoffice") or shutil.which("soffice")
    if result:
        return Path(result)

    return None


def pptx_to_pdf(
    libreoffice_bin: Path, pptx_path: Path, output_dir: Path
) -> Path | None:
    """Convert a PPTX file to PDF using LibreOffice headless.

    Returns the path to the generated PDF, or None on failure.
    """
    try:
        result = subprocess.run(
            [
                str(libreoffice_bin),
                "--headless",
                "--convert-to",
                "pdf",
                "--outdir",
                str(output_dir),
                str(pptx_path),
            ],
            capture_output=True,
            text=True,
            timeout=120,
        )
    except subprocess.TimeoutExpired:
        logger.error("LibreOffice timed out converting %s", pptx_path.name)
        return None

    if result.returncode != 0:
        logger.error(
            "LibreOffice failed for %s: %s",
            pptx_path.name,
            result.stderr[:300],
        )
        return None

    pdf_path = output_dir / f"{pptx_path.stem}.pdf"
    if not pdf_path.exists():
        logger.error("Expected PDF not found at %s", pdf_path)
        return None

    return pdf_path


def pdf_to_pngs(
    pdf_path: Path, output_dir: Path, dpi: int = DEFAULT_DPI
) -> list[Path]:
    """Convert a PDF to per-page PNG images using pdf2image (poppler).

    Returns list of generated PNG paths.
    """
    output_dir.mkdir(parents=True, exist_ok=True)

    try:
        images: list[Image.Image] = convert_from_path(
            str(pdf_path), dpi=dpi
        )
    except Exception as exc:
        logger.error("pdf2image failed for %s: %s", pdf_path.name, exc)
        return []

    png_paths: list[Path] = []
    for idx, img in enumerate(images):
        png_path = output_dir / f"slide_{idx}.png"
        img.save(str(png_path), "PNG")
        png_paths.append(png_path)
        logger.debug("  Saved %s", png_path.name)

    return png_paths


def render_golden_set(
    input_dir: Path,
    output_dir: Path,
    dpi: int = DEFAULT_DPI,
    force: bool = False,
) -> dict[str, list[Path]]:
    """Render all PPTX files in input_dir to reference PNGs.

    Args:
        input_dir: Directory containing golden PPTX files.
        output_dir: Base directory for reference PNG output.
        dpi: Resolution for PDF -> PNG conversion.
        force: Re-render even if output already exists.

    Returns:
        Dict mapping pptx_stem -> list of PNG paths.
    """
    lo_bin = find_libreoffice()
    if lo_bin is None:
        logger.error(
            "LibreOffice not found. Install it and ensure it is in PATH."
        )
        return {}

    logger.info("Using LibreOffice at %s", lo_bin)

    pptx_files = sorted(input_dir.glob("*.pptx"))
    if not pptx_files:
        logger.error("No PPTX files found in %s", input_dir)
        return {}

    logger.info("Rendering %d PPTX files to reference PNGs...", len(pptx_files))
    output_dir.mkdir(parents=True, exist_ok=True)

    results: dict[str, list[Path]] = {}

    for pptx_file in pptx_files:
        ref_dir = output_dir / pptx_file.stem

        if ref_dir.exists() and not force:
            existing = sorted(ref_dir.glob("slide_*.png"))
            if existing:
                logger.info(
                    "  Skipping %s (%d slides already rendered)",
                    pptx_file.name,
                    len(existing),
                )
                results[pptx_file.stem] = existing
                continue

        logger.info("  Rendering %s...", pptx_file.name)

        with tempfile.TemporaryDirectory(prefix="refrender_") as tmp_dir:
            tmp_path = Path(tmp_dir)

            # PPTX -> PDF
            pdf_path = pptx_to_pdf(lo_bin, pptx_file, tmp_path)
            if pdf_path is None:
                continue

            # PDF -> PNGs
            ref_dir.mkdir(parents=True, exist_ok=True)
            pngs = pdf_to_pngs(pdf_path, ref_dir, dpi=dpi)

            if pngs:
                results[pptx_file.stem] = pngs
                logger.info(
                    "  -> %d slide(s) rendered for %s",
                    len(pngs),
                    pptx_file.name,
                )

    logger.info("Done. Rendered %d PPTX files.", len(results))
    return results


def main() -> None:
    """CLI entry point."""
    parser = argparse.ArgumentParser(
        description="Render golden PPTX files to reference PNGs via LibreOffice"
    )
    parser.add_argument(
        "--input",
        type=Path,
        required=True,
        help="Directory containing golden PPTX files",
    )
    parser.add_argument(
        "--output",
        type=Path,
        required=True,
        help="Output directory for reference PNGs",
    )
    parser.add_argument(
        "--dpi",
        type=int,
        default=DEFAULT_DPI,
        help=f"DPI for PDF -> PNG conversion (default: {DEFAULT_DPI})",
    )
    parser.add_argument(
        "--force",
        action="store_true",
        help="Re-render even if output already exists",
    )
    parser.add_argument(
        "--verbose",
        action="store_true",
        help="Enable debug logging",
    )

    args = parser.parse_args()

    logging.basicConfig(
        level=logging.DEBUG if args.verbose else logging.INFO,
        format="%(levelname)s: %(message)s",
    )

    render_golden_set(
        input_dir=args.input.resolve(),
        output_dir=args.output.resolve(),
        dpi=args.dpi,
        force=args.force,
    )


if __name__ == "__main__":
    main()
