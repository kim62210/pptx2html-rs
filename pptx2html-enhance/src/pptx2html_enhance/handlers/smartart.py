"""SmartArt diagram to HTML/CSS conversion handler."""

from __future__ import annotations

import logging

from pptx2html_enhance.handlers.base import Handler
from pptx2html_enhance.models import UnresolvedElement
from pptx2html_enhance.providers.base import LLMProvider
from pptx2html_enhance.utils.html_patch import extract_html_output
from pptx2html_enhance.utils.prompts import SMARTART_SYSTEM, SMARTART_USER

logger = logging.getLogger(__name__)

# Default bounding box when size is not provided
_DEFAULT_WIDTH = 600
_DEFAULT_HEIGHT = 400


class SmartArtHandler(Handler):
    """Convert SmartArt XML/data-model into an HTML/CSS diagram via LLM."""

    async def process(
        self,
        element: UnresolvedElement,
        provider: LLMProvider,
    ) -> str | None:
        if not element.raw_xml and not element.data_model:
            logger.warning(
                "SmartArt element '%s' has no raw_xml or data_model, skipping",
                element.placeholder_id,
            )
            return None

        width = element.width_px or _DEFAULT_WIDTH
        height = element.height_px or _DEFAULT_HEIGHT

        prompt = SMARTART_USER.format(
            width=int(width),
            height=int(height),
            theme_colors="(not available)",
            layout_type=self._detect_layout_type(element),
            data_model=element.data_model or "(not available)",
            raw_xml=element.raw_xml or "(not available)",
        )

        try:
            response = await provider.generate(
                SMARTART_SYSTEM,
                prompt,
                max_tokens=4096,
            )
        except RuntimeError:
            logger.exception(
                "LLM call failed for SmartArt element '%s'",
                element.placeholder_id,
            )
            return None

        html_fragment = extract_html_output(response)
        if html_fragment is None:
            logger.warning(
                "Could not extract <html-output> from LLM response for '%s'",
                element.placeholder_id,
            )
            return None

        return html_fragment

    @staticmethod
    def _detect_layout_type(element: UnresolvedElement) -> str:
        """Try to infer the SmartArt layout type from raw XML hints."""
        xml = element.raw_xml or ""
        if "dgm:relIds" in xml:
            return "relationship diagram"
        if "hierChild" in xml or "hierRoot" in xml:
            return "hierarchy diagram"
        if "cycle" in xml.lower():
            return "cycle diagram"
        return "generic diagram"
