//! Default-off V11 vegetation consumer over admitted coupled-time slabs.
//! Binding: `SC-VEGETATION-001`, `SC-VEGETATIONTRANSACTION-001`, and
//! `SC-COUPLEDTIME-001`.
//!
//! The concrete stack implementation lives beside the existing `DirectV10`
//! owner because that owner has custody of the V10→V9→V8 projection, LSE,
//! hydrology, BGC, energy and thermal candidates. This boundary only joins
//! that actual stack to the vegetation V11 transaction protocol.

use std::collections::BTreeMap;

use openwepp_coupled_time::AcceptedSlabReceiptV1;
use openwepp_vegetation::v11::{
    V11ConstitutiveExecutor, V11ExecutionError, V11ImportedV10SegmentInput,
    V11ImportedV10SegmentOutput, V11ParentTransaction, VegetationConfigurationV11,
};

pub(crate) fn execute_direct_v11_segment<
    S: DirectV11ImportedStack<Error = DirectV11RealConsumerError> + DirectV11BgcScopeProvider,
>(
    configuration: &openwepp_vegetation::v11::VegetationConfigurationV11,
    parent: &V11ParentTransaction,
    receipt: &openwepp_coupled_time::AcceptedSlabReceiptV1,
    executor: &mut DirectV11VegetationExecutor<S>,
) -> Result<
    openwepp_vegetation::v11::V11AcceptedSegmentCandidate,
    V11ExecutionError<DirectV11RealConsumerError>,
> {
    let scope = executor
        .stack
        .v11_bgc_debit_scope(&configuration.imported_v10)
        .map_err(V11ExecutionError::Executor)?;
    openwepp_vegetation::v11::execute_v11_segment_with_bgc_scope(
        configuration,
        parent,
        receipt,
        Some(&scope),
        executor,
    )
}

#[cfg(test)]
pub(crate) fn execute_direct_v11_segment_with_post_bgc_fault<
    S: DirectV11ImportedStack<Error = DirectV11RealConsumerError> + DirectV11BgcScopeProvider,
>(
    configuration: &openwepp_vegetation::v11::VegetationConfigurationV11,
    parent: &V11ParentTransaction,
    receipt: &openwepp_coupled_time::AcceptedSlabReceiptV1,
    executor: &mut DirectV11VegetationExecutor<S>,
) -> Result<
    openwepp_vegetation::v11::V11AcceptedSegmentCandidate,
    V11ExecutionError<DirectV11RealConsumerError>,
> {
    let candidate = execute_direct_v11_segment(configuration, parent, receipt, executor)?;
    if candidate
        .shared_resource_transitions
        .iter()
        .any(|transition| transition.shared_resource_key.owner_id == "bgc")
    {
        let _ = executor.stack.take_staged_ending();
        return Err(V11ExecutionError::Executor(
            DirectV11RealConsumerError::Identity("injected post-BGC-transition fault"),
        ));
    }
    Ok(candidate)
}

pub(crate) fn accept_direct_v11_segment(
    parent: &mut V11ParentTransaction,
    configuration: &openwepp_vegetation::v11::VegetationConfigurationV11,
    candidate: openwepp_vegetation::v11::V11AcceptedSegmentCandidate,
    beginning: &DirectV10RealConsumerShadow,
) -> Result<(), openwepp_vegetation::v11::V11Error> {
    let scope = crate::v9_real_consumer_shadow::direct_v11_bgc_debit_scope(
        &configuration.imported_v10,
        beginning.lse_configuration(),
    )
    .map_err(|_| openwepp_vegetation::v11::V11Error::ResourceDebit)?;
    parent.accept_segment_with_bgc_scope(configuration, candidate, Some(&scope))
}
use thiserror::Error;

use crate::snow_stage3_terminal_handoff::{
    CompleteOwnerSet, SnowStage3HandoffError, SnowStage3OwnerExecutionReceipt,
    SnowStage3OwnerExecutor, SnowStage3TerminalHandoffRequest, locate_terminal_event,
};
use crate::v9_real_consumer_shadow::{
    DirectV10RealConsumerShadow, DirectV11RealConsumerError, DirectV11RealConsumerStack,
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

pub(crate) trait DirectV11BgcScopeProvider {
    fn v11_bgc_debit_scope(
        &self,
        vegetation_configuration: &openwepp_vegetation::VegetationConfiguration,
    ) -> Result<openwepp_vegetation::v11::V11BgcDebitScope, DirectV11RealConsumerError>;

    #[cfg(test)]
    fn take_staged_ending(&mut self) -> Option<DirectV10RealConsumerShadow>;
}

impl DirectV11BgcScopeProvider for DirectV11RealConsumerStack<'_> {
    fn v11_bgc_debit_scope(
        &self,
        vegetation_configuration: &openwepp_vegetation::VegetationConfiguration,
    ) -> Result<openwepp_vegetation::v11::V11BgcDebitScope, DirectV11RealConsumerError> {
        crate::v9_real_consumer_shadow::direct_v11_bgc_debit_scope(
            vegetation_configuration,
            self.beginning.lse_configuration(),
        )
    }

    #[cfg(test)]
    fn take_staged_ending(&mut self) -> Option<DirectV10RealConsumerShadow> {
        DirectV11RealConsumerStack::take_staged_ending(self)
    }
}

impl DirectV11BgcScopeProvider
    for crate::v9_real_consumer_shadow::DirectV11SnowCoveredRealConsumerStack<'_>
{
    fn v11_bgc_debit_scope(
        &self,
        vegetation_configuration: &openwepp_vegetation::VegetationConfiguration,
    ) -> Result<openwepp_vegetation::v11::V11BgcDebitScope, DirectV11RealConsumerError> {
        crate::v9_real_consumer_shadow::direct_v11_bgc_debit_scope(
            vegetation_configuration,
            self.beginning.lse_configuration(),
        )
    }

    #[cfg(test)]
    fn take_staged_ending(&mut self) -> Option<DirectV10RealConsumerShadow> {
        Self::take_staged_ending(self)
    }
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

#[derive(Debug, Error)]
pub enum DirectV11OwnerExecutionError {
    #[error(transparent)]
    V11(#[from] V11ExecutionError<DirectV11RealConsumerError>),
    #[error(transparent)]
    V11Authority(#[from] openwepp_vegetation::v11::V11Error),
    #[error(transparent)]
    Runtime(#[from] DirectV11RealConsumerError),
    #[error(transparent)]
    Handoff(#[from] SnowStage3HandoffError),
    #[error("V11 owner execution invariant failed: {0}")]
    Invariant(&'static str),
}

#[derive(Clone)]
struct PendingDirectV11OwnerExecution {
    parent: V11ParentTransaction,
    ending_shadow: DirectV10RealConsumerShadow,
}

/// Concrete Child 2C owner executor over the existing typed V11/LSE/BGC/
/// soil-thermal stack. It stages the V11 segment and its six non-vegetation
/// owner candidates first; the scheduler calls `commit_owner_execution` only
/// after its day-frame and handoff runtime candidates have committed.
#[derive(Clone)]
pub struct DirectV11SnowStage3OwnerExecutor<'a> {
    configuration: VegetationConfigurationV11,
    parent: V11ParentTransaction,
    accepted_slab: AcceptedSlabReceiptV1,
    stack: Option<DirectV11RealConsumerStack<'a>>,
    pending: Option<PendingDirectV11OwnerExecution>,
    committed_shadow: Option<DirectV10RealConsumerShadow>,
}

impl<'a> DirectV11SnowStage3OwnerExecutor<'a> {
    #[must_use]
    pub fn new(
        configuration: VegetationConfigurationV11,
        parent: V11ParentTransaction,
        accepted_slab: AcceptedSlabReceiptV1,
        stack: DirectV11RealConsumerStack<'a>,
    ) -> Self {
        Self {
            configuration,
            parent,
            accepted_slab,
            stack: Some(stack),
            pending: None,
            committed_shadow: None,
        }
    }

    #[must_use]
    pub fn committed_shadow(&self) -> Option<&DirectV10RealConsumerShadow> {
        self.committed_shadow.as_ref()
    }

    fn ending_owner_set(
        owners: &BTreeMap<String, openwepp_vegetation::v11::V11OwnerEnvelope>,
    ) -> Result<CompleteOwnerSet, DirectV11OwnerExecutionError> {
        CompleteOwnerSet::new(
            owners
                .iter()
                .map(|(owner_id, owner)| (owner_id.clone(), owner.state_bytes.clone()))
                .collect(),
        )
        .map_err(Into::into)
    }
}

impl SnowStage3OwnerExecutor for DirectV11SnowStage3OwnerExecutor<'_> {
    type Error = DirectV11OwnerExecutionError;

    fn stage_owner_execution(
        &mut self,
        request: &SnowStage3TerminalHandoffRequest,
    ) -> Result<SnowStage3OwnerExecutionReceipt, Self::Error> {
        if self.pending.is_some() || self.committed_shadow.is_some() {
            return Err(DirectV11OwnerExecutionError::Invariant(
                "owner executor already has a staged or committed candidate",
            ));
        }
        let event = locate_terminal_event(&request.event)?;
        let accepted_tick =
            event
                .accepted_event_tick
                .ok_or(DirectV11OwnerExecutionError::Invariant(
                    "terminal event did not select a tick",
                ))?;
        let support = self.accepted_slab.support();
        if support.start_ns() != accepted_tick
            || support.end_ns() != request.event.parent_end_tick
            || request.continuation.duration_ns.get() != support.duration_ns()
            || request.continuation.duration_ns.get() == 0
        {
            return Err(DirectV11OwnerExecutionError::Invariant(
                "typed owner slab is not the exact nonzero snow-free remainder",
            ));
        }
        let stack = self
            .stack
            .take()
            .ok_or(DirectV11OwnerExecutionError::Invariant(
                "typed owner stack is unavailable",
            ))?;
        let mut executor = DirectV11VegetationExecutor { stack };
        let acceptance_beginning = executor.stack.beginning.clone();
        let segment = execute_direct_v11_segment(
            &self.configuration,
            &self.parent,
            &self.accepted_slab,
            &mut executor,
        )?;
        let ending_shadow =
            executor
                .stack
                .take_staged_ending()
                .ok_or(DirectV11OwnerExecutionError::Invariant(
                    "typed owner stack did not return a staged ending",
                ))?;
        let mut parent = self.parent.clone();
        accept_direct_v11_segment(
            &mut parent,
            &self.configuration,
            segment.clone(),
            &acceptance_beginning,
        )?;
        let ending_owners = Self::ending_owner_set(&segment.ending_resource_owners)?;
        let receipt = SnowStage3OwnerExecutionReceipt::from_owner_set(
            "direct-v11-real-consumer-stack",
            ending_owners,
        )?;
        self.pending = Some(PendingDirectV11OwnerExecution {
            parent,
            ending_shadow,
        });
        Ok(receipt)
    }

    fn commit_owner_execution(&mut self) -> Result<(), Self::Error> {
        let pending = self
            .pending
            .take()
            .ok_or(DirectV11OwnerExecutionError::Invariant(
                "no typed owner candidate is staged",
            ))?;
        self.parent = pending.parent;
        self.committed_shadow = Some(pending.ending_shadow);
        Ok(())
    }
}
