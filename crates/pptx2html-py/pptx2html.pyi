from typing import Optional

class PresentationInfo:
    """Presentation metadata."""
    slide_count: int
    width_px: float
    height_px: float
    title: Optional[str]

def convert_file(path: str) -> str:
    """Convert a PPTX file to an HTML string."""
    ...

def convert_bytes(data: bytes) -> str:
    """Convert PPTX bytes to an HTML string."""
    ...

def convert(
    path: str,
    *,
    embed_images: bool = True,
    include_hidden: bool = False,
    slides: Optional[list[int]] = None,
) -> str:
    """Convert a PPTX file to HTML with options.

    Args:
        path: Path to the PPTX file.
        embed_images: Embed images as base64 data URIs (default: True).
        include_hidden: Include hidden slides (default: False).
        slides: List of 1-based slide indices to include (default: all).
    """
    ...

def get_info(path: str) -> PresentationInfo:
    """Get presentation metadata (slide count, size, title)."""
    ...
