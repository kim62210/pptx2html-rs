"""Shared fixtures for pptx2html-enhance tests."""

from __future__ import annotations

import pytest

from pptx2html_enhance.models import UnresolvedElement
from pptx2html_enhance.providers.base import LLMProvider


class MockLLMProvider(LLMProvider):
    """LLM provider that returns canned responses for testing."""

    def __init__(self, responses: dict[str, str] | None = None) -> None:
        self._responses = responses or {}
        self._default_response = "<html-output><div>mock content</div></html-output>"
        self.call_count = 0
        self.last_system_prompt: str | None = None
        self.last_user_prompt: str | None = None

    async def generate(
        self,
        system_prompt: str,
        user_prompt: str,
        *,
        max_tokens: int = 4096,
    ) -> str:
        self.call_count += 1
        self.last_system_prompt = system_prompt
        self.last_user_prompt = user_prompt

        # Check if any key from responses matches a substring in the user prompt
        for key, response in self._responses.items():
            if key in user_prompt:
                return response

        return self._default_response

    async def close(self) -> None:
        pass


class FailingLLMProvider(LLMProvider):
    """LLM provider that always raises RuntimeError."""

    async def generate(
        self,
        system_prompt: str,
        user_prompt: str,
        *,
        max_tokens: int = 4096,
    ) -> str:
        raise RuntimeError("Simulated API failure")

    async def close(self) -> None:
        pass


class SlowLLMProvider(LLMProvider):
    """LLM provider that takes a configurable delay before responding."""

    def __init__(self, delay: float = 60.0) -> None:
        self._delay = delay

    async def generate(
        self,
        system_prompt: str,
        user_prompt: str,
        *,
        max_tokens: int = 4096,
    ) -> str:
        import asyncio

        await asyncio.sleep(self._delay)
        return "<html-output><div>slow result</div></html-output>"

    async def close(self) -> None:
        pass


@pytest.fixture
def mock_provider() -> MockLLMProvider:
    return MockLLMProvider()


@pytest.fixture
def failing_provider() -> FailingLLMProvider:
    return FailingLLMProvider()


@pytest.fixture
def slow_provider() -> SlowLLMProvider:
    return SlowLLMProvider(delay=60.0)


@pytest.fixture
def sample_html() -> str:
    """Sample HTML with placeholder elements mimicking pptx2html-rs output."""
    return """\
<!DOCTYPE html>
<html>
<head><title>Test Presentation</title></head>
<body>
<div class="slide" style="position:relative; width:960px; height:540px;">
  <div style="position:absolute; left:100px; top:50px; width:400px; height:300px;">
    <div class="unresolved-element" id="unresolved-s0-e0" data-type="smartart" data-slide="0">
      <span>[SmartArt]</span>
    </div>
  </div>
  <div style="position:absolute; left:550px; top:200px; width:300px; height:100px;">
    <div class="unresolved-element" id="unresolved-s0-e1" data-type="math" data-slide="0">
      <span>[Math Equation]</span>
    </div>
  </div>
</div>
</body>
</html>"""


@pytest.fixture
def smartart_element() -> UnresolvedElement:
    return UnresolvedElement(
        placeholder_id="unresolved-s0-e0",
        element_type="smartart",
        slide_index=0,
        raw_xml='<dgm:relIds xmlns:dgm="http://schemas.openxmlformats.org/drawingml/2006/diagram" r:dm="rId1"/>',
        width_px=400,
        height_px=300,
    )


@pytest.fixture
def math_element() -> UnresolvedElement:
    return UnresolvedElement(
        placeholder_id="unresolved-s0-e1",
        element_type="math",
        slide_index=0,
        raw_xml='<m:oMath><m:f><m:num><m:r><m:t>a</m:t></m:r></m:num><m:den><m:r><m:t>b</m:t></m:r></m:den></m:f></m:oMath>',
    )


@pytest.fixture
def effects_element() -> UnresolvedElement:
    return UnresolvedElement(
        placeholder_id="unresolved-s1-e0",
        element_type="custom-geometry",
        slide_index=1,
        raw_xml='<a:effectLst><a:outerShdw blurRad="50800" dist="38100" dir="2700000"><a:srgbClr val="000000"><a:alpha val="40000"/></a:srgbClr></a:outerShdw></a:effectLst>',
    )
