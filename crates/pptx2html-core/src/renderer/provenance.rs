#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProvenanceSource {
    Slide,
    SlideListStyle,
    LayoutPlaceholder,
    MasterPlaceholder,
    LayoutBackground,
    MasterBackground,
    LayoutListStyle,
    MasterListStyle,
    MasterTextStyles,
    DefaultTextStyle,
    StyleRef,
    HardcodedDefault,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProvenanceSubject {
    SlideBackground,
    Shape,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RenderedProvenanceEntry {
    pub slide_index: usize,
    pub subject: ProvenanceSubject,
    pub shape_name: Option<String>,
    pub fill_source: Option<ProvenanceSource>,
    pub border_source: Option<ProvenanceSource>,
    pub text_source: Option<ProvenanceSource>,
    pub background_source: Option<ProvenanceSource>,
}
