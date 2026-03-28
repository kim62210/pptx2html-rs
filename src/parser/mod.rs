//! PPTX ZIP/XML parser
//! PPTX = ZIP archive containing OOXML (PresentationML) XML files

mod relationships;
mod slide_parser;
mod theme_parser;
mod xml_utils;

use std::io::{Cursor, Read};
use std::path::Path;

use quick_xml::events::Event;
use quick_xml::Reader;
use zip::ZipArchive;

use crate::error::{PptxError, PptxResult};
use crate::model::presentation::ClrMap;
use crate::model::{Emu, Presentation, Size};

pub struct PptxParser;

impl PptxParser {
    /// Parse PPTX from file path
    pub fn parse_file(path: &Path) -> PptxResult<Presentation> {
        let data = std::fs::read(path)?;
        Self::parse_bytes(&data)
    }

    /// Parse PPTX from byte data
    pub fn parse_bytes(data: &[u8]) -> PptxResult<Presentation> {
        let cursor = Cursor::new(data);
        let mut archive = ZipArchive::new(cursor)?;

        let mut presentation = Presentation::default();

        // 1. Parse slide size and slide list from presentation.xml
        let pres_xml = Self::read_entry(&mut archive, "ppt/presentation.xml")?;
        let (slide_size, slide_rel_ids) = Self::parse_presentation_xml(&pres_xml)?;
        presentation.slide_size = slide_size;

        // 2. Parse theme
        if let Ok(theme_xml) = Self::read_entry(&mut archive, "ppt/theme/theme1.xml") {
            presentation.themes.push(theme_parser::parse_theme(&theme_xml)?);
        }

        // 3. Parse relationships
        let rels_xml = Self::read_entry(&mut archive, "ppt/_rels/presentation.xml.rels")?;
        let rels = relationships::parse_relationships(&rels_xml)?;

        // 4. Parse ClrMap from slideMaster
        presentation.clr_map = Self::parse_clr_map_from_master(&mut archive);

        // 5. Parse each slide
        for rel_id in &slide_rel_ids {
            if let Some(slide_path) = rels.get(rel_id) {
                let full_path = format!("ppt/{slide_path}");
                if let Ok(slide_xml) = Self::read_entry(&mut archive, &full_path) {
                    let slide_rels_path = Self::rels_path_for(&full_path);
                    let slide_rels = if let Ok(rels_xml) =
                        Self::read_entry(&mut archive, &slide_rels_path)
                    {
                        relationships::parse_relationships(&rels_xml)?
                    } else {
                        std::collections::HashMap::new()
                    };

                    let slide = slide_parser::parse_slide(
                        &slide_xml,
                        &slide_rels,
                        &mut archive,
                    )?;
                    presentation.slides.push(slide);
                }
            }
        }

        Ok(presentation)
    }

    /// Read a ZIP entry
    fn read_entry<R: Read + std::io::Seek>(
        archive: &mut ZipArchive<R>,
        name: &str,
    ) -> PptxResult<String> {
        let mut file = archive
            .by_name(name)
            .map_err(|_| PptxError::MissingFile(name.to_string()))?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        Ok(contents)
    }

    /// Build relationships file path: ppt/slides/slide1.xml → ppt/slides/_rels/slide1.xml.rels
    fn rels_path_for(path: &str) -> String {
        if let Some((dir, file)) = path.rsplit_once('/') {
            format!("{dir}/_rels/{file}.rels")
        } else {
            format!("_rels/{path}.rels")
        }
    }

    /// Extract slide size and slide relationship ID list from presentation.xml
    fn parse_presentation_xml(xml: &str) -> PptxResult<(Size, Vec<String>)> {
        let mut reader = Reader::from_str(xml);
        let mut slide_size = Size::default();
        let mut slide_rel_ids = Vec::new();

        loop {
            match reader.read_event() {
                Ok(Event::Empty(ref e)) | Ok(Event::Start(ref e)) => {
                    let name = e.name();
                    let local = xml_utils::local_name(name.as_ref());
                    match local {
                        "sldSz" => {
                            for attr in e.attributes().flatten() {
                                let key = xml_utils::local_name(attr.key.as_ref());
                                let val = String::from_utf8_lossy(&attr.value);
                                match key {
                                    "cx" => slide_size.width = Emu::from_str(&val),
                                    "cy" => slide_size.height = Emu::from_str(&val),
                                    _ => {}
                                }
                            }
                        }
                        "sldId" => {
                            for attr in e.attributes().flatten() {
                                let key = std::str::from_utf8(attr.key.as_ref())
                                    .unwrap_or("");
                                if key.ends_with("id") && key.contains(':') {
                                    let val = String::from_utf8_lossy(&attr.value);
                                    slide_rel_ids.push(val.to_string());
                                }
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

        Ok((slide_size, slide_rel_ids))
    }

    /// Parse ClrMap from slideMaster1.xml
    fn parse_clr_map_from_master<R: Read + std::io::Seek>(
        archive: &mut ZipArchive<R>,
    ) -> ClrMap {
        let xml = match Self::read_entry(archive, "ppt/slideMasters/slideMaster1.xml") {
            Ok(xml) => xml,
            Err(_) => return ClrMap::default(),
        };

        let mut reader = Reader::from_str(&xml);
        let mut clr_map = ClrMap::default();

        loop {
            match reader.read_event() {
                Ok(Event::Empty(ref e)) | Ok(Event::Start(ref e)) => {
                    let name = e.name();
                    let local = xml_utils::local_name(name.as_ref());
                    if local == "clrMap" {
                        for attr in e.attributes().flatten() {
                            let key = xml_utils::local_name(attr.key.as_ref());
                            let val = String::from_utf8_lossy(&attr.value);
                            clr_map.set(key, &val);
                        }
                        break;
                    }
                }
                Ok(Event::Eof) => break,
                Err(_) => break,
                _ => {}
            }
        }

        clr_map
    }
}
