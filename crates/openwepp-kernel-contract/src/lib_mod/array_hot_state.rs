use std::collections::BTreeMap;
use std::fmt;

use openwepp_sim_contract::closure::{
    ClosureViolation, ClosureViolationKind, check_finite, check_max, check_min, check_range,
};
use openwepp_sim_contract::status::{
    BoundaryClass, SimulationPhase, SimulationStatus, StatusError,
};

use crate::lib_mod::core_types::{
    BoundarySurfacePair, BoundarySymbol, BoundaryValue, KernelWritebackPayload, SymbolId,
    SymbolRegistry, SymbolRegistryError, WritebackDecisionOutcome,
};
use crate::lib_mod::writeback::KernelWritebackDecision;

/// Dense array-authoritative state and flux slots keyed by [`SymbolId`].
#[derive(Debug, Clone, PartialEq)]
pub struct ArrayHotState {
    state_slots: Vec<Option<BoundaryValue>>,
    flux_slots: Vec<Option<BoundaryValue>>,
}

impl ArrayHotState {
    /// Build an empty dense state sized for the frozen registry.
    #[must_use]
    pub fn empty_for_registry(registry: &SymbolRegistry) -> Self {
        Self {
            state_slots: vec![None; registry.len()],
            flux_slots: vec![None; registry.len()],
        }
    }

    /// Build dense state from logical state and flux surfaces.
    ///
    /// # Errors
    ///
    /// Returns [`SymbolRegistryError::UnknownSymbol`] when any logical surface
    /// key is outside the frozen registry.
    pub fn from_btreemap_surfaces(
        registry: &SymbolRegistry,
        state_surface: &BTreeMap<BoundarySymbol, BoundaryValue>,
        flux_surface: &BTreeMap<BoundarySymbol, BoundaryValue>,
    ) -> Result<Self, SymbolRegistryError> {
        let mut state = Self::empty_for_registry(registry);
        for (symbol, value) in state_surface {
            let id = registry.id_of(symbol)?;
            state.set_state_value(id, Some(*value))?;
        }
        for (symbol, value) in flux_surface {
            let id = registry.id_of(symbol)?;
            state.set_flux_value(id, Some(*value))?;
        }
        Ok(state)
    }

    /// Return the dense state slot count.
    #[must_use]
    pub fn slot_count(&self) -> usize {
        self.state_slots.len()
    }

    /// Lookup a state value by dense id.
    #[must_use]
    pub fn state_value(&self, id: SymbolId) -> Option<BoundaryValue> {
        self.state_slots.get(id.as_usize()).copied().flatten()
    }

    /// Lookup a flux value by dense id.
    #[must_use]
    pub fn flux_value(&self, id: SymbolId) -> Option<BoundaryValue> {
        self.flux_slots.get(id.as_usize()).copied().flatten()
    }

    /// Insert, update, or remove a state value by dense id.
    ///
    /// # Errors
    ///
    /// Returns [`SymbolRegistryError::UnknownSymbolId`] when `id` is outside
    /// this dense state's registry-sized slot range.
    pub fn set_state_value(
        &mut self,
        id: SymbolId,
        value: Option<BoundaryValue>,
    ) -> Result<(), SymbolRegistryError> {
        let Some(slot) = self.state_slots.get_mut(id.as_usize()) else {
            return Err(SymbolRegistryError::UnknownSymbolId { id });
        };
        *slot = value;
        Ok(())
    }

    /// Insert, update, or remove a flux value by dense id.
    ///
    /// # Errors
    ///
    /// Returns [`SymbolRegistryError::UnknownSymbolId`] when `id` is outside
    /// this dense state's registry-sized slot range.
    pub fn set_flux_value(
        &mut self,
        id: SymbolId,
        value: Option<BoundaryValue>,
    ) -> Result<(), SymbolRegistryError> {
        let Some(slot) = self.flux_slots.get_mut(id.as_usize()) else {
            return Err(SymbolRegistryError::UnknownSymbolId { id });
        };
        *slot = value;
        Ok(())
    }

    /// Export dense state and flux slots to logical `BTreeMap` surfaces.
    #[must_use]
    pub fn export_btreemap_surfaces(&self, registry: &SymbolRegistry) -> BoundarySurfacePair {
        (
            export_slots(registry, &self.state_slots),
            export_slots(registry, &self.flux_slots),
        )
    }
}

fn export_slots(
    registry: &SymbolRegistry,
    slots: &[Option<BoundaryValue>],
) -> BTreeMap<BoundarySymbol, BoundaryValue> {
    let mut output = BTreeMap::new();
    for (id, symbol) in registry.iter() {
        let Some(value) = slots.get(id.as_usize()).copied().flatten() else {
            continue;
        };
        output.insert(symbol.clone(), value);
    }
    output
}

/// Dense surface class for id-backed writeback fields.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ArraySurfaceKind {
    State,
    Flux,
}

impl ArraySurfaceKind {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::State => "state",
            Self::Flux => "flux",
        }
    }
}

/// One id-backed scalar writeback field proposed by an array-authoritative path.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ArrayWritebackField {
    pub id: SymbolId,
    pub value: BoundaryValue,
    pub minimum: Option<f64>,
    pub maximum: Option<f64>,
}

impl ArrayWritebackField {
    #[must_use]
    pub const fn unbounded(id: SymbolId, value: BoundaryValue) -> Self {
        Self {
            id,
            value,
            minimum: None,
            maximum: None,
        }
    }

    #[must_use]
    pub const fn bounded(
        id: SymbolId,
        value: BoundaryValue,
        minimum: Option<f64>,
        maximum: Option<f64>,
    ) -> Self {
        Self {
            id,
            value,
            minimum,
            maximum,
        }
    }
}

/// Id-backed writeback payload for array-authoritative execution.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct ArrayWritebackPayload {
    pub state_updates: Vec<ArrayWritebackField>,
    pub flux_updates: Vec<ArrayWritebackField>,
}

impl ArrayWritebackPayload {
    #[must_use]
    pub fn empty() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn with_updates(
        mut state_updates: Vec<ArrayWritebackField>,
        mut flux_updates: Vec<ArrayWritebackField>,
    ) -> Self {
        state_updates.sort_by_key(|field| field.id);
        flux_updates.sort_by_key(|field| field.id);
        Self {
            state_updates,
            flux_updates,
        }
    }

    /// Resolve a logical writeback payload to dense ids once.
    ///
    /// # Errors
    ///
    /// Returns [`SymbolRegistryError::UnknownSymbol`] when any logical field
    /// names a symbol outside the frozen registry.
    pub fn from_logical_payload(
        registry: &SymbolRegistry,
        payload: &KernelWritebackPayload,
    ) -> Result<Self, SymbolRegistryError> {
        let state_updates = resolve_logical_fields(registry, &payload.state_updates)?;
        let flux_updates = resolve_logical_fields(registry, &payload.flux_updates)?;
        Ok(Self::with_updates(state_updates, flux_updates))
    }
}

fn resolve_logical_fields(
    registry: &SymbolRegistry,
    fields: &[crate::lib_mod::core_types::WritebackField],
) -> Result<Vec<ArrayWritebackField>, SymbolRegistryError> {
    fields
        .iter()
        .map(|field| {
            Ok(ArrayWritebackField {
                id: registry.id_of(&field.symbol)?,
                value: field.value,
                minimum: field.minimum,
                maximum: field.maximum,
            })
        })
        .collect()
}

/// Outcome surface for accepted array-authoritative writeback application.
#[derive(Debug, Clone, PartialEq)]
pub struct ArrayWritebackApplyResult {
    pub outcome: WritebackDecisionOutcome,
    pub status: SimulationStatus,
    pub applied_state_ids: Vec<SymbolId>,
    pub applied_flux_ids: Vec<SymbolId>,
}

/// Array writeback application errors.
#[derive(Debug)]
pub enum ArrayWritebackError {
    Status(StatusError),
    DecisionNotAccept { outcome: WritebackDecisionOutcome },
    SymbolRegistry(SymbolRegistryError),
}

impl fmt::Display for ArrayWritebackError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Status(source) => {
                write!(f, "failed constructing array writeback status: {source}")
            }
            Self::DecisionNotAccept { outcome } => {
                write!(
                    f,
                    "cannot apply array writeback for non-accept outcome: {outcome:?}"
                )
            }
            Self::SymbolRegistry(source) => {
                write!(
                    f,
                    "array writeback referenced an invalid symbol id: {source}"
                )
            }
        }
    }
}

impl std::error::Error for ArrayWritebackError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Status(source) => Some(source),
            Self::DecisionNotAccept { .. } => None,
            Self::SymbolRegistry(source) => Some(source),
        }
    }
}

impl From<StatusError> for ArrayWritebackError {
    fn from(value: StatusError) -> Self {
        Self::Status(value)
    }
}

impl From<SymbolRegistryError> for ArrayWritebackError {
    fn from(value: SymbolRegistryError) -> Self {
        Self::SymbolRegistry(value)
    }
}

/// Evaluate an id-backed writeback payload into deterministic status.
///
/// Logical symbol names are resolved only when a violation is present, keeping
/// the success path id-only.
pub fn evaluate_array_writeback(
    phase: SimulationPhase,
    registry: &SymbolRegistry,
    payload: &ArrayWritebackPayload,
) -> Result<KernelWritebackDecision, StatusError> {
    let mut violations = Vec::new();

    for field in &payload.state_updates {
        collect_array_field_violations(registry, ArraySurfaceKind::State, field, &mut violations);
    }

    for field in &payload.flux_updates {
        collect_array_field_violations(registry, ArraySurfaceKind::Flux, field, &mut violations);
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

/// Apply an accepted id-backed payload to dense array state.
///
/// # Errors
///
/// Returns [`ArrayWritebackError::DecisionNotAccept`] when `decision` is not an
/// accept decision, or [`ArrayWritebackError::SymbolRegistry`] when a field id
/// is outside the dense state slot range.
pub fn apply_array_writeback(
    phase: SimulationPhase,
    decision: &KernelWritebackDecision,
    payload: &ArrayWritebackPayload,
    state: &mut ArrayHotState,
) -> Result<ArrayWritebackApplyResult, ArrayWritebackError> {
    if decision.outcome != WritebackDecisionOutcome::Accept {
        return Err(ArrayWritebackError::DecisionNotAccept {
            outcome: decision.outcome,
        });
    }

    for field in &payload.state_updates {
        state.set_state_value(field.id, Some(field.value))?;
    }

    for field in &payload.flux_updates {
        state.set_flux_value(field.id, Some(field.value))?;
    }

    let status = SimulationStatus::ok(phase, crate::WRITEBACK_APPLY_MESSAGE_ID)?;

    Ok(ArrayWritebackApplyResult {
        outcome: WritebackDecisionOutcome::Apply,
        status,
        applied_state_ids: payload.state_updates.iter().map(|field| field.id).collect(),
        applied_flux_ids: payload.flux_updates.iter().map(|field| field.id).collect(),
    })
}

fn collect_array_field_violations(
    registry: &SymbolRegistry,
    surface: ArraySurfaceKind,
    field: &ArrayWritebackField,
    output: &mut Vec<ClosureViolation>,
) {
    if field_satisfies_array_writeback_domain(field) {
        return;
    }

    let subject = lazy_array_subject(registry, surface, field);
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

fn lazy_array_subject(
    registry: &SymbolRegistry,
    surface: ArraySurfaceKind,
    field: &ArrayWritebackField,
) -> String {
    let symbol = registry
        .symbol(field.id)
        .map_or("<unknown>", BoundarySymbol::as_str);
    format!(
        "{}:{symbol}[{}]",
        surface.as_str(),
        field.value.unit_label()
    )
}

fn field_satisfies_array_writeback_domain(field: &ArrayWritebackField) -> bool {
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

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use openwepp_sim_contract::status::{SimulationPhase, StatusClassification};

    use super::*;
    use crate::lib_mod::core_types::{
        KernelWritebackPayload, WritebackField, begin_symbol_registry_audit,
        finish_symbol_registry_audit,
    };
    use crate::lib_mod::writeback::{apply_kernel_writeback, evaluate_kernel_writeback};

    #[test]
    fn array_hot_state_round_trips_logical_surfaces() {
        let (state_surface, flux_surface, registry) = sample_surfaces();
        let hot_state =
            ArrayHotState::from_btreemap_surfaces(&registry, &state_surface, &flux_surface)
                .expect("array hot state should build");

        let (exported_state, exported_flux) = hot_state.export_btreemap_surfaces(&registry);

        assert_eq!(exported_state, state_surface);
        assert_eq!(exported_flux, flux_surface);
    }

    #[test]
    fn array_writeback_accept_matches_logical_writeback() {
        let (_, _, registry) = sample_surfaces();
        let logical_payload = sample_payload();
        let array_payload =
            ArrayWritebackPayload::from_logical_payload(&registry, &logical_payload)
                .expect("payload should resolve");

        begin_symbol_registry_audit(registry.clone()).expect("audit should begin");
        let array_decision =
            evaluate_array_writeback(SimulationPhase::HillslopeKernel, &registry, &array_payload)
                .expect("array decision should construct");
        let audit = finish_symbol_registry_audit().expect("audit should finish");

        let logical_decision =
            evaluate_kernel_writeback(SimulationPhase::HillslopeKernel, &logical_payload)
                .expect("logical decision should construct");

        assert_eq!(array_decision.outcome, logical_decision.outcome);
        assert_eq!(
            array_decision.status.message_id(),
            logical_decision.status.message_id()
        );
        assert_eq!(
            array_decision.status.classification(),
            StatusClassification::Nominal
        );
        assert!(array_decision.violations.is_empty());
        assert_eq!(audit.constructed_symbol_count(), 0);
    }

    #[test]
    fn array_writeback_reject_matches_logical_message_class_and_subject() {
        let (_, _, registry) = sample_surfaces();
        let logical_payload = KernelWritebackPayload::with_updates(
            vec![WritebackField::bounded(
                "storage",
                BoundaryValue::scalar(f64::NAN),
                Some(0.0),
                None,
            )],
            vec![WritebackField::bounded(
                "runoff",
                BoundaryValue::scalar(3.0),
                Some(0.0),
                Some(2.0),
            )],
        );
        let array_payload =
            ArrayWritebackPayload::from_logical_payload(&registry, &logical_payload)
                .expect("payload should resolve");

        begin_symbol_registry_audit(registry.clone()).expect("audit should begin");
        let array_decision =
            evaluate_array_writeback(SimulationPhase::HillslopeKernel, &registry, &array_payload)
                .expect("array decision should construct");
        let audit = finish_symbol_registry_audit().expect("audit should finish");

        let logical_decision =
            evaluate_kernel_writeback(SimulationPhase::HillslopeKernel, &logical_payload)
                .expect("logical decision should construct");

        assert_eq!(array_decision.outcome, logical_decision.outcome);
        assert_eq!(
            array_decision.status.message_id(),
            logical_decision.status.message_id()
        );
        assert_eq!(
            array_decision.status.message_id(),
            crate::WRITEBACK_REJECT_NON_FINITE_MESSAGE_ID
        );
        assert_eq!(
            array_decision.violations.len(),
            logical_decision.violations.len()
        );
        assert_eq!(
            array_decision.violations[0].subject,
            logical_decision.violations[0].subject
        );
        assert_eq!(audit.constructed_symbol_count(), 0);
    }

    #[test]
    fn array_writeback_apply_exports_same_maps_as_logical_apply() {
        let (state_surface, flux_surface, registry) = sample_surfaces();
        let logical_payload = sample_payload();
        let array_payload =
            ArrayWritebackPayload::from_logical_payload(&registry, &logical_payload)
                .expect("payload should resolve");

        let logical_decision =
            evaluate_kernel_writeback(SimulationPhase::HillslopeKernel, &logical_payload)
                .expect("logical decision should construct");
        let mut logical_state = state_surface.clone();
        let mut logical_flux = flux_surface.clone();
        apply_kernel_writeback(
            SimulationPhase::HillslopeKernel,
            &logical_decision,
            &logical_payload,
            &mut logical_state,
            &mut logical_flux,
        )
        .expect("logical apply should succeed");

        let array_decision =
            evaluate_array_writeback(SimulationPhase::HillslopeKernel, &registry, &array_payload)
                .expect("array decision should construct");
        let mut hot_state =
            ArrayHotState::from_btreemap_surfaces(&registry, &state_surface, &flux_surface)
                .expect("array hot state should build");
        let apply_result = apply_array_writeback(
            SimulationPhase::HillslopeKernel,
            &array_decision,
            &array_payload,
            &mut hot_state,
        )
        .expect("array apply should succeed");

        let (array_state, array_flux) = hot_state.export_btreemap_surfaces(&registry);

        assert_eq!(apply_result.outcome, WritebackDecisionOutcome::Apply);
        assert_eq!(array_state, logical_state);
        assert_eq!(array_flux, logical_flux);
    }

    fn sample_surfaces() -> (
        BTreeMap<BoundarySymbol, BoundaryValue>,
        BTreeMap<BoundarySymbol, BoundaryValue>,
        SymbolRegistry,
    ) {
        let mut state_surface = BTreeMap::new();
        state_surface.insert(BoundarySymbol::from("rainfall"), BoundaryValue::scalar(1.0));
        state_surface.insert(BoundarySymbol::from("storage"), BoundaryValue::scalar(2.0));
        let mut flux_surface = BTreeMap::new();
        flux_surface.insert(BoundarySymbol::from("runoff"), BoundaryValue::scalar(0.5));
        let registry = SymbolRegistry::from_surfaces(&state_surface, &flux_surface)
            .expect("registry should build");
        (state_surface, flux_surface, registry)
    }

    fn sample_payload() -> KernelWritebackPayload {
        KernelWritebackPayload::with_updates(
            vec![
                WritebackField::bounded("storage", BoundaryValue::scalar(3.0), Some(0.0), None),
                WritebackField::bounded("rainfall", BoundaryValue::scalar(1.5), Some(0.0), None),
            ],
            vec![WritebackField::bounded(
                "runoff",
                BoundaryValue::scalar(0.75),
                Some(0.0),
                Some(2.0),
            )],
        )
    }
}
