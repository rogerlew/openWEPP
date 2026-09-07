# F full-release failure comparison B

Static: bounded independent read-only diagnosis of completed parent-run logs,
named source consumers and child stderr. Lightweight Python parsed failure
names, hashes and binary64 values; no build/test/reconstruction/source edit
was executed by B. Root/work-package AGENTS apply; find-agents ran before edit.

## Findings first

- FFB-1 / P2, introduced stale source guard:
  `stage3_evaluation_validation_tests/persistent_tests.rs:313` expects the
  superseded direct provider-call spelling. F now invokes the typed
  `provider.evaluate_resolved(...)` at `stage3_solver/evaluation.rs:774`.
  Preserve the independent resolved call and boundary-propagation obligation
  by following that actual typed dispatcher; do not remove/lower the guard.
  Parent/worker received prospective GO for the narrow correction below.
- FFB-2 / P2, worktree environment:22 newly failing tests cannot launch the
  required repo-local Python (or explicitly assert it missing). The additional
  owcmp env failure has a source-established missing `.venv/bin/python`
  prerequisite. These23 failures never establish their intended scientific
  or governance outcomes. F's isolated worktree has no `.venv`; main has one.
  Restore declared equivalent local test tooling, then run corrective gates.
- FFB-3 / P2, new warning detail: F reports both `coupling_iteration` and
  `ending_snow_hint` unused; A reported only `ending_snow_hint`. The first is
  F-added dead-field warning after feedback removal, not wholly inherited.
  Preserve the legacy observation schema while narrowly resolving/dispositioning
  it before warning-denied lint. Do not silently blanket-allow unused code.

No new runtime-physics failing test name was found. This is not full PASS or
science/timing admission: retained baseline failures and unexecuted corrective
gates remain explicit.

## Exact runs and accounting

| Run | Log SHA-256 | Recorded result |
| --- | --- | --- |
| A full release01 | `5f4c279602e193c305048e7613337c688728daf7dc87124abbe44083c892917e` | 4202 run,4022 pass,180 fail,75 skip;513.757s tests |
| F full release01 | `13c1f3d6ebed0a36e38debc33cc4da2dfbb41376fcd985dc7aa7bf596bf6fb6e` | 4207 run,4005 pass (1 slow),202 fail,75 skip;149.392s tests;707.666715935s wrapper;exit100 |

F's `.log.json` records isolated F cwd, checkout2b56e6e, `nix develop --offline
-c env CARGO_PROFILE_RELEASE_LTO=false cargo nextest run --release --workspace
--profile full`,64MiB RUST_MIN_STACK and offline Cargo. The log has8076 lines.
These full-gate durations are NOT controlled performance comparisons.

Deduplicating the immediate/final status listings by complete test name gives
178 common failing names,24 newly failing names and2 A failures absent from F:
`180−2+24=202`. This is more informative than saying22 new failures. The five
added F helper/full-reference tests are not among failed names.

## Every newly failing name

Module names below are integration test targets unless marked orchestrator.
Line numbers refer to final failure sections in `raw/F-full-release-01.log`.

| Line | Module / test | Exact family |
| --- | --- | --- |
| 4333 | assurance_v2_amendment_contract / focused_runner_rejects_an_off_archive_receipt | missing repo Python at command launch |
| 4143 | assurance_v2_amendment_contract / focused_runner_rejects_forged_receipt_matrix | missing repo Python at command launch |
| 5856 | cqr_quality_evidence_handoff_contract / cqr_quality_evidence_self_test_passes | missing repo Python at command launch |
| 5894 | cqr_quality_evidence_handoff_contract / missing_evidence_is_typed_invalid_without_collection | missing repo Python at command launch |
| 5875 | cqr_quality_evidence_handoff_contract / recollection_requires_typed_noncurrent_receipt_and_explicit_directive | missing repo Python at command launch |
| 5932 | hphys0298_paired_lineage_partition_contract / hphys0298_harness_rejects_historical_hrsnow_water_equiv_pairing | missing repo Python at command launch |
| 5970 | hphys0311_snow_carry_source_line_parity_contract / hphys0311_runner_negative_fixture_fails_closed_on_missing_source_line | explicit `.venv/bin/python` missing assertion |
| 5989 | hphys0312_prior_year_terminal_snowpack_lineage_contract / hphys0312_runner_negative_fixture_fails_closed_on_missing_source_line | explicit `.venv/bin/python` missing assertion |
| 6008 | hphys0313_snowpack_settling_carry_recursion_contract / hphys0313_runner_negative_fixture_fails_closed_on_missing_source_line | missing repo Python at command launch |
| 6065 | owcmp_cli_contract / owcmp_env_checks_temp_manifest_and_rejects_inventory_run | environment summary returns nonzero; missing repo-venv prerequisite |
| 6123 | quality_observatory_merged_coverage_contract / quality_observatory_self_test_passes | missing repo Python at command launch |
| 6104 | quality_observatory_workflow_contract / controller_is_self_consistent_and_observes_only_quality_runs | missing repo Python at command launch |
| 6142 | quality_observatory_workflow_contract / deterministic_preflight_defers_a_competing_quality_run | missing repo Python at command launch |
| 6161 | root_zone_hydraulic_authority_contract / generator_is_byte_reproducible_and_poison_inventory_is_complete | missing repo Python at command launch |
| 6180 | root_zone_hydraulic_authority_contract / independent_calculator_schemas_and_manifest_execute | missing repo Python at command launch |
| 6256 | snow_free_half_hour_forcing_authority_contract / independent_calculator_regenerates_frozen_vectors_exactly | missing repo Python at command launch |
| 6731 | snowfrost_fidelity_c_diagnostics_contract / snowfrost_c_cli_emits_diagnostic_only_sfcc_frozen_k_payload | missing repo Python at command launch |
| 6750 | snowfrost_fidelity_c_diagnostics_contract / snowfrost_c_curves_are_bounded_monotonic_and_impedance_ordered | missing repo Python at command launch |
| 6809 | snowfrost_fidelity_c_diagnostics_contract / snowfrost_c_records_salinity_sensitivity_without_production_promotion | missing repo Python at command launch |
| 6655 | v10_nighttime_authority_contract / independent_nighttime_calculator_regenerates_frozen_vectors | missing repo Python at command launch |
| 6674 | vegetation_boundary_authority_contract / independent_coupled_reference_vectors_pass | missing repo Python at command launch |
| 6693 | vegetation_boundary_authority_contract / v3_historical_oracle_is_immutable_and_isolated_execution_cannot_rewrite_authority | missing repo Python at command launch |
| 6712 | vegetation_boundary_authority_contract / v5_historical_oracle_is_immutable_and_isolated_execution_cannot_rewrite_authority | missing repo Python at command launch |
| 7020 | orchestrator persistent_tests / covered_resolved_source_routes_provider_boundary_into_carrier | stale direct-call source literal at313 |

The22 direct interpreter failures are traced to each test's exact Command path,
not inferred solely from ENOENT. owcmp differs: it launches `python3`, but
`tools/owcmp/suite_manifest.py:133–172` explicitly requires the repo-venv file
and returns overall FAIL when missing. Its JSON stdout was not printed by the
test, so other possible checks (notably pyarrow) are not claimed passed or
failed. Missing venv alone is independently sufficient. Tooling/source for
this owcmp path is unchanged between A and F.

## Shared and no-longer-failing cases

The corrected canonical-manifest source guard now passes, as expected from its
reviewed integration-only extraction fix. Also no longer failing is
`h2637_default_malformed_routing_coefficients_fails_closed`: A stopped during
flat fixture copy on an existing ignored output directory; F's clean fixture
reaches the intended refusal. No source change to `laned_shadow_h2637.rs`.

One common failing name changes the observed stage for the same fixture reason:
`h2637_default_mixed_routing_coefficients_fails_closed` (F6046) gets beyond A's
copy error and sees `laned_active_default_eligibility`:18 lanes with routing
coefficients and1 without. The test instead expects the Stage3 seed refusal
first. This is a newly exposed preexisting ordering/expectation failure, not
an F physics delta or an environment-only PASS.

The other shared core diagnostics agree after distinguishing thread IDs,
worktree paths and moved source lines from actual error operands. Covered
budget/receiver-support/dependent-output/topology/parent-source failures retain
their same support intervals and typed causes. Generated restart reference
still fails the same E0308/E0599 in `evidence_fixture.rs:163,404`; it is not a
new production-crate compiler failure. The separately noted extra dead-field
warning appears alongside it and requires its own QA disposition.

All six F nested watershed child stderr files referenced by the log were read
under `/tmp/openwepp-roger-F-af041db248b7/`. Five are exactly337 bytes with SHA
`1c56b7717a09235f0f6b6e1fcdb4f5740c778123e1526a46f0ada1496309ca1a`
(forest litter cannot enter Stage3 open-snow); the sixth is240 bytes with SHA
`46e458f1c1f3f42739fe79a608b5883912d1261624a5c5e79b1c2d71c1246335`
(pending parent-finalization source). These are byte-identical to the already
retained A child raw copies; the main F log retains each original path. None
is a new F custody failure or a missing-Python child.

## Same-release LSE diagnostic-vector parity

A line6705 and F line7503 fail the same natural iteration-limit exact golden
assertion at `covered_oracle_conformance_tests.rs:989`. Independently extracted
both actual and expected15-element vectors from the raw failures. Actual
vectors are literal-identical and, after parsing, every binary64 bit matches
between A and F. Expected vectors are also literal-identical. Actual values:

```text
[10845740.51192798, 18521900.195331838, -160970.26801191425,
 -190251.5065555011, -9122365.618271498, -20550647.817955893,
 185017.85333670734, 583132.3693539437, -401.89639289493334,
 -148.6868489708855, 4021.0827423780142, -263190.3474301763,
 -9195.931982311033, -284.4408469346596, 355.1919041193066]
```

Both reach Rejected/IterationLimit with the preceding exact occupancy/bounds,
50-iteration and340-backtrack assertions satisfied. Neither executes the
subsequent pivot/matrix/ordered-row assertions after the failing vector. This
establishes A/F same-release parity of the printed diagnostic vector, NOT full
erroneous DTO parity or golden conformance. R's matching evidence remains
separate. No vector/tolerance/build flag was changed; retain both FAIL results
and the conditional authority disposition in A-failure-classification-review-b.

## Narrow corrective route and QA status

Prospective FFB-1 guard correction: assert exactly one typed
`provider.evaluate_resolved(CoveredTerminalTrialRequestV1` within the resolved
seam; retain `boundary: covered_substep_boundary`, returned transition boundary
and existing join-error identity. Follow the actual provider module and assert
one direct callback in the cfg-appropriate Feedback/FeedForward match arms;
reject invocation of the terminal-coupling helper there. This preserves the
separate resolved obligation rather than blessing the missing old spelling.
Concrete correction and focused test execution remain parent/worker-owned.

### Corrective cut review

Static: reviewed the formatted `persistent_tests.rs` cut SHA256
`2d8ce81ff9350eb808feb205da86dd6cf660c0610f22f301183d0f2c0bc85426`.
The test now requires exactly one actual typed resolved-provider call, preserves
both outbound and returned boundary checks and the original join-error key,
and follows the provider implementation to require the two cfg-appropriate
direct callback arms. It rejects both terminal and feedback coupling-helper
calls in that dispatcher. B GO on this correction; execution is NOT RUN by B.

Prospective B GO on a field-only
`#[cfg_attr(not(test), expect(dead_code, reason = ...))]` annotation for
`CoveredTerminalTrialRequestV1::coupling_iteration`. The inspected semantic
reads are test-only; retaining the field and derived Debug keeps the shared
audit schema unchanged. This is an explicit, configuration-scoped lint
expectation, not a blanket suppression or a physics change. Leave inherited
`ending_snow_hint` untouched. The concrete annotation and current-cut strict
lint/focused test results remain parent/worker-owned, not passed by this review.

Environment corrective checks must actually execute the23 selected tests after
restoring declared worktree tooling; source classification is not a substitute.
Do not revive frozen planner workflows or change physics to fix tooling.
Required full terminal verification, detailed mechanism custody/trace and
scientific gates remain pending. QA diagnosis complete; HOLD overall F full
PASS/admission until the named corrective gates and remaining obligations have
their own evidence. No controlled performance conclusion is drawn here.
