"""Abstract LLM provider interface."""

from __future__ import annotations

from abc import ABC, abstractmethod


class LLMProvider(ABC):
    """Base class for LLM API providers."""

    @abstractmethod
    async def generate(
        self,
        system_prompt: str,
        user_prompt: str,
        *,
        max_tokens: int = 4096,
    ) -> str:
        """Generate a completion from the LLM.

        Args:
            system_prompt: System/instruction message.
            user_prompt: User message with the conversion request.
            max_tokens: Maximum tokens in the response.

        Returns:
            The generated text content.

        Raises:
            RuntimeError: If the API call fails.
        """

    @abstractmethod
    async def close(self) -> None:
        """Release any held resources (HTTP clients, etc.)."""
