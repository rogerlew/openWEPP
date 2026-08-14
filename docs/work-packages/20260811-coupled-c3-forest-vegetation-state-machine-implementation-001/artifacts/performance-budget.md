# Performance Budget

Status: `PASS / frozen Milestone 6 benchmark matrix executed post-review`

Benchmark surfaces are strict parse/hash validation, radiation
integral/coefficient construction, one stratum-day, overlapping mixed stand,
active water/N competition, and rollback. The delegated comparator captured the
baseline hardware and elapsed distributions before any performance-motivated
edit. Acceptance is deterministic completion without a sample greater than 2x
the first clean sample for the same command/hardware; this is an engineering
budget, not scientific authority.

## Milestone 6 Execution

Evidence mode: `Ran`

The delegated comparator ran release-mode tests on Linux
`6.8.0-136-generic`, an Intel Xeon E5-2697 v2 at 2.70 GHz, and
`rustc 1.92.0 (ded5c06cf 2025-12-08)`. Each surface received one warm run and
five timed samples. Raw logs, timings, metadata, the command log, and the JSON
summary are retained in `artifacts/m6-benchmark-20260813234912/`.

| Surface | Exact fixture/test identity | Samples (seconds) | Median | Maximum / first clean | Result |
|---|---|---:|---:|---:|---|
| strict V7 parse/hash | `v7_configuration_state_and_migration_inputs_have_no_default_path` | 0.26, 0.24, 0.25, 0.25, 0.24 | 0.25 | 1.0000 | PASS |
| exact two-rank radiation | `radiation::tests::v3_two_rank_visible_direct_fixture_and_named_poisons` | 0.16, 0.17, 0.16, 0.16, 0.17 | 0.16 | 1.0625 | PASS |
| public sealed one-stratum candidate | `v7_public_candidate_is_sealed_and_energy_owner_consumes_real_capped_operands` | 0.32, 0.31, 0.32, 0.32, 0.34 | 0.32 | 1.0625 | PASS |
| cap-active two-rank rerouting | `occupancy_solver::capped_pass::tests::upper_cap_changes_final_release_received_by_descendant` | 0.19, 0.19, 0.19, 0.19, 0.19 | 0.19 | 1.0000 | PASS |
| active water/N plus all-owner rollback | `v7_default_off_diagnostic_commits_all_owners_once_and_rolls_back_every_phase` | 1.13, 1.16, 1.11, 1.14, 1.10 | 1.13 | 1.02655 | PASS |

The first exact-name attempts for the two crate-local tests selected zero tests.
The timestamped command metadata records the correction, but the first runner
did not retain separate raw zero-filter logs and retained the invalid command
string for the corrected sample set. More importantly, its parse/default and
abundant one-occupancy diagnostic labels did not execute the claimed parse/hash
and active-competition surfaces. Independent review therefore rejected this
initial matrix as closure evidence. It remains historical evidence only and is
superseded below without being rewritten.

## Corrected Authoritative Matrix

Status: `PASS / independently challenged surfaces replaced`

The final run is retained in
`artifacts/m6-benchmark-final-20260814-20260814004247/`. Its command log records
the exact command actually executed for every surface, every warm and sample
log is present, and a zero-test guard passed for all five commands.

| Surface | Exact fixture/test identity | Samples (seconds) | Median | Maximum / first clean | Result |
|---|---|---:|---:|---:|---|
| strict V7 configuration parse and canonical hash | `config::tests::identity_rebound_v7_configuration_parses_strictly` | 0.23, 0.16, 0.15, 0.15, 0.15 | 0.15 | 1.0000 | PASS |
| strict complete state parse and configuration/state identity | `transaction::milestone_one_tests::complete_two_tile_two_stratum_state_is_exact` | 0.16, 0.16, 0.15, 0.16, 0.15 | 0.16 | 1.0000 | PASS |
| exact two-rank radiation | `radiation::tests::v3_two_rank_visible_direct_fixture_and_named_poisons` | 0.16, 0.16, 0.16, 0.16, 0.16 | 0.16 | 1.0000 | PASS |
| public sealed candidate and independent energy owner | `v7_public_candidate_is_sealed_and_energy_owner_consumes_real_capped_operands` | 0.33, 0.33, 0.32, 0.31, 0.32 | 0.32 | 1.0000 | PASS |
| real two-stratum shared-layer water and NH4/NO3 competition plus rollback | `v7_real_diagnostic_activates_shared_water_and_species_n_competition` | 0.46, 0.45, 0.47, 0.44, 0.44 | 0.45 | 1.02174 | PASS |

The scarce mixed-stand diagnostic asserts two positive partial water
authorizations, four positive partial species-preserving nitrogen
authorizations, bounded finalized use, exact water/BGC debit and transaction
lineage, and byte-identical rollback on the same fixture. This is the
authoritative Milestone 6 matrix. All samples complete and the largest ratio is
1.02174, inside the frozen 2x engineering budget. It makes no activation,
calibration, empirical-validation, or transferability claim.
