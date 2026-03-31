use super::color::Color;
use super::slide::Shape;
use super::style::{
    Alignment, Border, Fill, GlowEffect, OuterShadow, StrikethroughType, TextCapitalization,
    UnderlineType,
};

/// Slide master -- contains background, shapes, text styles, ClrMap
#[derive(Debug, Clone, Default)]
pub struct SlideMaster {
    pub theme_idx: usize,
    pub background: Option<Fill>,
    pub clr_map: super::presentation::ClrMap,
    pub tx_styles: TxStyles,
    pub shapes: Vec<Shape>,
}

/// Slide layout -- inherits from a master
#[derive(Debug, Clone)]
pub struct SlideLayout {
    pub master_idx: usize,
    pub layout_type: Option<String>,
    pub background: Option<Fill>,
    pub clr_map_ovr: Option<ClrMapOverride>,
    pub show_master_sp: bool,
    pub shapes: Vec<Shape>,
}

impl Default for SlideLayout {
    fn default() -> Self {
        Self {
            master_idx: 0,
            layout_type: None,
            background: None,
            clr_map_ovr: None,
            show_master_sp: true,
            shapes: Vec::new(),
        }
    }
}

/// Text styles defined in slide master (titleStyle, bodyStyle, otherStyle)
#[derive(Debug, Clone, Default)]
pub struct TxStyles {
    pub title_style: Option<ListStyle>,
    pub body_style: Option<ListStyle>,
    pub other_style: Option<ListStyle>,
}

/// Level-based text style container (lvl1pPr through lvl9pPr)
#[derive(Debug, Clone, Default)]
pub struct ListStyle {
    pub levels: [Option<ParagraphDefaults>; 9],
}

/// Default paragraph properties for a given level
#[derive(Debug, Clone, Default)]
pub struct ParagraphDefaults {
    pub alignment: Option<Alignment>,
    pub margin_left: Option<f64>,
    pub indent: Option<f64>,
    pub line_spacing: Option<SpacingValue>,
    pub space_before: Option<SpacingValue>,
    pub space_after: Option<SpacingValue>,
    pub bullet: Option<super::slide::Bullet>,
    pub def_run_props: Option<RunDefaults>,
}

/// Default run (character) properties
#[derive(Debug, Clone, Default)]
pub struct RunDefaults {
    pub font_size: Option<f64>,
    pub letter_spacing: Option<f64>,
    pub baseline: Option<i32>,
    pub capitalization: Option<TextCapitalization>,
    pub underline: Option<UnderlineType>,
    pub strikethrough: Option<StrikethroughType>,
    pub bold: Option<bool>,
    pub italic: Option<bool>,
    pub color: Option<Color>,
    pub font_latin: Option<String>,
    pub font_ea: Option<String>,
}

/// Spacing value -- percentage or absolute points
#[derive(Debug, Clone)]
pub enum SpacingValue {
    Percent(f64),
    Points(f64),
}

/// ClrMap override (slide/layout can override master's ClrMap)
#[derive(Debug, Clone)]
pub enum ClrMapOverride {
    UseMaster,
    Override(super::presentation::ClrMap),
}

/// Placeholder identification
#[derive(Debug, Clone, Default)]
pub struct PlaceholderInfo {
    pub ph_type: Option<PlaceholderType>,
    pub idx: Option<u32>,
}

/// ECMA-376 placeholder types
#[derive(Debug, Clone)]
pub enum PlaceholderType {
    Title,
    CtrTitle,
    SubTitle,
    Body,
    Obj,
    Chart,
    Tbl,
    Dgm,
    Media,
    ClipArt,
    Pic,
    Dt,
    Ftr,
    SldNum,
    Hdr,
    SldImg,
}

impl PlaceholderType {
    pub fn from_ooxml(val: &str) -> Option<Self> {
        Some(match val {
            "title" => Self::Title,
            "ctrTitle" => Self::CtrTitle,
            "subTitle" => Self::SubTitle,
            "body" => Self::Body,
            "obj" => Self::Obj,
            "chart" => Self::Chart,
            "tbl" => Self::Tbl,
            "dgm" => Self::Dgm,
            "media" => Self::Media,
            "clipArt" => Self::ClipArt,
            "pic" => Self::Pic,
            "dt" => Self::Dt,
            "ftr" => Self::Ftr,
            "sldNum" => Self::SldNum,
            "hdr" => Self::Hdr,
            "sldImg" => Self::SldImg,
            _ => return None,
        })
    }
}

/// Shape style reference (<p:style> element)
#[derive(Debug, Clone, Default)]
pub struct ShapeStyleRef {
    pub fill_ref: Option<StyleRef>,
    pub ln_ref: Option<StyleRef>,
    pub effect_ref: Option<StyleRef>,
    pub font_ref: Option<FontRef>,
}

#[derive(Debug, Clone, Default)]
pub struct StyleRef {
    pub idx: u32,
    pub color: Color,
}

#[derive(Debug, Clone, Default)]
pub struct FontRef {
    pub idx: String,
    pub color: Color,
}

/// Effect style entry from <a:effectStyleLst>
#[derive(Debug, Clone, Default)]
pub struct EffectStyle {
    pub outer_shadow: Option<OuterShadow>,
    pub glow: Option<GlowEffect>,
}

/// Theme format scheme (fillStyleLst, lnStyleLst, effectStyleLst, bgFillStyleLst)
#[derive(Debug, Clone, Default)]
pub struct FmtScheme {
    pub fill_style_lst: Vec<Fill>,
    pub ln_style_lst: Vec<Border>,
    pub effect_style_lst: Vec<EffectStyle>,
    pub bg_fill_style_lst: Vec<Fill>,
}

impl FmtScheme {
    /// Retrieve fill style by 1-based idx.
    /// idx 1~999 → fill_style_lst, idx 1001+ → bg_fill_style_lst.
    pub fn get_fill_style(&self, idx: u32) -> Option<&Fill> {
        if idx == 0 {
            return None;
        }
        if idx >= 1001 {
            self.bg_fill_style_lst.get((idx - 1001) as usize)
        } else {
            self.fill_style_lst.get((idx - 1) as usize)
        }
    }

    /// Retrieve line style by 1-based idx.
    pub fn get_line_style(&self, idx: u32) -> Option<&Border> {
        if idx == 0 {
            return None;
        }
        self.ln_style_lst.get((idx - 1) as usize)
    }
}
