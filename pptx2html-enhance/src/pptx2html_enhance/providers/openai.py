"""OpenAI LLM provider."""

from __future__ import annotations

import logging
import os

from pptx2html_enhance.providers.base import LLMProvider

logger = logging.getLogger(__name__)


class OpenAIProvider(LLMProvider):
    """OpenAI API provider using the openai SDK."""

    def __init__(
        self,
        api_key: str | None = None,
        model: str = "gpt-4o",
    ) -> None:
        try:
            import openai
        except ImportError as exc:
            raise ImportError(
                "openai package is required: pip install pptx2html-enhance[openai]"
            ) from exc

        resolved_key = api_key or os.environ.get("OPENAI_API_KEY", "")
        if not resolved_key:
            raise ValueError(
                "OpenAI API key required via api_key param or OPENAI_API_KEY env var"
            )

        self._client = openai.AsyncOpenAI(api_key=resolved_key)
        self._model = model

    async def generate(
        self,
        system_prompt: str,
        user_prompt: str,
        *,
        max_tokens: int = 4096,
    ) -> str:
        try:
            response = await self._client.chat.completions.create(
                model=self._model,
                max_tokens=max_tokens,
                messages=[
                    {"role": "system", "content": system_prompt},
                    {"role": "user", "content": user_prompt},
                ],
            )
            content = response.choices[0].message.content
            if content is None:
                raise RuntimeError("OpenAI returned empty content")
            return content
        except RuntimeError:
            raise
        except Exception as exc:
            logger.exception("OpenAI API call failed")
            raise RuntimeError(f"OpenAI API error: {exc}") from exc

    async def close(self) -> None:
        await self._client.close()
