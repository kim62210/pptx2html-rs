from typing import Optional

class PresentationInfo:
    """Presentation metadata."""
    slide_count: int
    width_px: float
    height_px: float
    title: Optional[str]

class UnresolvedElement:
    """Metadata about an element rendered as a placeholder."""
    slide_index: int
    element_type: str  # "smartart" | "ole" | "math" | "custom-geometry"
    placeholder_id: str
    raw_xml: Optional[str]
    data_model: Optional[str]

class ConversionResult:
    """Result of PPTX conversion with metadata."""
    html: str
    unresolved_elements: list[UnresolvedElement]
    slide_count: int

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
    scale: float = 1.0,
) -> str:
    """Convert a PPTX file to HTML with options.

    Args:
        path: Path to the PPTX file.
        embed_images: Embed images as base64 data URIs (default: True).
        include_hidden: Include hidden slides (default: False).
        slides: List of 1-based slide indices to include (default: all).
        scale: Whole-slide zoom factor (default: 1.0).
    """
    ...

def convert_with_metadata(
    path: str,
    *,
    embed_images: bool = True,
    include_hidden: bool = False,
    slides: Optional[list[int]] = None,
    scale: float = 1.0,
) -> ConversionResult:
    """Convert a PPTX file to HTML with metadata about unresolved elements.

    Args:
        path: Path to the PPTX file.
        embed_images: Embed images as base64 data URIs (default: True).
        include_hidden: Include hidden slides (default: False).
        slides: List of 1-based slide indices to include (default: all).
        scale: Whole-slide zoom factor (default: 1.0).

    Returns:
        ConversionResult with html, unresolved_elements, and slide_count.
    """
    ...

def convert_bytes_with_metadata(
    data: bytes,
    *,
    embed_images: bool = True,
    include_hidden: bool = False,
    slides: Optional[list[int]] = None,
    scale: float = 1.0,
) -> ConversionResult:
    """Convert PPTX bytes to HTML with metadata about unresolved elements.

    Args:
        data: PPTX file bytes.
        embed_images: Embed images as base64 data URIs (default: True).
        include_hidden: Include hidden slides (default: False).
        slides: List of 1-based slide indices to include (default: all).
        scale: Whole-slide zoom factor (default: 1.0).

    Returns:
        ConversionResult with html, unresolved_elements, and slide_count.
    """
    ...

def get_info(path: str) -> PresentationInfo:
    """Get presentation metadata (slide count, size, title)."""
    ...
