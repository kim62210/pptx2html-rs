//! Data model representing PPTX document structure
//! Based on ECMA-376 Part 1 (PresentationML)

pub mod color;
mod geometry;
pub mod hierarchy;
pub mod presentation;
pub mod slide;
mod style;

pub use color::{Color, ColorKind, ColorModifier, ResolvedColor};
pub use geometry::{Emu, Position, Size};
pub use hierarchy::{
    ClrMapOverride, FmtScheme, FontRef, ListStyle, ParagraphDefaults, PlaceholderInfo,
    PlaceholderType, RunDefaults, ShapeStyleRef, SlideLayout, SlideMaster, SpacingValue, StyleRef,
    TxStyles,
};
pub use presentation::{ClrMap, Presentation};
pub use slide::{
    AutoFit, Bullet, PictureData, Shape, ShapeType, Slide, TableCell, TableData, TableRow,
    TextBody, TextMargins, TextParagraph, TextRun, VerticalAlign,
};
pub use style::{
    Alignment, Border, BorderStyle, Fill, FontStyle, GradientFill, GradientStop, SolidFill,
    TextStyle,
};
