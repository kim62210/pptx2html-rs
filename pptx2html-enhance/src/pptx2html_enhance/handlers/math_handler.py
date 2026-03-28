"""OMML (Office Math) to MathML/KaTeX conversion handler.

Two-stage approach:
  1. Rule-based OMML -> MathML for common patterns (fractions, scripts, roots).
  2. LLM fallback for complex formulas.
"""

from __future__ import annotations

import logging
import re

from pptx2html_enhance.handlers.base import Handler
from pptx2html_enhance.models import UnresolvedElement
from pptx2html_enhance.providers.base import LLMProvider
from pptx2html_enhance.utils.html_patch import extract_html_output
from pptx2html_enhance.utils.prompts import MATH_SYSTEM, MATH_USER

logger = logging.getLogger(__name__)


class MathHandler(Handler):
    """Convert OMML math equations to browser-renderable MathML."""

    async def process(
        self,
        element: UnresolvedElement,
        provider: LLMProvider,
    ) -> str | None:
        if not element.raw_xml:
            logger.warning(
                "Math element '%s' has no raw_xml, skipping",
                element.placeholder_id,
            )
            return None

        # Stage 1: rule-based conversion for common patterns
        mathml = self._omml_to_mathml(element.raw_xml)
        if mathml is not None:
            logger.debug(
                "Rule-based conversion succeeded for '%s'",
                element.placeholder_id,
            )
            return self._wrap_math(mathml)

        # Stage 2: LLM fallback
        return await self._llm_convert(element, provider)

    def _omml_to_mathml(self, omml_xml: str) -> str | None:
        """Rule-based OMML to MathML conversion for common patterns.

        Handles: fractions (m:f), superscripts (m:sSup), subscripts (m:sSub),
        square roots (m:rad), runs (m:r).

        Returns MathML inner content or None if too complex for rule-based.
        """
        parts: list[str] = []

        # Simple fraction: <m:f> ... <m:num>...</m:num> <m:den>...</m:den> </m:f>
        fraction_pattern = re.compile(
            r"<m:f\b[^>]*>.*?<m:num>(.*?)</m:num>.*?<m:den>(.*?)</m:den>.*?</m:f>",
            re.DOTALL,
        )
        for match in fraction_pattern.finditer(omml_xml):
            num_text = self._extract_text_runs(match.group(1))
            den_text = self._extract_text_runs(match.group(2))
            parts.append(f"<mfrac><mrow>{num_text}</mrow><mrow>{den_text}</mrow></mfrac>")

        # Superscript: <m:sSup> <m:e>base</m:e> <m:sup>exp</m:sup> </m:sSup>
        sup_pattern = re.compile(
            r"<m:sSup\b[^>]*>.*?<m:e>(.*?)</m:e>.*?<m:sup>(.*?)</m:sup>.*?</m:sSup>",
            re.DOTALL,
        )
        for match in sup_pattern.finditer(omml_xml):
            base_text = self._extract_text_runs(match.group(1))
            sup_text = self._extract_text_runs(match.group(2))
            parts.append(f"<msup><mrow>{base_text}</mrow><mrow>{sup_text}</mrow></msup>")

        # Subscript: <m:sSub> <m:e>base</m:e> <m:sub>sub</m:sub> </m:sSub>
        sub_pattern = re.compile(
            r"<m:sSub\b[^>]*>.*?<m:e>(.*?)</m:e>.*?<m:sub>(.*?)</m:sub>.*?</m:sSub>",
            re.DOTALL,
        )
        for match in sub_pattern.finditer(omml_xml):
            base_text = self._extract_text_runs(match.group(1))
            sub_text = self._extract_text_runs(match.group(2))
            parts.append(f"<msub><mrow>{base_text}</mrow><mrow>{sub_text}</mrow></msub>")

        # Square root: <m:rad> ... <m:e>content</m:e> </m:rad>
        rad_pattern = re.compile(
            r"<m:rad\b[^>]*>.*?<m:e>(.*?)</m:e>.*?</m:rad>",
            re.DOTALL,
        )
        for match in rad_pattern.finditer(omml_xml):
            content_text = self._extract_text_runs(match.group(1))
            parts.append(f"<msqrt><mrow>{content_text}</mrow></msqrt>")

        if not parts:
            # Try to extract plain text runs as a simple equation
            text_runs = self._extract_text_runs(omml_xml)
            if text_runs and "<mi>" in text_runs:
                parts.append(text_runs)

        if not parts:
            return None

        return "".join(parts)

    @staticmethod
    def _extract_text_runs(xml_fragment: str) -> str:
        """Extract <m:t> text runs from OMML and wrap in MathML <mi>/<mn>/<mo>."""
        run_pattern = re.compile(r"<m:t\b[^>]*>(.*?)</m:t>", re.DOTALL)
        result_parts: list[str] = []
        for match in run_pattern.finditer(xml_fragment):
            text = match.group(1).strip()
            if not text:
                continue
            if text.isdigit():
                result_parts.append(f"<mn>{text}</mn>")
            elif len(text) == 1 and not text.isalnum():
                result_parts.append(f"<mo>{text}</mo>")
            else:
                result_parts.append(f"<mi>{text}</mi>")
        return "".join(result_parts)

    @staticmethod
    def _wrap_math(mathml_inner: str) -> str:
        """Wrap MathML content in a display math element."""
        return f'<math xmlns="http://www.w3.org/1998/Math/MathML" display="block">{mathml_inner}</math>'

    async def _llm_convert(
        self,
        element: UnresolvedElement,
        provider: LLMProvider,
    ) -> str | None:
        """Use LLM to convert complex OMML to MathML."""
        prompt = MATH_USER.format(raw_xml=element.raw_xml)

        try:
            response = await provider.generate(
                MATH_SYSTEM,
                prompt,
                max_tokens=2048,
            )
        except RuntimeError:
            logger.exception(
                "LLM call failed for Math element '%s'",
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
