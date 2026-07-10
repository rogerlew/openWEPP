# Coverage Closure

Evidence label: Static/Ran.

Status: `EXECUTED`

ADR-0021 coverage closure is required because characterization tests were added.

Tier assignment: `science-sensitive diagnostic/runtime`.

Rationale: `laned_shadow.rs` carries Lane D runtime diagnostic shadow behavior
and cites `SC-OFEROUTE-001#INV-OFEROUTE-012`.

Targeted coverage source:

- LCOV: `/tmp/openwepp-cqr-nightly-10-runner-laned-shadow-targeted.lcov`
- JSON:
  `/tmp/openwepp-cqr-nightly-10-runner-laned-shadow-targeted-llvmcov.json`

Final target coverage:

- Lines: `684/699` (`97.85407725321888%`)
- Regions: `842/877` (`96.00912200684151%`)
- Functions: `47/52` (`90.38461538461539%`)
- Instantiations: `47/52` (`90.38461538461539%`)
- Branches: `0/0`

ADR-0021 eligible production surface split at `#[cfg(test)]`
(`laned_shadow.rs:578`):

- Production lines: `321/330` (`97.27272727272728%`)
- Production regions: `406/437` (`92.90617848970251%`)
- Test lines: `348/349` (`99.7134670487106%`)
- Test regions: `430/434` (`99.07834101382488%`)

Threshold status:

- Science-tier target line threshold: PASS (`>=90%`).
- Science-tier target region threshold: PASS (`>=90%`).
- Science-tier production line threshold: PASS (`>=90%`).
- Science-tier production region threshold: PASS (`>=90%`).
- Target CRAP threshold: PASS; every deduplicated target row is `<=30`.

Per-function floor notes:

- The originally above-threshold rows now have cargo-crap function coverage
  above the per-function floor: `observe_row` `95.58823529411765%`,
  `validate_lane_day_operands` `100.0%`, and `commit_day` `100.0%`.
- The threshold-adjacent `finalize` row is now `84.61538461538461%`.
- `build_day_rate_series` is `100.0%`.
- `build_cascade_segments` is `100.0%`.
- Remaining uncovered llvm-cov function instantiations are closure/test
  monomorphizations or diagnostic/environment surfaces with CRAP `<=2.0`; the
  concrete diagnostic helpers are now directly covered by
  `diagnostic_profile_helpers_cover_opt_in_surfaces_without_public_outputs`.
- No `// COVERAGE-EXCLUDE` justification is needed for this package.

## Obligation-to-Test Binding

ADR-0021 obligation binding is required because this package materially added
characterization tests for a science-sensitive Lane D diagnostic runtime
module. The target file owns diagnostic-shadow collection, routing-source
reconstruction, dynamic operand validation, profile-slot accounting, and
summary finalization. It does not own Lane D active/default selector policy,
coefficient materialization, direct-publication producer construction, public
output serialization, or H2637 fixture content.

| Obligation or invariant | Behavior surface | Current known tests | Status |
|---|---|---|---|
| `SC-OFEROUTE-001#INV-OFEROUTE-010` | active/default fallback isolation and active/shadow mutual exclusion | Existing `tests/integration/laned_shadow_h2637.rs` selector tests; package did not alter selector code | Bound by existing integration surface |
| `SC-OFEROUTE-001#INV-OFEROUTE-012` | Lane D shadow diagnostic seam and routed source reconstruction | `positive_uniform_shape_day_routes_and_classifies_routed_melt_source`, `positive_uniform_shape_day_without_routed_melt_classifies_lump_only_source`, `day_change_commits_zero_source_day_and_finalize_commits_tail_day`, existing H2637 shadow tests | Bound for target-owned collector behavior |
| Lane D shadow dynamic friction operand sourcing | finite/non-negative rainfall, routed melt, LAI, and canopy height operands | `validate_lane_day_operands_rejects_invalid_dynamic_inputs`, existing source guard `laned_shadow_consumes_live_dynamic_friction_operands`, existing dynamic operand module tests | Bound for target-owned validation and source-guard posture |
| Lane D shadow dynamic operand completeness | missing buffered dynamic operands fail closed before cascade/rate construction | `missing_buffered_operands_fail_closed_before_cascade` | Bound for target-owned fail-closed handoff behavior |
| Protected output identity | shadow diagnostics do not alter HBP/parquet protected outputs | Existing ignored H2637 native-shadow fixture test; package made no production changes | No new output risk; retained as higher-cost oracle |
| Diagnostic profile surfaces | opt-in profile accounting/reporting does not affect manifest or public outputs | `diagnostic_profile_helpers_cover_opt_in_surfaces_without_public_outputs`, existing `runner_profile_slots_accumulate_for_routed_day` | Bound for target-owned profile helper behavior |

Out-of-scope obligation disposition:

- Coefficient authority and management YAML materialization remain owned by
  direct-publication/management packages and integration tests, not this
  collector-only CQR package.
- Lane D active routing production closure, default activation, and HBP/public
  consumer paths remain outside this behavior-preserving test package.
- No new contract authority, public output surface, selector behavior, parser
  projection, or serialization path was added.
