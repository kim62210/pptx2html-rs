use crate::model::{TextParagraph, TextRun};

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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextWrapPolicy {
    Normal,
    Emergency,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScriptCategory {
    LatinLike,
    EastAsian,
    Complex,
    Emoji,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScriptSegment {
    pub category: ScriptCategory,
    pub text: String,
}

pub fn classify_wrap_policy(
    paragraphs: &[TextParagraph],
    available_width_px: f64,
    font_scale: Option<f64>,
) -> TextWrapPolicy {
    let max_token_width_px = paragraphs
        .iter()
        .map(|para| longest_unbreakable_span_width_px(&para.runs, font_scale))
        .fold(0.0, f64::max);

    if max_token_width_px > available_width_px.max(1.0) {
        TextWrapPolicy::Emergency
    } else {
        TextWrapPolicy::Normal
    }
}

fn longest_unbreakable_span_width_px(runs: &[TextRun], font_scale: Option<f64>) -> f64 {
    let mut max_width_px: f64 = 0.0;
    let mut current_width_px: f64 = 0.0;

    for run in runs {
        if run.is_break {
            max_width_px = max_width_px.max(current_width_px);
            current_width_px = 0.0;
            continue;
        }

        let font_size_pt = run.style.font_size.unwrap_or(18.0) * font_scale.unwrap_or(1.0);
        let font_size_px = font_size_pt * (96.0 / 72.0);

        for ch in run.text.chars() {
            if ch.is_whitespace() {
                max_width_px = max_width_px.max(current_width_px);
                current_width_px = 0.0;
                continue;
            }

            let glyph_width_px = estimated_glyph_em_width(ch) * font_size_px;

            if is_east_asian_char(ch) {
                max_width_px = max_width_px.max(current_width_px);
                current_width_px = 0.0;
                max_width_px = max_width_px.max(glyph_width_px);
                continue;
            }

            current_width_px += glyph_width_px;
        }
    }

    max_width_px.max(current_width_px)
}

fn estimated_glyph_em_width(ch: char) -> f64 {
    match ch {
        '\u{4E00}'..='\u{9FFF}'
        | '\u{3400}'..='\u{4DBF}'
        | '\u{3040}'..='\u{30FF}'
        | '\u{AC00}'..='\u{D7A3}' => 1.0,
        '\u{0590}'..='\u{05FF}' | '\u{0600}'..='\u{06FF}' | '\u{0750}'..='\u{077F}' => 0.75,
        'A'..='Z' | '0'..='9' => 0.62,
        'a'..='z' => 0.56,
        '-' | '_' | '/' | '\\' | '.' | ',' | ':' | ';' => 0.35,
        _ if ch.is_whitespace() => 0.0,
        _ => 0.6,
    }
}

pub fn classify_script_category(text: &str) -> ScriptCategory {
    if text.chars().any(is_complex_script_char) {
        ScriptCategory::Complex
    } else if text.chars().any(is_emoji_char) {
        ScriptCategory::Emoji
    } else if text.chars().any(is_east_asian_char) {
        ScriptCategory::EastAsian
    } else {
        ScriptCategory::LatinLike
    }
}

pub fn segment_by_script(text: &str) -> Vec<ScriptSegment> {
    let mut segments = Vec::new();
    let mut current_category: Option<ScriptCategory> = None;
    let mut current_text = String::new();
    let mut keep_with_current = false;

    for ch in text.chars() {
        if ch.is_whitespace() || is_cluster_joiner(ch) {
            current_text.push(ch);
            if is_cluster_joiner(ch) {
                keep_with_current = true;
            }
            continue;
        }

        let category = if is_emoji_char(ch) {
            ScriptCategory::Emoji
        } else if is_complex_script_char(ch) {
            ScriptCategory::Complex
        } else if is_east_asian_char(ch) {
            ScriptCategory::EastAsian
        } else {
            ScriptCategory::LatinLike
        };

        if keep_with_current || current_category == Some(category) || current_category.is_none() {
            if current_category.is_none() {
                current_category = Some(category);
            }
            current_text.push(ch);
            keep_with_current = false;
            continue;
        }

        if let Some(prev_category) = current_category {
            segments.push(ScriptSegment {
                category: prev_category,
                text: std::mem::take(&mut current_text),
            });
        }
        current_category = Some(category);
        current_text.push(ch);
        keep_with_current = false;
    }

    if let Some(category) = current_category {
        segments.push(ScriptSegment {
            category,
            text: current_text,
        });
    } else if !text.is_empty() {
        segments.push(ScriptSegment {
            category: ScriptCategory::LatinLike,
            text: text.to_string(),
        });
    }

    segments
}

fn is_cluster_joiner(ch: char) -> bool {
    matches!(
        ch,
        '\u{200D}'
            | '\u{FE0E}'
            | '\u{FE0F}'
            | '\u{0300}'..='\u{036F}'
            | '\u{1AB0}'..='\u{1AFF}'
            | '\u{1DC0}'..='\u{1DFF}'
            | '\u{20D0}'..='\u{20FF}'
            | '\u{FE20}'..='\u{FE2F}'
            | '\u{093C}'
            | '\u{094D}'
            | '\u{09BC}'
            | '\u{0A3C}'
            | '\u{0ABC}'
            | '\u{0B3C}'
            | '\u{0CBC}'
            | '\u{0D4D}'
            | '\u{0E31}'
            | '\u{0E34}'..='\u{0E3A}'
            | '\u{0E47}'..='\u{0E4E}'
            | '\u{0EB1}'
            | '\u{0EC8}'..='\u{0ECD}'
    )
}

fn is_emoji_char(ch: char) -> bool {
    matches!(
        ch,
        '\u{1F300}'..='\u{1FAFF}'
            | '\u{2600}'..='\u{26FF}'
            | '\u{2700}'..='\u{27BF}'
    )
}

fn is_east_asian_char(ch: char) -> bool {
    matches!(
        ch,
        '\u{4E00}'..='\u{9FFF}'
            | '\u{3400}'..='\u{4DBF}'
            | '\u{3040}'..='\u{30FF}'
            | '\u{AC00}'..='\u{D7A3}'
    )
}

fn is_complex_script_char(ch: char) -> bool {
    matches!(
        ch,
        '\u{0590}'..='\u{05FF}'
            | '\u{0600}'..='\u{06FF}'
            | '\u{0750}'..='\u{077F}'
            | '\u{0900}'..='\u{097F}'
            | '\u{0980}'..='\u{09FF}'
            | '\u{0A00}'..='\u{0A7F}'
            | '\u{0A80}'..='\u{0AFF}'
            | '\u{0B00}'..='\u{0B7F}'
            | '\u{0B80}'..='\u{0BFF}'
            | '\u{0C00}'..='\u{0C7F}'
            | '\u{0C80}'..='\u{0CFF}'
            | '\u{0D00}'..='\u{0D7F}'
            | '\u{0D80}'..='\u{0DFF}'
            | '\u{0E00}'..='\u{0E7F}'
            | '\u{0E80}'..='\u{0EFF}'
            | '\u{0F00}'..='\u{0FFF}'
            | '\u{1000}'..='\u{109F}'
            | '\u{1780}'..='\u{17FF}'
            | '\u{08A0}'..='\u{08FF}'
            | '\u{FB50}'..='\u{FDFF}'
            | '\u{FE70}'..='\u{FEFF}'
    )
}

#[cfg(test)]
mod tests {
    use crate::model::{FontStyle, TextParagraph, TextRun, TextStyle};

    use super::{
        classify_script_category, classify_wrap_policy, segment_by_script, ScriptCategory,
        TextWrapPolicy,
    };

    #[test]
    fn classify_wrap_policy_keeps_regular_sentence_normal() {
        let paragraphs = vec![TextParagraph {
            runs: vec![TextRun {
                text: "This sentence should wrap at spaces without emergency breaks".into(),
                style: TextStyle {
                    font_size: Some(18.0),
                    ..Default::default()
                },
                font: FontStyle::default(),
                hyperlink: None,
                is_break: false,
            }],
            ..Default::default()
        }];

        assert_eq!(
            classify_wrap_policy(&paragraphs, 220.0, None),
            TextWrapPolicy::Normal
        );
    }

    #[test]
    fn classify_wrap_policy_marks_long_unbreakable_token_as_emergency() {
        let paragraphs = vec![TextParagraph {
            runs: vec![TextRun {
                text: "SupercalifragilisticexpialidociousWithoutSpaces".into(),
                style: TextStyle {
                    font_size: Some(18.0),
                    ..Default::default()
                },
                font: FontStyle::default(),
                hyperlink: None,
                is_break: false,
            }],
            ..Default::default()
        }];

        assert_eq!(
            classify_wrap_policy(&paragraphs, 120.0, None),
            TextWrapPolicy::Emergency
        );
    }

    #[test]
    fn classify_wrap_policy_marks_split_mixed_font_token_as_emergency() {
        let paragraphs = vec![TextParagraph {
            runs: vec![
                TextRun {
                    text: "overflow".into(),
                    style: TextStyle {
                        font_size: Some(18.0),
                        ..Default::default()
                    },
                    font: FontStyle::default(),
                    hyperlink: None,
                    is_break: false,
                },
                TextRun {
                    text: "detector".into(),
                    style: TextStyle {
                        font_size: Some(18.0),
                        ..Default::default()
                    },
                    font: FontStyle::default(),
                    hyperlink: None,
                    is_break: false,
                },
            ],
            ..Default::default()
        }];

        assert_eq!(
            classify_wrap_policy(&paragraphs, 160.0, None),
            TextWrapPolicy::Emergency
        );
    }

    #[test]
    fn classify_script_category_detects_complex_script_text() {
        assert_eq!(
            classify_script_category("مرحبا بالعالم"),
            ScriptCategory::Complex
        );
    }

    #[test]
    fn classify_script_category_detects_indic_text_as_complex() {
        assert_eq!(
            classify_script_category("नमस्ते दुनिया"),
            ScriptCategory::Complex
        );
    }

    #[test]
    fn classify_script_category_detects_emoji_text() {
        assert_eq!(classify_script_category("👩‍💻"), ScriptCategory::Emoji);
    }

    #[test]
    fn segment_by_script_splits_latin_and_complex_runs() {
        let segments = segment_by_script("Hello مرحبا world");
        assert_eq!(segments.len(), 3);
        assert_eq!(segments[0].category, ScriptCategory::LatinLike);
        assert_eq!(segments[1].category, ScriptCategory::Complex);
        assert_eq!(segments[2].category, ScriptCategory::LatinLike);
    }

    #[test]
    fn segment_by_script_splits_latin_and_indic_runs() {
        let segments = segment_by_script("Hello नमस्ते world");
        assert_eq!(segments.len(), 3);
        assert_eq!(segments[0].category, ScriptCategory::LatinLike);
        assert_eq!(segments[1].category, ScriptCategory::Complex);
        assert_eq!(segments[2].category, ScriptCategory::LatinLike);
    }

    #[test]
    fn segment_by_script_keeps_emoji_zwj_cluster_together() {
        let segments = segment_by_script("Hello 👩‍💻 world");
        assert_eq!(segments.len(), 3);
        assert_eq!(segments[0].text, "Hello ");
        assert!(segments[1].text.starts_with("👩‍💻"));
        assert!(segments[2].text.ends_with("world"));
    }

    #[test]
    fn segment_by_script_keeps_combining_mark_with_base_character() {
        let segments = segment_by_script("Hello क़ world");
        let complex_segment = segments
            .iter()
            .find(|segment| segment.category == ScriptCategory::Complex)
            .expect("complex segment");
        assert!(complex_segment.text.starts_with("क़"));
    }

    #[test]
    fn segment_by_script_splits_latin_and_emoji_runs() {
        let segments = segment_by_script("A👩‍💻B");
        assert_eq!(segments.len(), 3);
        assert_eq!(segments[0].category, ScriptCategory::LatinLike);
        assert_eq!(segments[1].category, ScriptCategory::Emoji);
        assert_eq!(segments[1].text, "👩‍💻");
        assert_eq!(segments[2].category, ScriptCategory::LatinLike);
    }

    #[test]
    fn classify_wrap_policy_keeps_cjk_sentence_normal() {
        let paragraphs = vec![TextParagraph {
            runs: vec![TextRun {
                text: "자동줄바꿈이가능한한글문장은긴토큰처럼취급되면안됩니다".into(),
                style: TextStyle {
                    font_size: Some(18.0),
                    ..Default::default()
                },
                font: FontStyle::default(),
                hyperlink: None,
                is_break: false,
            }],
            ..Default::default()
        }];

        assert_eq!(
            classify_wrap_policy(&paragraphs, 90.0, Some(0.7)),
            TextWrapPolicy::Normal
        );
    }
}
