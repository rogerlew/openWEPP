//! Default-off V11 vegetation consumer over admitted coupled-time slabs.
//! Binding: `SC-VEGETATION-001`, `SC-VEGETATIONTRANSACTION-001`, and
//! `SC-COUPLEDTIME-001`.
//!
//! The concrete stack implementation lives beside the existing `DirectV10`
//! owner because that owner has custody of the V10→V9→V8 projection, LSE,
//! hydrology, BGC, energy and thermal candidates. This boundary only joins
//! that actual stack to the vegetation V11 transaction protocol.

use openwepp_vegetation::v11::{
    V11ConstitutiveExecutor, V11ImportedV10SegmentInput, V11ImportedV10SegmentOutput,
};

/// Actual orchestrator-owned imported constitutive stack.
///
/// Implementations clone/stage the complete owner set, execute the unchanged
/// `DirectV10` projection and covered-owner chain with the authenticated slab
/// duration bits, and return no live mutation on error.
pub trait DirectV11ImportedStack {
    type Error;

    fn execute_imported_v10_stack(
        &mut self,
        input: &V11ImportedV10SegmentInput,
    ) -> Result<V11ImportedV10SegmentOutput, Self::Error>;
}

/// Explicit default-off adapter; no production selector references this type.
pub struct DirectV11VegetationExecutor<S> {
    pub stack: S,
}

impl<S: DirectV11ImportedStack> V11ConstitutiveExecutor for DirectV11VegetationExecutor<S> {
    type Error = S::Error;

    fn execute_v10_segment(
        &mut self,
        input: &V11ImportedV10SegmentInput,
    ) -> Result<V11ImportedV10SegmentOutput, Self::Error> {
        self.stack.execute_imported_v10_stack(input)
    }
}
