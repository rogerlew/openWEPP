# Kernel-profile and readiness matrix

Evidence mode: `Static + Ran`

Contract set:

- `SC-SNOWENERGY-001` contract revision 58; no process-solver V58;
- `SC-LANDSURFACEENERGY-001` v26;
- `SC-VEGETATIONTRANSACTION-001` v17;
- `SC-SURFACELIQUID-001` v28 (v27 nonfinal-map authority plus
  `INV-035/C-025` native inactive-prefix transition);
- `SC-COUPLEDTIME-001` v14 (v13 nonfinal-map authority plus
  `INV-031/C-014` native inactive-prefix chronology).

Orthogonal status:

```text
science_implementation_status = NOT_IMPLEMENTED
calibration_evidence_status = NOT_APPLICABLE
identifiability_status = NOT_APPLICABLE
```

`NOT_IMPLEMENTED` is the deliberate contract-first state proven by the
behavioral expected-red compile. It is not a missing authority/profile row and
does not authorize a production claim.

| Profile obligation | Status | Evidence and rationale |
|---|---|---|
| Canonical files and lifecycle registry updated | `PASS` | Five canonical contracts and `index.md`; versions/dates/change logs reconcile. Snow revision 58 explicitly distinguishes contract metadata from forbidden process-solver V58. |
| Required logical schema sections present | `PASS` | Each existing contract retains purpose/scope, anchors, variables/units, algorithm, guards, invariants, obligations, aliases, numeric/calibration posture, tests, BEI, gaps, and change history. Late amendment-section consolidation remains nonblocking hygiene identified by QA. |
| Algorithm and branch behavior reproducible | `PASS` | Amendments bind orthogonal role/regime dispatch, `Initial@0`, predictor `@1`, contiguous trials `@(n+1)`, final `@(N+2)`, `M=N+3<=8`, no post-convergence role, and separate success/pre-final/final-physical/final-constructor outcomes. |
| Typed guard/error map complete | `PASS` | Snow/LSE/vegetation/surface amendment tables map role, identity, physical, final-constructor, state-leak, and publication triggers. Vegetation binds role/ordinal to `DirectV11RealConsumerError::AdaptiveRefinement`, identity to `VEGTXN-E-011`, incomplete/per-map final custody to `E-013`, restart/replay to `E-014`, complete shared-owner mismatch to `E-015`, and partial-owner/rollback defects to `E-007`. CoupledTime maps role/support to `ERR-CT-010`, leaks to `016`, and premature publication to `018`; `ERR-CT-021` and undefined SurfaceLiquid `E-012` are absent from the new amendment. |
| Unit-governance map complete | `PASS` | No dimensional value, conversion, tolerance, or published numeric surface changes. Scoped unit lint passed in `covered_nonfinal_contract_postreview_static_gates_20260903T0157Z.log`. |
| Calibration and identifiability disposition | `NOT_APPLICABLE` | The amendment changes private custody construction timing, not a parameter, observation operator, objective, constitutive equation, or calibration surface. Existing contract calibration sections remain authoritative. |
| Test-vector obligations reflected before production | `PASS` | Source/contract anti-evasion plus executable crate tests assert success and each pre-final/final-physical/final-constructor counter matrix, per-constructor success/zero-nonfinal counts, exact typed role including independent multisecant trial index and ordinal, ordinary/native frozen/mixed/thaw/wet/multi-OFE parity, zero native snow-free litter/surface/WB14 physical calls, retained inactive bytes, exact typed error class and validation precedence for injected failures and single/combined poisons, zero promotion/fallback, rollback, zero map publication, and parent-only publication. The source guard rejects derived or manual serialization and `Clone`/`Copy` as well as promotion conversions. The strengthened behavioral population remains expected red on absent implementation APIs. |
| Binding Exposure Index consolidated | `PASS` | All five strict checks passed in `covered_nonfinal_contract_postreview_static_gates_20260903T0157Z.log`. |
| Comparator governance | `NOT_APPLICABLE` | This optimization asserts exact differential physical-prefix equality and owner custody; it neither uses nor adjudicates a legacy comparator delta. |
| External constitutive suite posture | `NOT_APPLICABLE` | No equation, parameter, tolerance, suite binding, cohort, or required-case posture changes. Existing process suites remain required for terminal package validation. |

## Obligation-to-test binding

| Canonical obligation | Pre-implementation test binding |
|---|---|
| `OBL-SNOWENERGY-C-054` | `canonical_covered_success_has_m_minus_one_private_maps_and_one_private_final_envelope`; `canonical_covered_failure_matrix_never_completes_or_publishes_a_failed_envelope`; physical-prefix parity/poison tests below. |
| `OBL-LANDSURFACEENERGY-C-016` | `canonical_covered_physical_prefix_matches_forced_complete_regime_matrix`; `canonical_covered_physical_prefix_poisons_reject_with_exact_rollback`. |
| `OBL-VEGTRANSACTION-P-005/C-007` | success/failure and per-constructor counters, all final-constructor injection cases, exact error class/precedence, zero map publication, and move-only/non-wire/nonpromotion source guard. |
| `OBL-SURFACELIQUID-C-024` | ordinary/native parity, zero native snow-free litter/WB14 physical-call counters, retained inactive-native custody bytes, one-ULP/identity/regime poisons, exact error class/precedence, and rollback. |
| `OBL-COUPLEDTIME-013` | exact charged-role ordinal vector, explicit success/pre-final/final-physical/final-constructor counters, exact role/identity/physical rejection stages, zero map publication, one accepted-parent publication, and rollback. |

The behavioral test module is
`crates/openwepp-hillslope-orchestrator/src/snow_stage3_v11_adaptive_production_tests.rs`.
The supplemental contract/source anti-evasion test is
`tests/integration/stage3_native_vegetation_laned_throughput_recovery.rs`.

Production implementation remains unauthorized until both independent
verification agents accept the amended authority, disposition, readiness
matrix, and behavioral expected-red population.
