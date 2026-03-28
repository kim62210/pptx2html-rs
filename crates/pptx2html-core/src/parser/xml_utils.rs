/// Strip XML namespace prefix and return local name
/// e.g. "p:sldSz" → "sldSz", "a:rPr" → "rPr"
pub fn local_name(name: &[u8]) -> &str {
    let s = std::str::from_utf8(name).unwrap_or("");
    s.rsplit_once(':').map(|(_, local)| local).unwrap_or(s)
}

/// Extract attribute value as string
pub fn attr_str(attrs: &quick_xml::events::BytesStart<'_>, key: &str) -> Option<String> {
    attrs.attributes().flatten().find_map(|a| {
        let k = local_name(a.key.as_ref());
        if k == key {
            Some(String::from_utf8_lossy(&a.value).to_string())
        } else {
            None
        }
    })
}
