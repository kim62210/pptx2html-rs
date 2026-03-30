//! Data model representing PPTX document structure
//! Based on ECMA-376 Part 1 (PresentationML)

pub mod color;
mod geometry;
pub mod hierarchy;
pub mod presentation;
pub mod slide;
mod style;

pub use color::{Color, ColorKind, ColorModifier, ResolvedColor};
pub use geometry::{CustomGeometry, Emu, GeometryPath, PathCommand, PathFill, Position, Size};
pub use hierarchy::{
    ClrMapOverride, FmtScheme, FontRef, ListStyle, ParagraphDefaults, PlaceholderInfo,
    PlaceholderType, RunDefaults, ShapeStyleRef, SlideLayout, SlideMaster, SpacingValue, StyleRef,
    TxStyles,
};
pub use presentation::{ClrMap, FontScheme, Presentation};
pub use slide::{
    AutoFit, Bullet, BulletAutoNum, BulletChar, ChartData, CropRect, GroupData, ParagraphDefRPr,
    PictureData, Shape, ShapeType, Slide, TableCell, TableData, TableRow, TextBody, TextMargins,
    TextParagraph, TextRun, UnresolvedElement, UnresolvedType, UnsupportedData, VerticalAlign,
};
pub use style::{
    Alignment, Border, BorderStyle, DashStyle, Fill, FontStyle, GlowEffect, GradientFill,
    GradientStop, ImageFill, LineCap, LineEnd, LineEndSize, LineEndType, LineJoin, OuterShadow,
    ShapeEffects, SolidFill, TextShadow, TextStyle,
};
