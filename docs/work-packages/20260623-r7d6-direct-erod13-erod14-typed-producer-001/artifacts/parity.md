# Parity Evidence

Status: executed-held.

## Focused Fixtures

- Ran: `r7d5_erosion_active_publication_fails_closed_without_direct_sediment_producer`
  passed. The guard still fails closed when active erosion publication lacks
  direct producer authority.
- Ran: `r7d6_typed_erosion_producer_populates_publication_operands` passed.
  The fixture proves typed direct EROD13/EROD14/EROD15 publication with
  nonzero detachment and class concentrations, direct HBP aliases, and
  `compatibility_edge_invocation_count = 0`.
- Ran: `cargo test -p openwepp-runner direct_production --lib` passed. The
  production direct executor still excludes compatibility entrypoints and
  runs without compatibility edges after the R7D6 span-count changes.

## H2637

- Ran: fresh compatibility output:
  `/tmp/r7d4-h2637-5day/manifests/r7d6-compat-current/` exited `0`.
- Ran: direct output:
  `/tmp/r7d4-h2637-5day/manifests/direct-r7d6-zero-lddend/` exited `0`.
- Direct runtime counters on the direct manifest:
  `phase_span_runs = 2509`, `direct_phase_entries = 4791`,
  `direct_compute_operations = 2623`, `direct_state_mutations = 2731`,
  `downstream_operand_productions = 2617`, `shadow_projections = 2509`,
  and `compatibility_edge_invocations = 0`.
- WAT byte identity holds:
  `/tmp/r7d4-h2637-5day/default/H2637.wat.parquet` equals
  `/tmp/r7d4-h2637-5day/direct/H2637.wat.parquet`.
- PASS sediment parity is clean after changing the MOFE03 default
  `erod14_lddend` seed from fabricated `0.3` to `0.0`: `tdet`, `tdep`, and
  `sedcon_1..5` have no row differences in the `pyarrow` comparison.

## Residuals

- `HOLD-R7D6-PASS-HBP-PEAKRO-COMPATIBILITY-ZERO-RESIDUAL`: PASS/HBP still
  differ because direct publication emits typed direct WB16 `peakro` values
  while compatibility PASS rows publish `0.0`.
- PASS residual values:
  - row 1: compatibility `0.0`, direct `3.591689245524811e-06`
  - row 2: compatibility `0.0`, direct `4.837293745180717e-07`
  - row 3: compatibility `0.0`, direct `9.939800459642262e-07`
  - row 4: compatibility `0.0`, direct `4.726157673358129e-07`
  - row 5: compatibility `0.0`, direct `3.63e-08`
  - row 6: compatibility `0.0`, direct `3.63e-08`
- R7D6 must not force direct `peakro` back to compatibility zero because
  `SC-HYDRAULICS-001` and `SC-SED-001` require WB16 peak-duration authority
  for active erosion coupling. R7D7 must adjudicate the publication authority:
  either compatibility output is missing WB16 peak publication and must be
  corrected, or direct publication needs a contract-approved serialization
  policy that preserves byte identity without discarding typed WB16 state.
