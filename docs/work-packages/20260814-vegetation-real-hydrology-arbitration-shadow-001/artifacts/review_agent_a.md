# Independent Rust Correctness Re-Review

Status: `complete / GO`

Evidence class: `Static + Ran`

Reviewer role: primary Rust numerical and contract-correctness reviewer

Disposition: `GO`

## Findings

No unresolved correctness, science-contract, runtime, serialization, error-
taxonomy or duplication finding remains in the declared Child-2 boundary.

### RUST-REV-001 — High — CLOSED — aggregate authorization overbooking

The shared proportional allocator now canonically orders every source group,
uses a checked compensated demand reconstruction, bounds every authorization by
its demand and remaining source supply, assigns the closing positive request
the bounded remainder, and applies a deterministic one-ULP correction only if
the canonical returned sum still exceeds supply. Derived overflow fails with
`ResourceProtocolViolation::NonFinite` rather than producing a candidate.

The same `canonical_resource_amount_sum()` join is used by the shadow debit and
the vegetation receiving-owner validator. Regression vectors cover the
original binary64 overdraw witness, a distinct one-ULP canonical-sum witness,
individual `A <= D`, request-order reversal, finite-operand total overflow,
zero supply, and finalizing every returned maximum into a valid owner
candidate. This closes `SC-VEGETATIONTRANSACTION-001@2` steps 2 and 4 and the
bounded same-layer allocation obligation in `SC-WATBAL-001#INV-WATBAL-101`.

### RUST-REV-002 — High — CLOSED — reason precedence and eligible competition

Both the real owner and the shared vegetation validator now derive competition
from positive eligible requests, not raw request count. Exact-zero demand is
classified first; rooting and frozen exclusions precede supply classification;
zero beginning liquid produces `LiquidStorageLimit`; and
`CompetingDemand` requires multiple positive eligible requests and positive
beginning supply.

Vectors cover two positive requests on zero supply, a mixed eligible/excluded
same-layer set, one nonzero storage-limited request, competing eligible
requests, full supply, zero demand, frozen exclusion and rooting exclusion.

### RUST-REV-003 — Medium — CLOSED — duplicated soil-water aggregation

The ET path, subsurface path and shadow candidate now delegate to the single
`direct_runtime::aggregate_direct_soil_water()` implementation. The former
triplicated frozen-depth/residual-water arithmetic is gone, so clamp,
validation and unit behavior cannot silently diverge among those consumers.

### RUST-REV-004 — Medium — CLOSED — V7/V8 contract honesty

The package, source comments and owner artifacts now describe this endpoint as
an unchanged V7 potential/fixed-authorization re-solve inside a V8-precursor
root/OFE envelope. They no longer claim the complete V8 source identity or V8
constitutive solve. Complete resource/surface-class/optional-layer identity and
the joint root/ground batch remain explicit later-consumer obligations.

### RUST-REV-005 — Medium — CLOSED — signed-zero domain classification

Zero-demand, zero-supply and exactly-unfrozen predicates use numerical zero
equality, so `-0.0` and `+0.0` form the one exact zero class required by the
vegetation authority. The focused signed-zero vector exercises request,
beginning supply, frozen depth and frozen water together. It prevents a valid
negative zero from being mislabeled as positive demand, competition, or
partial frost.

## Other Reviewed Seams

- `D/A/F`: request, authorization and finalized-use identities remain typed and
  one-for-one; `0 <= F <= A <= D` is checked before owner debit. Unused
  authorization is not debited.
- State and rollback: the adapter seeds the actual production
  `DirectDayFrame`, retains the complete immutable beginning `DirectRunFrame`,
  builds the ending frame in a clone, and returns no candidate on validation or
  debit failure. Exact full depletion avoids the kg/m2-to-m round-trip residue.
- Serialization: the canonical byte sequence is truthfully a bounded
  arbitration projection. Whole-frame structural equality, not the bounded
  fingerprint alone, protects all other production state.
- Error taxonomy: identity, operand and bound failures retain typed
  `VEGTXN-E-001..003` categories; shared protocol failures map by category and
  partial frost fails explicitly as unsupported operand state. No production
  `unwrap`, `expect`, silent numerical default or canonicalize-and-proceed path
  was introduced.
- Source identity: transaction, interval, owner, OFE/lane, configured layer
  order, requester, water key, basis, beginning liquid and frozen fact are
  checked before the candidate is exposed.
- Multi-OFE and selectors: low-level multi-lane source separation is tested,
  while the public bridge explicitly rejects routed multi-OFE execution. The
  integration source guard proves no runner, selector or production dispatch
  path references the shadow API.
- Registration and impact: the module and integration target are registered;
  exact production owner, debit, subsurface aggregate, shadow and shared
  resource-transaction paths are present in the impact map. The admission gate
  reports 45 admitted contracts and nine changed science surfaces.
- Duplication: no substantial mirrored Rust science logic remains in the
  reviewed write set. The dependency-neutral direct owner delegates allocation
  to the shared kernel primitive, and aggregate storage delegates to the shared
  direct-runtime helper.

## Residual Risk And Missing Tests

- The exact-terminal comparator rerun passed after the review-driven numerical
  corrections: the orchestrator quick profile completed 507/507 and the
  admission gate reported 45 contracts and nine changed science surfaces. The
  subsequent changes are review/evidence Markdown only.
- Fingerprint tests establish canonical byte equality at the joins but do not
  mutate every bounded projection field independently. This is nonblocking for
  Child 2 because the complete frame is retained and compared structurally; a
  mature published serializer should add exhaustive field-sensitivity vectors.
- The intentional distinction between an exact small finalized debit and the
  native R4N legacy-ET `< 1e-10 m` clamp lacks a dedicated near-zero vector.
  `legacy-et-isolation.md` records the boundary, and importing that legacy
  clamp into this owner would be contract drift.
- Valid partial-frost production days and routed multi-OFE coordination remain
  explicitly unsupported. They require typed forcing/custody and scheduler
  work in the later real-consumer package; this GO is not evidence for them.
- Exhaustive phase-injection rollback through the later land-surface consumer
  remains a Child-4 obligation. Current tests cover the bounded Child-2 owner,
  authorization, finalization, debit and reconciliation failure seams.

## Ran Evidence

On the exact reviewed worktree:

- `cargo test -p openwepp-kernel-contract --lib -- --nocapture` — PASS, 55/55.
- `cargo test -p openwepp-vegetation --lib -- --nocapture` — PASS, 227/227.
- `cargo test -p openwepp-biogeochemistry --lib -- --nocapture` — PASS, 6/6.
- `cargo test -p openwepp-hillslope-orchestrator vegetation_real_hydrology_shadow -- --nocapture` — PASS, 13/13.
- `cargo test --test vegetation_real_hydrology_shadow_contract -- --nocapture` — PASS, 3/3.
- `cargo clippy -p openwepp-kernel-contract -p openwepp-vegetation -p openwepp-hillslope-orchestrator --all-targets -- -D warnings` — PASS.
- `cargo fmt --all -- --check` — PASS.
- `git diff --check` — PASS before writing this report.
- `bash tools/release/check_science_contract_admission.sh --base-ref 0db196012 --worktree` — PASS, 45 contracts and nine changed science surfaces.
- `markdown-doc lint --path docs/work-packages/20260814-vegetation-real-hydrology-arbitration-shadow-001` — PASS, 22 files and zero findings before writing this report.
- Exact-terminal comparator suite recorded in `comparator-results.md` — PASS,
  including affected-crate checks/strict Clippy, authority suites, anti-evasion,
  507/507 orchestrator quick tests, admission at 45 contracts/nine surfaces,
  formatting, diff hygiene and package Markdown lint, with no retries.

## Line-Count Governance

`WARN accepted`: `vegetation_real_hydrology_shadow.rs` is 2,118 lines. The
package records why the bounded Child-2 increment co-locates private seam tests
and requires a snapshot/arbitration/candidate/V7-bridge split before Child 4.
This is not a waiver for further growth.

## Final Disposition

`GO`. The declared default-off, single-OFE Child-2 shadow boundary is
numerically bounded, identity-complete for its stated V8-precursor envelope,
rollback-safe, selector-isolated and free of substantial duplicated science
logic. No Rust correctness blocker remains. Package closure still requires the
remaining independent terminal-verifier disposition and must preserve the
documented partial-frost, routed multi-OFE and activation exclusions.
