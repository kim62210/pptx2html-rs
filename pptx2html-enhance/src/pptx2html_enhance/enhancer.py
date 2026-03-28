"""Main orchestrator for LLM-powered HTML enhancement."""

from __future__ import annotations

import asyncio
import logging

from pptx2html_enhance.handlers.base import Handler
from pptx2html_enhance.handlers.effects import EffectsHandler
from pptx2html_enhance.handlers.math_handler import MathHandler
from pptx2html_enhance.handlers.smartart import SmartArtHandler
from pptx2html_enhance.models import EnhanceReport, EnhancementResult, UnresolvedElement
from pptx2html_enhance.providers.base import LLMProvider
from pptx2html_enhance.utils.html_patch import patch_html

logger = logging.getLogger(__name__)

# Default concurrency limit for LLM calls
_DEFAULT_MAX_CONCURRENT = 5

# Mapping from element_type strings to handler instances
_HANDLER_MAP: dict[str, Handler] = {
    "smartart": SmartArtHandler(),
    "math": MathHandler(),
    "custom-geometry": EffectsHandler(),
}


class Enhancer:
    """Orchestrate LLM-powered replacement of placeholder elements in HTML.

    Groups unresolved elements by type, dispatches to appropriate handlers,
    processes them concurrently (with rate-limit semaphore), and patches the
    original HTML.
    """

    def __init__(
        self,
        provider: LLMProvider,
        *,
        timeout: float = 30.0,
        max_concurrent: int = _DEFAULT_MAX_CONCURRENT,
    ) -> None:
        self._provider = provider
        self._timeout = timeout
        self._semaphore = asyncio.Semaphore(max_concurrent)
        self._handlers: dict[str, Handler] = dict(_HANDLER_MAP)

    def register_handler(self, element_type: str, handler: Handler) -> None:
        """Register a custom handler for an element type."""
        self._handlers[element_type] = handler

    async def enhance(
        self,
        html: str,
        unresolved_elements: list[UnresolvedElement],
    ) -> str:
        """Process all unresolved elements and patch the HTML.

        Elements are processed concurrently. If an LLM call fails or times out,
        the original placeholder is preserved (graceful degradation).

        Args:
            html: Original HTML from pptx2html-rs.
            unresolved_elements: Metadata from ConversionResult.

        Returns:
            Enhanced HTML with successful replacements applied.
        """
        report = await self.enhance_with_report(html, unresolved_elements)
        return report.html

    async def enhance_with_report(
        self,
        html: str,
        unresolved_elements: list[UnresolvedElement],
    ) -> _EnhanceOutput:
        """Like enhance() but also returns a detailed report."""
        if not unresolved_elements:
            return _EnhanceOutput(
                html=html,
                report=EnhanceReport(total=0),
            )

        tasks = [
            self._process_element(elem)
            for elem in unresolved_elements
        ]
        results: list[EnhancementResult] = await asyncio.gather(*tasks)

        # Build replacement map (only successful results)
        replacements: dict[str, str] = {}
        succeeded = 0
        failed = 0
        for result in results:
            if result.success:
                replacements[result.placeholder_id] = result.html_fragment  # type: ignore[arg-type]
                succeeded += 1
            else:
                failed += 1

        patched_html = patch_html(html, replacements)

        report = EnhanceReport(
            total=len(unresolved_elements),
            succeeded=succeeded,
            failed=failed,
            results=results,
        )

        logger.info(
            "Enhancement complete: %d/%d elements replaced",
            succeeded,
            report.total,
        )

        return _EnhanceOutput(html=patched_html, report=report)

    async def _process_element(
        self,
        element: UnresolvedElement,
    ) -> EnhancementResult:
        """Process a single element with timeout and concurrency control."""
        handler = self._handlers.get(element.element_type)
        if handler is None:
            logger.debug(
                "No handler for element type '%s', skipping '%s'",
                element.element_type,
                element.placeholder_id,
            )
            return EnhancementResult(
                placeholder_id=element.placeholder_id,
                error=f"No handler for element type '{element.element_type}'",
            )

        async with self._semaphore:
            try:
                fragment = await asyncio.wait_for(
                    handler.process(element, self._provider),
                    timeout=self._timeout,
                )
            except TimeoutError:
                logger.warning(
                    "Timeout processing element '%s' (%.1fs)",
                    element.placeholder_id,
                    self._timeout,
                )
                return EnhancementResult(
                    placeholder_id=element.placeholder_id,
                    error=f"Timeout after {self._timeout}s",
                )
            except Exception as exc:
                logger.exception(
                    "Unexpected error processing element '%s'",
                    element.placeholder_id,
                )
                return EnhancementResult(
                    placeholder_id=element.placeholder_id,
                    error=str(exc),
                )

        if fragment is None:
            return EnhancementResult(
                placeholder_id=element.placeholder_id,
                error="Handler returned None",
            )

        return EnhancementResult(
            placeholder_id=element.placeholder_id,
            html_fragment=fragment,
        )


class _EnhanceOutput:
    """Internal container for enhance_with_report() return value."""

    __slots__ = ("html", "report")

    def __init__(self, html: str, report: EnhanceReport) -> None:
        self.html = html
        self.report = report
