"""DrawingML visual effects to CSS conversion handler.

Mostly rule-based; LLM fallback only for complex compound effects.
"""

from __future__ import annotations

import json
import logging
import re

from pptx2html_enhance.handlers.base import Handler
from pptx2html_enhance.models import UnresolvedElement
from pptx2html_enhance.providers.base import LLMProvider
from pptx2html_enhance.utils.html_patch import extract_html_output
from pptx2html_enhance.utils.prompts import EFFECTS_SYSTEM, EFFECTS_USER

logger = logging.getLogger(__name__)

# EMU to pixel conversion (914400 EMU = 1 inch = 96px)
_EMU_PER_PX = 914400 / 96


class EffectsHandler(Handler):
    """Convert DrawingML effect properties into CSS style strings."""

    async def process(
        self,
        element: UnresolvedElement,
        provider: LLMProvider,
    ) -> str | None:
        if not element.raw_xml:
            logger.warning(
                "Effects element '%s' has no raw_xml, skipping",
                element.placeholder_id,
            )
            return None

        # Stage 1: rule-based conversion
        css_props = self._extract_css(element.raw_xml)
        if css_props:
            style = "; ".join(f"{k}: {v}" for k, v in css_props.items())
            return f'<div style="{style}"></div>'

        # Stage 2: LLM fallback for complex effects
        return await self._llm_convert(element, provider)

    def _extract_css(self, raw_xml: str) -> dict[str, str]:
        """Rule-based extraction of common DrawingML effects to CSS properties."""
        css: dict[str, str] = {}

        # outerShdw -> box-shadow
        shadow = self._parse_outer_shadow(raw_xml)
        if shadow:
            css["box-shadow"] = shadow

        # glow -> box-shadow with spread
        glow = self._parse_glow(raw_xml)
        if glow:
            existing = css.get("box-shadow", "")
            if existing:
                css["box-shadow"] = f"{existing}, {glow}"
            else:
                css["box-shadow"] = glow

        # softEdge -> filter: blur()
        soft_edge = self._parse_soft_edge(raw_xml)
        if soft_edge:
            css["filter"] = soft_edge

        return css

    @staticmethod
    def _parse_outer_shadow(xml: str) -> str | None:
        """Parse <a:outerShdw> into a CSS box-shadow value."""
        match = re.search(r"<a:outerShdw\b([^>]*)>", xml)
        if match is None:
            return None

        attrs = match.group(1)

        dist_match = re.search(r'dist="(\d+)"', attrs)
        dir_match = re.search(r'dir="(\d+)"', attrs)
        blur_match = re.search(r'blurRad="(\d+)"', attrs)

        dist_emu = int(dist_match.group(1)) if dist_match else 0
        direction = int(dir_match.group(1)) if dir_match else 0
        blur_emu = int(blur_match.group(1)) if blur_match else 0

        dist_px = dist_emu / _EMU_PER_PX
        blur_px = blur_emu / _EMU_PER_PX

        # Direction is in 60000ths of a degree, convert to radians
        import math

        angle_rad = math.radians(direction / 60000)
        x_offset = dist_px * math.cos(angle_rad)
        y_offset = dist_px * math.sin(angle_rad)

        # Try to find color
        color = "rgba(0, 0, 0, 0.35)"
        color_match = re.search(r'<a:srgbClr val="([0-9A-Fa-f]{6})"', xml)
        if color_match:
            hex_color = color_match.group(1)
            r, g, b = int(hex_color[:2], 16), int(hex_color[2:4], 16), int(hex_color[4:6], 16)
            alpha_match = re.search(r'<a:alpha val="(\d+)"', xml)
            alpha = int(alpha_match.group(1)) / 100000 if alpha_match else 0.35
            color = f"rgba({r}, {g}, {b}, {alpha:.2f})"

        return f"{x_offset:.1f}px {y_offset:.1f}px {blur_px:.1f}px {color}"

    @staticmethod
    def _parse_glow(xml: str) -> str | None:
        """Parse <a:glow> into a CSS box-shadow with spread."""
        match = re.search(r"<a:glow\b([^>]*)>", xml)
        if match is None:
            return None

        attrs = match.group(1)
        rad_match = re.search(r'rad="(\d+)"', attrs)
        rad_px = int(rad_match.group(1)) / _EMU_PER_PX if rad_match else 5

        color = "rgba(255, 215, 0, 0.6)"
        color_match = re.search(
            r"<a:glow[^>]*>.*?<a:srgbClr val=\"([0-9A-Fa-f]{6})\"",
            xml,
            re.DOTALL,
        )
        if color_match:
            hex_color = color_match.group(1)
            r, g, b = int(hex_color[:2], 16), int(hex_color[2:4], 16), int(hex_color[4:6], 16)
            color = f"rgba({r}, {g}, {b}, 0.6)"

        return f"0 0 {rad_px:.1f}px {rad_px / 2:.1f}px {color}"

    @staticmethod
    def _parse_soft_edge(xml: str) -> str | None:
        """Parse <a:softEdge> into CSS filter: blur()."""
        match = re.search(r'<a:softEdge\b[^>]*rad="(\d+)"', xml)
        if match is None:
            return None
        rad_px = int(match.group(1)) / _EMU_PER_PX
        return f"blur({rad_px:.1f}px)"

    async def _llm_convert(
        self,
        element: UnresolvedElement,
        provider: LLMProvider,
    ) -> str | None:
        """Use LLM to convert complex compound effects."""
        prompt = EFFECTS_USER.format(raw_xml=element.raw_xml)

        try:
            response = await provider.generate(
                EFFECTS_SYSTEM,
                prompt,
                max_tokens=1024,
            )
        except RuntimeError:
            logger.exception(
                "LLM call failed for Effects element '%s'",
                element.placeholder_id,
            )
            return None

        raw = extract_html_output(response)
        if raw is None:
            logger.warning(
                "Could not extract <html-output> from LLM response for '%s'",
                element.placeholder_id,
            )
            return None

        # Try to parse as JSON CSS properties
        try:
            css_dict = json.loads(raw)
            if isinstance(css_dict, dict):
                style = "; ".join(f"{k}: {v}" for k, v in css_dict.items())
                return f'<div style="{style}"></div>'
        except (json.JSONDecodeError, TypeError):
            pass

        # Fall back to using the raw output as HTML
        return raw
