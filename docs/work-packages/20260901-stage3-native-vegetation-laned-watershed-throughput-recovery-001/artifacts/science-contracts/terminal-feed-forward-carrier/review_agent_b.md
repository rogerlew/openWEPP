# Independent contract-first QA review B

Latest verdict: `PASS` for final corrected ordered manifest
`a8a6677672c7f8f34acdaca30f3b94388902d022fe5886f28e4532762ffce804`.
The final narrow rereview at the end of this artifact supersedes the prior
manifest verdicts and first-pass `HOLD` for this revision.

Evidence mode: `Static + Ran` (read-only manifest/hash reproduction; no Cargo
command was executed by this reviewer).

Review target: exact ordered four-file manifest
`8bc5f02fe1f3f777fc01f7f488c7f14ad068139314acff3fe5e83ac389967be3`.
The documented recipe reproduced that value exactly:

```text
sha256sum docs/specifications/science-contracts/contracts/SC-SNOWENERGY-001.md \
  docs/specifications/science-contracts/index.md \
  tests/integration/snow_terminal_enthalpy_event_numerics_contract.rs \
  docs/work-packages/20260901-stage3-native-vegetation-laned-watershed-throughput-recovery-001/package.md \
  | sha256sum
```

## Findings

### `FF-B-001` — HIGH — the exact `400 -> 200` premise is not reproducible from the cited evidence

Paths:

- `docs/specifications/science-contracts/contracts/SC-SNOWENERGY-001.md:1338`
- `docs/specifications/science-contracts/contracts/SC-SNOWENERGY-001.md:3400`
- `docs/specifications/science-contracts/contracts/SC-SNOWENERGY-001.md:3442`
- `docs/work-packages/20260901-stage3-native-vegetation-laned-watershed-throughput-recovery-001/package.md:158`
- `docs/work-packages/20260901-stage3-native-vegetation-laned-watershed-throughput-recovery-001/artifacts/science-contracts/terminal-feed-forward-carrier/contract_ref.md:12`
- `docs/work-packages/20260901-stage3-native-vegetation-laned-watershed-throughput-recovery-001/artifacts/terminal-heavy-gates/carrier_physical_evidence_attribution_one_ofe_release.log:26`

The retained release attribution proves `map_started_count=400` and
`map_completed_count=400`. Its `identity_join_count=800` is explicitly two
later Stage-3 joins inside each measured map scope; it is not a count of
terminal coupling groups. The contract-cycle summary then asserts that those
400 calls are exactly 200 logical provider groups, each with exactly two
successful calls, bitwise-identical outputs, and exact-zero coupling deltas,
but it names no capture artifact, command, source/binary identity, hash, or
per-group record that reproduces that second inference. Current source does
show a generic `0..32` loop and a separate real-capture fixture checks two-call
groups, but that does not prove that the exact release workload represented by
the retained 400-call log contains exactly 200 such groups.

This is closure-blocking because an aggregate `200` result can still omit one
required group and duplicate another, or remove calls outside the intended
generic replay. Before production edits, retain a source-identified artifact
that joins the exact 400 provider calls to an exact logical-group-key multiset.
For every key, record role, discovery/exact path, support, attempt, beginning,
forcing, topology, owner identity, iterations exactly `[0, 1]`, both successful
result identities, and exact-zero deltas. The same oracle should define the
post-change one-call multiset and separately account for terminal-batch and
canonical final-map calls that are intentionally unchanged. Cite that artifact
and its hash from the contract guard/evidence map; otherwise downgrade the
`[DIRECT][Ran]` and exact-count claims.

### `FF-B-002` — HIGH — the expected-red gate reads a different behavior-test source than the package assigns

Paths:

- `tests/integration/snow_terminal_enthalpy_event_numerics_contract.rs:106`
- `tests/integration/snow_terminal_enthalpy_event_numerics_contract.rs:108`
- `tests/integration/snow_terminal_enthalpy_event_numerics_contract.rs:119`
- `docs/work-packages/20260901-stage3-native-vegetation-laned-watershed-throughput-recovery-001/package.md:1010`

The expected-red test searches the literal contents of
`v9_real_consumer_shadow_wb14_tests.rs`, while the package assigns revision-61
role/count/reference/poison/rollback tests to
`runoff_reconciliation/cqr_row5_tests.rs`. `read_to_string` does not expand
Rust `include!` files, and the CQR file is not part of the string searched by
this assertion. Implementing the tests at the declared write-set location can
therefore leave the production-seam gate red after a correct implementation.
Conversely, the current three substring checks can be made green by inert
markers without proving that executable tests exist or cover the contract
matrix.

Align the gate and package ownership before production work. Read the actual
declared test source(s), require independently runnable test cases/audit output
for the exact group multiset, forced-reference parity, competing-poison error
precedence, and rollback/publication state, and record their exact commands.
The gate must not become green from symbol-name presence alone.

### `FF-B-003` — HIGH — no numeric post-implementation performance-retention gate is bound to revision 61

Paths:

- `docs/specifications/science-contracts/contracts/SC-SNOWENERGY-001.md:3442`
- `docs/specifications/science-contracts/contracts/SC-SNOWENERGY-001.md:3465`
- `docs/work-packages/20260901-stage3-native-vegetation-laned-watershed-throughput-recovery-001/package.md:158`
- `docs/work-packages/20260901-stage3-native-vegetation-laned-watershed-throughput-recovery-001/artifacts/science-contracts/terminal-feed-forward-carrier/readiness-matrix.md:26`

The contract binds the call-count and science/counter result, and the readiness
matrix says only that wall-time retention remains post-implementation. Neither
the reviewed package increment nor the revision-61 obligation defines a
numeric baseline, target subtotal, total-runtime rule, repetitions/statistic,
RSS ceiling, or mandatory reversion disposition. This package has already
reverted other candidates when their target-plus-total retention failed; the
new selected throughput correction needs the same prospective rule. Without
it, a costly adapter/audit implementation can satisfy `400 -> 200` while
regressing total runtime and still appear contract-complete.

Before implementation, bind the exact current-source/binary baseline and the
post-change command/protocol. Require both the target carrier subtotal and
whole-run statistic to improve by the prospectively selected material amount
(or explicitly justify a different package threshold), retain exact
source/outlet/storage/clamp and `48/56/20/32/4`, enforce the existing RSS
ceiling, and require full reversion on retention failure. Keep this engineering
SLO outside process-physics tolerances, but make it a package increment gate.

## Non-blocking debt and accepted surfaces

- The manifest recipe and ordered identity are reproducible exactly.
- The expected-red classification is truthful at this manifest: the three
  intended production symbols and three behavior-test names are absent.
- Contract-level coverage is otherwise strong: it retains all named adaptive,
  discovery/exact/final and batch distinctions; requires complete-result
  parity; includes sole-call, binding, duplicate, feedback, cross-group,
  partial-output, exact/final, and retry-rebase poisons; and requires exact
  error precedence, byte-identical rollback, zero intermediate retention or
  publication, consumer closure, and publication chronology. `FF-B-001` and
  `FF-B-002` are about making that matrix executable and non-tautological, not
  weakening it.
- `calibration_evidence_status = NOT_APPLICABLE` and
  `identifiability_status = NOT_APPLICABLE` are appropriate: revision 61 adds
  no parameter, observation operator, objective, constitutive equation,
  tolerance, or empirical claim.
- Registry revision/date/no-V61 wording agrees with the canonical contract.

## Recommendation

`HOLD` before production edits. Correct `FF-B-001` through `FF-B-003`, issue a
new exact four-file manifest, disposition the findings, and obtain independent
verification. The authority direction is maintainable and testable once these
reproducibility and retention gates are made concrete.

## Corrected-manifest rereview

Evidence mode: `Static + Ran-record review`. This reviewer reran only cheap
read-only hash, count, stale-pin, diff-whitespace, and source-inspection
commands. The Cargo, binding-exposure, unit-compliance, and formatting results
below are recorded execution evidence inspected from `contract_ref.md`; this
reviewer did not rerun those gates.

Review target: exact ordered four-file manifest
`98306e53fe56612c1d0213dc41693f3a8578a1a3b1bf0df84e89eee711f1ee7f`.
The documented ordered recipe reproduced that value exactly. `git diff
--check` is clean for all four files.

### Finding dispositions

| Finding | Original severity | Rereview disposition | Evidence |
| --- | --- | --- | --- |
| `FFTC-A-001` | HIGH | `CLOSED` | `INV-SNOWENERGY-088` now makes exactly-one local to each typed evaluator invocation. Lane, support, role, attempt, beginning state/joint and owners, forcing, topology, mode, and trial-start coordinates remain bound. Equal bits across outer invocations cannot authorize reuse; discovery/exact, single/batch, and same-map `FinalAccepted` paths remain independent. No global key or duplicate registry is authorized. |
| `FFTC-A-002` | HIGH | `CLOSED` | The integration assertion is explicitly a structural pre-red and now scans the package-owned CQR source. `contract_ref.md`, the readiness matrix, and C-056 make independently executed in-crate behavior tests plus compile-time negative-capability evidence the postimplementation acceptance authority. Symbol presence alone cannot close the obligation. |
| `FFTC-A-003` | HIGH | `CLOSED` | Static recount finds 26 revision-61 assertions and no revision-58 assertion. The target has 62 tests, 22 ignored, hence 40 active tests; recorded execution reports 39 pass and exactly the one named structural expected-red, consistent with that inventory. Historical process-version prose remains unchanged. |
| `FFTC-A-004` | MEDIUM | `CLOSED` | The core Guard Map follows source-real order: outer validation, request construction, provider attachment/projection/owner/forcing/topology validation, the sole carrier and its validation, boundary/result join, terminal transition, then separate exact/final completion. Current sources confirm the boundary carrier performs production-physics and duration validation before transition. The map uses existing `Kernel`, `TurbulentTransfer`, `TerminalNumerics`, and `TerminalCustody` variants, retains the provider wrapper's exact custody behavior, and adds no invented error, hoisted validator, or fallback. |
| `FF-B-001` | HIGH | `CLOSED` | The exact-release pre-change claim is now only the directly observed 400 complete calls. The proposed 200 pre-change groups are expressly an inference from static and focused-capture evidence. Exact-head acceptance instead requires the post-change exact 200-invocation role/path multiset while retaining `20/32/4` adaptive topology, supports, attempts, decisions, and chronology. |
| `FF-B-002` | HIGH | `CLOSED` | `FEED_FORWARD_TESTS` now resolves to `runoff_reconciliation/cqr_row5_tests.rs`, matching package ownership. The structural assertion is truthfully red: all three production symbols and all three named behavior tests are absent. The full recorded target has exactly that one structural failure; behavioral acceptance remains a separate execution requirement. |
| `FF-B-003` | HIGH | `CLOSED` | C-056 and the package bind one source/binary-identified baseline and an unambiguous keep/revert gate. The ceilings exactly equal each baseline minus `750,000 us`: provider `2,049,833 -> 1,299,833 us` and total `4,984,488 -> 4,234,488 us`. Three CPU-0 runs of one unchanged postimplementation binary must meet both median ceilings; every run must also preserve exact science/count/multiset identity and remain at or below `65,536 KiB` RSS. Any failure requires full revision-61 production reversion. |

No blocking review finding remains.

### Matrix and gate assessment

- The role/path/count oracle covers every `Full`, `Retry`, `Half1`, `Half2`,
  and `Root` invocation in discovery and exact-endpoint modes, exact 200-call
  multiset identity, and unchanged ordinary canonical-map and terminal-batch
  cadence.
- The forced two-call oracle requires bitwise equality to both reference
  results, exact-zero coupling deltas, and complete transition, owner, boundary,
  LSE/vegetation/soil/surface/WB14, receipt, ledger, and diagnostic identity.
- Poison coverage includes the sole provider, exact/final evaluations, every
  role/support/attempt/beginning/topology/forcing/owner binding,
  cross-invocation reuse, retry rebasing, changed support, and partial output.
  Refusal preserves error precedence, byte-identical rollback, and zero trial,
  arena, owner, receipt, or publication retention.
- Calibration and identifiability remain correctly `NOT_APPLICABLE`: revision
  61 adds no constitutive relation, parameter, observation operator, objective,
  tolerance, or empirical claim.
- Recorded preimplementation gates are coherent: full integration target 39
  pass / exactly one named structural expected-red / 22 ignored; strict
  binding-exposure check 51 rows `PASS`; unit-compliance lint `PASS`; and
  `cargo fmt --all -- --check` `PASS`.

### Non-blocking debt and follow-up

- A few profile/test labels still use the shorthand "logical group" and the
  future behavior-test name says "duplicate." Implement those names and
  assertions so they unambiguously mean invocation-local sole-call and
  cross-invocation non-reuse, not a process-global uniqueness registry.
- The structural gate is intentionally not implementation acceptance. The
  named CQR behavior tests and compile-time negative-capability proof must be
  authored and executed before production changes are accepted.

### Corrected-manifest verdict

`PASS` / `GO` for the corrected revision-61 contract cycle. All
`FFTC-A-001..004` and `FF-B-001..003` dispositions are supported by the exact
manifest. This approves proceeding to the tests-first implementation and
independent-verification phases; it does not by itself approve production
retention or package closure, which remain conditional on the executable
behavior matrix and numeric keep/revert gate above.

## Final A-004 taxonomy-amendment confirmation

Evidence mode: `Static + read-only manifest reproduction`; no Cargo command
was rerun for this narrow confirmation.

Review target: exact ordered four-file manifest
`a8a6677672c7f8f34acdaca30f3b94388902d022fe5886f28e4532762ffce804`.
The documented ordered recipe reproduced that value exactly, and `git diff
--check` remains clean for the manifest files.

`FFTC-A-004` is `CLOSED` at this manifest. The core invariant and Guard Map now
agree with the source-real evaluator path:

1. provider-wrapper attachment/projection/custody failures retain their exact
   `TerminalCustody` behavior, including `"covered probe carrier fixed point"`;
2. the post-provider boundary/result join fails through
   `snow.terminal_trial_boundary_support_join`, which converts to
   `DirectSnowStage3EvaluationError::Kernel`;
3. `stage3_hourly_surface_energy` and required terminal diagnostics occur
   after that join and before transition, retaining existing `Kernel` or
   `TurbulentTransfer` authority; and
4. terminal transition retains existing `TerminalNumerics` or `Kernel`
   authority, while exact/final failures remain separate.

The map neither invents an error variant nor hoists validation, and it binds
the adjacent boundaries to competing-poison tests.

No B finding regressed:

- `FF-B-001` remains `CLOSED`: only 400 calls are direct pre-change release
  evidence; 200 pre-change invocations remain explicitly inferred, and the
  post-change exact 200-invocation role/path multiset remains mandatory.
- `FF-B-002` remains `CLOSED`: the structural pre-red still reads
  `cqr_row5_tests.rs` and remains explicitly separate from executable behavior
  acceptance.
- `FF-B-003` remains `CLOSED`: the three-run unchanged-binary CPU-0 gate still
  requires medians `provider_carrier <= 1,299,833 us` and
  `run_wall_us <= 4,234,488 us`, exact identity/multiset preservation, per-run
  RSS `<= 65,536 KiB`, and full production reversion on any failure.

Final verdict: `PASS` / `GO` for the final corrected revision-61 contract
cycle. No blocking QA finding remains. Production acceptance and retention
remain conditional on the already-declared tests-first behavior evidence,
independent verification, and numeric keep/revert gate.
