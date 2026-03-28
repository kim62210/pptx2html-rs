use quick_xml::Reader;
use quick_xml::events::Event;

use super::xml_utils;
use crate::error::PptxResult;
use crate::model::presentation::{ColorScheme, Theme};
use crate::model::{Border, BorderStyle, Color, Emu, Fill, FmtScheme, SolidFill};

pub fn parse_theme(xml: &str) -> PptxResult<Theme> {
    let mut reader = Reader::from_str(xml);
    let mut theme = Theme::default();
    let mut in_color_scheme = false;
    let mut current_color_role: Option<String> = None;
    let mut in_major_font = false;
    let mut in_minor_font = false;

    // FmtScheme state
    let mut in_fmt_scheme = false;
    let mut fmt_list_kind: Option<FmtKind> = None;
    let mut current_fill_color: Option<Color> = None;
    let mut in_ln = false;
    let mut current_ln_width: f64 = 0.0;
    let mut current_ln_color: Option<Color> = None;

    loop {
        match reader.read_event() {
            Ok(Event::Start(ref e)) => {
                let name = e.name();
                let local = xml_utils::local_name(name.as_ref());
                match local {
                    "clrScheme" => {
                        in_color_scheme = true;
                        if let Some(name) = xml_utils::attr_str(e, "name") {
                            theme.name = name;
                        }
                    }
                    "dk1" | "lt1" | "dk2" | "lt2" | "accent1" | "accent2" | "accent3"
                    | "accent4" | "accent5" | "accent6" | "hlink" | "folHlink"
                        if in_color_scheme =>
                    {
                        current_color_role = Some(local.to_string());
                    }
                    "majorFont" => in_major_font = true,
                    "minorFont" => in_minor_font = true,
                    // FmtScheme
                    "fmtScheme" => in_fmt_scheme = true,
                    "fillStyleLst" if in_fmt_scheme => {
                        fmt_list_kind = Some(FmtKind::Fill);
                    }
                    "lnStyleLst" if in_fmt_scheme => {
                        fmt_list_kind = Some(FmtKind::Ln);
                    }
                    "bgFillStyleLst" if in_fmt_scheme => {
                        fmt_list_kind = Some(FmtKind::BgFill);
                    }
                    // Line element inside lnStyleLst
                    "ln" if matches!(fmt_list_kind, Some(FmtKind::Ln)) => {
                        in_ln = true;
                        current_ln_width = xml_utils::attr_str(e, "w")
                            .map(|w| Emu::parse_emu(&w).to_pt())
                            .unwrap_or(0.0);
                        current_ln_color = None;
                    }
                    // solidFill inside fill/bg lists
                    "solidFill" if fmt_list_kind.is_some() => {
                        current_fill_color = None;
                    }
                    // Color elements (Start variant, may have child modifiers)
                    "srgbClr" if fmt_list_kind.is_some() => {
                        if let Some(val) = xml_utils::attr_str(e, "val") {
                            if in_ln {
                                current_ln_color = Some(Color::rgb(val));
                            } else {
                                current_fill_color = Some(Color::rgb(val));
                            }
                        }
                    }
                    "schemeClr" if fmt_list_kind.is_some() => {
                        if let Some(val) = xml_utils::attr_str(e, "val") {
                            if in_ln {
                                current_ln_color = Some(Color::theme(val));
                            } else {
                                current_fill_color = Some(Color::theme(val));
                            }
                        }
                    }
                    _ => {}
                }
            }
            Ok(Event::Empty(ref e)) => {
                let name = e.name();
                let local = xml_utils::local_name(name.as_ref());
                match local {
                    "srgbClr" if current_color_role.is_some() => {
                        if let Some(val) = xml_utils::attr_str(e, "val") {
                            set_color_scheme(
                                &mut theme.color_scheme,
                                current_color_role.as_deref().unwrap_or(""),
                                &val,
                            );
                        }
                        current_color_role = None;
                    }
                    "sysClr" if current_color_role.is_some() => {
                        if let Some(val) = xml_utils::attr_str(e, "lastClr") {
                            set_color_scheme(
                                &mut theme.color_scheme,
                                current_color_role.as_deref().unwrap_or(""),
                                &val,
                            );
                        }
                        current_color_role = None;
                    }
                    "latin" if in_major_font => {
                        if let Some(typeface) = xml_utils::attr_str(e, "typeface") {
                            theme.font_scheme.major_latin = typeface;
                        }
                    }
                    "latin" if in_minor_font => {
                        if let Some(typeface) = xml_utils::attr_str(e, "typeface") {
                            theme.font_scheme.minor_latin = typeface;
                        }
                    }
                    "ea" if in_major_font => {
                        if let Some(typeface) = xml_utils::attr_str(e, "typeface") {
                            if !typeface.is_empty() {
                                theme.font_scheme.major_east_asian = Some(typeface);
                            }
                        }
                    }
                    "ea" if in_minor_font => {
                        if let Some(typeface) = xml_utils::attr_str(e, "typeface") {
                            if !typeface.is_empty() {
                                theme.font_scheme.minor_east_asian = Some(typeface);
                            }
                        }
                    }
                    // Empty color elements inside fmtScheme lists
                    "srgbClr" if fmt_list_kind.is_some() => {
                        if let Some(val) = xml_utils::attr_str(e, "val") {
                            if in_ln {
                                current_ln_color = Some(Color::rgb(val));
                            } else {
                                current_fill_color = Some(Color::rgb(val));
                            }
                        }
                    }
                    "schemeClr" if fmt_list_kind.is_some() => {
                        if let Some(val) = xml_utils::attr_str(e, "val") {
                            if in_ln {
                                current_ln_color = Some(Color::theme(val));
                            } else {
                                current_fill_color = Some(Color::theme(val));
                            }
                        }
                    }
                    // noFill in fill/bg lists
                    "noFill" if fmt_list_kind.is_some() && !in_ln => {
                        push_fill(&fmt_list_kind, &mut theme.fmt_scheme, Fill::None);
                    }
                    _ => {}
                }
            }
            Ok(Event::End(ref e)) => {
                let name = e.name();
                let local = xml_utils::local_name(name.as_ref());
                match local {
                    "clrScheme" => in_color_scheme = false,
                    "majorFont" => in_major_font = false,
                    "minorFont" => in_minor_font = false,
                    "dk1" | "lt1" | "dk2" | "lt2" | "accent1" | "accent2" | "accent3"
                    | "accent4" | "accent5" | "accent6" | "hlink" | "folHlink" => {
                        current_color_role = None;
                    }
                    // End of FmtScheme sections
                    "fmtScheme" => {
                        in_fmt_scheme = false;
                        fmt_list_kind = None;
                    }
                    "fillStyleLst" | "lnStyleLst" | "bgFillStyleLst" => {
                        fmt_list_kind = None;
                    }
                    // End of solidFill in fill/bg lists
                    "solidFill" if fmt_list_kind.is_some() && !in_ln => {
                        if let Some(color) = current_fill_color.take() {
                            push_fill(
                                &fmt_list_kind,
                                &mut theme.fmt_scheme,
                                Fill::Solid(SolidFill { color }),
                            );
                        }
                    }
                    // End of solidFill inside line
                    "solidFill" if in_ln => {
                        // color captured in current_ln_color
                    }
                    // End of line element
                    "ln" if in_ln => {
                        in_ln = false;
                        let border = Border {
                            width: current_ln_width,
                            color: current_ln_color.take().unwrap_or_default(),
                            style: if current_ln_width > 0.0 {
                                BorderStyle::Solid
                            } else {
                                BorderStyle::None
                            },
                            ..Default::default()
                        };
                        theme.fmt_scheme.ln_style_lst.push(border);
                    }
                    // End of color elements (with modifiers)
                    "srgbClr" | "schemeClr" if fmt_list_kind.is_some() => {
                        // Colors already captured
                    }
                    _ => {}
                }
            }
            Ok(Event::Eof) => break,
            Err(e) => return Err(crate::error::PptxError::Xml(e)),
            _ => {}
        }
    }

    Ok(theme)
}

enum FmtKind {
    Fill,
    Ln,
    BgFill,
}

fn push_fill(kind: &Option<FmtKind>, fmt: &mut FmtScheme, fill: Fill) {
    match kind {
        Some(FmtKind::Fill) => fmt.fill_style_lst.push(fill),
        Some(FmtKind::BgFill) => fmt.bg_fill_style_lst.push(fill),
        _ => {}
    }
}

fn set_color_scheme(scheme: &mut ColorScheme, role: &str, hex: &str) {
    let hex = hex.to_string();
    match role {
        "dk1" => scheme.dk1 = hex,
        "lt1" => scheme.lt1 = hex,
        "dk2" => scheme.dk2 = hex,
        "lt2" => scheme.lt2 = hex,
        "accent1" => scheme.accent1 = hex,
        "accent2" => scheme.accent2 = hex,
        "accent3" => scheme.accent3 = hex,
        "accent4" => scheme.accent4 = hex,
        "accent5" => scheme.accent5 = hex,
        "accent6" => scheme.accent6 = hex,
        "hlink" => scheme.hlink = hex,
        "folHlink" => scheme.fol_hlink = hex,
        _ => {}
    }
}
