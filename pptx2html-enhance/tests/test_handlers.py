"""Tests for element type handlers."""

from __future__ import annotations

import pytest

from pptx2html_enhance.handlers.effects import EffectsHandler
from pptx2html_enhance.handlers.math_handler import MathHandler
from pptx2html_enhance.handlers.smartart import SmartArtHandler
from pptx2html_enhance.models import UnresolvedElement

from .conftest import FailingLLMProvider, MockLLMProvider


class TestSmartArtHandler:
    """Tests for SmartArtHandler."""

    async def test_generates_prompt_with_dimensions(
        self, smartart_element: UnresolvedElement, mock_provider: MockLLMProvider,
    ) -> None:
        handler = SmartArtHandler()
        await handler.process(smartart_element, mock_provider)

        assert mock_provider.call_count == 1
        assert "400" in mock_provider.last_user_prompt
        assert "300" in mock_provider.last_user_prompt

    async def test_returns_none_when_no_data(
        self, mock_provider: MockLLMProvider,
    ) -> None:
        element = UnresolvedElement(
            placeholder_id="test-empty",
            element_type="smartart",
            slide_index=0,
        )
        handler = SmartArtHandler()
        result = await handler.process(element, mock_provider)

        assert result is None
        assert mock_provider.call_count == 0

    async def test_returns_none_on_llm_failure(
        self, smartart_element: UnresolvedElement, failing_provider: FailingLLMProvider,
    ) -> None:
        handler = SmartArtHandler()
        result = await handler.process(smartart_element, failing_provider)
        assert result is None

    async def test_detects_hierarchy_layout(self) -> None:
        element = UnresolvedElement(
            placeholder_id="test",
            element_type="smartart",
            raw_xml="<dgm:data><hierChild/></dgm:data>",
        )
        layout = SmartArtHandler._detect_layout_type(element)
        assert "hierarchy" in layout


class TestMathHandler:
    """Tests for MathHandler."""

    async def test_rule_based_fraction(
        self, mock_provider: MockLLMProvider,
    ) -> None:
        """Simple fraction should be converted without LLM call."""
        element = UnresolvedElement(
            placeholder_id="math-frac",
            element_type="math",
            raw_xml='<m:f><m:num><m:r><m:t>x</m:t></m:r></m:num><m:den><m:r><m:t>2</m:t></m:r></m:den></m:f>',
        )
        handler = MathHandler()
        result = await handler.process(element, mock_provider)

        assert result is not None
        assert "<mfrac>" in result
        assert "<mi>x</mi>" in result
        assert "<mn>2</mn>" in result
        # Should NOT have called the LLM
        assert mock_provider.call_count == 0

    async def test_rule_based_superscript(
        self, mock_provider: MockLLMProvider,
    ) -> None:
        element = UnresolvedElement(
            placeholder_id="math-sup",
            element_type="math",
            raw_xml='<m:sSup><m:e><m:r><m:t>x</m:t></m:r></m:e><m:sup><m:r><m:t>2</m:t></m:r></m:sup></m:sSup>',
        )
        handler = MathHandler()
        result = await handler.process(element, mock_provider)

        assert result is not None
        assert "<msup>" in result
        assert mock_provider.call_count == 0

    async def test_rule_based_subscript(
        self, mock_provider: MockLLMProvider,
    ) -> None:
        element = UnresolvedElement(
            placeholder_id="math-sub",
            element_type="math",
            raw_xml='<m:sSub><m:e><m:r><m:t>a</m:t></m:r></m:e><m:sub><m:r><m:t>i</m:t></m:r></m:sub></m:sSub>',
        )
        handler = MathHandler()
        result = await handler.process(element, mock_provider)

        assert result is not None
        assert "<msub>" in result
        assert mock_provider.call_count == 0

    async def test_rule_based_sqrt(
        self, mock_provider: MockLLMProvider,
    ) -> None:
        element = UnresolvedElement(
            placeholder_id="math-sqrt",
            element_type="math",
            raw_xml='<m:rad><m:radPr><m:degHide m:val="1"/></m:radPr><m:deg/><m:e><m:r><m:t>x</m:t></m:r></m:e></m:rad>',
        )
        handler = MathHandler()
        result = await handler.process(element, mock_provider)

        assert result is not None
        assert "<msqrt>" in result
        assert mock_provider.call_count == 0

    async def test_llm_fallback_for_complex_formula(
        self, mock_provider: MockLLMProvider,
    ) -> None:
        """Complex XML that rule-based cannot handle should fall back to LLM."""
        element = UnresolvedElement(
            placeholder_id="math-complex",
            element_type="math",
            raw_xml="<m:oMathPara><m:nary><m:naryPr/></m:nary></m:oMathPara>",
        )
        handler = MathHandler()
        result = await handler.process(element, mock_provider)

        # LLM was called because rule-based returned nothing
        assert mock_provider.call_count == 1

    async def test_returns_none_when_no_xml(
        self, mock_provider: MockLLMProvider,
    ) -> None:
        element = UnresolvedElement(
            placeholder_id="math-empty",
            element_type="math",
        )
        handler = MathHandler()
        result = await handler.process(element, mock_provider)
        assert result is None


class TestEffectsHandler:
    """Tests for EffectsHandler."""

    async def test_outer_shadow_to_box_shadow(
        self, effects_element: UnresolvedElement, mock_provider: MockLLMProvider,
    ) -> None:
        handler = EffectsHandler()
        result = await handler.process(effects_element, mock_provider)

        assert result is not None
        assert "box-shadow" in result
        # Rule-based, no LLM call needed
        assert mock_provider.call_count == 0

    async def test_glow_effect(
        self, mock_provider: MockLLMProvider,
    ) -> None:
        element = UnresolvedElement(
            placeholder_id="fx-glow",
            element_type="custom-geometry",
            raw_xml='<a:effectLst><a:glow rad="63500"><a:srgbClr val="FF0000"/></a:glow></a:effectLst>',
        )
        handler = EffectsHandler()
        result = await handler.process(element, mock_provider)

        assert result is not None
        assert "box-shadow" in result
        assert mock_provider.call_count == 0

    async def test_soft_edge_to_blur(
        self, mock_provider: MockLLMProvider,
    ) -> None:
        element = UnresolvedElement(
            placeholder_id="fx-soft",
            element_type="custom-geometry",
            raw_xml='<a:effectLst><a:softEdge rad="25400"/></a:effectLst>',
        )
        handler = EffectsHandler()
        result = await handler.process(element, mock_provider)

        assert result is not None
        assert "blur(" in result
        assert mock_provider.call_count == 0

    async def test_returns_none_when_no_xml(
        self, mock_provider: MockLLMProvider,
    ) -> None:
        element = UnresolvedElement(
            placeholder_id="fx-empty",
            element_type="custom-geometry",
        )
        handler = EffectsHandler()
        result = await handler.process(element, mock_provider)
        assert result is None
