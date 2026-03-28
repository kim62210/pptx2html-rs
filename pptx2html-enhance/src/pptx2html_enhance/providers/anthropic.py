"""Anthropic (Claude) LLM provider."""

from __future__ import annotations

import logging
import os

from pptx2html_enhance.providers.base import LLMProvider

logger = logging.getLogger(__name__)


class AnthropicProvider(LLMProvider):
    """Claude API provider using the anthropic SDK."""

    def __init__(
        self,
        api_key: str | None = None,
        model: str = "claude-sonnet-4-20250514",
    ) -> None:
        try:
            import anthropic
        except ImportError as exc:
            raise ImportError(
                "anthropic package is required: pip install pptx2html-enhance[anthropic]"
            ) from exc

        resolved_key = api_key or os.environ.get("ANTHROPIC_API_KEY", "")
        if not resolved_key:
            raise ValueError(
                "Anthropic API key required via api_key param or ANTHROPIC_API_KEY env var"
            )

        self._client = anthropic.AsyncAnthropic(api_key=resolved_key)
        self._model = model

    async def generate(
        self,
        system_prompt: str,
        user_prompt: str,
        *,
        max_tokens: int = 4096,
    ) -> str:
        try:
            response = await self._client.messages.create(
                model=self._model,
                max_tokens=max_tokens,
                system=system_prompt,
                messages=[{"role": "user", "content": user_prompt}],
            )
            return response.content[0].text
        except Exception as exc:
            logger.exception("Anthropic API call failed")
            raise RuntimeError(f"Anthropic API error: {exc}") from exc

    async def close(self) -> None:
        await self._client.close()
