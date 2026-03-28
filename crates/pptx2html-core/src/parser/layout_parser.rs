use std::collections::HashMap;
use std::io::{Read, Seek};

use quick_xml::Reader;
use quick_xml::events::Event;
use zip::ZipArchive;

use super::master_parser::parse_placeholder_attrs;
use super::xml_utils;
use crate::error::{PptxError, PptxResult};
use crate::model::*;

/// Parse slideLayout XML into SlideLayout
pub fn parse_slide_layout<R: Read + Seek>(
    xml: &str,
    _rels: &HashMap<String, String>,
    _archive: &mut ZipArchive<R>,
) -> PptxResult<SlideLayout> {
    let mut reader = Reader::from_str(xml);
    let mut layout = SlideLayout::default();

    let mut depth: Vec<String> = Vec::new();
    let mut in_sp_tree = false;
    let mut current_shape: Option<LayoutShapeBuilder> = None;
    let mut in_nv_pr = false;

    // Parse root element attributes
    // We handle them in the first Start event for sldLayout

    loop {
        match reader.read_event() {
            Ok(Event::Start(ref e)) => {
                let local = xml_utils::local_name(e.name().as_ref()).to_string();
                depth.push(local.clone());

                match local.as_str() {
                    "sldLayout" => {
                        // Root element attributes
                        if let Some(t) = xml_utils::attr_str(e, "type") {
                            layout.layout_type = Some(t);
                        }
                        if let Some(show) = xml_utils::attr_str(e, "showMasterSp") {
                            layout.show_master_sp = show != "0" && show != "false";
                        }
                    }
                    "spTree" => in_sp_tree = true,
                    "sp" if in_sp_tree => {
                        current_shape = Some(LayoutShapeBuilder::default());
                    }
                    "nvPr" if current_shape.is_some() => {
                        in_nv_pr = true;
                    }
                    "clrMapOvr" => {
                        // Will check child elements
                    }
                    _ => {}
                }
            }
            Ok(Event::Empty(ref e)) => {
                let local = xml_utils::local_name(e.name().as_ref()).to_string();

                match local.as_str() {
                    // Placeholder in shape
                    "ph" if in_nv_pr && current_shape.is_some() => {
                        if let Some(sb) = current_shape.as_mut() {
                            sb.placeholder = Some(parse_placeholder_attrs(e));
                        }
                    }
                    // ClrMapOverride with masterClrMapping (use master's ClrMap)
                    "masterClrMapping" => {
                        layout.clr_map_ovr = Some(ClrMapOverride::UseMaster);
                    }
                    // Position/size for shapes — only inside <a:xfrm>
                    "off" if current_shape.is_some() && depth.iter().any(|d| d == "xfrm") => {
                        if let Some(sb) = current_shape.as_mut() {
                            sb.position.x =
                                Emu::parse_emu(&xml_utils::attr_str(e, "x").unwrap_or_default());
                            sb.position.y =
                                Emu::parse_emu(&xml_utils::attr_str(e, "y").unwrap_or_default());
                        }
                    }
                    "ext" if current_shape.is_some() && depth.iter().any(|d| d == "xfrm") => {
                        if let Some(sb) = current_shape.as_mut() {
                            sb.size.width =
                                Emu::parse_emu(&xml_utils::attr_str(e, "cx").unwrap_or_default());
                            sb.size.height =
                                Emu::parse_emu(&xml_utils::attr_str(e, "cy").unwrap_or_default());
                        }
                    }
                    // overrideClrMapping (override ClrMap)
                    "overrideClrMapping" => {
                        let mut clr_map = crate::model::presentation::ClrMap::default();
                        for attr in e.attributes().flatten() {
                            let key = xml_utils::local_name(attr.key.as_ref());
                            let val = String::from_utf8_lossy(&attr.value);
                            clr_map.set(key, &val);
                        }
                        layout.clr_map_ovr = Some(ClrMapOverride::Override(clr_map));
                    }
                    _ => {}
                }
            }
            Ok(Event::End(ref e)) => {
                let local = xml_utils::local_name(e.name().as_ref()).to_string();
                depth.pop();

                match local.as_str() {
                    "spTree" => in_sp_tree = false,
                    "nvPr" => in_nv_pr = false,
                    "sp" if current_shape.is_some() => {
                        if let Some(sb) = current_shape.take() {
                            layout.shapes.push(sb.build());
                        }
                    }
                    _ => {}
                }
            }
            Ok(Event::Eof) => break,
            Err(e) => return Err(PptxError::Xml(e)),
            _ => {}
        }
    }

    Ok(layout)
}

#[derive(Default)]
struct LayoutShapeBuilder {
    position: Position,
    size: Size,
    placeholder: Option<PlaceholderInfo>,
}

impl LayoutShapeBuilder {
    fn build(self) -> Shape {
        Shape {
            position: self.position,
            size: self.size,
            placeholder: self.placeholder,
            ..Default::default()
        }
    }
}
