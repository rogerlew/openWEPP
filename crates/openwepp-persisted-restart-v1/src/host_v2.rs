//! Atomic isolated host for admitted V2 checkpoints.

use crate::{
    ExpectedRestartStaticContextV2, IsolatedRestoredCheckpointV2, RestartAdmissionFailureV2,
    admit_checkpoint_v2,
};

/// Orchestrator-independent atomic host. Runtime installation is intentionally
/// a later integration step because the production shadow does not yet expose
/// a V2 soil-owner installation API.
#[derive(Clone, Debug, PartialEq)]
pub struct DirectV10RestartHostV2 {
    admitted: IsolatedRestoredCheckpointV2,
}

impl DirectV10RestartHostV2 {
    #[must_use]
    pub const fn from_isolated(admitted: IsolatedRestoredCheckpointV2) -> Self {
        Self { admitted }
    }

    #[must_use]
    pub const fn admitted(&self) -> &IsolatedRestoredCheckpointV2 {
        &self.admitted
    }
}

pub fn admit_and_install_checkpoint_v2(
    target: &mut DirectV10RestartHostV2,
    bytes: &[u8],
    context: &ExpectedRestartStaticContextV2<'_>,
) -> Result<(), RestartAdmissionFailureV2> {
    let admitted = admit_checkpoint_v2(bytes, context)?;
    *target = DirectV10RestartHostV2::from_isolated(admitted);
    Ok(())
}
