use quick_xml::events::Event;
use quick_xml::Reader;

use crate::error::PptxResult;
use crate::model::presentation::{ColorScheme, Theme};
use super::xml_utils;

pub fn parse_theme(xml: &str) -> PptxResult<Theme> {
    let mut reader = Reader::from_str(xml);
    let mut theme = Theme::default();
    let mut in_color_scheme = false;
    let mut current_color_role: Option<String> = None;
    let mut in_major_font = false;
    let mut in_minor_font = false;

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
                    _ => {}
                }
            }
            Ok(Event::Empty(ref e)) => {
                let name = e.name();
                let local = xml_utils::local_name(name.as_ref());
                match local {
                    "srgbClr" if current_color_role.is_some() => {
                        if let Some(val) = xml_utils::attr_str(e, "val") {
                            set_color_scheme(&mut theme.color_scheme, current_color_role.as_deref().unwrap_or(""), &val);
                        }
                        current_color_role = None;
                    }
                    "sysClr" if current_color_role.is_some() => {
                        if let Some(val) = xml_utils::attr_str(e, "lastClr") {
                            set_color_scheme(&mut theme.color_scheme, current_color_role.as_deref().unwrap_or(""), &val);
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
                            theme.font_scheme.major_east_asian = Some(typeface);
                        }
                    }
                    "ea" if in_minor_font => {
                        if let Some(typeface) = xml_utils::attr_str(e, "typeface") {
                            theme.font_scheme.minor_east_asian = Some(typeface);
                        }
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
