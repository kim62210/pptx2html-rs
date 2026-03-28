"""LLM prompt templates for element conversion."""

from __future__ import annotations

SMARTART_SYSTEM = (
    "You are a PPTX SmartArt to HTML/CSS converter. "
    "You produce self-contained HTML/CSS blocks that faithfully reproduce "
    "the visual layout of SmartArt diagrams. "
    "Output ONLY the HTML/CSS code block wrapped in <html-output> tags. "
    "No explanations, no markdown fences."
)

SMARTART_USER = """\
Convert the following SmartArt diagram data into a self-contained HTML/CSS block.

## Constraints
- Output must fit within {width}px x {height}px bounding box
- Use only inline styles (no external CSS references)
- Use flexbox or CSS grid for layout
- Text must be readable (min 12px font size)
- Use these theme colors if available: {theme_colors}
- The diagram type is: {layout_type}

## Data Model
{data_model}

## Raw XML Reference
{raw_xml}

Output ONLY the HTML/CSS code block, no explanations.
Wrap your output in <html-output> tags."""

MATH_SYSTEM = (
    "You are an OMML (Office Math Markup Language) to MathML/KaTeX converter. "
    "Convert the given Office Math XML into valid MathML that can be rendered in a browser. "
    "Output ONLY the MathML wrapped in <html-output> tags."
)

MATH_USER = """\
Convert the following Office Math XML (OMML) into browser-renderable MathML.

## Constraints
- Output valid MathML wrapped in a <math> element
- Use display="block" for standalone equations
- Preserve all operators, fractions, superscripts, subscripts, and roots
- If the formula is simple enough for KaTeX, you may output a <span class="katex"> wrapper instead

## OMML XML
{raw_xml}

Output ONLY the MathML or KaTeX HTML, no explanations.
Wrap your output in <html-output> tags."""

EFFECTS_SYSTEM = (
    "You are a DrawingML visual effects to CSS converter. "
    "Convert the given DrawingML effect XML into equivalent CSS properties. "
    "Output ONLY a JSON object mapping CSS property names to values, "
    "wrapped in <html-output> tags."
)

EFFECTS_USER = """\
Convert the following DrawingML effect properties into CSS.

## Constraints
- Map outerShdw to box-shadow
- Map glow to box-shadow with spread
- Map reflection to appropriate CSS (opacity gradient)
- Map softEdge to filter: blur()
- Output a JSON object with CSS property-value pairs

## DrawingML XML
{raw_xml}

Output ONLY the JSON object with CSS properties.
Wrap your output in <html-output> tags."""
