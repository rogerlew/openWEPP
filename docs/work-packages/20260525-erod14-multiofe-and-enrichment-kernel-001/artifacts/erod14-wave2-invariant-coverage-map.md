# Erod14 wave2 invariant coverage map

Status: completed
Evidence mode: mixed

## Static
- `INV-SED-008` downslope-variability and multi-OFE case semantics:
  - runtime case classifier enforces case 1..4 branch predicates from `erod14_case`, `Qj-1`, `Vj`, `Qj`, `Fh`, `Fp`.
  - covered by `erod14_contract_vector_rejects_case_classification_mismatch` and case-four vector.
- `INV-SED-009` class-wise enrichment mass-conservation:
  - runtime computes `gend_i`, `sedmax_i`, reproportions unconstrained classes, and rejects no-feasible-reproportion states.
  - covered by nominal vector assertions (`gend_i <= sedmax_i`) and unreproportionable-mass failure vector.
- Typed guard family coverage:
  - missing symbol -> `HKERNEL-EROD14-WAVE2-E-001`,
  - non-finite symbol -> `HKERNEL-EROD14-WAVE2-E-002`,
  - domain/closure violation -> `HKERNEL-EROD14-WAVE2-E-003`.

## Ran
- `cargo test --test erod14_wave2_multiofe_enrichment_kernel_contract` passed all 6 vectors.
