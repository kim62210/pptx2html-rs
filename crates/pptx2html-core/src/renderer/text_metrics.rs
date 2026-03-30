#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FontResolutionSource {
    ExplicitRun,
    ParagraphDefaults,
    InheritedDefaults,
    FontRef,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FontResolutionEntry {
    pub slide_index: usize,
    pub shape_name: Option<String>,
    pub run_text: String,
    pub requested_typeface: Option<String>,
    pub resolved_typeface: Option<String>,
    pub source: Option<FontResolutionSource>,
    pub fallback_used: bool,
}
