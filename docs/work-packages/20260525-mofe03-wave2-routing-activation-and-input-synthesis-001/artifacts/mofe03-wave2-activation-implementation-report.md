# MOFE03 Wave2 Activation Implementation Report

Status: complete
Evidence mode: static+ran
Date: 2026-05-25

## Static
Implemented production Wave-2 activation and ingress seeding in:
- `crates/openwepp-runner/src/hillslope/mod.rs`

Key behavior implemented:
- Runner now seeds MOFE03 surfaces from `seed_wb11_runtime_surface_inputs(...)` via `seed_mofe03_wave2_runtime_surface_inputs(...)`.
- Activation policy:
  - when `erod14_wave2_enabled` is present, enforce binary `0|1` typed validation;
  - when absent, derive default from topology: enabled if `nelem > 1`, disabled otherwise.
- Enabled path now deterministically seeds required EROD14 Wave-2 ingress families with typed guards:
  - core scalars (`erod14_qout`, `erod14_qin`, `erod14_qostar`, `erod14_slplen`, `erod14_class_count`, coefficient seeds),
  - case-state scalars (`erod14_case`, `erod14_Qj_minus_1`, `erod14_Vj`, `erod14_Qj`, `erod14_Fh`, `erod14_Fp`, `theta`, `erod14_beta`),
  - class-indexed symbols (`erod14_fall_*`, `erod14_frcflw_*`, `erod14_frac_*`, `erod14_fidel_*`, `erod14_tcf1_*`, `erod14_ssa_class_*`).
- Execution provenance now records Wave-2 evidence surfaces:
  - `kernel_phase_message_ids`,
  - `erod14_wave2_enabled`,
  - `erod14_wave2_kernel_status_seen`.

Mechanical modularization applied during remediation:
- split Wave-2 seeding into focused helpers (`resolve_*`, `seed_*`, and case-scalar builder routines) to satisfy lint posture without changing runtime intent.

## Ran
- `cargo test -p openwepp --test cli03_runner_contract_derived_tests mofe03 -- --nocapture`
- `cargo test --workspace`
