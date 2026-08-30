# Review A — snow thermodynamics and adaptive numerics

Status: **COMPLETE — GO**

Review snapshot: commit `2a9ca2d845bb4f128441ab01f79b341033a31c7d`
with the shared terminal worktree visible on 2026-08-29. The worktree was dirty
and concurrently active. This review changed only this artifact.

## Verdict

**GO for the Review A thermodynamics/adaptive-numerics scope.** No material
open finding remains in the conditional fixed-point policy, exact 60-second
floor, WB14 factorization-lineage comparison, snow/soil receipt termination,
conservation closure, event/discrete fail-closed behavior, or diagnostic
retention posture reviewed here. This is an independent domain review, not a
substitute for the package's separate terminal diff reconciliation,
full-workspace campaign, other reviews, or dual verification.

## Findings by severity

| Severity | Count | Disposition |
|---|---:|---|
| Critical | 0 | None. |
| High | 0 | None. |
| Medium | 0 | None. |
| Low | 0 | None. |

Non-finding watch item: the v7 maximum installed-endpoint receipt energy
residual uses 99.86% of `TOL-SNOWENERGY-005` (`9.98625182546675205e-10`
against `1e-9 J m^-2`). This is not an admitted drift or closure failure: the
bound remains strict, first-value-above/nonfinite poisons reject, cap
exhaustion fails closed, and the independently validated physical-energy
ledger remains over 700 times tighter than its unchanged `1e-6 J m^-2`
threshold.

## Static: contract-to-code review

### Conditional exact-floor contraction

- `SC-SNOWENERGY-001@25`, `INV-SNOWENERGY-049/052` requires raw Picard at the
  exact floor and permits `w=0.5` only after an authentic `A/B/A` Stage-3
  cycle. `open_snow.rs` gates detection with
  `support_duration_ns == STAGE3_V11_ADAPTIVE_MINIMUM_SUPPORT_NS`; the switch
  starts false and is monotonic only for the current unpublished solve.
  Supports above the floor retain the separately authorized support-scaled
  `0.25..=0.5` relaxation. Supports below the floor receive no weight.
- The detector compares candidate `n` with `n-2` using the existing native-unit
  convergence predicate and requires `n != n-1`. That predicate authenticates
  both Stage-3 fingerprints and exact lane set, schema, terminal-event model,
  lane/interval identity, layer cardinality, binary64 density, settling,
  initial mass/liquid lineage, and the named mass/depth/temperature/energy
  norms. An event, topology, density, or structural change therefore cannot
  masquerade as the admitted cycle.
- The damped state is unpublished numerical state. Relaxation refuses schema,
  terminal-model, layer-count, input-lineage, represented-mass, zero/nonzero,
  or resolved/terminal/dormant posture changes. Candidate density and settling
  remain unblended; thickness and temperature are reconstructed from relaxed
  extensive state plus exact density. The Stage-3 fingerprint is regenerated,
  persistent-state and cumulative mass/energy closure validators run, and soil
  state/snapshot digests are canonically resealed and validated.
- Finalization does not publish a relaxed iterate. Sealed Stage-3 and soil
  candidates are replayed and required byte-identical. Snow--soil receipt
  topology and joins are checked; identity-only receipt resealing is replayed
  through both owners and again required byte-identical. Iteration 96 exits as
  typed `FixedPointIterationLimit`; it cannot install a candidate.

### Exact floor, chronology, and events

- The production constant is exactly `60_000_000_000 ns`. Parent/hard-boundary
  supports must divide this quantum, candidate supports are constructed in
  integer quanta, and the restart validator rejects the historical 600-ms
  substitution. The floor executes one direct physical trial and propagates
  any failure; it does not fabricate a split or a sub-floor continuation.
- Wider supports execute direct and two-child composed paths from immutable
  beginnings. Refinable failures reduce the result-blind quantum count;
  non-refinable failures propagate. Only a passing composed result is
  installed. Accepted child predecessor/ending owner digests are rejoined
  before mutation.
- Terminal direct/composed comparison separately binds event tick,
  terminating/post-active lane sets, parcel cardinality/posture, and ordered
  child event receipts. Event mismatch sets the exact discrete rejection flag.
  Floor failure, event/topology poison, missing receipt, duplicate/reordered
  chronology, and cap exhaustion all remain fail-closed.

### WB14 `ReceiptLineage` correction

- `SC-SURFACELIQUID-001@13` narrowly classifies only
  `surface_liquid.wb14_parent_working_state.next_child_ordinal` and
  `per_ofe_authorities.<ofe_id>.receipts` digest-key/history identity as
  per-trial factorization lineage. Direct `H` and composed `H/2 + H/2` paths
  necessarily have different child counts and transaction-local receipt keys,
  so those fields are excluded from cross-factorization physical mismatch.
- Each path still includes the full receipt-lineage digest in its own exact
  discrete SHA-256. Accepted-path receipt maps, predecessor chains, supports,
  payloads, ordering, replay, and custody are independently validated. All
  non-WB14 receipt membership/order surfaces remain exact cross-path
  predicates. Event posture, owner/OFE/tile topology, cumulative WB14 working
  state, mass/energy state, and rollback are not reclassified.
- The comparison audit uses the same `ReceiptLineage` exclusion as production,
  preventing the former diagnostic-only false mismatch without altering the
  production decision.

### Conservation, receipt termination, and diagnostics

- `TOL-SNOWENERGY-005` retains bit-for-bit the equal/opposite heat actually
  consumed by snow and soil. Installed-endpoint reconstruction is an audit;
  only finite residuals `<=1e-9 J m^-2` and temperatures `<=1e-8 K` may rebind
  exact installed owner identities. Larger residuals retry and the unchanged
  cap fails closed. The physical ledger tolerance is not used to terminate the
  fixed point.
- Adaptive/fixed-point telemetry is disabled by default and lives only in
  opt-in thread-local state. Guards clear/disable it, and comments plus code
  keep it outside owner identity, receipts, controller decisions, restart, and
  persisted wire. Runner emission is confined to ignored test-only telemetry
  fixtures. No production-persisted microstepping diagnostic was found.

## Ran: current focused evidence

- `nix develop -c cargo test -p openwepp-hillslope-orchestrator covered_convergence_policy_tests -- --nocapture`:
  **PASS**, 16/16. This covers raw exact-floor default, conditional period-two
  activation, exact density, event/topology poisons, reconstructed
  thickness/temperature/fingerprint, cumulative closure, immutable inputs,
  and unit-specific norms.
- `nix develop -c cargo test -p openwepp-hillslope-orchestrator wb14_per_ofe_receipt_keys_are_lineage_while_other_receipt_ordering_is_exact -- --nocapture`:
  **PASS**, 1/1.
- `nix develop -c cargo test -p openwepp-hillslope-orchestrator receipt_factorization_lineage_is_bound_per_path_but_not_cross_path_physics -- --nocapture`:
  **PASS**, 1/1.
- `nix develop -c cargo test -p openwepp-hillslope-orchestrator cold_open_snow_candidate_at_exact_floor_fails_closed -- --nocapture`:
  **PASS**, 1/1.
- `nix develop -c cargo test -p openwepp-hillslope-orchestrator noncontracting_receipt_reseal_density_mapping_exhausts_96_fail_closed -- --nocapture`:
  **PASS**, 1/1; the isolated worker observed the expected typed exact-floor
  `FixedPointIterationLimit` and the parent test accepted only that fail-closed
  outcome.
- `nix develop -c cargo test -p openwepp-hillslope-orchestrator odd_quanta_tile_exactly_and_never_call_carrier_below_floor -- --nocapture`:
  **PASS**, 1/1.
- `nix develop -c cargo test -p openwepp-hillslope-orchestrator accepted_support_requires_event_as_ordered_publication_tail -- --nocapture`:
  **PASS**, 1/1.
- `nix develop -c cargo test -p openwepp-hillslope-orchestrator receipt_reseal_one_ulp_density_mapping_reenters_and_converges_before_install -- --nocapture`:
  **PASS**, 1/1.
- `nix develop -c cargo test --test snow_stage3_shared_carrier_authority_contract -- --nocapture`:
  **PASS**, 6/6.
- Scoped `git diff --check` over the reviewed solver/controller/contracts and
  package: **PASS**.
- A combined follow-on surface-liquid-contract/assurance command was stopped
  before those two stages ran because the exact-current full-workspace
  campaign acquired the shared Cargo lock. No result is claimed for those two
  stages in this review; the package gate ledger records their prior focused
  passes, and they are not used to replace the tests above.

## Ran: canonical v7 one-day evidence

Reviewed `/tmp/adaptive_microstep_amendment/one-day-final-v7-opt.log`
(`sha256 5406d0d8afcf74c0cd3d85858af39d57498cb0b103e6233053b349e463729b2b`)
and `.time`
(`sha256 cfa364b6870f19ba2759a5f573ebb468aaf677554d5732e9189df6f0af5608c8`).
The exact command was:

`env RUST_MIN_STACK=67108864 CARGO_PROFILE_TEST_OPT_LEVEL=3 nix develop --command cargo test -p openwepp-runner cqr_stage3_one_day_qualification_with_telemetry -- --ignored --nocapture`

Result: **PASS**, exit 0; all 48 parents; optimized test body `357.55 s`;
compilation-inclusive wall `561.39 s`. The test completed the committed
qualification snapshot, real downstream publication consumer, archive fold,
and output transaction assertions.

- Controller totals: 497 accepted, 206 rejected, 703 direct trials, 1,368
  split-child trials, 975 publication supports, and 61 publication events.
- Accepted widths: `19x60 s`, `112x120 s`, `354x180 s`, `3x360 s`,
  `6x420 s`, `1x900 s`, and `2x1800 s`. Thus 478/497 (96.18%) accepted
  supports exceed the exact fallback floor; the day does not depend on
  ordinary floor stepping.
- Limiting reasons: 190 phase-bucket rejections and 16 other rejections;
  fixed-point audit reports 155 nonconverged trials, while adaptive comparison
  reports 16 scaled rejections and zero exact-discrete rejections. Event and
  phase-plus-event rejection buckets are both zero on this fixture.
- Independent ledger audit: 1,578 validated ledgers; maximum mass residual
  `1.77635683940025046e-15 kg m^-2` against `1e-9`; maximum energy residual
  `1.39698386192321777e-9 J m^-2` against `1e-6`.
- Receipt audit: maximum energy residual
  `9.98625182546675205e-10 J m^-2` against `1e-9`; maximum endpoint
  temperature residual `4.37694325228221714e-12 K` against `1e-8`.

These results support the amended objective: the former approximately
1,435-accept/1,500-reject near-floor blocker is replaced by a closing,
fail-closed 497/206 controller distribution without changing conservation,
event, receipt, rollback, floor, convergence-tolerance, or 96-cap authority.
