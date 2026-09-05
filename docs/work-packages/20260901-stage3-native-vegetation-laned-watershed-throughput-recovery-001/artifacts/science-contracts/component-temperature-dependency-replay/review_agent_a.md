# Independent contract review A

Evidence mode: `Static + Ran`

Reviewer: independent component-temperature dependency-replay reviewer

Ordered five-file manifest reviewed:
`4faf1cdd6ebd618d12a0a23501cf4f9df85d536f61b4dad74dd70ec4e452ed13`

Verdict: `CHANGES REQUIRED`

Promotion recommendation: `HOLD` before production implementation.

## Findings

| Finding | Severity | Summary | Required disposition |
|---|---|---|---|
| `CTDR-A-001` | high | The amendment names a topology-generic static dependency graph but does not define its node schema or complete direct-edge construction, so dependency completeness is not independently reproducible. | Add a canonical node/dependency table or equivalent pseudocode that covers both liquid passes, reciprocal longwave, leaf/current-and-maximum state, CI and vapor terms, hydraulics, occupancy outputs, shared rows, ground-facing output, and ordered residual/tolerance assembly. |
| `CTDR-A-002` | high | The evidence lifetime is internally ambiguous: the proof retains one iteration's successful nodes, is called single-use, and is forbidden to cross a probe, while one Jacobian base must serve all `8*N` component probes. | Separate the sweep-owned immutable base evidence from a fresh non-Clone single-probe replay capability; define creation, borrow/consume, second-use behavior, and exact expiry boundaries. |
| `CTDR-A-003` | high | Eligibility misses, integrity violations, and first-error behavior are conflated. Some stale/foreign/graph failures are required to run the complete evaluator, while the oracle calls the same cases poisons and requires identical first errors and rollback. | Add a source-ordered trigger/outcome table distinguishing canonical complete-evaluation admission from typed integrity failure, including stale/foreign/second-use behavior and competing-error precedence. |
| `CTDR-A-004` | medium | Binding-exposure/profile and lifecycle evidence metadata disagree inside the reviewed manifest. | Use only the schema vocabulary in the profile row and reconcile it with the core BEI; reconcile the contract and index `evidence_level` fields or explicitly disposition the inherited mismatch. |
| `CTDR-A-005` | medium | The canonical keep/revert text does not bind the three-run pre-v31 baseline that the readiness artifact claims. | Require three exact CPU-0 runs of one unchanged preimplementation binary, record source/binary identities and the same profile command/environment, and compare pre/post medians under the stated conjunctive gate. |

### `CTDR-A-001` — the dependency graph is not implementation-authoritative

The amendment requires every direct data read to become an edge and every
transitively reachable node to be replayed
(`SC-LANDSURFACEENERGY-001.md:2863-2880`), but its three dependency bullets are
examples rather than a complete graph definition (`:2885-2904`). The current
complete evaluator has materially distinct ordered regions:

- top-to-bottom preliminary liquid preparation, wet flux, finalization, and
  routing (`solver_covered_evaluation.rs:1976-2008`);
- whole-column reciprocal longwave (`:2009-2040`);
- ordered occupancy evaluation and routed/final equality (`:2041-2064`);
- lower-boundary and shared-canopy reductions (`:2065-2210`); and
- raw/tolerance normalization and complete evaluation-output assembly
  (`:2289-2378`).

The missing edge authority is consequential. An upper wet-temperature probe
changes its finalized liquid and the next occupancy's incident rain. That
changes every lower wet fraction and dry component area, which changes lower
sun/shade current and maximum vapor rates, hydraulic residuals and tolerances,
component energy/tolerances, shared heat/vapor, subsequent liquid routing, and
the final output fields. Separately, every component temperature changes the
reciprocal-longwave column, including other component-energy rows and the
ground-facing longwave output. Sun/shade temperatures also change leaf current
and beta-one maximum state, surface humidity/resistance, CI/carbon fields,
vapor terms, hydraulics, component energy, and shared reductions.

The broad rule “every direct read” is a sound conservative principle, but it is
not sufficient to reproduce or review the graph independently. A finite
forced-complete fixture cannot prove absence of an omitted edge over arbitrary
occupancy cardinality and all branch postures. Bind the canonical node taxonomy
and direct-edge generator (or an equally explicit source-aligned dependency
table), including graph direction and the lower-occupancy transitive closure.
The implementation may conservatively mark additional nodes reachable, but it
must never infer independence from an unlisted edge.

### `CTDR-A-002` — sweep evidence and single-probe custody are conflated

The contract says `ValidatedCoveredComponentProbeReplay` retains the current
iteration's successful node values, cannot cross a probe, and is subject to a
second-use poison (`SC-LANDSURFACEENERGY-001.md:2863-2865,2881-2883,2928-2932,
2952-2954`). Those statements do not define how the same successful base
evaluation supplies all `8*N` admitted component probes without cloning or
cross-probe retention.

The source-real seam already makes the needed distinction. One
`ValidatedCoveredJacobianBase` owns the current evaluation and frozen branches
for the whole sweep (`solver_covered_solve.rs:245-261,543-555`), while each
minus and plus probe borrows that base independently (`:556-595`). The contract
should bind a sweep-owned, immutable, non-wire base whose lifetime ends before
the next Newton iteration, plus a fresh per-probe move-only/view capability
that is consumed exactly once. “Second use” then applies to the per-probe
capability, not to the sweep base. The capability must remain bound to the exact
validated inputs, potential/final caps posture, frozen-branch set, current trial,
column index, perturbation direction, iteration, and sweep.

Without that split, a literal implementation either cannot produce the required
16 replay probes in the authentic `N=2` fixture or must violate the stated
non-Clone/single-boundary rule.

### `CTDR-A-003` — guard outcomes and error precedence need an exact matrix

The invariant row sends an unknown edge, multi-coordinate change, stale
evidence, or graph mismatch through the complete evaluator
(`SC-LANDSURFACEENERGY-001.md:261`), and the algorithm repeats that every
ineligible case uses the complete evaluator (`:2871-2872,2906-2913,2954`). The
test obligation then calls stale/foreign evidence and second use “poisons,”
requires identical first errors, and requires rollback (`:2923-2933`). It is
unclear whether those are:

1. ordinary replay-ineligibility followed by the one canonical complete
   evaluator, with any error coming only from that evaluator;
2. internal proof-integrity violations that fail closed before evaluation; or
3. test-only impossible constructions.

This distinction matters to ADR-0044 and the contract's simultaneous “complete
evaluator on mismatch” and “no fallback” wording. The complete evaluator is a
legitimate canonical evaluation branch when selected before replay; it must not
be described or implemented as error recovery after a partial replay.

Add a trigger/outcome/error table. It should bind the current source order:
trial admission; top-rain and top-to-bottom liquid prepass; reciprocal
longwave; occupancy-order sun current, shade current, sun maximum, shade
maximum, wet, hydraulic/root, energy/tolerance and liquid finalization;
lower-boundary/shared work; then result assembly. Paired poisons must compare
the admitted path against an unchanged complete-evaluator oracle and establish
the same earliest error without executing an alternative replay or solver.

### `CTDR-A-004` — metadata and Binding Exposure Index descriptions conflict

The actual BEI row classifies the amendment as `maps-to-existing-INV`
(`SC-LANDSURFACEENERGY-001.md:577`), while the profile summary calls its binding
classification `new-INV` (`:2959`). `new-INV` is not allowed Binding Exposure
Index vocabulary. The new invariant and obligation IDs themselves are unique:
`INV-LANDSURFACEENERGY-164` occurs once in the authority table and once in the
Guard Map, and `OBL-LANDSURFACEENERGY-C-020` has one defining obligation
paragraph. The profile should nevertheless use the same allowed classification
as the core row and describe the IDs as introduced by revision 31 separately.

The contract front matter reports `evidence_level:
static+independent_oracle` (`SC-LANDSURFACEENERGY-001.md:14`), while the registry
reports `static+independent_oracle+contract_vectors`
(`science-contracts/index.md:56`). This mismatch predates revision 31, but the
reviewed increment updates both lifecycle surfaces and the readiness artifact
asserts canonical/registry agreement. Reconcile or explicitly disposition it;
do not imply the structural expected red is itself an independent behavioral
oracle.

### `CTDR-A-005` — the prospective baseline protocol differs by artifact

The canonical contract requires freezing “a pre-v31 exact-release baseline”
and explicitly requires only the postimplementation side to use three CPU-0
repetitions (`SC-LANDSURFACEENERGY-001.md:2935-2944`). The readiness matrix,
however, says a three-run CPU-0 pre-v31 baseline is mandatory
(`readiness-matrix.md:33`). The latter is package evidence, not canonical
authority. A single pre-run compared with a post median would not support the
claimed `100000 us` median improvement robustly and leaves room for inconsistent
profile instrumentation.

Make the canonical rule conjunctive and symmetric: three exact CPU-0 runs from
one unchanged pre-v31 binary and three from one unchanged post-v31 binary, the
same exact command/environment/profile buckets, recorded source and binary
hashes, comparison of both medians, every post RSS `<=65536 KiB`, exact science,
workload, probe accounting, and full-solve bits. Any failed conjunct still
requires full production-increment reversion.

## Positive observations

- The exact accounting is correct. For `N` occupancies and `S` soil nodes,
  `2*(10*N+3+S) - 2*(1+S) - 8*N = 12*N+4`; with `N=2`, `S=6`, this is exactly
  `58 = 14 + 16 + 28`. The remaining 14 columns are eight hydraulic, four beta,
  and two shared-canopy-air coordinates.
- The amendment preserves the canonical centered/inward stencil, minus-before-
  plus order, normalized-residual arithmetic, dense matrix, pivoting,
  backtracking, and one canonical solver. Its bans on analytic/automatic
  derivatives, coloring, simultaneous perturbation, sparse LU, cross-boundary
  caches, approximation, hardcoded fixture topology, and alternate solver
  fallback are appropriate.
- Reciprocal longwave, wet routing, leaf/CI/hydraulic descendants, and shared
  rows are all recognized in the prose. Finding `CTDR-A-001` asks that this
  scientifically correct intent become a complete reproducible graph
  specification, not that the dependency set be narrowed.
- The structural expected red is truthfully classified. It proves only that six
  named production seams are absent and is explicitly denied behavioral
  acceptance authority (`contract_ref.md:46-60`; `readiness-matrix.md:29-34`).
  The forced-complete in-crate differential, branch/bound/error matrix,
  authentic counters, full-solve equality, rollback, and release qualification
  remain mandatory before implementation acceptance.
- `CALIBRATION_NOT_APPLICABLE` is correct: the amendment changes no equation,
  dimensional value, parameter, observation operator, output, publication
  schema, or empirical claim.

## Checks run

Ran the exact ordered manifest recipe from `contract_ref.md`.

Result: `PASS`; digest
`4faf1cdd6ebd618d12a0a23501cf4f9df85d536f61b4dad74dd70ec4e452ed13`.

Ran:

```text
.venv/bin/python tools/check_sc_binding_exposure.py --strict docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001.md
bash tools/release/check_sc_unit_compliance.sh --path docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001.md
env RUST_MIN_STACK=67108864 nix develop -c cargo test --test land_surface_energy_balance_authority_contract version_thirty_one_binds_component_temperature_dependency_replay -- --exact --nocapture
```

Result: `PASS`; strict BEI reported 15 consolidated rows, unit compliance had
no findings, and the focused authority assertion passed 1/1.

Ran the exact structural seam test.

Result: `EXPECTED_RED`; it failed on exactly the six absent graph/replay/audit
symbols recorded in `contract_ref.md`.

Ran scoped `git diff --check` over the ordered five-file increment.

Result: `PASS`.

No production correctness or performance claim was accepted. Production work
should remain on `HOLD` until the five findings are amended, independently
dispositioned, and re-reviewed.

## Corrected six-file re-review A — 2026-09-04

Evidence mode: `Static + Ran`

Ordered six-file manifest reviewed:
`c0b04030f83f23ad36338601d7962420d12c703f38894ba54d355148c2e967dc`

Verdict: `CHANGES REQUIRED`

Promotion recommendation: retain `HOLD` before production implementation.

### Findings-first result

| Finding | Severity | Status | Evidence and required action |
|---|---|---|---|
| `CTDR-A-R1-001` | high | `OPEN` | Cross-review B-04 required every corrected current-version-pin file to be added to the corrected manifest. The LSE pins are statically corrected, but `snow_stage3_shared_carrier_authority_contract.rs` and `stage3_native_vegetation_laned_throughput_recovery.rs` are outside the six-file digest. Expand the ordered manifest to include both files and recompute it so the reviewed closure cannot drift. |
| `CTDR-A-R1-002` | medium | `OPEN` | Cross-review B-06 is only partially closed. `rust_item_is_test_gated` examines four preceding lines and recognizes only the literal `cfg(test)`, so a top-level item under `#[cfg(any(test, ...))]`, another disabled `cfg`, or `#[cfg(test)]` separated by more attributes passes. The mask also is not a complete Rust literal lexer. Reject every conditional/disconnected item using a non-evadable parser or extend the lexical guard to complete contiguous attributes and all Rust literal forms. |

### Original finding closure audit

| Finding | Re-review status | Evidence |
|---|---|---|
| `CTDR-A-001` | `CLOSED` | The normative node table and direct-edge generator now cover the two source-distinct wet regions/finalizations, lower routing closure, reciprocal longwave, leaf current/maximum/CI, vapor and hydraulics, route-match, terminal release/stemflow, shared rows, residual normalization and result assembly (`SC-LANDSURFACEENERGY-001.md:2875-2940`). Graph schema versioning, stable expansion/hash rules, inclusive closure and conservative unknown handling make the dependency authority independently reproducible. |
| `CTDR-A-002` | `CLOSED` | The amendment now distinguishes one immutable whole-sweep base from a fresh per-signed-probe non-Clone capability and binds validated input, caps/frozen posture, current bits, iteration/sweep, graph, coordinate, sign, perturbation, probe and stencil identity (`:2863-2873`). Consumption on success/error and zero-mutation drop are explicit. |
| `CTDR-A-003` | `CLOSED` | The trigger/outcome table cleanly separates pre-replay ordinary complete-evaluator selection, direct proof-integrity failure and direct post-start node failure; it forbids evaluator/solver recovery after replay begins and binds source-ordered paired-error evidence (`:2932-2956`). |
| `CTDR-A-004` | `CLOSED` | Contract and registry both report `static+independent_oracle+contract_vectors`. The BEI and profile use schema-valid `maps-to-existing-INV`; the authority test checks the exact BEI cell and separately checks the revision-31 new-ID note. INV-164 remains unique to one authority row plus one Guard Map row, and C-020 has one defining obligation. |
| `CTDR-A-005` | `CLOSED` | The canonical contract now binds the exact CPU-0 command, unchanged pre source manifest and binary path/hash, three raw JSON tuples, field names, sorted-middle medians, exact candidate ceilings, one-build/same-binary candidate repetitions, per-run RSS/science/count requirements and full-revert outcome (`:3006-3036`). The raw baseline log agrees, including binary SHA-256 `9a91c82f...e573f`. |

### Cross-review closure audit

- `B-01` is closed: the exact baseline command, hashes, three raw
  `(run_wall_us, physical_phase_wall_us.potential, rss_kib)` tuples, medians and
  ceilings are recorded canonically and in the raw log.
- `B-02` is closed: `58/14/16/28` is scoped to one fully centered `N=2,S=6`
  sweep. Local reset/seal semantics and the separate release aggregator cover
  potential/final, centered/inward, success/error/short-circuit identities
  without treating fixture counts as release totals (`:2967-2990`).
- `B-03` is closed by the BEI/profile/test reconciliation described under
  `CTDR-A-004`.
- `B-04` is behaviorally corrected but not manifest-closed. Static inspection
  finds all three named LSE assertions at `contract_version: 31`; however the
  two edited files are not members of the corrected manifest despite the
  review's explicit requirement. The focused shared-carrier assertion passes.
  The two larger package assertions currently fail earlier on unrelated stale
  `SC-COUPLEDTIME-001` and `SC-SNOWENERGY-001` version pins, so no whole-target
  pass is claimed.
- `B-05` is closed: the graph and OBL now enumerate the duplicated wet path,
  routed/final equality, complete lower descendants, ground release/stemflow,
  evaluation-field equality and exact error chronology.
- `B-06` remains open as `CTDR-A-R1-002`. The structural assertion is correctly
  classified as non-behavioral expected-red evidence, but its claimed
  anti-evasion hardening is incomplete.

### Structural-guard counterexample

`rust_item_is_test_gated` reads only the four lines immediately preceding the
marker and returns true only when that slice contains the exact text
`cfg(test)` (`land_surface_energy_balance_authority_contract.rs:125-137`). A
top-level item guarded by `#[cfg(any(test, feature = "diagnostic"))]` therefore
passes both this check and `rust_item_is_top_level`. So does an item with
`#[cfg(test)]` followed by five ordinary attributes. These items are absent
from a normal production build but satisfy the current “non-test-gated” gate.

The literal masker handles ordinary quoted strings but is not a complete Rust
lexer. For example, a raw string containing an early literal quote can end the
masker's ordinary-string scan before a later marker, exposing source text that
is still inside the raw literal. Because dispatcher/helper consumption is also
checked with the same masker, raw literal identifiers can evade those checks.
An AST-based item/call inspection is preferred; a bounded lexical correction
must at minimum parse raw/byte/raw-byte/string/character literals and the full
contiguous attribute block and reject every `cfg`-conditioned required item.

### Re-review checks run

Ran the exact ordered six-file manifest recipe.

Result: `PASS`; digest matched
`c0b04030f83f23ad36338601d7962420d12c703f38894ba54d355148c2e967dc`.

Ran strict Binding Exposure Index and SC unit-compliance lints.

Result: `PASS`; 15 consolidated BEI rows and no unit findings.

Ran the focused revision-31 authority assertion.

Result: `PASS`, 1/1.

Ran the complete LSE authority integration target.

Result: `PASS_WITH_EXPECTED_RED`; 22 passed, exactly the named revision-31
structural test failed on all seven absent production items, and zero were
ignored.

Ran the cross-review B-04 shared-carrier assertion.

Result: `PASS`, 1/1. The two package assertions statically contain the corrected
LSE v31 pins but are presently red on unrelated current-version reconciliation,
as noted above.

Ran scoped `git diff --check` over the six-file manifest.

Result: `PASS`.

All substantive graph, lifetime, error, accounting, evidence-metadata and
retention corrections are sound. Production implementation remains blocked
only on manifest-complete B-04 custody and non-evadable B-06 structural
classification; neither may be silently marked closed.

## Final corrected eight-file re-review A — 2026-09-04

Evidence mode: `Static + Ran`

Ordered eight-file manifest reviewed:
`cbba3ea117e088677e80de7099af5e03a1dc5e4e7444430795dda8ba7ca71da4`

Verdict: `PASS`

Promotion recommendation: `GO` for the bounded revision-31 production
implementation. This does not waive the forced-complete behavioral oracle,
authentic counters, candidate release qualification, independent
implementation review/verification, or full-revert rule.

### Findings-first result

No open findings remain.

1. `CTDR-A-R1-001` / B-04 is closed. The three named LSE assertions all bind
   `contract_version: 31`, and their two owning integration files are exact
   members four and five of the frozen manifest. The current focused tests in
   those files stop earlier on unrelated, out-of-manifest current-version pins
   (`SC-SNOWENERGY-001` v58, `SC-COUPLEDTIME-001` v16, and
   `SC-SNOWENERGY-001` v60 respectively), so this review makes static custody
   closure only and does not misreport those complete tests as passing.
2. `CTDR-A-R1-002` / B-06 is closed. `rust_code_mask` recognizes cooked,
   arbitrary-hash raw and raw-byte strings plus escaped/unescaped character
   and byte-character literals. `rust_char_literal_end` preserves named,
   static, and placeholder lifetimes because it masks an apostrophe only when
   the parsed scalar or escape is followed by a closing apostrophe. The active
   fixtures exercise `'}'`, `'\''`, `b'}'`, `b'\''`, embedded quotes in raw
   strings, arbitrary raw-byte hashes, and lifetime preservation.
3. Attribute classification now walks backward through whitespace, arbitrary
   stacked/multiline attributes, `pub`, `pub(...)`, and applicable function
   modifiers. It rejects both `cfg` and `cfg_attr`, independent of expression.
   The active adversary places all seven required items inside a nested
   container with brace-bearing character literals and direct conditional
   attributes; every item is found, classified gated, and rejected as
   non-top-level. A second adversary places the helper call behind `cfg(test)`
   in the real-dispatcher-shaped body; the unconditional-identifier predicate
   rejects it. The structural expected red therefore cannot be satisfied by
   the previously identified comment/literal, conditional, nested,
   visibility/modifier, or gated-call decoys.

### Prior-finding closure audit

- `CTDR-A-001` and B-05 remain closed. The normative versioned/hashed graph
  schema, stable node families, complete direct-edge generator, inclusive
  closure, and conservative unknown handling cover reciprocal longwave, both
  wet evaluations/finalizations, upper-to-all-lower routing, route-match,
  terminal release/stemflow, leaf current/maximum/CI, vapor/hydraulics,
  sensible/energy/tolerances, shared rows, normalization, and result assembly
  in source-real order.
- `CTDR-A-002` remains closed. One immutable generation/input/caps/frozen/
  trial/iteration/sweep/graph-bound base serves the sweep, while each signed
  probe receives a fresh non-Clone coordinate/sign/perturbation/probe/stencil-
  bound capability consumed on success or error. Expiry and zero-mutation drop
  semantics are explicit.
- `CTDR-A-003` remains closed. Ordinary ineligibility and behavior-identical
  graph mismatch select the complete evaluator only before replay; stale,
  foreign, wrong-binding, transfer, and second-use integrity violations fail
  typed; post-start errors return directly at source position with no complete
  evaluator or alternate solver fallback. Paired first-error and rollback
  evidence remains mandatory.
- `CTDR-A-004` and B-03 remain closed. Contract/index evidence metadata agree
  on `static+independent_oracle+contract_vectors`; the BEI/profile use the
  schema-valid `maps-to-existing-INV` classification and separately identify
  new IDs `INV-164/C-020`. Strict BEI reports 15 consolidated rows.
- `CTDR-A-005` and B-01 remain closed. The exact CPU-0 command, frozen source
  manifest, binary identity, three raw baseline tuples, sorted-middle medians,
  candidate ceilings, unchanged candidate identity protocol, per-run science/
  workload/RSS requirements, and total reversion on any failed conjunct are
  canonical and agree with the retained raw log.
- B-02 remains closed. The exact `58 = 14 + 16 + 28` identity is correctly
  scoped to one fully centered `N=2,S=6` sweep; generic formulas, reset/seal
  rules, stencil/status accounting, and separately scoped release aggregation
  prevent fixture counts from being represented as whole-run totals.

The amendment still forbids analytic/automatic derivatives, graph coloring,
simultaneous perturbation, sparse Jacobian/LU, changed pivoting, approximation,
cross-sweep/iteration/map/retry caches, hardcoded fixture topology, recovery
fallback, and an alternate solver. The structural test remains classification
evidence only; it does not stand in for the required executable oracle.

### Final re-review checks run

Ran the exact ordered eight-file manifest recipe twice, before and after the
focused checks.

Result: `PASS`; both computations matched
`cbba3ea117e088677e80de7099af5e03a1dc5e4e7444430795dda8ba7ca71da4`.

Ran both active structural-parser adversarial tests.

Result: `PASS`, 2/2.

Ran the focused revision-31 authority assertion.

Result: `PASS`, 1/1.

Ran the exact revision-31 structural production-seam assertion.

Result: `EXPECTED_RED`; it failed on exactly the seven named absent production
items, with zero unexpected item or parser failure.

Ran the complete LSE authority integration target.

Result: `PASS_WITH_EXPECTED_RED`; 24 passed, exactly the named revision-31
structural seam failed, and zero were ignored. The only warning was an
unrelated pre-existing dead-code warning for two terminal-request fields.

Ran strict Binding Exposure Index, SC unit-compliance, and workspace formatting
checks.

Result: `PASS`; 15 BEI rows were consolidated, unit compliance reported no
findings, and `cargo fmt --all -- --check` was clean.

Static contract/source feasibility, manifest custody, correction tests, and
the focused results support `PASS/GO` for implementation of this bounded
contract. Production correctness, performance retention, and package closure
remain unclaimed until their later gates actually run and pass.

## Superseding final eight-file re-review A — 2026-09-04

Evidence mode: `Static + Ran`

Ordered eight-file manifest reviewed:
`23d90c29038eb836b1ce8f2105588f44d382f2ae5b726a7bc55467b842ac058b`

Verdict: `PASS`

Promotion recommendation: `GO` for the bounded revision-31 production
implementation only. The earlier `cbba3e...` verdict is superseded because its
structural-connectivity characterization was too broad; this section is the
controlling re-review disposition.

### Findings-first result

No open findings remain.

1. `CTDR-RRB-002` is closed. The expected-red test now asks only whether each
   of seven named markers has a parseable body, has no directly attached
   `cfg`/`cfg_attr`, and appears at brace depth zero. It contains no dispatcher
   invocation or helper graph/evidence/audit consumption assertion.
2. The authority and evidence artifacts accurately limit this classifier.
   They expressly state that it cannot prove invocation, control-flow
   reachability, graph/evidence consumption, counter provenance, or numerical
   behavior, and that empty, skeleton, token-only, dead, or disconnected
   declarations may make it green without changing implementation readiness or
   satisfying `OBL-LANDSURFACEENERGY-C-020`.
3. The active adversarial dispatcher intentionally contains a `cfg(test)`-only
   call, a same-named local binding, a bare reference, and an `if false` call.
   The self-test merely confirms those misleading tokens exist; it does not
   infer connectivity from them. This directly guards the corrected evidence
   classification.
4. Connectivity and consumption remain mandatory, but only executable
   post-implementation evidence can establish them: tests must exercise the
   real dispatcher, observe authentic sealed per-sweep and release counters,
   and pass the forced-complete node/evaluation/residual/Jacobian/full-solve
   parity oracle. Structural green alone leaves `Production implementation and
   behavior` blocked and cannot authorize retention.

### Complete closure audit

- `CTDR-A-001` / B-05 remain closed by the normative graph schema, stable node
  and edge families, hash construction, inclusive closure, conservative unknown
  handling, both wet regions/finalizations, all-lower routing, reciprocal
  longwave, leaf current/maximum/CI, vapor/hydraulics, shared rows, route-match,
  terminal release/stemflow, normalization, and result assembly.
- `CTDR-A-002` remains closed by the separate immutable sweep base and fresh
  non-Clone, single-use signed-probe capabilities with exact identity bindings,
  expiry, error consumption, and zero-mutation drop.
- `CTDR-A-003` remains closed by the eligibility/integrity/error outcome table,
  exact source order, pre-replay complete selection, direct integrity failure,
  direct post-start error, same-first-error oracle, rollback, and no recovery
  fallback.
- `CTDR-A-004` / B-03 remain closed: contract and index evidence metadata
  agree; BEI/profile use `maps-to-existing-INV`; the new-ID note is separate;
  `INV-164` and `C-020` retain the intended authority/Guard Map/obligation
  placement.
- `CTDR-A-005` / B-01 remain closed by the exact frozen three-run CPU-0
  baseline and symmetric candidate protocol, source/binary identities, raw
  fields and tuples, sorted-middle medians, conjunctive ceilings, per-run RSS,
  unchanged science/workload, and complete-revert rule.
- B-02 remains closed: `58 = 14 + 16 + 28` is one authentic fully centered
  `N=2,S=6` sweep identity, with generic formulas and separately reset/sealed
  sweep and release aggregation covering potential/final, stencil, completion,
  error, and short-circuit classes.
- B-04 / `CTDR-A-R1-001` remain closed: the three LSE assertions bind revision
  31 and both owning integration files remain exact manifest members four and
  five. This is static manifest custody; their broader tests still encounter
  unrelated out-of-manifest current-version reconciliation before the LSE
  assertions and are not claimed passing.
- B-06 / `CTDR-A-R1-002` / `CTDR-RRB-001` remain closed for the deliberately
  narrow source classifier. Comment and cooked/raw/raw-byte/character/
  byte-character literal masking, lifetime preservation, stacked/multiline
  `cfg`/`cfg_attr`, visibility/modifier traversal, and nested-item adversaries
  are covered by active tests. None of that is promoted to connectivity or
  behavioral authority.

All prohibitions remain intact: no analytic/automatic derivative, graph
coloring, simultaneous perturbation, sparse Jacobian/LU, pivot change,
approximation, cross-sweep/iteration/map/retry cache, hardcoded fixture
topology, fallback, or alternate solver is authorized.

### Superseding re-review checks run

Ran the exact ordered eight-file manifest recipe before and after the focused
checks.

Result: `PASS`; both computations matched
`23d90c29038eb836b1ce8f2105588f44d382f2ae5b726a7bc55467b842ac058b`.

Ran both active structural-parser adversarial tests.

Result: `PASS`, 2/2.

Ran the focused revision-31 authority assertion.

Result: `PASS`, 1/1.

Ran the exact narrow item-presence seam.

Result: `EXPECTED_RED`; exactly the seven named unconditional-top-level items
were absent.

Ran the complete LSE authority integration target.

Result: `PASS_WITH_EXPECTED_RED`; 24 passed, exactly the narrow revision-31
item-presence test failed, and zero were ignored. The only warning was the
unrelated pre-existing dead-code warning for two terminal-request fields.

Ran strict BEI, SC unit-compliance, and workspace formatting checks.

Result: `PASS`; 15 BEI rows were consolidated, unit compliance reported no
findings, and `cargo fmt --all -- --check` was clean.

The corrected contract-first authority is source-real, manifest-complete, and
truthful about the limits of structural evidence. Production behavior,
performance retention, implementation verification, and package closure remain
unclaimed until their executable gates actually pass.

## Fresh corrected same-v31 re-review A — 2026-09-04

Evidence mode: `Static + Ran`

Ordered eight-file manifest reviewed:
`9fa2a1fc1a5bc53dcd66ed43e7d09a41891be01d78de0db143e273e25d468986`

Verdict: `CHANGES REQUIRED`

Promotion recommendation: retain `HOLD` before a fresh revision-31
reimplementation. This section supersedes the prior `23d90c...` contract
verdict and withdraws the initially recorded `9fa2...` approval after a full
direct-read audit. It does not approve the rejected/reverted candidate or close
any implementation, behavior, performance, verification, or release gate.

### Findings-first result

One high contract-authority blocker remains.

`CTDR-A-R2-001` / `CTDR-RRB-003` — HIGH — the normative direct-edge
generator is incomplete. Source-real reads require at least these missing
edges:

- `probe[o,wet] -> route.finalize[o]`, because the first finalizer reads the
  wet-temperature coordinate directly for liquid enthalpy as well as reading
  `route.wet[o]`;
- `route.prepare[o] -> occ.wet[o]`, because the second wet evaluation reads
  the prepared liquid/wet fraction directly;
- `longwave.column -> occ.output[o]`, because each occupancy result stores its
  component longwave array, not only energy/tolerance descendants;
- `result.ground_release -> result.output` and
  `result.ground_stemflow -> result.output`, because both terminal routed
  values are fields of the complete evaluation;
- `lower.ground_output -> shared.heat`, `shared.vapor`, and
  `shared.tolerance`, because the shared residuals and their scales directly
  consume lower-boundary sensible/vapor operands.

If the implementation defines `shared.heat`/`shared.vapor` as owning the
constituent reduced/reference operands later reused by `shared.tolerance`, it
must also enumerate the corresponding shared-to-tolerance edges. If the nodes
remain independent computations from common operands as in the current source,
the explicit existing `occ.output -> shared reductions` edges plus the missing
`lower.ground_output -> shared.tolerance` edge are sufficient. The dependency
must not remain implicit in the grouped `shared/lower nodes` row.

The corrected manifest must enumerate these direct reads, update the focused
authority assertions, and retain the independently enumerated exact-edge/
golden-hash and edge-removal-fails obligations. Reachability through a broad
output edge is not a substitute for the normative direct record.

1. Production revert is complete for this increment. Static inspection of
   `solver_covered_solve.rs`, `solver_covered_evaluation.rs`, `solver_tests.rs`,
   and the runner qualification found none of the seven replay graph/base/
   capability/audit/function symbols and no residual component-dependency-
   replay mechanism. The narrow production-seam assertion remains red on
   exactly those seven symbols.
2. The corrected fallibility/crossability matrix is source-real. A component
   sun/shade temperature probe re-enters `leaf_trial_state`, whose existing
   saturation, VPD, carbon/quadratic, conductance, CI/bracket, and accepted-
   branch guards can be crossed by an otherwise admitted temperature probe;
   `occ.leaf.current` therefore requires authentic paired replay/forced-
   complete first-error and rollback vectors. No test-only error seam is
   authorized.
3. `occ.leaf.maximum` is accurately classified as fallible but not currently
   established crossable. It reuses the perturbed temperature/environment but
   calls the same fallible leaf routine at beta one after current-leaf success.
   The contract requires a guard-by-guard implication and authentic beta/
   branch boundary successes, while explicitly requiring reclassification and
   a paired error vector if a successful-base/admitted-probe counterexample is
   found. It does not assume an impossible poison.
4. The other named fallible families are correctly separated from infallible
   computation/assembly. Routing/finalization, longwave, hydraulics,
   route-match, and lower-ground guards depend on validated or immutable
   operands, admitted temperature bounds, successful predecessors, or one
   shared canonical finalizer. They require reviewable implication proofs plus
   authentic boundary/branch successes and exact fields. Probe propagation,
   routed identities, vapor/sensible/energy/tolerance/output, shared rows,
   residual assembly/normalization, and terminal result assembly return no
   independent typed error after those predecessors and therefore require
   exact source-order field parity without synthetic failures.
5. The unmodified differential corpus is a catch-all for any naturally
   occurring error in either mode and requires identical first existing error,
   no fallback, and byte-exact beginning/custody rollback. Mutation seams,
   fault injection, noncanonical perturbations, impossible branches, forged
   intermediates, direct private-node tests, synthetic error branches, and
   test-only physics entry points are expressly forbidden.

### Rejected-implementation finding audit

All 11 findings from `implementation_review_a.md` and
`implementation_review_b.md` are accepted and remain
`open-until-reimplementation` in `disposition.md`: five
`CTDR-IMPL-A-001..005` findings and six `CTDR-IB-001..006` findings. None is
misrepresented as closed by the authority correction.

- `CTDR-IMPL-A-001` requires complete and replay paths to call one shared
  canonical node/tail implementation; mirrored physical, tolerance, residual,
  branch, or output arithmetic is forbidden.
- `CTDR-IMPL-A-002` and `CTDR-IB-002` require graph-driven source-order
  execution plus an independently enumerated exact node/direct-edge set and
  golden hash for generic `N=1,S=1` and authentic `N=2,S=6` topologies.
  Reachability samples or a coarse hand-coded replay mask cannot close them;
  removing or changing any required edge must fail the oracle.
- `CTDR-IMPL-A-003`, `CTDR-IB-003`, and `CTDR-IB-005` require efficient exact
  custody of generation, every referenced input, caps values/posture, frozen
  branches, graph, successful base evaluation, trial, coordinate, sign,
  perturbation, exact probe, and actual stencil. Immutable typed borrows/
  generations or compact seals minted at validation are admitted; `Debug`,
  length-only/hash-only proxies, probe clones, and repeated hot-path whole-
  input/probe/graph scans are not.
- `CTDR-IMPL-A-004` and `CTDR-IB-001` remain open for the corrected source-real
  error matrix, exact node/evaluation/residual/Jacobian/pivot/norm/trajectory/
  full potential-and-final solve parity, rollback, and no-fallback evidence.
- `CTDR-IMPL-A-005` and `CTDR-IB-004` require distinct authenticated map,
  solve, Newton-iteration, and sweep identities. `Completed` means all required
  probes finished; `Failed` records the source-real first error and actual
  attempted counts; `RejectedBeforeProbe` is a column stencil/admission result.
  `ShortCircuited` exists only if the unchanged solver has a real non-error
  early-ending sweep path, otherwise its variant/counter/population is absent.
  Per-sweep and release aggregates must reconcile every supported status and
  both potential/final solve classes.
- `CTDR-IB-006` remains open for cohesive graph, custody, parity, error/
  rollback, audit, and release test ownership before size/reviewability limits.

### Earlier contract closure remains sound

Apart from `CTDR-A-R2-001`, the graph framework retains stable source-aligned
node families, inclusive closure, version/topology hashing, conservative
unknown handling, and the intended reciprocal-longwave, duplicated-wet,
all-lower-routing, leaf/CI/hydraulic, shared, route-match, terminal,
normalization, and final-output coverage. The missing direct records prevent
the generator itself from being called complete. Eligibility versus integrity
versus post-start error behavior remains ordered and fallback-free. Exact
`58 = 14 + 16 + 28` accounting remains one fully centered `N=2,S=6` sweep,
not a release total. BEI/profile metadata, frozen baseline/candidate protocol,
manifest custody, and the narrow structural-evidence classification remain
unchanged and correct.

No analytic/automatic derivative, graph coloring, simultaneous perturbation,
sparse Jacobian/LU, pivot change, approximation, cross-sweep/iteration/map/
retry cache, hardcoded fixture topology, recovery fallback, or alternate solver
is authorized.

### Fresh re-review checks run

Ran the exact ordered eight-file manifest recipe before the focused checks.

Result: `PASS`; digest matched
`9fa2a1fc1a5bc53dcd66ed43e7d09a41891be01d78de0db143e273e25d468986`.

Ran both active structural-parser adversarial tests.

Result: `PASS`, 2/2.

Ran the focused revision-31 authority assertion.

Result: `PASS`, 1/1.

Ran the exact narrow production-seam assertion.

Result: `EXPECTED_RED`; all and only the seven named replay declarations were
absent.

Ran the complete LSE authority integration target.

Result: `PASS_WITH_EXPECTED_RED`; 24 passed, exactly the revision-31
item-presence test failed, and zero were ignored. The only warning was the
unrelated pre-existing dead-code warning for two terminal-request fields.

Ran strict BEI, SC unit-compliance, and workspace formatting checks.

Result: `PASS`; 15 BEI rows were consolidated, unit compliance reported no
findings, and `cargo fmt --all -- --check` was clean.

The source-real fallibility/crossability correction and rejected-finding
disposition are otherwise feasible, and every rejected-candidate finding
remains open. The incomplete normative direct-edge generator blocks a fresh
implementation from beginning against this manifest; production readiness and
retention remain `HOLD`.

## Final direct-edge-corrected re-review A — 2026-09-04

Evidence mode: `Static + Ran + Expected-red`

Ordered eight-file manifest reviewed:
`767bc190704e006a51789dc8b6b27f7763f7674e3e2f17a8f2bf57d6b34b1583`

Verdict: `PASS`

Promotion recommendation: `GO` for a fresh tests-first revision-31
reimplementation against this exact authority manifest. Production readiness,
candidate retention, independent implementation review, verification, and
release remain `HOLD` until their executable gates pass. This section
supersedes the `9fa2...` verdict above and closes `CTDR-A-R2-001` /
`CTDR-RRB-003`; it does not approve the rejected and reverted implementation.

### Findings

No open contract-authority finding remains.

1. `CTDR-A-R2-001` / `CTDR-RRB-003` — HIGH — `CLOSED`. The normative
   direct-edge generator now explicitly includes all ten source-real reads
   omitted from the prior manifest: wet probe to first finalization, routing
   preparation to the second wet evaluation, column longwave to every
   occupancy output, both terminal routing results to final output,
   lower/ground output to each shared node, and shared heat/vapor to shared
   tolerance. The focused authority test asserts each relationship. Exact
   direct-edge expansion and golden hashes for `N=1,S=1` and real `N=2,S=6`,
   plus edge-removal/change failure, remain mandatory; transitive reachability
   alone is expressly insufficient.
2. The disposition table is structurally valid after removal of the
   table-breaking blank line: one eight-column header/separator is followed
   continuously by all 16 contract/corrected-review finding rows, including
   `CTDR-RRB-003`. The separate implementation-review table retains exactly
   five `CTDR-IMPL-A-*` and six `CTDR-IB-*` rows, all `accepted` and
   `open-until-reimplementation`.
3. The corrected feasibility matrix remains source-real. Current-leaf errors
   are fallible and canonically crossable and require authentic paired
   replay/forced-complete error-order and rollback vectors. Maximum-leaf is
   fallible but not currently established crossable and requires a
   guard-by-guard implication plus authentic boundary successes, with mandatory
   reclassification if a real counterexample is found. The remaining named
   fallible families require implication and authentic success-boundary
   evidence; infallible computation/assembly requires exact field parity and
   no invented failures. The differential catch-all covers every naturally
   occurring error. Synthetic hooks, forged intermediates, noncanonical
   probes, and test-only physics entries remain forbidden.
4. All earlier implementation obligations remain binding and open: one shared
   canonical node/evaluator tail; graph-driven complete direct-edge execution;
   efficient exact generations/borrows or compact validation-point seals for
   all input, caps, frozen, graph, base, trial, coordinate, sign,
   perturbation, probe, and stencil custody; exact first-error/no-fallback and
   byte rollback; distinct authentic map/solve/iteration/sweep identities;
   truthful completion/failure/admission status and counters; full node,
   residual, Jacobian, pivot, norm, trajectory and potential/final solve bit
   parity; and cohesive graph/custody/parity/error/audit/release tests. No
   analytic/AD derivative, coloring, simultaneous perturbation, sparse solve,
   pivot change, approximation, cache, fallback, or hardcoded fixture topology
   is admitted.
5. Accounting and retention remain exact. A fully centered `N=2,S=6` sweep is
   `58 = 14 + 16 + 28`, not a release total. The frozen source manifest,
   binary hash, raw timing/RSS tuples, sorted-middle medians and candidate
   ceilings remain internally consistent. Candidate qualification must use the
   identical CPU-0 command, one unchanged source/binary identity across three
   runs, both timing ceilings, exact science and `48/56/20/32/4`, authentic
   aggregate records containing a qualifying `58/14/16/28` sweep, full-solve
   bits, and per-run `rss_kib <= 65536`; any failed conjunct requires full v31
   production revert.
6. Production is truthfully reverted. Static inspection of
   `solver_covered_evaluation.rs`, `solver_covered_solve.rs`, `solver_tests.rs`,
   and the runner qualification finds none of the seven required replay
   declarations. The expected-red classifier proves only unconditional,
   non-`cfg` top-level item presence; skeleton, dead, token-only, or
   disconnected declarations cannot change readiness. Real-dispatcher
   counters and the forced-complete behavioral oracle must prove invocation,
   connectivity, consumption, provenance, and numerical equality.

### Final evidence

- `Ran:` the exact ordered eight-file recipe from `contract_ref.md`; digest
  matched `767bc190704e006a51789dc8b6b27f7763f7674e3e2f17a8f2bf57d6b34b1583`.
- `Ran:` structural-parser adversaries — `PASS`, 2/2.
- `Ran:` focused revision-31 authority assertion — `PASS`, 1/1, including all
  corrected direct-read relationships.
- `Ran:` narrow structural production seam — `EXPECTED_RED`; all and only the
  seven required declarations are absent.
- `Ran:` complete LSE authority target — `PASS_WITH_EXPECTED_RED`; 24 passed,
  exactly the named v31 seam failed, and zero were ignored. The sole warning is
  the unrelated pre-existing terminal-request dead-code warning.
- `Ran:` strict BEI — `PASS`, 15 consolidated rows; SC unit-compliance —
  `PASS`; workspace formatting — `PASS`; scoped manifest `git diff --check` —
  `PASS`.
- `Static:` all three parent-owned stale LSE version pins require revision 31
  and their two owning integration files are manifest members four and five.
  The frozen baseline log agrees with the contract/reference source manifest,
  binary hash, three raw tuples, medians, ceilings, science counts, and RSS
  rule.

The corrected same-v31 contract-first increment is source-real,
manifest-complete, and ready to govern a new implementation. No production,
performance-retention, verification, or package-completion claim is made.
