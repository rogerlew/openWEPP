# Coverage Closure

Evidence label: Static/Ran.

Status: `EXECUTED`

ADR-0021 coverage closure is required because characterization tests were added.

Tier assignment: `kernel/science-sensitive`.

Targeted coverage source:

- LCOV: `/tmp/openwepp-cqr-nightly-09-watershed-detachment-targeted.lcov`
- JSON: `/tmp/openwepp-cqr-nightly-09-watershed-detachment-targeted-llvmcov.json`

Target module:
`crates/openwepp-watershed-orchestrator/src/lib_mod/kernel/routing/01_ws22_ws23_ws26_detachment.rs`

Final target coverage:

- Lines: `1331/1373` (`96.94100509832484%`)
- Regions: `1348/1399` (`96.35453895639743%`)
- Functions: `45/45` (`100.0%`)
- Instantiations: `46/73` (`63.013698630136986%`)
- Branches: `0/0`

Threshold status:

- Science-tier target line threshold: PASS (`>=90%`).
- Science-tier target region threshold: PASS (`>=90%`).
- Target CRAP threshold: PASS; every deduplicated target row is `<=30`.

Per-function floor notes:

- Covered llvm-cov function instantiations in the target file have no
  region-floor rows below `75%`.
- Cargo-crap line-coverage proxy rows at or near the floor:
  `ws26_dcap_expanding_width_outcome` is `75.40983606557377%` with CRAP
  `10.204395962657667`, and `ws26_dcap_width_step` is `75.0%` with CRAP
  `4.25`.
- No `// COVERAGE-EXCLUDE` justification is needed for this package.

Disposition: accepted for this CQR package. The science-tier target-level
line/region thresholds pass, every covered eligible target function satisfies
the per-function floor, all above-threshold CRAP rows are closed, and no
production code was changed.

## Obligation-to-Test Binding

ADR-0021 obligation binding is required because this package materially added
characterization tests for a science-tier watershed channel sediment module.
The target file owns private WS20/WS22/WS23/WS24/WS26/WS27/WS30 channel
detachment, detachment-capacity, branch-transition, and shape helpers. It does
not own hydrologic runon/peak routing, public serialization, watershed output
publication, hillslope erosion production, or parser projection surfaces.

Applicable contract bindings:

| Contract obligation / invariant | Target applicability | Bound characterization evidence |
|---|---|---|
| `SC-ROUTE-001#INV-ROUTE-008` spatially-varied flow/shear domains | WS20/WS22/WS26 helpers consume slope, flow, shear, fall-velocity, and table lookup terms that feed sediment branch decisions. | `wshedimpl20_fall_velocity_and_shdist_cover_boundaries`, `wshedimpl22_table_lookup_characterizes_direction_and_bounds`, `wshedimpl26_dcap_midlayer_step_characterizes_terminals_and_caps`, `wshedimpl26_dcap_expanding_width_characterizes_terminal_and_cap_paths`. |
| `SC-ROUTE-001#INV-ROUTE-009` sediment continuity with inlet/lateral source accounting | WS23 detachment helpers compute class fluxes from `gstu`, `dlat`, `du`, `dx`, potential load, transport capacity, and final next-flux vectors. | `wshedimpl23_detach_case4_closure_path_is_finite`, `wshedimpl23_detach_case4_iterative_loop_low_shear_is_characterized`, `wshedimpl23_detach_leaf_helpers_characterize_sums_and_flux_guards`, `wshedimpl23_detach_start_and_iteration_helpers_cover_terminals`. |
| `SC-ROUTE-001#INV-ROUTE-010` explicit detachment/deposition branch and transport-capacity iteration semantics | The target owns `dcap`, `detach.for` iterative closure, `case12` transition continuation, `enddet` bracketing, and shape-driven branch continuity covered by WSHEDIMPL22-30 contract amendments. | `wshedimpl23_detach_case4_closure_path_is_finite`, `wshedimpl23_detach_case4_iterative_loop_low_shear_is_characterized`, `wshedimpl23_detach_start_and_iteration_helpers_cover_terminals`, `wshedimpl24_transition_rejects_nonpositive_length`, `wshedimpl26_dcap_characterizes_expanding_width_path`, `wshedimpl26_dcap_midlayer_step_characterizes_terminals_and_caps`, `wshedimpl26_dcap_expanding_width_characterizes_terminal_and_cap_paths`, `wshedimpl26_dcap_low_width_shear_outcome_characterizes_terminals`, `wshedimpl27_enddet_bracket_characterizes_iteration_cap`, `wshedimpl30_shape_and_rectangular_fallback_characterize_guards`. |
| `SC-ROUTE-001#INV-ROUTE-011` and `SC-SED-001#INV-SED-010` sediment payload/class completeness at the routing boundary | Target helper inputs require class-cardinality consistency across class fractions, incoming/lateral/upstream sediment fluxes, particle diameters, and specific gravities. | `wshedimpl23_detach_rejects_invalid_validation_inputs`, `wshedimpl23_detach_leaf_helpers_characterize_sums_and_flux_guards`, `wshedimpl26_dcap_low_width_shear_outcome_characterizes_terminals`. |
| `SC-ROUTE-001#OBL-ROUTE-P-003` and `OBL-ROUTE-C-003` particle-class continuity for `qsed*`, `Tc`, and detachment/deposition outcomes | The target computes per-class detachment allocation, transport-capacity capped fluxes, and class-fraction allocations. | `wshedimpl23_detach_leaf_helpers_characterize_sums_and_flux_guards`, `wshedimpl23_detach_case4_closure_path_is_finite`, `wshedimpl23_detach_case4_iterative_loop_low_shear_is_characterized`, `wshedimpl26_dcap_characterizes_expanding_width_path`, `wshedimpl26_dcap_low_width_shear_outcome_characterizes_terminals`. |
| `SC-ROUTE-001#OBL-ROUTE-P-004`, `OBL-ROUTE-C-004`, and `SC-SED-001#OBL-SED-C-002` fail-explicit behavior for malformed sediment/channel states | Target helpers reject invalid cardinality, nonpositive segment lengths, invalid shape values, invalid sums, and negative/non-finite final fluxes with typed channel domain errors. | `wshedimpl23_detach_rejects_invalid_validation_inputs`, `wshedimpl23_detach_leaf_helpers_characterize_sums_and_flux_guards`, `wshedimpl24_transition_rejects_nonpositive_length`, `wshedimpl30_shape_and_rectangular_fallback_characterize_guards`. |

WSHEDIMPL amendment bindings covered by the same tests:

- WSHEDIMPL22: `dcap` and `case34/enddet` execution with fail-closed class
  fraction requirements: bound by the WS26 and WS27 characterization tests.
- WSHEDIMPL23: baseline-authoritative `detach.for` iterative closure for WS21
  case-4 rows: bound by the two WS23 case-4 exact-output tests and the
  start/iteration helper test.
- WSHEDIMPL24: `case12.for` deposition-to-detachment transition continuation:
  bound by `wshedimpl24_transition_rejects_nonpositive_length`.
- WSHEDIMPL26: `dcap(flagm=2)` max-detachment limiter semantics: bound by the
  WS26 midlayer, expanding-width, low-width-shear, and existing
  `wshedimpl26_dcap_flagm2_caps_detachment_rate_at_maxe` tests.
- WSHEDIMPL27: `enddet.for` `xdbig`/`xdsmal` bracket progression: bound by
  `wshedimpl27_enddet_bracket_characterizes_iteration_cap` and existing
  `wshedimpl27_enddet_helper_exercises_xdbig_and_midpoint_rebracketing`.
- WSHEDIMPL30: erodible-lane shape-transition and rectangular fallback:
  bound by `wshedimpl30_shape_and_rectangular_fallback_characterize_guards`.

Out-of-scope obligation disposition:

- `SC-ROUTE-001#INV-ROUTE-001` through `INV-ROUTE-007`, `INV-ROUTE-012` through
  `INV-ROUTE-014`, and `OBL-ROUTE-P/C-001..002` govern runon, runoff, peak,
  duration, applicability, hourly carry, and governance surfaces outside this
  target module.
- `SC-SED-001` hillslope producer obligations (`OBL-SED-P-*`) are not owned by
  this watershed channel routing helper package; only the routing consumer /
  cross-domain class-completeness obligations above are applicable.
- No new contract authority, public output surface, parser projection, or
  serialization path was added by this package.
