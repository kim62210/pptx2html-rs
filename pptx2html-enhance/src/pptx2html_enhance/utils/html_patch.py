"""HTML DOM manipulation for replacing placeholder elements."""

from __future__ import annotations

import logging
import re

from bs4 import BeautifulSoup, Tag

logger = logging.getLogger(__name__)


def patch_html(html: str, replacements: dict[str, str]) -> str:
    """Replace placeholder elements in HTML with generated content.

    Finds elements by their ``id`` attribute and replaces them with the
    provided HTML fragments.  If a placeholder ID is not found in the
    document, it is silently skipped.

    Args:
        html: Original HTML string from pptx2html-rs.
        replacements: Mapping of ``{placeholder_id: new_html_content}``.

    Returns:
        Patched HTML string with placeholders replaced.
    """
    if not replacements:
        return html

    soup = BeautifulSoup(html, "lxml")

    for placeholder_id, new_content in replacements.items():
        element = soup.find(id=placeholder_id)
        if element is None:
            logger.debug("Placeholder '%s' not found in HTML, skipping", placeholder_id)
            continue

        if not isinstance(element, Tag):
            continue

        new_soup = BeautifulSoup(new_content, "lxml")
        new_body = new_soup.find("body")
        if new_body is None:
            logger.debug("Replacement for '%s' has no parseable body", placeholder_id)
            continue

        # Create a wrapper div preserving the original element's positioning
        wrapper = soup.new_tag("div")
        wrapper["id"] = placeholder_id
        wrapper["class"] = "enhanced-element"

        # Copy inline style (position, size) from original placeholder
        original_parent = element.parent
        if original_parent and isinstance(original_parent, Tag):
            style = original_parent.get("style", "")
            if style:
                wrapper["data-original-style"] = str(style)

        # Move all children from the parsed fragment into the wrapper
        for child in list(new_body.children):
            wrapper.append(child.extract())

        element.replace_with(wrapper)

    return str(soup)


def extract_html_output(text: str) -> str | None:
    """Extract content from <html-output> tags in LLM response.

    Args:
        text: Raw LLM response text.

    Returns:
        The content between <html-output> tags, or None if not found.
    """
    match = re.search(
        r"<html-output>(.*?)</html-output>",
        text,
        re.DOTALL,
    )
    if match:
        return match.group(1).strip()
    return None
