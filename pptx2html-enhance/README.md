# pptx2html-enhance

LLM-powered enhancement layer for [pptx2html-rs](../README.md) output.

Replaces placeholder elements (SmartArt, Math, custom geometry) in pptx2html-rs HTML output with actual rendered content using LLM-generated HTML/CSS.

## Install

```bash
# Core (no LLM provider)
pip install pptx2html-enhance

# With Anthropic (Claude)
pip install pptx2html-enhance[anthropic]

# With OpenAI
pip install pptx2html-enhance[openai]

# All providers
pip install pptx2html-enhance[all]

# Development
pip install -e ".[dev]"
```

## Quick Start

```python
import asyncio
import pptx2html  # Rust Python bindings
from pptx2html_enhance import enhance

# Convert PPTX with metadata
result = pptx2html.convert_with_metadata("presentation.pptx")

# Enhance unresolved elements with LLM
enhanced_html = asyncio.run(enhance(
    result.html,
    [
        {
            "placeholder_id": elem.placeholder_id,
            "element_type": elem.element_type,
            "slide_index": elem.slide_index,
            "raw_xml": elem.raw_xml,
            "data_model": elem.data_model,
        }
        for elem in result.unresolved_elements
    ],
    provider="anthropic",  # or "openai"
))
```

## API

### `enhance(html, unresolved_elements, **kwargs) -> str`

Main entry point. Replaces placeholder elements with LLM-generated content.

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `html` | `str` | required | HTML from pptx2html-rs |
| `unresolved_elements` | `list[dict]` | required | Element metadata |
| `provider` | `str \| LLMProvider` | `"anthropic"` | LLM provider |
| `api_key` | `str \| None` | `None` | API key (env var fallback) |
| `model` | `str \| None` | `None` | Model override |
| `timeout` | `float` | `30.0` | Per-element timeout (seconds) |
| `max_concurrent` | `int` | `5` | Max concurrent LLM calls |

### `enhance_html(html, metadata_json, **kwargs) -> str`

Same as `enhance()` but accepts metadata as a JSON string.

### `Enhancer` class

For full control over the pipeline:

```python
from pptx2html_enhance import Enhancer
from pptx2html_enhance.providers.anthropic import AnthropicProvider

provider = AnthropicProvider(model="claude-sonnet-4-20250514")
enhancer = Enhancer(provider, timeout=60.0, max_concurrent=3)

output = await enhancer.enhance_with_report(html, elements)
print(f"Replaced {output.report.succeeded}/{output.report.total} elements")
enhanced_html = output.html
```

## Element Handlers

| Element Type | Handler | Strategy |
|-------------|---------|----------|
| `smartart` | `SmartArtHandler` | LLM generates HTML/CSS diagram from data model + raw XML |
| `math` | `MathHandler` | Rule-based OMML->MathML (fractions, scripts, roots), LLM fallback |
| `custom-geometry` | `EffectsHandler` | Rule-based DrawingML->CSS (shadow, glow, blur), LLM fallback |
| `ole` | (none) | Skipped (no handler registered) |

### Custom Handlers

```python
from pptx2html_enhance.handlers.base import Handler

class MyOleHandler(Handler):
    async def process(self, element, provider):
        # Your implementation
        return "<div>OLE content</div>"

enhancer.register_handler("ole", MyOleHandler())
```

## Design

- **Graceful degradation**: LLM failure preserves the original placeholder
- **Concurrent processing**: Elements processed in parallel with configurable semaphore
- **Two-stage conversion**: Rule-based first (fast, deterministic), LLM fallback for complex cases
- **Provider-agnostic**: Anthropic and OpenAI built-in, extensible via `LLMProvider` ABC
- **No hard dependency on pptx2html**: Works with pre-generated HTML + metadata JSON

## Testing

```bash
pip install -e ".[dev]"
python -m pytest tests/ -v
```

32 tests covering the full pipeline, handlers, HTML patching, and edge cases.

## License

MIT
