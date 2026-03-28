//! Placeholder matching algorithm
//!
//! OOXML placeholders are matched by (type, idx) between slide, layout, and master.
//! This module implements the priority-based matching used during inheritance resolution.

use crate::model::{PlaceholderInfo, PlaceholderType, Shape};

/// Which txStyles list to use based on placeholder type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextStyleSource {
    TitleStyle,
    BodyStyle,
    OtherStyle,
}

/// Find matching placeholder in target shapes by type+idx.
///
/// Priority:
///   1. Match both type and idx
///   2. Match type only (idx is None or doesn't match)
///   3. Match idx only (type is None)
pub fn find_matching_placeholder<'a>(
    ph: &PlaceholderInfo,
    candidates: &'a [Shape],
) -> Option<&'a Shape> {
    let mut type_match: Option<&'a Shape> = None;
    let mut idx_match: Option<&'a Shape> = None;

    for shape in candidates {
        let cand_ph = match &shape.placeholder {
            Some(p) => p,
            None => continue,
        };

        let type_eq = match (&ph.ph_type, &cand_ph.ph_type) {
            (Some(a), Some(b)) => ph_type_eq(a, b),
            _ => false,
        };

        let idx_eq = match (ph.idx, cand_ph.idx) {
            (Some(a), Some(b)) => a == b,
            _ => false,
        };

        // Priority 1: both type and idx match
        if type_eq && idx_eq {
            return Some(shape);
        }

        // Priority 2: type matches (record first match)
        if type_eq && type_match.is_none() {
            type_match = Some(shape);
        }

        // Priority 3: idx matches when source type is None (record first match)
        if idx_eq && ph.ph_type.is_none() && idx_match.is_none() {
            idx_match = Some(shape);
        }
    }

    type_match.or(idx_match)
}

/// Determine which txStyles list to use based on placeholder type
pub fn text_style_source(ph_type: Option<&PlaceholderType>) -> TextStyleSource {
    match ph_type {
        Some(PlaceholderType::Title | PlaceholderType::CtrTitle) => TextStyleSource::TitleStyle,
        Some(
            PlaceholderType::Body | PlaceholderType::SubTitle | PlaceholderType::Obj,
        ) => TextStyleSource::BodyStyle,
        _ => TextStyleSource::OtherStyle,
    }
}

/// Compare two PlaceholderType values for equality
fn ph_type_eq(a: &PlaceholderType, b: &PlaceholderType) -> bool {
    std::mem::discriminant(a) == std::mem::discriminant(b)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{PlaceholderInfo, PlaceholderType, Shape};

    fn make_shape_with_ph(ph_type: Option<PlaceholderType>, idx: Option<u32>) -> Shape {
        Shape {
            placeholder: Some(PlaceholderInfo { ph_type, idx }),
            ..Default::default()
        }
    }

    fn make_shape_no_ph() -> Shape {
        Shape::default()
    }

    #[test]
    fn match_both_type_and_idx() {
        let candidates = vec![
            make_shape_with_ph(Some(PlaceholderType::Title), Some(0)),
            make_shape_with_ph(Some(PlaceholderType::Body), Some(1)),
        ];
        let ph = PlaceholderInfo {
            ph_type: Some(PlaceholderType::Body),
            idx: Some(1),
        };
        let result = find_matching_placeholder(&ph, &candidates);
        assert!(result.is_some());
        let matched = result.unwrap();
        let mp = matched.placeholder.as_ref().unwrap();
        assert!(matches!(mp.ph_type, Some(PlaceholderType::Body)));
        assert_eq!(mp.idx, Some(1));
    }

    #[test]
    fn match_type_only_when_idx_differs() {
        let candidates = vec![
            make_shape_with_ph(Some(PlaceholderType::Title), Some(0)),
            make_shape_with_ph(Some(PlaceholderType::Body), Some(5)),
        ];
        let ph = PlaceholderInfo {
            ph_type: Some(PlaceholderType::Body),
            idx: Some(99),
        };
        let result = find_matching_placeholder(&ph, &candidates);
        assert!(result.is_some());
        let mp = result.unwrap().placeholder.as_ref().unwrap();
        assert!(matches!(mp.ph_type, Some(PlaceholderType::Body)));
    }

    #[test]
    fn match_idx_only_when_type_is_none() {
        let candidates = vec![
            make_shape_with_ph(Some(PlaceholderType::Title), Some(0)),
            make_shape_with_ph(Some(PlaceholderType::Body), Some(3)),
        ];
        let ph = PlaceholderInfo {
            ph_type: None,
            idx: Some(3),
        };
        let result = find_matching_placeholder(&ph, &candidates);
        assert!(result.is_some());
        assert_eq!(result.unwrap().placeholder.as_ref().unwrap().idx, Some(3));
    }

    #[test]
    fn no_match_returns_none() {
        let candidates = vec![
            make_shape_with_ph(Some(PlaceholderType::Title), Some(0)),
        ];
        let ph = PlaceholderInfo {
            ph_type: Some(PlaceholderType::Body),
            idx: Some(99),
        };
        assert!(find_matching_placeholder(&ph, &candidates).is_none());
    }

    #[test]
    fn skip_shapes_without_placeholder() {
        let candidates = vec![make_shape_no_ph()];
        let ph = PlaceholderInfo {
            ph_type: Some(PlaceholderType::Title),
            idx: Some(0),
        };
        assert!(find_matching_placeholder(&ph, &candidates).is_none());
    }

    #[test]
    fn priority_1_over_priority_2() {
        let candidates = vec![
            make_shape_with_ph(Some(PlaceholderType::Body), Some(10)),
            make_shape_with_ph(Some(PlaceholderType::Body), Some(1)),
        ];
        let ph = PlaceholderInfo {
            ph_type: Some(PlaceholderType::Body),
            idx: Some(1),
        };
        let result = find_matching_placeholder(&ph, &candidates).unwrap();
        assert_eq!(result.placeholder.as_ref().unwrap().idx, Some(1));
    }

    #[test]
    fn text_style_source_title() {
        assert_eq!(
            text_style_source(Some(&PlaceholderType::Title)),
            TextStyleSource::TitleStyle
        );
        assert_eq!(
            text_style_source(Some(&PlaceholderType::CtrTitle)),
            TextStyleSource::TitleStyle
        );
    }

    #[test]
    fn text_style_source_body() {
        assert_eq!(
            text_style_source(Some(&PlaceholderType::Body)),
            TextStyleSource::BodyStyle
        );
        assert_eq!(
            text_style_source(Some(&PlaceholderType::SubTitle)),
            TextStyleSource::BodyStyle
        );
        assert_eq!(
            text_style_source(Some(&PlaceholderType::Obj)),
            TextStyleSource::BodyStyle
        );
    }

    #[test]
    fn text_style_source_other() {
        assert_eq!(
            text_style_source(Some(&PlaceholderType::Dt)),
            TextStyleSource::OtherStyle
        );
        assert_eq!(text_style_source(None), TextStyleSource::OtherStyle);
    }
}
