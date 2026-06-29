use std::path::PathBuf;

use crate::policy::SidecarPolicy;

#[derive(Debug, Clone)]
pub struct RunnerLaunchRequest {
    pub hillslope_binary: PathBuf,
    pub run_dir: PathBuf,
    pub run_file: PathBuf,
    pub output_dir: PathBuf,
    pub sidecar_policy: SidecarPolicy,
    pub legacy_sidecar_discovery: bool,
    pub manifest_path: Option<PathBuf>,
}

#[derive(Debug, Clone)]
pub struct HillslopeRunRequest {
    pub run_dir: PathBuf,
    pub run_file: PathBuf,
    pub output_dir: PathBuf,
    pub sidecar_policy: SidecarPolicy,
    pub legacy_sidecar_discovery: bool,
    pub manifest_path: Option<PathBuf>,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum HillslopeRuntimeSelection {
    #[default]
    DefaultCandidate,
    Compatibility,
    DirectSkeletonNoop,
    DirectSkeletonShadowOnly,
    DirectPublicationFrameShadow,
    DirectPublicationFrameCutover,
    DirectProductionExecutor,
}

impl HillslopeRuntimeSelection {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DefaultCandidate => "default-candidate",
            Self::Compatibility => "compatibility",
            Self::DirectSkeletonNoop => "direct-skeleton-noop",
            Self::DirectSkeletonShadowOnly => "direct-skeleton-shadow-only",
            Self::DirectPublicationFrameShadow => "direct-publication-frame-shadow",
            Self::DirectPublicationFrameCutover => "direct-publication-frame-cutover",
            Self::DirectProductionExecutor => "direct-production-executor",
        }
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum HillslopeDefaultRuntimeActivation {
    Disabled,
    #[default]
    DirectProductionCandidate,
}

impl HillslopeDefaultRuntimeActivation {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Disabled => "disabled",
            Self::DirectProductionCandidate => "direct-production-candidate",
        }
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct HillslopeRuntimeSelectionPolicy {
    requested: HillslopeRuntimeSelection,
    default_activation: HillslopeDefaultRuntimeActivation,
}

impl HillslopeRuntimeSelectionPolicy {
    #[must_use]
    pub const fn new(
        requested: HillslopeRuntimeSelection,
        default_activation: HillslopeDefaultRuntimeActivation,
    ) -> Self {
        Self {
            requested,
            default_activation,
        }
    }

    #[must_use]
    pub const fn requested(self) -> HillslopeRuntimeSelection {
        self.requested
    }

    #[must_use]
    pub const fn default_activation(self) -> HillslopeDefaultRuntimeActivation {
        self.default_activation
    }

    #[must_use]
    pub const fn resolve(self) -> HillslopeRuntimeSelectionResolution {
        match (self.requested, self.default_activation) {
            (
                HillslopeRuntimeSelection::DefaultCandidate,
                HillslopeDefaultRuntimeActivation::DirectProductionCandidate,
            ) => HillslopeRuntimeSelectionResolution {
                requested: self.requested,
                selected: HillslopeRuntimeSelection::DirectProductionExecutor,
                default_activation: self.default_activation,
                selection_reason: "default-candidate-activation-gate-direct-production",
                fallback_reason: None,
            },
            (
                HillslopeRuntimeSelection::DefaultCandidate,
                HillslopeDefaultRuntimeActivation::Disabled,
            ) => HillslopeRuntimeSelectionResolution {
                requested: self.requested,
                selected: HillslopeRuntimeSelection::Compatibility,
                default_activation: self.default_activation,
                selection_reason: "default-candidate-disabled-deprecated-compatibility-selection",
                fallback_reason: Some("direct-default-candidate-gate-disabled"),
            },
            (HillslopeRuntimeSelection::Compatibility, _) => HillslopeRuntimeSelectionResolution {
                requested: self.requested,
                selected: HillslopeRuntimeSelection::Compatibility,
                default_activation: self.default_activation,
                selection_reason: "explicit-deprecated-compatibility-selection",
                fallback_reason: None,
            },
            (HillslopeRuntimeSelection::DirectProductionExecutor, _) => {
                HillslopeRuntimeSelectionResolution {
                    requested: self.requested,
                    selected: HillslopeRuntimeSelection::DirectProductionExecutor,
                    default_activation: self.default_activation,
                    selection_reason: "explicit-direct-production",
                    fallback_reason: None,
                }
            }
            (selection, _) => HillslopeRuntimeSelectionResolution {
                requested: self.requested,
                selected: selection,
                default_activation: self.default_activation,
                selection_reason: "explicit-nondefault-runtime",
                fallback_reason: None,
            },
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HillslopeRuntimeSelectionResolution {
    requested: HillslopeRuntimeSelection,
    selected: HillslopeRuntimeSelection,
    default_activation: HillslopeDefaultRuntimeActivation,
    selection_reason: &'static str,
    fallback_reason: Option<&'static str>,
}

impl HillslopeRuntimeSelectionResolution {
    #[must_use]
    pub const fn requested(self) -> HillslopeRuntimeSelection {
        self.requested
    }

    #[must_use]
    pub const fn selected(self) -> HillslopeRuntimeSelection {
        self.selected
    }

    #[must_use]
    pub const fn default_activation(self) -> HillslopeDefaultRuntimeActivation {
        self.default_activation
    }

    #[must_use]
    pub const fn selection_reason(self) -> &'static str {
        self.selection_reason
    }

    #[must_use]
    pub const fn fallback_reason(self) -> Option<&'static str> {
        self.fallback_reason
    }
}

#[derive(Debug, Clone)]
pub struct HillslopeRunReport {
    pub output_pass: PathBuf,
    pub output_loss: PathBuf,
    pub optional_outputs: Vec<PathBuf>,
    pub manifest_path: PathBuf,
    pub sidecar_warnings: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct ReleaseLintReport {
    pub checked_binaries: Vec<PathBuf>,
}
