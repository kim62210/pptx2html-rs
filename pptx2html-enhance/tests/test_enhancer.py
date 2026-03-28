"""Tests for the main Enhancer orchestrator."""

from __future__ import annotations

import pytest

from pptx2html_enhance.enhancer import Enhancer
from pptx2html_enhance.models import UnresolvedElement

from .conftest import FailingLLMProvider, MockLLMProvider, SlowLLMProvider


class TestEnhancer:
    """Tests for the Enhancer class."""

    async def test_full_pipeline_replaces_placeholder(
        self,
        sample_html: str,
        smartart_element: UnresolvedElement,
        mock_provider: MockLLMProvider,
    ) -> None:
        """End-to-end: enhance() should replace a placeholder in the HTML."""
        enhancer = Enhancer(mock_provider)
        result = await enhancer.enhance(sample_html, [smartart_element])

        assert "mock content" in result
        assert "[SmartArt]" not in result

    async def test_graceful_degradation_on_llm_failure(
        self,
        sample_html: str,
        smartart_element: UnresolvedElement,
        failing_provider: FailingLLMProvider,
    ) -> None:
        """When LLM fails, original placeholder should be preserved."""
        enhancer = Enhancer(failing_provider)
        result = await enhancer.enhance(sample_html, [smartart_element])

        # Original placeholder should still be there
        assert "[SmartArt]" in result

    async def test_timeout_preserves_placeholder(
        self,
        sample_html: str,
        smartart_element: UnresolvedElement,
        slow_provider: SlowLLMProvider,
    ) -> None:
        """Elements that exceed timeout should be preserved."""
        enhancer = Enhancer(slow_provider, timeout=0.05)
        result = await enhancer.enhance(sample_html, [smartart_element])

        assert "[SmartArt]" in result

    async def test_concurrent_processing(
        self,
        sample_html: str,
        smartart_element: UnresolvedElement,
        math_element: UnresolvedElement,
    ) -> None:
        """Multiple elements should be processed concurrently."""
        provider = MockLLMProvider(responses={
            "SmartArt": "<html-output><div>Diagram</div></html-output>",
        })
        enhancer = Enhancer(provider, max_concurrent=2)

        elements = [smartart_element, math_element]
        result = await enhancer.enhance(sample_html, elements)

        # SmartArt matched custom response, math used default
        assert "Diagram" in result

    async def test_report_counts(
        self,
        sample_html: str,
        smartart_element: UnresolvedElement,
        mock_provider: MockLLMProvider,
    ) -> None:
        """enhance_with_report() should return accurate counts."""
        enhancer = Enhancer(mock_provider)
        output = await enhancer.enhance_with_report(
            sample_html, [smartart_element],
        )

        assert output.report.total == 1
        assert output.report.succeeded == 1
        assert output.report.failed == 0

    async def test_report_counts_with_failure(
        self,
        sample_html: str,
        smartart_element: UnresolvedElement,
        failing_provider: FailingLLMProvider,
    ) -> None:
        enhancer = Enhancer(failing_provider)
        output = await enhancer.enhance_with_report(
            sample_html, [smartart_element],
        )

        assert output.report.total == 1
        assert output.report.succeeded == 0
        assert output.report.failed == 1

    async def test_no_elements_returns_original(
        self,
        sample_html: str,
        mock_provider: MockLLMProvider,
    ) -> None:
        """Empty element list should return HTML unchanged."""
        enhancer = Enhancer(mock_provider)
        result = await enhancer.enhance(sample_html, [])

        assert result == sample_html
        assert mock_provider.call_count == 0

    async def test_unknown_element_type_skipped(
        self,
        sample_html: str,
        mock_provider: MockLLMProvider,
    ) -> None:
        """An element with no registered handler should be skipped gracefully."""
        element = UnresolvedElement(
            placeholder_id="unresolved-s0-e0",
            element_type="unknown-type",
        )
        enhancer = Enhancer(mock_provider)
        output = await enhancer.enhance_with_report(sample_html, [element])

        assert output.report.failed == 1
        assert "No handler" in output.report.results[0].error
