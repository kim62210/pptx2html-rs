use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum CapabilityStage {
    Parsed,
    Resolved,
    Rendered,
    FidelityTested,
}

impl CapabilityStage {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Parsed => "parsed",
            Self::Resolved => "resolved",
            Self::Rendered => "rendered",
            Self::FidelityTested => "fidelity-tested",
        }
    }
}

impl fmt::Display for CapabilityStage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SupportTier {
    Exact,
    Approximate,
    Fallback,
    Unparsed,
}

impl SupportTier {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Exact => "exact",
            Self::Approximate => "approximate",
            Self::Fallback => "fallback",
            Self::Unparsed => "unparsed",
        }
    }
}

impl fmt::Display for SupportTier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FeatureFamily {
    Shapes,
    Text,
    Tables,
    Images,
    Layout,
    Charts,
    Media,
    Unsupported,
}

impl FeatureFamily {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Shapes => "shapes",
            Self::Text => "text",
            Self::Tables => "tables",
            Self::Images => "images",
            Self::Layout => "layout",
            Self::Charts => "charts",
            Self::Media => "media",
            Self::Unsupported => "unsupported",
        }
    }
}

impl fmt::Display for FeatureFamily {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FeatureCapability {
    pub family: FeatureFamily,
    pub feature: String,
    pub tier: SupportTier,
    pub stage: Option<CapabilityStage>,
    pub notes: Option<String>,
}

impl FeatureCapability {
    pub fn new(
        family: FeatureFamily,
        feature: impl Into<String>,
        tier: SupportTier,
        stage: Option<CapabilityStage>,
    ) -> Result<Self, &'static str> {
        let capability = Self {
            family,
            feature: feature.into(),
            tier,
            stage,
            notes: None,
        };
        capability.validate()?;
        Ok(capability)
    }

    pub fn highest_completed_stage(&self) -> Option<CapabilityStage> {
        self.stage
    }

    pub fn validate(&self) -> Result<(), &'static str> {
        match (self.tier, self.stage) {
            (SupportTier::Unparsed, None) => Ok(()),
            (SupportTier::Unparsed, Some(_)) => {
                Err("unparsed capabilities must not declare a completed stage")
            }
            (SupportTier::Exact, Some(CapabilityStage::FidelityTested)) => Ok(()),
            (SupportTier::Exact, _) => {
                Err("exact capabilities must be fidelity-tested")
            }
            (SupportTier::Approximate | SupportTier::Fallback, Some(_)) => Ok(()),
            (SupportTier::Approximate | SupportTier::Fallback, None) => {
                Err("approximate and fallback capabilities must declare a completed stage")
            }
        }
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct CapabilityMatrix {
    pub features: Vec<FeatureCapability>,
}

impl CapabilityMatrix {
    pub fn new(features: Vec<FeatureCapability>) -> Self {
        Self { features }
    }
}

#[cfg(test)]
mod tests {
    use super::{CapabilityStage, FeatureCapability, FeatureFamily, SupportTier};

    #[test]
    fn capability_support_tier_display_is_stable() {
        assert_eq!(SupportTier::Exact.to_string(), "exact");
        assert_eq!(SupportTier::Approximate.to_string(), "approximate");
        assert_eq!(SupportTier::Fallback.to_string(), "fallback");
        assert_eq!(SupportTier::Unparsed.to_string(), "unparsed");
    }

    #[test]
    fn capability_stage_display_is_stable() {
        assert_eq!(CapabilityStage::Parsed.to_string(), "parsed");
        assert_eq!(CapabilityStage::Resolved.to_string(), "resolved");
        assert_eq!(CapabilityStage::Rendered.to_string(), "rendered");
        assert_eq!(CapabilityStage::FidelityTested.to_string(), "fidelity-tested");
    }

    #[test]
    fn capability_highest_completed_stage_uses_latest_completed_checkpoint() {
        let mut capability = FeatureCapability::new(
            FeatureFamily::Text,
            "paragraph spacing",
            SupportTier::Approximate,
            Some(CapabilityStage::Resolved),
        )
        .expect("capability should be valid");
        capability.notes = Some("Needs PowerPoint-reference verification".to_string());

        assert_eq!(capability.highest_completed_stage(), Some(CapabilityStage::Resolved));
    }

    #[test]
    fn capability_rejects_invalid_tier_stage_pairs() {
        let unparsed = FeatureCapability::new(
            FeatureFamily::Charts,
            "chart renderer",
            SupportTier::Unparsed,
            Some(CapabilityStage::Parsed),
        );
        assert!(unparsed.is_err());

        let exact = FeatureCapability::new(
            FeatureFamily::Shapes,
            "preset shape svg",
            SupportTier::Exact,
            Some(CapabilityStage::Rendered),
        );
        assert!(exact.is_err());
    }
}
