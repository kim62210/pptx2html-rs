"""Abstract handler interface for element-type conversion."""

from __future__ import annotations

from abc import ABC, abstractmethod

from pptx2html_enhance.models import UnresolvedElement
from pptx2html_enhance.providers.base import LLMProvider


class Handler(ABC):
    """Base class for element-type-specific conversion handlers."""

    @abstractmethod
    async def process(
        self,
        element: UnresolvedElement,
        provider: LLMProvider,
    ) -> str | None:
        """Convert an unresolved element into an HTML fragment.

        Args:
            element: The unresolved element metadata.
            provider: LLM provider for generating content.

        Returns:
            HTML fragment string to replace the placeholder, or None on failure.
        """
