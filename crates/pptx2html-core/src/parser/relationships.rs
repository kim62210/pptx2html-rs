use std::collections::HashMap;

use quick_xml::Reader;
use quick_xml::events::Event;

use super::xml_utils;
use crate::error::PptxResult;

/// Parse .rels file into {rId → target_path} map
pub fn parse_relationships(xml: &str) -> PptxResult<HashMap<String, String>> {
    let mut reader = Reader::from_str(xml);
    let mut rels = HashMap::new();

    loop {
        match reader.read_event() {
            Ok(Event::Empty(ref e)) | Ok(Event::Start(ref e)) => {
                let name = e.name();
                let local = xml_utils::local_name(name.as_ref());
                if local == "Relationship" {
                    let mut id = String::new();
                    let mut target = String::new();
                    for attr in e.attributes().flatten() {
                        let key = std::str::from_utf8(attr.key.as_ref()).unwrap_or("");
                        let val = String::from_utf8_lossy(&attr.value).to_string();
                        match key {
                            "Id" => id = val,
                            "Target" => target = val,
                            _ => {}
                        }
                    }
                    if !id.is_empty() && !target.is_empty() {
                        rels.insert(id, target);
                    }
                }
            }
            Ok(Event::Eof) => break,
            Err(e) => return Err(crate::error::PptxError::Xml(e)),
            _ => {}
        }
    }

    Ok(rels)
}
