"""pptx2html-enhance: LLM-powered enhancement layer for pptx2html-rs output.

Public API:
    enhance()      -- Enhance HTML with metadata list
    enhance_html() -- Enhance HTML with metadata JSON string
    Enhancer       -- Full control over the enhancement pipeline
"""

from __future__ import annotations

import json
import logging

from pptx2html_enhance.enhancer import Enhancer
from pptx2html_enhance.models import EnhanceReport, EnhancementResult, UnresolvedElement
from pptx2html_enhance.providers.base import LLMProvider

__all__ = [
    "enhance",
    "enhance_html",
    "Enhancer",
    "EnhanceReport",
    "EnhancementResult",
    "LLMProvider",
    "UnresolvedElement",
]

logger = logging.getLogger(__name__)


def _create_provider(
    provider_name: str,
    api_key: str | None,
    model: str | None,
) -> LLMProvider:
    """Instantiate an LLM provider by name."""
    if provider_name == "anthropic":
        from pptx2html_enhance.providers.anthropic import AnthropicProvider

        kwargs: dict = {}
        if api_key is not None:
            kwargs["api_key"] = api_key
        if model is not None:
            kwargs["model"] = model
        return AnthropicProvider(**kwargs)

    if provider_name == "openai":
        from pptx2html_enhance.providers.openai import OpenAIProvider

        kwargs = {}
        if api_key is not None:
            kwargs["api_key"] = api_key
        if model is not None:
            kwargs["model"] = model
        return OpenAIProvider(**kwargs)

    raise ValueError(
        f"Unknown provider '{provider_name}'. Supported: 'anthropic', 'openai'"
    )


async def enhance(
    html: str,
    unresolved_elements: list[dict] | list[UnresolvedElement],
    *,
    provider: str | LLMProvider = "anthropic",
    api_key: str | None = None,
    model: str | None = None,
    timeout: float = 30.0,
    max_concurrent: int = 5,
) -> str:
    """Enhance pptx2html-rs output by replacing placeholders with LLM-generated content.

    Args:
        html: HTML string from pptx2html-rs.
        unresolved_elements: List of element metadata (dicts or UnresolvedElement).
        provider: Provider name ("anthropic"/"openai") or a LLMProvider instance.
        api_key: Optional API key (falls back to environment variable).
        model: Optional model name override.
        timeout: Timeout per element in seconds.
        max_concurrent: Max concurrent LLM calls.

    Returns:
        Enhanced HTML string with successful replacements applied.
    """
    # Normalize elements
    elements: list[UnresolvedElement] = [
        UnresolvedElement.from_dict(e) if isinstance(e, dict) else e
        for e in unresolved_elements
    ]

    # Create or use provider
    llm: LLMProvider
    owns_provider = False
    if isinstance(provider, str):
        llm = _create_provider(provider, api_key, model)
        owns_provider = True
    else:
        llm = provider

    enhancer = Enhancer(llm, timeout=timeout, max_concurrent=max_concurrent)
    try:
        return await enhancer.enhance(html, elements)
    finally:
        if owns_provider:
            await llm.close()


async def enhance_html(
    html: str,
    metadata_json: str,
    **kwargs,
) -> str:
    """Same as enhance() but accepts metadata as a JSON string.

    The JSON should be an array of objects with at least
    ``placeholder_id`` and ``element_type`` fields.

    Args:
        html: HTML string from pptx2html-rs.
        metadata_json: JSON array string of unresolved element metadata.
        **kwargs: Forwarded to enhance().

    Returns:
        Enhanced HTML string.
    """
    elements = json.loads(metadata_json)
    if not isinstance(elements, list):
        raise ValueError("metadata_json must be a JSON array")
    return await enhance(html, elements, **kwargs)
