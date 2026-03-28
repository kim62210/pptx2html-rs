"""Data models for pptx2html-enhance."""

from __future__ import annotations

from dataclasses import dataclass, field


@dataclass(frozen=True, slots=True)
class UnresolvedElement:
    """Metadata about a PPTX element rendered as a placeholder by pptx2html-rs.

    Attributes:
        placeholder_id: HTML element ID (e.g. "unresolved-s0-e0").
        element_type: One of "smartart", "ole", "math", "custom-geometry".
        slide_index: 0-based slide index.
        raw_xml: Original XML snippet from the PPTX.
        data_model: Optional structured JSON for the element.
        width_px: Bounding box width in pixels (if available).
        height_px: Bounding box height in pixels (if available).
    """

    placeholder_id: str
    element_type: str
    slide_index: int = 0
    raw_xml: str | None = None
    data_model: str | None = None
    width_px: float | None = None
    height_px: float | None = None

    @classmethod
    def from_dict(cls, data: dict) -> UnresolvedElement:
        """Create from a dict (e.g. deserialized JSON metadata)."""
        return cls(
            placeholder_id=data["placeholder_id"],
            element_type=data["element_type"],
            slide_index=data.get("slide_index", 0),
            raw_xml=data.get("raw_xml"),
            data_model=data.get("data_model"),
            width_px=data.get("width_px"),
            height_px=data.get("height_px"),
        )


@dataclass(slots=True)
class EnhancementResult:
    """Result of enhancing a single element.

    Attributes:
        placeholder_id: The placeholder that was processed.
        html_fragment: Generated HTML/CSS content, or None on failure.
        error: Error message if processing failed.
    """

    placeholder_id: str
    html_fragment: str | None = None
    error: str | None = None

    @property
    def success(self) -> bool:
        return self.html_fragment is not None


@dataclass(slots=True)
class EnhanceReport:
    """Summary of an enhance() run.

    Attributes:
        total: Number of elements processed.
        succeeded: Number of successful replacements.
        failed: Number of failures (original placeholder preserved).
        results: Per-element results.
    """

    total: int = 0
    succeeded: int = 0
    failed: int = 0
    results: list[EnhancementResult] = field(default_factory=list)
