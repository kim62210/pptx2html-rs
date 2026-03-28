"""Tests for HTML DOM patching utility."""

from __future__ import annotations

from pptx2html_enhance.utils.html_patch import extract_html_output, patch_html


class TestPatchHtml:
    """Tests for patch_html()."""

    def test_replace_single_placeholder(self, sample_html: str) -> None:
        """Replace a single placeholder element by ID."""
        replacements = {
            "unresolved-s0-e0": "<div>SmartArt Diagram</div>",
        }
        result = patch_html(sample_html, replacements)

        assert "SmartArt Diagram" in result
        # Original placeholder text should be gone
        assert "[SmartArt]" not in result
        # The other placeholder should still be there
        assert "[Math Equation]" in result

    def test_replace_multiple_placeholders(self, sample_html: str) -> None:
        """Replace multiple placeholders in one pass."""
        replacements = {
            "unresolved-s0-e0": "<div>Diagram</div>",
            "unresolved-s0-e1": "<math><mfrac><mi>a</mi><mi>b</mi></mfrac></math>",
        }
        result = patch_html(sample_html, replacements)

        assert "Diagram" in result
        assert "<mfrac>" in result
        assert "[SmartArt]" not in result
        assert "[Math Equation]" not in result

    def test_missing_placeholder_is_noop(self, sample_html: str) -> None:
        """A replacement for a non-existent ID should not modify the HTML."""
        replacements = {
            "nonexistent-id": "<div>Should not appear</div>",
        }
        result = patch_html(sample_html, replacements)

        assert "Should not appear" not in result
        assert "[SmartArt]" in result

    def test_empty_replacements(self, sample_html: str) -> None:
        """Empty replacements dict returns HTML unchanged."""
        result = patch_html(sample_html, {})
        # Should be equivalent (modulo parser normalization)
        assert "[SmartArt]" in result
        assert "[Math Equation]" in result

    def test_replacement_preserves_id(self, sample_html: str) -> None:
        """The replacement wrapper should keep the original placeholder ID."""
        replacements = {
            "unresolved-s0-e0": "<div>New Content</div>",
        }
        result = patch_html(sample_html, replacements)
        assert 'id="unresolved-s0-e0"' in result

    def test_replacement_adds_enhanced_class(self, sample_html: str) -> None:
        """The replacement wrapper should have the enhanced-element class."""
        replacements = {
            "unresolved-s0-e0": "<div>New Content</div>",
        }
        result = patch_html(sample_html, replacements)
        assert 'class="enhanced-element"' in result


class TestExtractHtmlOutput:
    """Tests for extract_html_output()."""

    def test_extracts_content_from_tags(self) -> None:
        text = "Here is the output:\n<html-output><div>Hello</div></html-output>\nDone."
        result = extract_html_output(text)
        assert result == "<div>Hello</div>"

    def test_strips_whitespace(self) -> None:
        text = "<html-output>\n  <p>Content</p>\n</html-output>"
        result = extract_html_output(text)
        assert result == "<p>Content</p>"

    def test_returns_none_when_no_tags(self) -> None:
        text = "Just some text without any tags."
        result = extract_html_output(text)
        assert result is None

    def test_handles_multiline_content(self) -> None:
        text = "<html-output>\n<div>\n  <span>Line 1</span>\n  <span>Line 2</span>\n</div>\n</html-output>"
        result = extract_html_output(text)
        assert "<span>Line 1</span>" in result
        assert "<span>Line 2</span>" in result
