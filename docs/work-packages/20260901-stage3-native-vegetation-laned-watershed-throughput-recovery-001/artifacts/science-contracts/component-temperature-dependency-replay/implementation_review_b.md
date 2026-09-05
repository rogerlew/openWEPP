# Independent implementation review B — final seventh-cut rereview

Evidence class: `Static + Ran`.

Verdict: **APPROVE**.

This report supersedes every earlier implementation-review-B verdict. I reviewed
the current contents of the ordered 16 paths in
`/tmp/component_replay_review_cut3_sha256.txt` against
`SC-LANDSURFACEENERGY-001` revision 31, `INV-LANDSURFACEENERGY-164`, and
`OBL-LANDSURFACEENERGY-C-020`. The ordered manifest digest was
`edc3f0b94c393e537b0115403548b779754f33308e53a26b1041932e9915be71`
at review start and conclusion, with identical entries.

## Findings

No blocking correctness, test-quality, lint, documentation, or maintainability
finding remains in this seventh cut.

### `CTDR-IB-R7-001` — CLOSED

Path:

- `crates/openwepp-land-surface-energy/src/solver_component_dependency_replay_test_support.rs:1`

The only change from the reviewed sixth-cut manifest is the replacement of the
increment-owned `use super::*` with explicit imports. All other 15 reviewed
paths are byte-identical to the sixth cut. The relevant evidence configuration
now passes:

```text
cargo clippy -p openwepp-land-surface-energy --all-targets \
  --features test-support -- -D warnings
```

This closes the sole sixth-cut HOLD finding without changing production
behavior, the replay API, test semantics, or package feature containment.

## Prior-finding adjudication

### `CTDR-IB-R3-001` / `CTDR-IMPL-A-R6-002` — CLOSED and reconfirmed

The source-real guard evidence remains unchanged. Probe-side implications run
before the probed column evaluation and reconstruct the actual canonical
operands for all leaf and non-leaf fallible guards. Authentic branch witnesses
cover inactive, zero-PAR, respiration-dominated and positive assimilation;
rainy, dry, exact-capacity, zero-area, inactive-root, beta-one and low-light
cases; both potential and fixed-final solve classes; the authentic limiting
root; and all six exact zero-scale tolerance boundaries. Surface CO2 and VPD
remain correctly treated as crossable with source-real first-error, no-fallback,
and byte-complete rollback vectors. The focused 14-test replay suite still
passes after the import-only change.

### `CTDR-IB-R3-002` / `CTDR-IMPL-A-R6-003` — CLOSED and reconfirmed

Feature-enabled rustdoc again resolves the real public test-support wrapper and
passes both compiler-negative tests. The diagnostics observed during this
review reach the intended compiler properties: `E0382` on post-conversion reuse
of the consumed wrapper, and the concrete requirement that the source lifetime
outlive `'static`. The wrapper and method remain behind the default-off
`test-support` feature. No private capability mutation or test-only production
construction seam was introduced.

### Remaining R5/R6 closures — RECONFIRMED

- Capability consumption remains constructor-minted and O(1), with direct
  graph/component/occupancy/execution-plan custody and no proxy scan.
- Exact N=1/S=1 and N=2/S=6 graph, direct-edge, inclusive-closure, solve-class,
  component/sign, full-owner, and full/halved/line-search backtracking evidence
  is unchanged.
- Potential/fixed-final exact per-probe and dense-sweep parity, lifecycle,
  stencil/bucket/map/solve/sweep reconciliation, and authentic 58/14/16/28
  audit accounting remain unchanged.
- Audit-disabled record operands remain erased; graph construction and closure
  stay topology-cached, and candidate RSS sampling still precedes the
  forced-complete oracle.

No solver fallback, alternate numerical path, result cache, private mutation,
hardcoded N=2/S=6 production branch, or unaccounted hot-path graph/string/map
allocation was found.

## Ran evidence

- Ordered 16-file manifest: **PASS** at start and conclusion, exact digest
  `edc3f0b94c393e537b0115403548b779754f33308e53a26b1041932e9915be71`.
- Feature-enabled LSE all-target warnings-denied Clippy: **PASS**.
- Focused component-dependency replay library filter: **PASS 14/14**, with 140
  tests filtered.
- Full LSE library suite: **PASS 154/154**.
- Feature-enabled LSE rustdoc: **PASS 2/2**, with observed intended `E0382`
  moved-value and concrete lifetime-outlive diagnostics.
- Authentic single release-profile replay/forced-complete runner parity:
  **PASS 1/1**, including internal accepted-owner/backtracking records and
  byte-identical HBP, WAT Parquet, and PASS Parquet outputs.
- Runner all-target `cargo check` and `cargo fmt --all -- --check`: **PASS**.
  Runner checking emits only the inherited, non-v31 orchestrator dead-code
  warning for
  `CoveredTerminalTrialRequestV1::{coupling_iteration, ending_snow_hint}`.
- Exact revision-31 authority and structural-seam tests were not rerun in this
  import-only cut; their paths are byte-identical to the already-passing sixth
  cut, and the behavioral/doc/lint gates above provide proportionate
  no-regression evidence.
- Required three-run release timing/RSS gate: **NOT RUN**, as directed. This
  review does not infer or replace separate performance-retention approval.

## Non-blocking debt and follow-up

- `transaction.rs` remains 2,987 lines; runner qualification and
  `solver_covered_solve.rs` remain 2,941; and
  `solver_covered_evaluation.rs` remains 2,922. They remain below the 3,000-line
  ceiling, but transaction has little growth headroom.
- `non_stage3_branch_guards` remains a misleading test variable name because
  the enumerated `v10_*` gas branches are reachable under V11 represented-snow
  authority.
- Audit `begin` still replaces an active collector and `take` returns an empty
  default when none exists; typed misuse feedback would improve diagnostics.

QA disposition: **APPROVE**. The explicit-import correction closes the only
sixth-cut QA blocker, the feature-enabled warnings-denied gate passes, and the
focused, full-library, compiler-negative, and authentic output-parity checks
show no regression in the previously approved correctness evidence.
