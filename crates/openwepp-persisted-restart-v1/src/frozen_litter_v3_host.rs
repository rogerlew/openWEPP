//! Atomic host installation for admitted frozen-litter V3 checkpoints.

use crate::{
    ExpectedFrozenLitterCheckpointContextV3, FrozenLitterCheckpointAdmissionErrorV3,
    IsolatedRestoredFrozenLitterCheckpointV3, admit_frozen_litter_checkpoint_v3,
};

#[derive(Clone, Debug, PartialEq)]
pub struct DirectFrozenLitterRestartHostV3 {
    admitted: IsolatedRestoredFrozenLitterCheckpointV3,
}

impl DirectFrozenLitterRestartHostV3 {
    #[must_use]
    pub const fn from_isolated(admitted: IsolatedRestoredFrozenLitterCheckpointV3) -> Self {
        Self { admitted }
    }

    #[must_use]
    pub const fn admitted(&self) -> &IsolatedRestoredFrozenLitterCheckpointV3 {
        &self.admitted
    }
}

/// Admission and every nested replay finish before the target is replaced.
/// Any failure therefore leaves the prior host bit-for-bit unchanged.
pub fn admit_and_install_frozen_litter_checkpoint_v3(
    target: &mut DirectFrozenLitterRestartHostV3,
    bytes: &[u8],
    context: &ExpectedFrozenLitterCheckpointContextV3<'_>,
) -> Result<(), FrozenLitterCheckpointAdmissionErrorV3> {
    let admitted = admit_frozen_litter_checkpoint_v3(bytes, context)?;
    *target = DirectFrozenLitterRestartHostV3::from_isolated(admitted);
    Ok(())
}

pub fn install_frozen_litter_checkpoint_v3(
    target: &mut DirectFrozenLitterRestartHostV3,
    candidate: DirectFrozenLitterRestartHostV3,
) {
    *target = candidate;
}
