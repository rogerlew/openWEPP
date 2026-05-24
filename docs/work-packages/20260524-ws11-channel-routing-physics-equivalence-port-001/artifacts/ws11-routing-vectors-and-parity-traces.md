# WS11 Routing Vectors and Parity Traces

Status: `hold`
Evidence mode: `Static + Ran`

## Static
- Contract-derived vector source authority
  - `SC-ROUTE-001` WS11 addendum (`INV-ROUTE-006`, `INV-ROUTE-007`)
  - `SC-SYSTEM-001` WS11 addendum (`INV-SYSTEM-005`, `INV-SYSTEM-006`)
  - `SC-HYDRAULICS-001` WS11 consumer-coupling vectors
- WS11 vector set under test
  1. `ipeak = 1` nominal finite/non-negative publish
  2. `ipeak = 2` nominal finite/non-negative publish
  3. `ipeak = 3` routed closure (`roff = qpo * durrof`)
  4. `ipeak >= 4` routed closure (`roff = qpo * durrof`)
  5. Missing/non-finite/domain `ipeak` failures preserve
     `WKERNEL-WS10-CHANNEL-E-001..003`
  6. Branch distinction vector: `ipeak = 1` and `ipeak = 4` produce different
     channel outputs (no single-gain collapse)

## Ran
- Pre-implementation gate run
```bash
cargo test --test ws11_channel_routing_physics_equivalence_contract
```
  - result: **fail** (`2 passed; 4 failed`)
  - failing vectors captured before production edits:
    - missing/non-finite/domain `ipeak` enforcement
    - branch-distinction vector
- Post-implementation verification run
```bash
cargo test --test ws11_channel_routing_physics_equivalence_contract
```
  - result: **pass** (`6 passed; 0 failed`)
  - all WS11 vectors above pass.
- WS11 parity-trace scope note
  - Baseline mini-lane evidence exists from persisted logs under
    `/tmp/ws11_proto_Y1ukQn/output/`:
    - `ws11_mode3_ipeak1.stdout.log`: completes successfully.
    - `ws11_mode3_ipeak2.stdout.log`: completes successfully.
    - `ws11_mode3_ipeak3.stderr.log`: aborts with `SIGFPE` at
      `/workdir/wepp-forest/src/wshchr.for:342` (`wshchr_`).
  - Because baseline routed branch `ipeak=3` aborts, full WS11
    baseline-vs-openWEPP branch parity traces for routed lanes cannot be
    closed in this package.

## Hold-Lift Remediation Plan (Parity Trace Lane)
- Objective
  - convert blocked routed-branch parity lane into an explicit upstream-baseline
    blocker record with deterministic replay assets, then resume parity closure
    once comparator authority for routed branches is available.
- Planned write-set
  - `tests/integration/ws11_channel_routing_legacy_parity_trace.rs` (new,
    baseline blocker capture + openWEPP trace emission)
  - `docs/work-packages/20260524-ws11-channel-routing-physics-equivalence-port-001/artifacts/ws11-routing-vectors-and-parity-traces.md` (this file)
  - `docs/work-packages/20260524-ws11-channel-routing-physics-equivalence-port-001/artifacts/gate-results.md`
- Step sequence
  1. Add a deterministic WS11 parity-trace integration test that emits
     branch-keyed JSON trace files for `ipeak = 1,2,3,4` using the existing
     strict topology/runtime fixture from
     `ws11_channel_routing_physics_equivalence_contract.rs`.
  2. Stage baseline-reference traces and persist provenance manifests for
     successful branches (`ipeak=1,2`) and failure manifests for blocked
     routed branches (`ipeak=3`, and `ipeak=4` if it reproduces the same
     failure mode).
  3. Compare baseline vs openWEPP trace JSON values where baseline traces are
     runnable (`ipeak=1,2`) and publish blocked-status records for routed
     branches.
  4. Re-run closeout gates and update disposition/review/verification artifacts
     with parity-lane closure status.
- Acceptance criteria
  - persisted comparator JSON reports for all runnable baseline branch vectors.
  - no unresolved branch-level parity mismatches on runnable branches for
    `ws10_channel_1_qpo`, `ws10_channel_1_durrof`, `ws10_channel_1_roff`.
  - routed branch baseline crash condition is either:
    - closed with runnable baseline traces, or
    - explicitly accepted as an upstream baseline blocker with authority
      references and scoped impact statement.
