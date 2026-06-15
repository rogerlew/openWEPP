# CQR06 Public API Surface Parity Report

Evidence class: Static

Public crate-visible functions retained in the target module:

- `pub(crate) fn run_lateral_transfer(request: &HillslopeKernelRequest<'_>) -> Result<KernelRunResponse, Wb11HydrologyKernelGuardError>`
- `pub(crate) fn run_drainage(request: &HillslopeKernelRequest<'_>) -> Result<KernelRunResponse, Wb11HydrologyKernelGuardError>`
- `pub(crate) fn wb14_ksatadj_flag(request: &HillslopeKernelRequest<'_>, phase_class: HillslopeKernelPhaseClass) -> Result<bool, Wb11HydrologyKernelGuardError>`
- `pub(crate) fn wb14_load_top_two_layer_ksatadj_metrics(request: &HillslopeKernelRequest<'_>, phase_class: HillslopeKernelPhaseClass) -> Result<(f64, f64, f64), Wb11HydrologyKernelGuardError>`
- `pub(crate) fn resolve_wb14_effective_soil_conductivity(request: &HillslopeKernelRequest<'_>, phase_class: HillslopeKernelPhaseClass, soil_conductivity: f64) -> Result<f64, Wb11HydrologyKernelGuardError>`

Intentional public API changes: none.

New symbols are private module structs or private associated helpers only.
