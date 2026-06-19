use std::collections::BTreeMap;

use openwepp_sim_contract::closure::{
    ClosureViolation, ClosureViolationKind, check_finite, check_max, check_min, check_range,
};
use openwepp_sim_contract::status::{
    BoundaryClass, SimulationPhase, SimulationStatus, StatusError,
};

use crate::lib_mod::core_types::{
    BoundarySymbol, BoundaryValue, IndexedKernelWritebackPayload, IndexedWritebackField,
    IndexedWritebackSurface, KernelWritebackPayload, SymbolRegistry, SymbolRegistryError,
    WritebackDecisionOutcome, WritebackField,
};

/// Outcome surface for writeback evaluation.
#[derive(Debug, Clone, PartialEq)]
pub struct KernelWritebackDecision {
    pub outcome: WritebackDecisionOutcome,
    pub status: SimulationStatus,
    pub violations: Vec<ClosureViolation>,
}

/// Outcome surface for accepted writeback application.
#[derive(Debug, Clone, PartialEq)]
pub struct KernelWritebackApplyResult {
    pub outcome: WritebackDecisionOutcome,
    pub status: SimulationStatus,
    pub applied_state_symbols: Vec<BoundarySymbol>,
    pub applied_flux_symbols: Vec<BoundarySymbol>,
}

/// Writeback-application errors.
#[derive(Debug)]
pub enum WritebackError {
    Status(StatusError),
    DecisionNotAccept { outcome: WritebackDecisionOutcome },
    SymbolRegistry(SymbolRegistryError),
}

impl std::fmt::Display for WritebackError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Status(source) => write!(f, "failed constructing writeback status: {source}"),
            Self::DecisionNotAccept { outcome } => {
                write!(
                    f,
                    "cannot apply writeback for non-accept outcome: {outcome:?}"
                )
            }
            Self::SymbolRegistry(source) => write!(f, "indexed writeback registry error: {source}"),
        }
    }
}

impl std::error::Error for WritebackError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Status(source) => Some(source),
            Self::DecisionNotAccept { .. } => None,
            Self::SymbolRegistry(source) => Some(source),
        }
    }
}

impl From<StatusError> for WritebackError {
    fn from(value: StatusError) -> Self {
        Self::Status(value)
    }
}

impl From<SymbolRegistryError> for WritebackError {
    fn from(value: SymbolRegistryError) -> Self {
        Self::SymbolRegistry(value)
    }
}

/// Evaluate a kernel writeback payload into deterministic accept/reject status.
pub fn evaluate_kernel_writeback(
    phase: SimulationPhase,
    payload: &KernelWritebackPayload,
) -> Result<KernelWritebackDecision, StatusError> {
    let mut violations = Vec::new();

    for field in &payload.state_updates {
        collect_field_violations("state", field, &mut violations);
    }

    for field in &payload.flux_updates {
        collect_field_violations("flux", field, &mut violations);
    }

    if violations.is_empty() {
        let status = SimulationStatus::ok(phase, crate::WRITEBACK_ACCEPT_MESSAGE_ID)?;
        Ok(KernelWritebackDecision {
            outcome: WritebackDecisionOutcome::Accept,
            status,
            violations,
        })
    } else {
        let has_non_finite = violations
            .iter()
            .any(|violation| violation.kind == ClosureViolationKind::NonFinite);

        let status = if has_non_finite {
            SimulationStatus::non_finite_failure(
                phase,
                crate::WRITEBACK_REJECT_NON_FINITE_MESSAGE_ID,
            )?
        } else {
            SimulationStatus::domain_failure(
                phase,
                BoundaryClass::DomainViolation,
                crate::WRITEBACK_REJECT_DOMAIN_MESSAGE_ID,
            )?
        };

        Ok(KernelWritebackDecision {
            outcome: WritebackDecisionOutcome::Reject,
            status,
            violations,
        })
    }
}

/// Evaluate an id-backed kernel writeback payload into deterministic
/// accept/reject status.
pub fn evaluate_indexed_kernel_writeback(
    phase: SimulationPhase,
    payload: &IndexedKernelWritebackPayload,
) -> Result<KernelWritebackDecision, StatusError> {
    let mut violations = Vec::new();

    for field in &payload.state_updates {
        collect_indexed_field_violations("state", field, &mut violations);
    }

    for field in &payload.flux_updates {
        collect_indexed_field_violations("flux", field, &mut violations);
    }

    if violations.is_empty() {
        let status = SimulationStatus::ok(phase, crate::WRITEBACK_ACCEPT_MESSAGE_ID)?;
        Ok(KernelWritebackDecision {
            outcome: WritebackDecisionOutcome::Accept,
            status,
            violations,
        })
    } else {
        let has_non_finite = violations
            .iter()
            .any(|violation| violation.kind == ClosureViolationKind::NonFinite);

        let status = if has_non_finite {
            SimulationStatus::non_finite_failure(
                phase,
                crate::WRITEBACK_REJECT_NON_FINITE_MESSAGE_ID,
            )?
        } else {
            SimulationStatus::domain_failure(
                phase,
                BoundaryClass::DomainViolation,
                crate::WRITEBACK_REJECT_DOMAIN_MESSAGE_ID,
            )?
        };

        Ok(KernelWritebackDecision {
            outcome: WritebackDecisionOutcome::Reject,
            status,
            violations,
        })
    }
}

/// Apply an accepted writeback payload to orchestrator-owned state/flux maps.
pub fn apply_kernel_writeback(
    phase: SimulationPhase,
    decision: &KernelWritebackDecision,
    payload: &KernelWritebackPayload,
    state_surface: &mut BTreeMap<BoundarySymbol, BoundaryValue>,
    flux_surface: &mut BTreeMap<BoundarySymbol, BoundaryValue>,
) -> Result<KernelWritebackApplyResult, WritebackError> {
    if decision.outcome != WritebackDecisionOutcome::Accept {
        return Err(WritebackError::DecisionNotAccept {
            outcome: decision.outcome,
        });
    }

    let mut state_updates: Vec<&WritebackField> = payload.state_updates.iter().collect();
    state_updates.sort_by_key(|field| field.symbol.as_str());

    let mut flux_updates: Vec<&WritebackField> = payload.flux_updates.iter().collect();
    flux_updates.sort_by_key(|field| field.symbol.as_str());

    for field in &state_updates {
        state_surface.insert(field.symbol.clone(), field.value);
    }

    for field in &flux_updates {
        flux_surface.insert(field.symbol.clone(), field.value);
    }

    let status = SimulationStatus::ok(phase, crate::WRITEBACK_APPLY_MESSAGE_ID)?;

    Ok(KernelWritebackApplyResult {
        outcome: WritebackDecisionOutcome::Apply,
        status,
        applied_state_symbols: state_updates
            .iter()
            .map(|field| field.symbol.clone())
            .collect(),
        applied_flux_symbols: flux_updates
            .iter()
            .map(|field| field.symbol.clone())
            .collect(),
    })
}

/// Apply an accepted id-backed writeback payload to indexed authority, then
/// materialize only the updated ids to logical state/flux maps for compatibility
/// with unmigrated downstream phases.
pub fn apply_indexed_kernel_writeback(
    phase: SimulationPhase,
    decision: &KernelWritebackDecision,
    payload: &IndexedKernelWritebackPayload,
    indexed_surface: &mut IndexedWritebackSurface,
    registry: &SymbolRegistry,
    state_surface: &mut BTreeMap<BoundarySymbol, BoundaryValue>,
    flux_surface: &mut BTreeMap<BoundarySymbol, BoundaryValue>,
) -> Result<KernelWritebackApplyResult, WritebackError> {
    if decision.outcome != WritebackDecisionOutcome::Accept {
        return Err(WritebackError::DecisionNotAccept {
            outcome: decision.outcome,
        });
    }

    let mut state_updates: Vec<&IndexedWritebackField> = payload.state_updates.iter().collect();
    state_updates.sort_by_key(|field| field.id);

    let mut flux_updates: Vec<&IndexedWritebackField> = payload.flux_updates.iter().collect();
    flux_updates.sort_by_key(|field| field.id);

    let mut resolved_state_updates = Vec::with_capacity(state_updates.len());
    for field in &state_updates {
        let symbol = registry
            .symbol(field.id)
            .cloned()
            .ok_or(SymbolRegistryError::UnknownSymbolId { id: field.id })?;
        resolved_state_updates.push((field.id, field.value, symbol));
    }

    let mut resolved_flux_updates = Vec::with_capacity(flux_updates.len());
    for field in &flux_updates {
        let symbol = registry
            .symbol(field.id)
            .cloned()
            .ok_or(SymbolRegistryError::UnknownSymbolId { id: field.id })?;
        resolved_flux_updates.push((field.id, field.value, symbol));
    }

    let mut applied_state_symbols = Vec::with_capacity(resolved_state_updates.len());
    for (id, value, symbol) in resolved_state_updates {
        indexed_surface.set_state_value(id, Some(value));
        state_surface.insert(symbol.clone(), value);
        applied_state_symbols.push(symbol);
    }

    let mut applied_flux_symbols = Vec::with_capacity(resolved_flux_updates.len());
    for (id, value, symbol) in resolved_flux_updates {
        indexed_surface.set_flux_value(id, Some(value));
        flux_surface.insert(symbol.clone(), value);
        applied_flux_symbols.push(symbol);
    }

    let status = SimulationStatus::ok(phase, crate::WRITEBACK_APPLY_MESSAGE_ID)?;

    Ok(KernelWritebackApplyResult {
        outcome: WritebackDecisionOutcome::Apply,
        status,
        applied_state_symbols,
        applied_flux_symbols,
    })
}

fn collect_field_violations(
    scope: &str,
    field: &WritebackField,
    output: &mut Vec<ClosureViolation>,
) {
    if field_satisfies_writeback_domain(field) {
        return;
    }

    let subject = format!("{scope}:{}[{}]", field.symbol, field.value.unit_label());
    let value = field.value.as_f64();

    collect_check(
        check_finite(
            "INV-WRITEBACK-001",
            crate::WRITEBACK_REJECT_NON_FINITE_MESSAGE_ID,
            subject.as_str(),
            value,
        ),
        output,
    );

    match (field.minimum, field.maximum) {
        (Some(minimum), Some(maximum)) => collect_check(
            check_range(
                "INV-WRITEBACK-002",
                crate::WRITEBACK_REJECT_DOMAIN_MESSAGE_ID,
                subject.as_str(),
                value,
                minimum,
                maximum,
            ),
            output,
        ),
        (Some(minimum), None) => collect_check(
            check_min(
                "INV-WRITEBACK-003",
                crate::WRITEBACK_REJECT_DOMAIN_MESSAGE_ID,
                subject.as_str(),
                value,
                minimum,
            ),
            output,
        ),
        (None, Some(maximum)) => collect_check(
            check_max(
                "INV-WRITEBACK-004",
                crate::WRITEBACK_REJECT_DOMAIN_MESSAGE_ID,
                subject.as_str(),
                value,
                maximum,
            ),
            output,
        ),
        (None, None) => {}
    }
}

fn collect_indexed_field_violations(
    scope: &str,
    field: &IndexedWritebackField,
    output: &mut Vec<ClosureViolation>,
) {
    if indexed_field_satisfies_writeback_domain(field) {
        return;
    }

    let subject = format!("{scope}:id:{}[{}]", field.id, field.value.unit_label());
    let value = field.value.as_f64();

    collect_check(
        check_finite(
            "INV-WRITEBACK-001",
            crate::WRITEBACK_REJECT_NON_FINITE_MESSAGE_ID,
            subject.as_str(),
            value,
        ),
        output,
    );

    match (field.minimum, field.maximum) {
        (Some(minimum), Some(maximum)) => collect_check(
            check_range(
                "INV-WRITEBACK-002",
                crate::WRITEBACK_REJECT_DOMAIN_MESSAGE_ID,
                subject.as_str(),
                value,
                minimum,
                maximum,
            ),
            output,
        ),
        (Some(minimum), None) => collect_check(
            check_min(
                "INV-WRITEBACK-003",
                crate::WRITEBACK_REJECT_DOMAIN_MESSAGE_ID,
                subject.as_str(),
                value,
                minimum,
            ),
            output,
        ),
        (None, Some(maximum)) => collect_check(
            check_max(
                "INV-WRITEBACK-004",
                crate::WRITEBACK_REJECT_DOMAIN_MESSAGE_ID,
                subject.as_str(),
                value,
                maximum,
            ),
            output,
        ),
        (None, None) => {}
    }
}

fn field_satisfies_writeback_domain(field: &WritebackField) -> bool {
    let value = field.value.as_f64();

    if !value.is_finite() {
        return false;
    }

    match (field.minimum, field.maximum) {
        (Some(minimum), Some(maximum)) => {
            minimum <= maximum && (minimum..=maximum).contains(&value)
        }
        (Some(minimum), None) => value >= minimum,
        (None, Some(maximum)) => value <= maximum,
        (None, None) => true,
    }
}

fn indexed_field_satisfies_writeback_domain(field: &IndexedWritebackField) -> bool {
    let value = field.value.as_f64();

    if !value.is_finite() {
        return false;
    }

    match (field.minimum, field.maximum) {
        (Some(minimum), Some(maximum)) => {
            minimum <= maximum && (minimum..=maximum).contains(&value)
        }
        (Some(minimum), None) => value >= minimum,
        (None, Some(maximum)) => value <= maximum,
        (None, None) => true,
    }
}

fn collect_check(result: Result<(), Box<ClosureViolation>>, output: &mut Vec<ClosureViolation>) {
    if let Err(violation) = result {
        output.push(*violation);
    }
}
