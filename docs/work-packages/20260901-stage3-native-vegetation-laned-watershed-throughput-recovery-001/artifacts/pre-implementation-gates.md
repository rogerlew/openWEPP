# Pre-implementation gates

Status: `PASS — PHASE 3 AUTHORIZED`

Evidence mode: `Static + Ran`

## Intake and baseline

- exact source: `a28c55c2d0f06e0c4aab58642f1009f70f82b3d3`;
- live tracked tree was clean at kickoff; four unrelated untracked files under
  `tmp/pdfs/r156-review/` are preserved;
- optimized inherited DFF run: FAIL at simulated `1800..1860 s` after `33.56
  s` test wall; no completed day; command wall including release build `335.64
  s`; log `/tmp/phase1-baseline-logs/phase1_baseline_20260901T200857Z.log`,
  SHA-256 `cd88148824402cc0ec93af01592f6b4e546421e5616cda723c521df93b968d0`;
- production runner/Lane D trace: BLOCKED because current runner rejects the
  selectors and accepted Stage 3 publication returns `laned_active: None`.

## First independent design review

`rust_code_reviewer`: HOLD. High findings: incomplete ordinary-solver/regime
classification; receipt reseal ambiguity; conflated closure tolerances;
ungrounded four-map cap; unmeasured engineering targets; ambiguous Lane D
units; unresolved snow contract gaps 011/014/015/016.

`rust_qa_reviewer`: HOLD. Critical/high findings: workload and sensitivity
evidence improperly sequenced; Lane D unmeasurable on the real runner; the
purported native century fixture is only 45 years; tolerance tests did not bind
the production seam; release evidence covered a test executable rather than
the CLI; sampling/projection protocol and operand lineage were insufficient.

No canonical contract, production solver, or production consumer file was
edited before these verdicts.

## Amendment disposition

- Package phases now distinguish inherited baseline characterization and
  prospective authority from mandatory post-implementation production-seam
  sensitivity and representative qualification. The latter remain blocking
  Phase-3/4 gates.
- The deletion map now names ordinary Picard, every historical selector and
  solver call site, and an exhaustive physical regime partition. Frozen,
  mixed, thaw/refreeze, and terminal-crossing states are active sets/events of
  one covered conservation system, not solver regimes.
- The tolerance artifact separates exact transfer custody, `1e-9 J m^-2` CN
  endpoint reconstruction, and `1e-6 J m^-2` physical closure; defines
  construct-once receipt sealing; retains exact branch/topology identity; and
  binds the full sensitivity matrix to the future production seam.
- The evaluation budget is eight authentic maps with explicit charge
  accounting and no coordinate finite differences or physical replay tail.
- The performance SLO is labeled as an engineering qualification target,
  derives the `500 us/OFE-day` allocation from the 100-year limit, and uses
  batched 30-sample micro measurements plus complete long-run samples.
- The H.J. Andrews input is correctly classified as 45 years. A constructed
  century stress fixture must assert exactly 36,525 days and cannot claim
  independent climate validation.
- Lane D operand lineage now fixes metres/metres-per-second/cubic-metres and
  the lane-area-to-mesh-area conversion at exact consumer call sites.
- The exact authority write set now includes every potentially affected
  canonical contract, and the new integration-test filename is fixed as
  `stage3_native_vegetation_laned_throughput_recovery.rs`.

## Release-runner characterization

The release CLI build passed; binary SHA-256 is
`9bda52dd0f1b581428b45d1f18ed19e629a104026f46ab98a64053d12c852975`.
The native 16,438-day/1-OFE and H2637 732-day/19-OFE inputs both failed before
day zero with typed missing Stage-3 owner-seed validation. Durable logs and
hashes are recorded in `performance-budget.md`. This closes the Phase-1
characterization action, not any throughput claim.

## Phase-2 authority and expected red

Canonical non-versioned amendments now bind `INV-SNOWENERGY-082`,
`TOL-SNOWENERGY-007`, and cross-contract Stage-3/LSE/coupled-time/
surface-liquid/runoff/Lane-D/water-balance custody. No V58 or Lane receipt V2
was introduced. Focused command:

`nix develop -c cargo test --test solver_architecture_authority_contract --test stage3_native_vegetation_laned_throughput_recovery -- --nocapture`

Result: contract/architecture `5 passed`. Review rejected the initial three
source-string reds as insufficiently behavioral. They were replaced before
production edits by (a) a crate-private production-admission boundary/budget
matrix that currently fails compilation only because the canonical seam types
do not exist and (b) a subprocess test that executes the normal authenticated
Stage-3 runner with a warm 50-mm liquid day and no historical selectors; it
fails specifically because the real direct runner does not configure the
authenticated Stage-3 -> Lane-D seam before publication. Full command,
toolchain, tested-source identity, exit status, and diagnostics are retained in
`artifacts/logs/phase2_expected_red.txt`. The exhaustive repository production
source anti-accretion scan remains an integration expected red.

## Independent Phase-2 design acceptance

The exact tested-source digest is
`667f21801dab4acc24effaed3018a39b4540d3201acbc6a4a75cf7e3cf05cfd7`.
Both independent reviewers reproduced it. Correctness: PASS; QA: PASS; both
explicitly authorized Phase 3. The accepted design includes the complete
wrong-tolerance/cross-coordinate/CoE/nonfinite/rollback matrix, exact live
N-owner runner topology, typed wrong-area association, all-`cfg` production
visibility rejection, and structural proof that the real covered entry invokes
one canonical seam without transforming or discarding its result.

## Covered nonfinal physical-only contract-first increment

Static: profiling of the canonical covered path showed every nonfinal charged
map constructed the full V8/vegetation/material/BGC/joint/complete-owner
envelope even though only the independently charged final map can publish it.
The cross-contract amendment preserves the physical map, native versus
ordinary regime, role, ordinal, support, eight-map budget, same-solver adaptive
response, rollback, and exact final publication. It gives nonfinal roles only
a private unpublishable physical endpoint and reserves complete owner custody
for `FinalAccepted`.

Ran: strict Binding Exposure Index checks passed for all five affected
contracts; the scoped unit-governance lint and diff check also passed. Log:
`artifacts/logs/covered_nonfinal_contract_static_gates_20260903T0142Z.log`.

Ran: the contract-derived test
`canonical_covered_nonfinal_maps_are_private_physical_endpoints` selected one
test and failed at the first absent role-specific production type,
`CanonicalCoveredIterationMapV1`. This is the required pre-implementation red;
all contract IDs, versions, obligations, and exposure rows passed first. Log:
`artifacts/logs/covered_nonfinal_physical_only_expected_red_20260903T013512Z.log`.

Production implementation remains unauthorized for this increment until two
independent contract reviews, complete finding disposition, and two independent
verification verdicts pass.

Both initial reviewers returned `HOLD`. All twelve findings were accepted and
amended before production edits. The amendments now separate private final
envelope construction from accepted-parent publication, define exact success
and three failure count matrices, bind zero-based role ordinals, make native/
ordinary regime dispatch orthogonal to map role, remove provisional-reuse
authority from canonical covered finalization, map exact typed failures, and
reconcile lifecycle metadata. The complete disposition and readiness matrix
are under `artifacts/science-contracts/covered-nonfinal-physical-only/`.

Ran: the strengthened executable behavioral pre-red failed only because its
test-only role-audit, failure-injection, parity, and poison APIs are absent;
`artifacts/logs/covered_nonfinal_behavioral_expected_red_20260903T015519Z.log`.
The supplemental source anti-evasion red is
`artifacts/logs/covered_nonfinal_source_antievasion_expected_red_20260903T015716Z.log`.
The first dual verification rejected incomplete counter, native-zero-work,
privacy, and typed-error assertions. Every finding was accepted. The amended
behavioral population now binds exact role cardinality and trial index,
success/pre-final/final-physical/final-constructor matrices, seven ordered
constructor counters, native litter/surface/WB14 zero calls, retained inactive
bytes, all six regime/poison cases, exact error class and precedence, rollback,
and zero fallback/promotion. The source guard also rejects derived/manual wire
traits and `Clone`/`Copy` on the physical-only result.

Ran: all five strict BEI checks, scoped unit lint, and diff check passed. The
latest behavioral and source populations fail only on deliberately absent
implementation APIs/types:

- `artifacts/logs/covered_nonfinal_contract_reverification_static_gates_20260903T0205Z.log`;
- `artifacts/logs/covered_nonfinal_behavioral_expected_red_reverification_20260903T0211Z.log`;
- `artifacts/logs/covered_nonfinal_source_antievasion_expected_red_reverification_20260903T0207Z.log`.

Ran/Static: repeat independent verification returned `PASS` and
`PASS-WITH-NOTES`. The sole traceability note was immediately recorded as
`VB-CNFP-002`; no technical or science-authority finding remains. Full-file
review manifest:
`ea628948b21522359c52a788a7e065f4f1d2428f8c2ff59fe1642bd282ee4b44`.
Production implementation of this bounded amendment is authorized; runtime
conformance remains pending.

The first production attempt exposed a test-authority defect before any green
claim: its six-regime parity and poison helpers reused one ordinary equilibrium
run and assigned expected booleans, error classes, rollback flags, and
constructor counts instead of observing the named real path. That attempt was
rejected. The package exact write set was amended before further fixture edits
to include the existing native-resident and WB14 cadence test modules. Runtime
conformance now requires installing real compatible V3/V4 residents, executing
case-specific physical operands through the actual cadence path, injecting
poisons at production validators, and incrementing counters only at the real
constructor/validation boundaries. The final map must consume the same retained
physical endpoint into its complete continuation without a second physical
evaluation.

## Shared immutable terminal-carrier custody increment

Static, prospective intent recorded 2026-09-04 before production edits:

- classification: performance-preserving runtime custody/data-movement
  architecture; no process equation, regime predicate, tolerance, physical
  map, adaptive cadence, receipt identity, public wire schema, or restart
  schema changes, so no science-contract amendment is required;
- problem: the terminal provider already stores candidate generations behind
  shared ownership, but provider selection, carrier construction, complete
  finalization, and result retention deep-clone the same immutable beginning
  `DirectV10RealConsumerShadow` several times per direct/split trial;
- intended correction: mint a private non-wire shared beginning handle only
  after the existing joint, owner-byte, soil-seal, and chronology validators;
  validate selection without allocating; preserve immutable beginning
  custody; and deep-materialize exactly one mutable ending shadow after
  physical success. No `Arc::make_mut`, external constructor, or pointer-based
  science identity is authorized;
- expected-red surface: focused tests must initially lack the shared typestate
  and one-materialization audit, then prove exact legacy-deep-copy/shared-path
  equality, unchanged typed-error precedence and two-map order, support retry
  and contiguous-child chronology, stale/wrong-support/joint/soil-seal poison
  rejection, and beginning/ending mutation isolation;
- retention gate: authentic release output and the exact
  `48/56/20/32/4` parent/publication/direct/split/microstep counts remain
  unchanged, while the provider custody/adoption/owner/retention buckets and
  total improve. Revert if a fresh clone/sub-bucket audit shows less than
  roughly `0.10 s` recoverable or target-plus-total timing does not improve;
- protected boundary: independent/restart-supplied values continue through
  existing constructors and validators. The shared handle is private runtime
  custody only and cannot admit, serialize, publish, or mutate state.

Ran outcome 2026-09-04: the expected red was genuine, focused implementation
and behavioral review passed, and three exact release diagnostics preserved
science/count identity. The total improvement was only `6.6--23.3 ms`, below
the prospective `0.10 s` retention threshold. The increment was rejected and
reverted; no shared terminal-carrier custody remains in production.

## Rejected outer adaptive terminal-trial finalization candidate

Static, prospective intent and rejection recorded 2026-09-04 before any
completed contract, test, or production edit:

- classification: contract-first temporal/transaction custody architecture;
  physical equations, all direct and composed child evaluations, comparison
  values, adaptive tolerances, event localization, minimum support, and
  accepted chronology remain unchanged;
- measured problem: the exact one-OFE diagnostic attributes `752,618 us`
  outside the carrier after subtracting provider-carrier, custody, projection,
  retention, and final result attribution from direct-plus-composed terminal
  execution. The current path constructs coupled subslab, complete-owner,
  accepted-clock, receipt, and publication-capable shells for 52 terminal
  trial children although only four microsteps are accepted;
- intended authority: a private, move-only, non-Clone, non-wire
  `NonfinalAdaptiveTerminalTrialV1` retains the authentic child's physical
  endpoint, full prognostic/discrete comparison operands, ledgers, support,
  identity, and terminal-event evidence. A minimal private trial-clock/slab
  identity may exist only where child-2 physical forcing and receipt identity
  require the exact child-1 predecessor; it is non-wire and never becomes
  accepted chronology. Direct/composed adjudication drops every rejected or
  unselected token and consumes only the selected direct or ordered composed
  tokens into the existing complete-owner/subslab finalizer;
- chronology constraints: composed child 2 always begins from child 1's exact
  physical ending; all 52 physical evaluations remain mandatory; direct and
  composed comparison is complete; rejected trials advance no accepted clock,
  history, owner, receipt, restart, or publication state; selected finalization
  preserves existing typed error precedence and atomic rollback;
- expected-red gates: forced-old complete-envelope differential parity for
  ordinary/native, event/no-event, and multilane cases; exact zero rejected-
  trial live-parent/live-clock/history/publication/complete-owner/owner-join/
  publishable-subslab counters and exactly one selected finalization per
  accepted microstep; separate counters may prove the minimum private trial
  chronology needed by composed child 2; move-only/no-serde/no-promotion source
  guard; identity, support, joint, topology, receipt, and one-ULP poison
  failures before live mutation;
- retention gate: three exact CPU-0 warm release diagnostics must preserve the
  source/outlet/storage/clamp values and `48/56/20/32/4` parent/publication/
  direct/split/microstep counts, keep RSS within `65,536 KiB`, and improve both
  the outside-carrier bucket and total runtime by at least `0.10 s` against the
  retained current-source diagnostic. Otherwise revert the increment;
- authority gate: amend `SC-SNOWENERGY-001`, `SC-COUPLEDTIME-001`, and
  `SC-VEGETATIONTRANSACTION-001`, add contract-derived expected-red tests, and
  complete two independent reviews, finding disposition, and two independent
  verifications before any production edit.

Static outcome: rejected before authority amendment. The detailed source audit
proved that composed child 2 consumes child 1's finalized consumer, private
accepted clock, complete-owner digest, WB14 working/replay state, physical
ledger set, lane/destination receipts, and (for terminal events) the accepted
zero-duration event mutation. Those products do not exist in the terminal
physical endpoint. Supplying them would require a new private equivalent of the
complete owner/clock/event receipt shell and would recreate most of the claimed
`752,618 us` opportunity. No `INV-088/C-056` authority, expected-red test, or
production typestate is retained. A narrower slab-independent precomputed-
payload sharing idea remains unselected because static evidence cannot support
the `0.10 s` retention threshold.

## Carrier structural-admission characterization

Static, prospective diagnostic intent recorded 2026-09-04 before profiling
edits:

- classification: test-build-only timing attribution under the existing
  opt-in adaptive-parent telemetry; this increment changes no production
  semantics and therefore precedes, but does not replace, any required
  contract-first implementation cycle;
- candidate: `SC-LANDSURFACEENERGY-001#INV-LANDSURFACEENERGY-159` could support
  one parent-generation-bound private structural admission plus one exact
  pointer-bound per-map forcing token, but only for immutable configuration,
  topology, and normalized forcing. Dynamic state, support, transaction,
  lower-boundary, hydrology, soil, surface, vegetation, BGC, residual, solver,
  and output validation remain mandatory on every one of the 52 maps;
- diagnostic buckets: duplicate forcing normalization/reseal/validation/hash;
  V8 immutable configuration/topology validation and index construction versus
  dynamic V8 projection; native V3 immutable configuration/index work versus
  dynamic native projection;
- pre-kill rule: do not amend authority or implement a reusable capability if
  the exact release diagnostic attributes less than `0.10 s` to safely
  removable immutable-static plus duplicate-forcing work;
- later retention rule if admitted: a V30 binding mapped to existing
  `INV-159` plus new `OBL-LANDSURFACEENERGY-C-019`, dual contract review and
  verification, exhaustive full/admitted differential and poison gates, and
  three exact CPU-0 warm runs improving both target and total medians by at
  least `0.10 s` with unchanged science/counts and RSS `<=65,536 KiB`.

Ran characterization: immutable configuration/topology plus duplicate forcing
work measured only `39,740 us`, so that cross-map cache alone is rejected. The
additional `63,319 us` native-V3 bucket validates the current LSE and surface
owner and is not cross-map-static. It may only be removed through a distinct
validation-once handoff sourced from the exact `FrozenLitterV3Resident`
validated revision at the existing native-validation position. V8 validates
distinct structural objects and does not attest to the resident V3/V2 objects.

## Selected combined parent-static and same-map validation-once increment

Static, prospective intent recorded 2026-09-04 before contract or production
edits:

- authority scope: existing `SC-LANDSURFACEENERGY-001#INV-159` only; add a
  V30 binding mapped to that invariant and a new `OBL-LANDSURFACEENERGY-C-019`.
  No new physics invariant or process-solver version is created;
- parent scope: cache only immutable configuration identity, topology,
  canonical indexes, and normalized forcing authority behind a private
  generation-bound, non-Clone/non-wire capability. Every map still validates
  its support, transaction, joint, dynamic state, boundaries, hydrology, soil,
  surface, vegetation, BGC, residual, solver, and output;
- native resident scope: after V8 and the fallible ingress-schedule derivation,
  join the exact immutable `FrozenLitterV3Resident` to its existing validated
  revision at the current native-validation position and mint a borrowed,
  map-bound move-only proof consumed immediately by native V3. Native V3 keeps
  the exact resident reference/identity join but does not repeat full LSE-state
  semantic validation or surface-owner canonicalization. The resident revision
  may remain only with the unchanged whole resident; the borrowed proof never
  survives the map, and V8 supplies no resident authority;
- measured opportunity: `39,740 us` parent-static/forcing plus `63,319 us`
  duplicate same-map validation, total `103,059 us`. This is only `3,059 us`
  above the retention floor, so any smaller implemented target or noisy total
  result requires full reversion;
- required expected red: authentic `1/52/52` parent-static/map-forcing/dynamic-
  projection counts, full-versus-admitted bitwise physical and final-owner
  parity for direct/Half1/Half2 and ordinary/native cases, exact call order,
  foreign generation/configuration/topology/support/transaction/forcing/
  state/surface/restart poisons, no digest-only admission, no cached dynamic
  projection/result, unchanged error precedence, and byte-identical rollback;
- retention: three exact CPU-0 warm runs must improve both target subtotal and
  total median by at least `0.10 s`, preserve exact science and
  `48/56/20/32/4` counts, and keep RSS `<=65,536 KiB`; otherwise revert.

Disposition: `REJECTED_BEFORE_BENCHMARK`. Dual contract review and verification
authorized the bounded attempt, but implementation proved that the plan must be
created above all adaptive stacks and borrowed through the complete terminal,
batch, single, and subslab graph. Those stacks deep-clone their configuration
objects, so a per-stack pointer-bound plan cannot establish one-parent custody
or authentic `1/52/52` accounting. Forcing authority also has to be minted from
the live prepared allocation before the ordinary/native split. The partial
implementation was fully reverted without weakening the expected-red tests.
See `artifacts/science-contracts/carrier-parent-static-validation-once/implementation.md`.

## Terminal precomputed-package attribution

Prospective diagnostic intent recorded 2026-09-04 before instrumentation:

- measure the three authentic `precomputed_terminal_package_v1` call sites and
  separately attribute carrier-chain validation, endpoint coalescence,
  payload/map/vector cloning, authority sealing, and test-audit cloning;
- use only test telemetry on the exact CPU-0 release probe and preserve exact
  science, `48/56/20/32/4` counts, and RSS evidence;
- do not select the narrower slab-independent shared-payload architecture
  unless removable repeated work is at least `0.10 s` and real consumer call
  counts prove the same physical payload is rebuilt or cloned across distinct
  slab bindings;
- revert all diagnostic Rust changes after recording the raw result. Any later
  custody change requires contract-first authority, independent review, and
  verification before production edits.

Disposition: `REJECTED_AND_REVERTED`. Exact CPU-0 release evidence at Rust
manifest `b71377b40c408db71ce49a6ae1280f57100df2447936c673a0001896dff4c73d`
and binary
`2300fbcfc62efd88fb5d70c8cc798dd2f33d79ce3266b9437f18cb30365bc968`
preserved exact science, complete telemetry, `48/56/20/32/4` counts, and
`61,212 KiB` RSS. It proved 20 slab-independent payload groups rebuilt across
distinct accepted-slab bindings, but all 108 authentic constructions consumed
only `29,937 us`. The candidate therefore fails the `0.10 s` materiality gate;
no custody contract or production implementation is authorized. Raw selected
output is retained in
`artifacts/terminal-heavy-gates/terminal_precomputed_attribution_one_ofe_release.log`.
The clean profiler revert produced current Rust manifest
`6e3f339d35ac2efa859bc51b7f08ce611dcd127800a54d83d6a781c2353ca906`;
the exact profiler-symbol search was empty, the instrumented package source
matched `HEAD`, and formatting, orchestrator check, runner check, and diff
hygiene passed.

## Physical-evidence assembly attribution

Prospective diagnostic intent recorded 2026-09-04 before instrumentation:

- measure all 52 authentic carrier maps and separately attribute the first
  Stage-3 beginning-state projection, the two later state digest/projection
  traversals, first and repeated V2 soil authority/operand construction,
  terminal snow/soil receipt construction and immediate validation/reseal,
  surface/WB14 replay copy and rehash, effective-surface hydrology-frame
  installation clone, adapter clone, and dynamic hydrology snapshot rebuild;
- expose inclusive enclosing and exclusive stage times plus exact call counts,
  and prove repeated work consumes the same immutable allocation/state/receipt
  identity within one map rather than merely equal bytes across maps;
- use only opt-in diagnostic telemetry in the exact CPU-0 release probe;
  preserve exact source/outlet/storage/clamp, complete telemetry,
  `48/56/20/32/4` workload counts, and RSS evidence;
- select production work only if the sum of source-proven removable duplicate
  stages is at least `0.10 s`. Do not count the required first construction,
  physical solve, independent canonical-solver ledger reconstruction, final
  accepted-owner reconstruction, publication/output/restart validation, or
  overlapping inclusive buckets;
- revert every profiler-only Rust change after recording the result. A selected
  implementation remains contract-first and requires independent review and
  verification before any production edit.

Disposition: `REJECTED_AND_REVERTED`. Exact CPU-0 evidence at Rust manifest
`b9f0e1018e72ac0aded97aa1d4004cba3e670d7ec856baa9e662f8584415155f`
and binary
`e54e71af09af60e1c7c993011bc794a9355ce8c11e6ac7987085ad0b9f73780e`
observed 400 complete physical-evidence map scopes, 800/800 matching same-map
joins, zero mismatch/incomplete scopes, `27,257 us` across all measured stages,
`19,136 us` in required/mixed work, and only `8,121 us` in the six
source-proven later duplicates. The 20 direct plus 32 split-child counters are
adaptive trial topology, not one-to-one physical-map calls; the instrumentation
corrects that static estimate. Science/counts and complete telemetry passed,
but RSS was `70,572 KiB`. The candidate fails both materiality and diagnostic
RSS gates, so no contract or production implementation is authorized. Raw
selected evidence is retained at
`artifacts/terminal-heavy-gates/carrier_physical_evidence_attribution_one_ofe_release.log`.

Cleanup note: every profiler symbol and local timer name was removed, and
workspace formatting, orchestrator check, runner-tests check, and diff hygiene
passed. Exact byte restoration to the pre-diagnostic `6e3f...` Rust manifest
was not possible because the diagnostic worker directly formatted standalone
`include!` fragments that workspace `cargo fmt` does not normally rewrite and
no per-file pre-edit hashes/snapshots were captured. Three captured
`carrier_engine.rs` layouts were restored; the remaining source is
behavior-equivalent, profiler-free formatting residue. The reconciled current
Rust manifest is
`86e30dd6fdcad99b77c67641379c08af70f5faf68acf441797f3e006d1b461bb`.
No exact-revert claim is made for this diagnostic.
