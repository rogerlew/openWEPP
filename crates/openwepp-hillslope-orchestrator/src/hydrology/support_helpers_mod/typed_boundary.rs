#[allow(clippy::wildcard_imports)]
use super::super::*;

impl Wb11HydrologyKernel {
    pub(crate) fn require_state_range(
        phase_class: HillslopeKernelPhaseClass,
        symbol: HillslopeProductionStateSymbol,
        value: f64,
        minimum: Option<f64>,
        maximum: Option<f64>,
    ) -> Result<(), Wb11HydrologyKernelGuardError> {
        Self::require_state_range_with(
            phase_class,
            || BoundarySymbol::from(symbol),
            value,
            minimum,
            maximum,
        )
    }

    // Range guards run on the success path thousands of times per OFE-day;
    // the symbol name is only needed when a check fails. These `_with`
    // variants defer the owned-`BoundarySymbol` construction (a heap `String`
    // plus the registry-audit hook) to the failure branch. Error values are
    // identical to the eager forms.
    pub(crate) fn require_state_range_with(
        phase_class: HillslopeKernelPhaseClass,
        symbol: impl Fn() -> BoundarySymbol,
        value: f64,
        minimum: Option<f64>,
        maximum: Option<f64>,
    ) -> Result<(), Wb11HydrologyKernelGuardError> {
        if let Some(minimum_value) = minimum
            && value < minimum_value - WB11_ZERO_THRESHOLD
        {
            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: symbol(),
                value,
                minimum,
                maximum,
            });
        }
        if let Some(maximum_value) = maximum
            && value > maximum_value + WB11_ZERO_THRESHOLD
        {
            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: symbol(),
                value,
                minimum,
                maximum,
            });
        }
        Ok(())
    }

    pub(crate) fn require_dynamic_state_range_with(
        phase_class: HillslopeKernelPhaseClass,
        symbol: impl Fn() -> BoundarySymbol,
        value: f64,
        minimum: Option<f64>,
        maximum: Option<f64>,
    ) -> Result<(), Wb11HydrologyKernelGuardError> {
        Self::require_state_range_with(phase_class, symbol, value, minimum, maximum)
    }

    pub(crate) fn hourly_symbol(root: &str, hour: usize) -> BoundarySymbol {
        BoundarySymbol::from(format!("{root}_{hour:04}"))
    }

    pub(crate) fn wb18_perc_state_symbol(field: &str, layer_index: usize) -> BoundarySymbol {
        BoundarySymbol::from(format!("wb18_perc_{field}_{layer_index:04}"))
    }

    pub(crate) fn wb19_dg_symbol(layer_index: usize) -> BoundarySymbol {
        BoundarySymbol::from(format!("wb19_dg_{layer_index:04}"))
    }

    pub(crate) fn wb19_thetdr_symbol(layer_index: usize) -> BoundarySymbol {
        BoundarySymbol::from(format!("wb19_thetdr_{layer_index:04}"))
    }

    pub(crate) fn wb19_bulk_density_kg_m3_symbol(layer_index: usize) -> BoundarySymbol {
        BoundarySymbol::from(format!("wb19_bulk_density_kg_m3_{layer_index:04}"))
    }

    pub(crate) fn frost_layer_symbol(root: &str, layer_index: usize) -> BoundarySymbol {
        BoundarySymbol::from(format!("{root}_{layer_index:04}"))
    }

    pub(crate) fn frost_fine_layer_symbol(
        root: &str,
        layer_index: usize,
        fine_index: usize,
    ) -> BoundarySymbol {
        BoundarySymbol::from(format!("{root}_{layer_index:04}_{fine_index:04}"))
    }

    pub(crate) fn unit_conversion_guard_error(
        phase_class: HillslopeKernelPhaseClass,
        symbol: BoundarySymbol,
        error: &openwepp_unit_boundary::BoundaryError,
    ) -> Wb11HydrologyKernelGuardError {
        match error {
            openwepp_unit_boundary::BoundaryError::NonFinite { value, .. } => {
                Wb11HydrologyKernelGuardError::NonFiniteStateSymbol {
                    phase_class,
                    symbol,
                    value: *value,
                }
            }
            openwepp_unit_boundary::BoundaryError::BelowMinimum { value, minimum, .. } => {
                Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                    phase_class,
                    symbol,
                    value: *value,
                    minimum: Some(*minimum),
                    maximum: None,
                }
            }
            openwepp_unit_boundary::BoundaryError::AboveMaximum { value, maximum, .. } => {
                Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                    phase_class,
                    symbol,
                    value: *value,
                    minimum: None,
                    maximum: Some(*maximum),
                }
            }
        }
    }

    // `as f64` and parsing the decimal string both round to nearest, so this
    // is bit-identical to the former string round-trip for every usize,
    // including values above 2^53.
    #[allow(clippy::cast_precision_loss)]
    pub(crate) fn diagnostic_count_to_f64(value: usize) -> f64 {
        value as f64
    }

}
