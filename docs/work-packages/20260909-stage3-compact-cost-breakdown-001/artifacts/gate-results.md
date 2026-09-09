# Gate results

Ran: observation-only characterization gates.

- PASS: comparator runner unavailability was already recorded as service usage
  exhaustion; no retry. Authorized parent-local fallback used, serialized.
- PASS: A cut2b bundle/source identity and executable/sidecar identity resolved.
- PASS: isolated release build for runner + LSE libraries. One inherited
  `ending_snow_hint` dead-code warning is outside the ten-file observer delta.
- PASS: compact-cost focused release tests, 3 passed / 0 failed. Covered exact
  accounting and off neutrality; live/stale/non-LIFO/overflow; clock reversal
  and synthetic error preservation.
- PASS: audit-only synthetic `Err(7)` propagation/count test and manually emitted
  Potential/FixedFinal audit lifecycle test, 1/1 each. These are not source-real
  solver/error or observer-on evidence; logs retain their exact test names.
- PASS: source-real Covered natural-failure/frozen-diagnostic/no-publication test
  and source-real Potential/FixedFinal beginning-state nonmutation test, 1/1 each.
  Compact observer session is inactive in these focused tests. Final runner
  series-03 is the actual observer-off/on valid-solve semantic evidence and its
  LSE populations prove Covered/GenericV3 Potential/FixedFinal participation.
- PASS: `rustfmt 1.95 --edition 2024 --config skip_children=true --check` on all
  ten isolated source files.
- PASS: warnings-denied release Clippy for the changed LSE library after three
  observer-only lint corrections. Initial attempt found those three issues and
  is superseded by the clean rerun.
- PASS: complete ten-file A-to-P patch, SHA-256 `d9d3203...1f720`, applies to A
  with `patch --dry-run --fuzz=0 -p1`.
- PASS: final 16-process series-03: 4 warmups + 12 measured runs, fresh process,
  balanced order, CPU0, no retries. All protected scientific/output/closure and
  work-count identities matched; every on run had accounting difference 0 ns.
- PASS: independent analyzer reconstruction of all six on-arm partitions and
  six pairs; portable analysis SHA-256 `7df4bfd8...3461d8`. A final metadata-only
  analyzer correction prefers retained sibling logs and emits receipt basenames;
  arithmetic is byte-equivalent after removing path keys and was not rerun.
- PASS: exact P ten-source-file + executable/sidecar + fixture/protocol/result
  custody bundle verified. External manifest SHA-256 `66a927d5...afd4ba` at
  `/tmp/openwepp-compact-cost-P-vKI6gb/final-custody-bundle`; A full-source
  recovery remains bound to its separately verified predecessor bundle.
- WARN: touched terminal execution (2,932), carrier (2,892), imported consumer
  (2,837), and runner execution (2,355) files exceed 2,000 lines; none reaches
  3,000. Hooks are small scopes at existing boundaries. Splitting these inherited
  modules would expand a characterization-only delta and is deferred.
- LIMIT: one permitted `perf record` refinement failed before runner execution:
  host `perf_event_paranoid=4` denied all events. No privilege change attempted.
- NOT RUN by scope: inherited full-workspace failure population, prior memory
  campaign, multi-OFE/year cases, derivative oracle, production qualification.

Measured executable SHA-256: `c2276fa9...0c858`; adjacent sidecar SHA-256:
`2809763a...a9a1`. Production remains HOLD.

## Exact focused-gate receipts

All commands used cwd `/tmp/openwepp-compact-cost-P-vKI6gb`, exit 0, and the
final frozen LSE test executable
`/workdir/.cache/openwepp/targets/compact-cost-P/release/deps/openwepp_land_surface_energy-29dc6221b95c33c1`
(SHA-256 `f57e0205...e724`) under `nix develop /workdir/openWEPP#default --offline -c`.
Arguments after the executable were, respectively:

- `compact_cost::tests --nocapture` (3 tests; direct console receipt).
- `solver_mechanism_audit::tests::compact_counts_and_errors_without_events --exact --nocapture`
  (`raw/gates/actual-error-test.log`, despite the historical filename this is synthetic).
- `solver_mechanism_audit::tests::potential_and_final_solves_join_one_authentic_map --exact --nocapture`
  (`raw/gates/authentic-map-test.log`, manually emitted audit events).
- `covered_oracle_conformance_tests::covered_natural_failures_match_frozen_diagnostics_and_publish_no_candidate --exact --nocapture`
  (`raw/gates/source-real-covered-failure.log`).
- `solver::tests::potential_and_final_do_not_mutate_beginning_problem --exact --nocapture`
  (`raw/gates/source-real-potential-final.log`).

Format and Clippy commands use the same cwd/Nix environment and exact ten files /
manifest build variables recorded in source-and-build-manifest; retained logs
are `raw/gates/rustfmt.log` and `raw/gates/clippy.log`, both exit 0. The Nix banner's
source-derived target is informational; explicit `CARGO_TARGET_DIR` governed the
artifacts and their hashes were recomputed after build.

Runner/orchestrator scoped Clippy initially reached the final runner crate but
failed on two inherited findings: an out-of-delta dead field and the pre-existing
106-line function containing a new thin timing scope. A bounded rerun retained
`-D warnings` while explicitly allowing only `dead_code` and
`clippy::too_many_lines`; it passed. Both failing and bounded logs are retained
as `raw/gates/runner-clippy*.log`. No lint allow attribute or source waiver was added.
