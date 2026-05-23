# PL17 Decomposition Kinetics Authority and Guard Map

Status: `complete`
Evidence mode: `Static`

## Kinetics Component Map

| component | key symbols | implementation surface | guard posture | contract authority |
|---|---|---|---|---|
| Temperature factor | `tmax`, `tmin`, `tmpfac` | `compute_equation_decomposition_seed_surface` | hard-fail on non-finite/out-of-domain derived factor | `SC-RESIDUE-001` PL17 addendum step 2 |
| Water factors | `prcp`, `Ws`, `swatfc`, `fwatfc`, `envinx` | `compute_equation_decomposition_seed_surface` | hard-fail on missing/non-finite forcing; bounded domain checks | `SC-RESIDUE-001` PL17 addendum steps 2-3 |
| Decay factors | `oratea`, `orater`, `exp(-envinx*rate)` | `compute_equation_decomposition_seed_surface` | slot/crop required symbol hard-fail; positive-rate enforcement | `SC-RESIDUE-001` `INV-RESIDUE-017/018`, `SC-PLANT-001` `INV-PLANT-022` |
| Seed pool update | `sumrtm_seed`, `sumsrm_seed` | `decomposition_phase_dispatch_for_state` payload assembly | non-negative/finiteness invariants before and after update | `SC-RESIDUE-001` PL17 addendum step 4 |
| Annual event modifiers | `fbrnog`, `frmove`, `frcut` + annual action selector | `compute_equation_decomposition_seed_surface` | domain-validated fractions; no silent fallback | `SC-RESIDUE-001` PL17 addendum step 5 |
| Perennial grazing modifier | `digest` + active grazing cycle | `compute_equation_decomposition_seed_surface` | active-cycle payload consistency checks and bounded digest domain | `SC-RESIDUE-001` PL17 addendum step 6 |

## Required-Symbol Guard Surface

`require_decomposition_equation_inputs` enforces typed hard-fail guards for:
- climate/water forcing: `tmax`, `tmin`, `prcp`, `Ws`
- projected slot/crop decomposition parameters: `pl_decomp_slot_*_oratea`, `pl_decomp_slot_*_orater`

No silent defaults are used for missing/non-finite/out-of-domain decomposition equation inputs.
