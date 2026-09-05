# Independent contract review B

Evidence mode: `Static + Ran + Expected-red`

Review target: `SC-LANDSURFACEENERGY-001` revision 31,
`INV-LANDSURFACEENERGY-164`, and `OBL-LANDSURFACEENERGY-C-020` at base commit
`a28c55c2d0f06e0c4aab58642f1009f70f82b3d3` plus the uncommitted five-file
contract-first increment.

Verdict: **NO-GO for production implementation**. The dependency-replay idea
is bounded to the canonical represented-snow solver and the centered-sweep
count algebra is correct, but the audit population and release comparison are
not yet frozen or unambiguous. Binding classification, current-version pins,
and source-real graph/error coverage also require correction before an
implementation-ready disposition.

## Findings

### B-01 — HIGH — the keep/revert comparison is not frozen or uniquely executable

The canonical amendment asks to freeze “a pre-v31 exact-release baseline” and
then compares three postimplementation repetitions
(`SC-LANDSURFACEENERGY-001.md:2935-2944`). The readiness matrix instead says the
missing baseline is a **three-run** CPU-0 baseline (`readiness-matrix.md:33`),
while `contract_ref.md:87-100` names neither the exact command nor the baseline
repetition/median rule. None of the five files records the pre-v31 source hash,
binary hash, three raw potential-bucket values, three raw wall values, median,
timing source, or exact profile field name.

Consequently “improve by at least 100000 us” has multiple valid
interpretations: post median versus one baseline run, versus a baseline median,
or versus a previously recorded package profile. “Total wall” could likewise
mean runner JSON wall, test-body wall, or external process elapsed time. The
RSS limit is clear per post run, but its measurement source is not named.

Required correction: before any v31 production edit, bind the exact release
command, immutable baseline source and binary hashes, three baseline
repetitions of that one binary, profile JSON field/path for potential
evaluation, wall and RSS measurement sources, median convention, and the exact
baseline-median-to-post-median comparison. Publish the raw baseline evidence in
the component artifact set.

### B-02 — HIGH — `58/14/16/28` is identifiable only per one centered sweep, not as the stated release-run count

For `N=2` and `S=6`, the coordinate count is 29. A full centered sweep has 58
admitted logical probes: 14 ground/soil anchors, 16 component-temperature
replays, and 28 complete evaluations. That arithmetic agrees with the current
solver topology (`solver_covered_solve.rs:545-609`) and the amendment
(`SC-LANDSURFACEENERGY-001.md:2914-2921`).

The release paragraph nevertheless requires every run to retain exact
`58/14/16/28` accounting alongside the 200-map runner workload
(`SC-LANDSURFACEENERGY-001.md:2935-2944`; `contract_ref.md:87-95`). A release run
can contain multiple potential/final solves, Newton sweeps, inward-bound probes,
and early errors. The contract does not define whether counters reset per
sweep, per solve, per map, or per process, how many qualifying centered sweeps
must be observed, or how inward probes are bucketed. A boundary column has only
one admitted evaluator call, so the generic `2*(...)` formula is not its
logical-call identity.

Required correction: define one authentic named `N=2, S=6`, fully interior
Jacobian-sweep observation and its counter reset/snapshot boundaries. Separately
define aggregate release counters or a per-sweep histogram, including potential
versus fixed-final identity, centered versus inward stencils, number of
qualifying sweeps, and treatment of rejected/short-circuited sweeps. Do not use
one fixture's 58-count as the unexplained whole-run total.

### B-03 — HIGH — Binding Exposure classification contradicts the canonical schema

The actual Binding Exposure Index row classifies v31 as
`maps-to-existing-INV` (`SC-LANDSURFACEENERGY-001.md:577`), which is valid under
`science-contract-spec.md` and maps to the now-core `INV-164/C-020`. The
amendment profile instead calls the same entry `new-INV`
(`SC-LANDSURFACEENERGY-001.md:2959`), a value absent from the allowed Binding
Exposure vocabulary. The contract-derived test requires the stray
`` `new-INV` `` substring (`land_surface_energy_balance_authority_contract.rs:323`)
but never asserts the classification cell of the BEI row
(`land_surface_energy_balance_authority_contract.rs:333-335`). Thus strict BEI
lint passes while the profile and test preserve a contradictory classification.

Required correction: use `maps-to-existing-INV` consistently and make the test
assert that exact classification on the extracted v31 BEI row, not a free
substring elsewhere in the contract.

### B-04 — HIGH — current revision pin reconciliation is incomplete outside the five-file target

The focused authority target's three v31 pins are current, but package-relevant
integration tests still require LSE contract versions that are absent from the
revision-31 document:

- `snow_stage3_shared_carrier_authority_contract.rs:232` requires
  `contract_version: 24`.
- `stage3_native_vegetation_laned_throughput_recovery.rs:1234` and `:1329`
  require `contract_version: 27`.

Static checks confirm the current contract contains neither literal. These are
runtime `contains` assertions, not historical metadata checks, so their targets
are stale-red even though `contract_ref.md:68-71` reports no stale pin in the
narrow LSE authority target.

Required correction: reconcile all three assertions to current lifecycle
authority, or replace the version assertions with the specific invariant/
obligation/exposure identity when the test is intentionally amendment-specific.
Add every corrected file to the reviewed manifest.

### B-05 — HIGH — the source-real dependency/error oracle omits named terminal liquid descendants and the duplicated wet path

The high-level graph correctly names reciprocal longwave and the
upper-wet-to-lower occupancy chain. The evaluator, however, executes a wet
temperature through two distinct source-ordered regions: the initial routing
loop (`solver_covered_evaluation.rs:1976-2007`) and the later per-occupancy
evaluation/finalization plus routed-result equality guard
(`solver_covered_evaluation.rs:2041-2060`; `:1343-1352`; `:1478-1488`). The last
occupancy's routed result also reaches the returned terminal
`ground_canopy_release_kg_m2_tile` and accumulated
`ground_stemflow_kg_m2_tile` (`solver_covered_evaluation.rs:2365-2367`). These
terminal descendants and the two evaluations/equality join are not enumerated
in the amendment's explicit cross-component list. The current “each fallible
dependency” wording supplies no frozen node/error-order ledger from which
oracle completeness can be audited.

This matters for first-error parity: wet saturation/finalization in the routing
loop precedes reciprocal longwave, while leaf and occupancy failures follow
longwave. A paired poison can expose a replay walker that computes all the
right values in the wrong order.

Required correction: add the two source-distinct wet evaluation regions,
routed/final equality join, final ground-release/stemflow descendants, and
their precise error precedence to the graph/OBL vector. Freeze an enumerable
node/edge and first-error matrix for sun, shade, wet, and stem probes. The
forced-complete oracle must compare these evaluation fields as well as residual
and Jacobian bits.

### B-06 — MEDIUM — the structural expected-red can turn green without a production seam

`revision_31_component_temperature_dependency_replay_structural_seam_is_expected_red`
concatenates two files and accepts six raw substrings
(`land_surface_energy_balance_authority_contract.rs:340-357`). A comment,
test-only item, unused function, or disconnected audit symbol can satisfy it.
It does not prove that the helper is called from
`covered_jacobian_probe_residuals`, that component coordinates select it, or
that noncomponent coordinates retain the complete evaluator.

The current failure is a valid absence classification, and `contract_ref.md`
correctly says it is not behavioral acceptance. Before expecting this test to
turn green, strengthen it with production-item/body extraction and call-site
assertions, while leaving executable forced-complete differential tests as the
behavior authority.

## Source-topology disposition

The bounded optimization is technically plausible. The current coordinate
layout is exactly ten values per occupancy followed by shared temperature,
shared humidity, ground, and soil (`solver_covered_evaluation.rs:1954-1970`),
and `covered_jacobian_probe_residuals` already supplies the one-coordinate and
represented-snow identity-anchor admission seam
(`solver_covered_solve.rs:286-324`). Reciprocal longwave is evaluated before
the occupancy leaf/hydraulic nodes (`solver_covered_evaluation.rs:2009-2055`),
and upper wet routing feeds every later occupancy (`:1976-2007`). The proposed
static union dependency graph therefore has a source-real implementation
location and does not require a new solver or derivative rule.

That feasibility does not cure B-01 through B-05. In particular, a blanket
“every direct read” rule is not enough to prove that an implementation's graph
is complete; the test obligation needs an enumerable source-real graph and
error ledger.

## Commands and results

- `Ran:` focused v31 authority test — `PASS`, 1/1.
- `Ran:` named structural seam test — `EXPECTED_RED`, with all six named
  symbols absent.
- `Ran:` strict Binding Exposure Index lint — `PASS`, 15 rows fully
  consolidated. This does not inspect the contradictory amendment-profile
  `new-INV` prose.
- `Ran:` SC unit-compliance lint — `PASS`, no findings.
- `Ran:` `cargo fmt --all -- --check` — `PASS`.
- `Ran:` scoped five-file `git diff --check` — `PASS`.
- `Ran:` exact ordered five-file manifest recipe —
  `4faf1cdd6ebd618d12a0a23501cf4f9df85d536f61b4dad74dd70ec4e452ed13`.
- `Static:` the current SC-LSE file contains neither `contract_version: 24`
  nor `contract_version: 27`; the three stale assertions are identified in
  B-04.

## Exit recommendation

`NO-GO`. Correct B-01 through B-05, strengthen or explicitly stage B-06,
recompute the expanded ordered manifest, rerun the full affected integration
targets, and obtain review-B revalidation before authoring production replay.

## Corrected final re-review — replacement eight-file manifest

Evidence mode: `Static + Ran + Expected-red`

Review target: the exact ordered eight-file manifest supplied for corrected
re-review. I independently recomputed the manifest using the recipe in
`contract_ref.md`:

```text
cbba3ea117e088677e80de7099af5e03a1dc5e4e7444430795dda8ba7ca71da4
```

Verdict: **NO-GO for production implementation**. B-01 through B-05 are
closed, and the character/byte-character, lifetime, raw-string, cfg,
visibility/modifier, nested-item, manifest-custody, graph, error-ledger, BEI,
pin, accounting, and baseline corrections are present. B-06 is still open
because the structural call-site check is identifier-aware but not call-aware
or unconditional as claimed.

### CTDR-RRB-002 — HIGH — a dead identifier occurrence still satisfies the
claimed unconditional dispatcher call

`rust_item_has_unconditional_identifier` extracts the dispatcher body, then
requires only (a) any token equal to the helper name and (b) no `cfg` or
`cfg_attr` attribute anywhere in that body
(`land_surface_energy_balance_authority_contract.rs:380-383`). It does not
require the token to be followed by a call argument list, and it does not
establish that the occurrence is on an unconditional execution path. For
example, each of these bodies makes the assertion at lines 857-861 true while
providing no unconditional replay dispatch:

```rust
fn covered_jacobian_probe_residuals() {
    let covered_component_temperature_probe_residuals = 0;
}

fn covered_jacobian_probe_residuals() {
    if false {
        covered_component_temperature_probe_residuals();
    }
}
```

A bare function reference or same-named local item is another false green.
The new adversarial test proves rejection only when a `cfg` attribute occurs
somewhere in the body (lines 475-510); it has no negative control for a
non-call token, bare reference, or dead non-cfg control-flow call. The four
helper-consumption checks at lines 864-873 have the same limitation: token
presence is labelled as consumption without proving a value-flow use.

This is the core condition of original finding B-06: the helper must be called
from `covered_jacobian_probe_residuals`, not merely named there. The separate
future forced-complete oracle remains mandatory behavior authority, but it
does not make a false-green contract-first structural gate truthful.

Required correction: make the structural predicate at least call-token aware
and add active negative fixtures for a same-named binding/reference and a dead
or conditional call. Either enforce the claimed unconditional production
dispatch structurally or narrow the artifact claim and retain an executable
gate that proves component coordinates actually take the helper while
noncomponent coordinates retain the complete evaluator. Apply equivalent
call/use-aware treatment to the helper-consumption assertions or describe
them truthfully as symbol-presence checks.

### Closed finding and cross-review audit

- `Static:` B-01 is closed. The exact CPU-0 release command, source manifest,
  binary path/hash, three raw `(run_wall_us,
  physical_phase_wall_us.potential,rss_kib)` tuples, sorted-middle medians,
  exact conjunctive candidate ceilings, unchanged-identity rules, per-run RSS
  gate, and full-revert disposition are frozen before implementation.
- `Static:` B-02 is closed. `58/14/16/28` is expressly one authentic fully
  centered `N=2,S=6` sweep, generic admitted-sign identities are separate,
  sweep reset/seal boundaries are exact, and release aggregation is a
  distinct solve/stencil/completion histogram rather than a fabricated run
  total.
- `Static:` B-03 and CTDR-A-004 are closed. The BEI classification is the
  schema-valid `maps-to-existing-INV`; revision 31's introduction of new IDs
  is a separate fact, and contract/index evidence levels agree.
- `Static:` B-04 and CTDR-RRA-002 are closed. The owning shared-carrier and
  Stage-3 integration files require `contract_version: 31` and are manifest
  members four and five. The ordered eight-file custody digest above matches.
- `Static:` B-05 and CTDR-A-001 through A-003 are closed. The stable graph and
  direct-edge generator cover both wet regions/finalizations, all-lower
  routing, reciprocal longwave, route equality, terminal release/stemflow,
  leaf/CI/vapor/hydraulic/shared/residual/output descendants, inclusive
  closure, source order, paired poisons, and the selection/integrity/post-start
  first-error ledger. Immutable sweep evidence and fresh single-use signed
  capabilities carry all required identities and lifetimes.
- `Static + Ran:` the lexer masks cooked strings, arbitrary-hash raw and raw
  byte strings, comments, character literals, and byte-character literals.
  Its literal parser handles escapes while refusing to consume named,
  `'static`, and placeholder lifetimes. Both active parser adversarial tests
  pass. The nested fixture places all seven required items behind cfg/cfg_attr
  inside a function; the closing-brace character literal prevents false
  top-level depth if masking regresses. Stacked/multiline attributes through
  `pub`, `pub(...)`, and function modifiers are rejected. This closes the
  lexer/nesting portion of B-06 and CTDR-RRA-001/RRB-001, but not
  CTDR-RRB-002.
- `Static:` B-01/A-005's performance rule is unambiguous: candidate medians
  must satisfy both `run_wall_us <= 4803570` and
  `physical_phase_wall_us.potential <= 253431`, and every candidate run must
  report `rss_kib <= 65536` under the same frozen command and identities.

### Commands and results

- `Ran:` exact ordered eight-file manifest recipe — `PASS`, digest
  `cbba3ea117e088677e80de7099af5e03a1dc5e4e7444430795dda8ba7ca71da4`.
- `Ran:` both exact parser anti-evasion tests — `PASS`, 1/1 each.
- `Ran:` exact revision-31 contract-derived assertion — `PASS`, 1/1.
- `Ran:` complete LSE authority target — `PASS_WITH_EXPECTED_RED`: 24 passed,
  exactly the named revision-31 structural seam failed on the seven absent
  production items, zero ignored, and no unrelated assertion failed.
- `Ran:` strict Binding Exposure Index lint — `PASS`, 15 consolidated rows.
- `Ran:` SC unit-compliance lint — `PASS`, no findings.
- `Ran:` `cargo fmt --all -- --check` through the Nix development shell —
  `PASS`.
- `Ran:` scoped eight-file `git diff --check` — `PASS`.
- The build emitted one unrelated existing dead-code warning for two terminal
  request fields in the hillslope orchestrator.

### Exit recommendation

`NO-GO`. Correct CTDR-RRB-002, add a self-test that would fail for the concrete
false-green examples, recompute the ordered manifest, and obtain corrected
review-B revalidation. Production implementation, the forced-complete
differential oracle, authentic counters, candidate release qualification, and
dual verification remain later mandatory gates and were not run or accepted
by this contract-first re-review.

## Final corrected re-review — manifest `23d90c29...058b`

Evidence mode: `Static + Ran + Expected-red`

Review target: the replacement ordered eight-file manifest. Independent
recomputation by the `contract_ref.md` recipe produced exactly:

```text
23d90c29038eb836b1ce8f2105588f44d382f2ae5b726a7bc55467b842ac058b
```

Verdict: **PASS / GO for production implementation**. This verdict supersedes
the NO-GO above only for this replacement manifest. No open reviewer-B finding
remains.

### CTDR-RRB-002 closure

`CTDR-RRB-002` is closed by truthful evidence-class separation, not by
pretending source text proves runtime connectivity:

- The structural assertion now checks only that seven named declarations are
  unconditional, non-cfg, top-level Rust items. It contains no dispatcher-call
  or graph/evidence-consumption assertion.
- The contract, `contract_ref.md`, readiness matrix, and disposition all state
  that the classifier cannot prove invocation, reachability, consumption,
  counter provenance, or numerical behavior. They expressly admit that empty,
  skeleton, token-only, dead, or disconnected declarations can green it and
  cannot establish implementation readiness.
- The adversarial fixture intentionally retains a cfg-only call, same-named
  local binding, bare reference, and dead call. These demonstrate why token
  occurrence is not connectivity evidence without asking the narrow classifier
  to infer control/data flow.
- `OBL-LANDSURFACEENERGY-C-020` instead reserves connectivity and consumption
  authority to post-implementation executable tests that exercise the real
  dispatcher, observe authentic sealed sweep/run counters, and pass the
  forced-complete node/evaluation/residual/Jacobian/full-solve oracle. Until
  that evidence exists, production implementation and behavior remain
  `BLOCKED`/`HOLD` even if the structural assertion turns green.

This resolves the original B-06 concern: a skeleton can no longer be
misrepresented as an implementation-ready production seam. The expected-red
is now an explicitly narrow declaration-presence classifier.

### Final closure audit

- `B-01`/`CTDR-A-005`: closed. One exact CPU-0 release command, immutable
  baseline source/binary identities, exact JSON fields, three raw tuples,
  sorted-middle medians, both `100000 us` candidate improvements, per-run RSS
  ceiling, scientific/workload/counter requirements, and full-revert rule are
  frozen and unambiguous.
- `B-02`: closed. Generic admitted-sign identities and the named fully centered
  `N=2,S=6` `58/14/16/28` sweep are distinct from separately reset release
  aggregation over solve, stencil, completion, failure, and short-circuit
  classes.
- `B-03`/`CTDR-A-004`: closed. BEI uses schema-valid
  `maps-to-existing-INV`; the new-ID fact is separate; contract and index use
  `static+independent_oracle+contract_vectors`.
- `B-04`/`CTDR-RRA-002`: closed. All three stale LSE pins require revision 31,
  both owning integration files are manifest members four and five, and the
  exact eight-file custody digest matches.
- `B-05`/`CTDR-A-001..003`: closed. The graph, direct-edge generator, inclusive
  closure, source order, wet/routing/terminal descendants, reciprocal longwave,
  leaf/CI/vapor/hydraulic/shared/residual/output nodes, error ledger, paired
  poisons, selection/integrity/post-start outcomes, and evidence lifetimes are
  complete and source-real.
- `B-06`/`CTDR-RRA-001`/`CTDR-RRB-001..002`: closed. Cooked/raw/raw-byte,
  arbitrary-hash, comment, character, byte-character, escape, lifetime,
  visibility/modifier, stacked/multiline cfg/cfg_attr, nested-item, and
  deliberately non-connective token adversaries are correctly classified;
  structural and executable evidence claims are now separated.

### Ran evidence

- Ordered eight-file manifest: `PASS`, exact digest above.
- Both exact structural-parser adversarial tests: `PASS`, 1/1 each.
- Exact revision-31 contract-derived assertion: `PASS`, 1/1.
- Complete LSE authority target: `PASS_WITH_EXPECTED_RED`; 24 passed, exactly
  the revision-31 structural seam failed on the seven absent production items,
  zero ignored, and no unrelated assertion failed.
- Strict Binding Exposure Index: `PASS`, 15 consolidated rows.
- SC unit compliance: `PASS`, no findings.
- `cargo fmt --all -- --check` in the Nix development shell: `PASS`.
- Scoped eight-file `git diff --check`: `PASS`.

The build's two-field orchestrator dead-code warning is unrelated and
pre-existing. The first host-shell Cargo attempt was not evidence because
`cargo` was not on that shell's path; all reported Rust results above ran via
`nix develop`.

### Scope of GO

This is contract-readiness GO only. It authorizes the production increment to
begin under the frozen contract. It does not accept an implementation,
forced-complete behavior, authentic counters, performance qualification,
release retention, package completion, or dual verification. Those gates
remain mandatory exactly as recorded in the contract and readiness matrix.

## Fresh feasibility-corrected re-review — manifest `9fa2a1fc...8986`

Evidence mode: `Static + Ran + Expected-red`

Review target: the corrected same-v31 ordered eight-file authority manifest.
I independently recomputed the prescribed ordered `sha256sum | sha256sum`
recipe as:

```text
9fa2a1fc1a5bc53dcd66ed43e7d09a41891be01d78de0db143e273e25d468986
```

Verdict: **NO-GO for reimplementation**. The previous PASS/GO applied only to
the superseded `23d90c29...058b` manifest. The revert and feasibility
correction are otherwise truthful, but the newly strengthened claim that the
normative direct-edge generator is complete is false against the source-real
evaluator.

### CTDR-RRB-003 — HIGH — the normative “complete” direct-edge generator still
omits source-real direct reads

The node table treats `route.wet`, `route.finalize`, `occ.wet`, `occ.liquid`,
`longwave.column`, `occ.output`, and `result.output` as distinct nodes. On that
declared granularity, at least these direct edges are missing from the
generator at `SC-LANDSURFACEENERGY-001.md:2934-2956`:

1. `probe[o,wet] -> route.finalize[o]`. The first finalization directly reads
   the wet-temperature probe as `block[8]`, independently of the wet-flux
   result (`solver_covered_evaluation.rs:1745-1754`). The probe row names
   `route.wet[o]` but not `route.finalize[o]`. Transitive reachability through
   wet flux does not record that second direct operand.
2. `route.prepare[o] -> occ.wet[o]`. The second wet-flux evaluation directly
   consumes `context.liquid`, which is the preparation retained at lines
   1798-1808 and passed to `covered_wet_flux` at lines 1069-1077. The
   `route.prepare` target list names the first `route.wet/finalize` and then
   vapor/hydraulic/sensible/energy/tolerance/liquid/output families, but omits
   the distinct `occ.wet` node.
3. `longwave.column -> occ.output[o]`. The reciprocal-column result supplies
   `component_longwave_w_m2` at lines 1789-1803, and that array is stored
   directly in each `CoveredOccupancyEvaluation.net_longwave_w_m2` at lines
   1387-1414. The longwave row targets component energy/tolerance,
   `lower.ground_output`, and `result.output`, but not the declared
   `occ.output` owner.
4. `result.ground_release/result.ground_stemflow -> result.output`. The final
   `CoveredColumnEvaluation` assembly directly installs both terminal values
   (`solver_covered_evaluation.rs:2092-2120`), but the generator has no direct
   target from either distinct result node to `result.output`.

These are not harmless conservative extras. They are source reads whose
omission makes the independently enumerated edge set and golden hash bless an
incomplete graph. A coarse reachability plan might still recompute enough
work, which is exactly the weakness reported by `CTDR-IMPL-A-002` and
`CTDR-IB-002`. The authority says omitted direct edges are forbidden and that
changing any required edge must fail; therefore those implementation-review
findings do not yet have a complete corrected authority target.

Required correction: enumerate every direct edge at the declared node
granularity, including the four families above, then audit every remaining
node output against every source operand before freezing the edge/hash oracle.
Make the authority assertion bind the corrected relationships rather than only
generic “complete direct-edge” prose. Recompute the eight-file manifest and
obtain a fresh review.

### Confirmed corrections

- `Static:` the rejected implementation is fully reverted. All seven
  graph/base/probe/audit/function symbols are absent from both production
  files; the hardened structural test remains red on exactly those seven.
- `Static:` the feasibility matrix correctly distinguishes source-real
  canonically crossable current-leaf errors, maximum-leaf and other fallible
  surfaces not established crossable from a successful base plus canonical
  admitted probe, and infallible computations/assembly under validated
  predecessors. It requires source-real paired errors only where crossable,
  guard-by-guard implication plus authentic boundary successes for each
  noncrossable fallible family, exact fields for infallible families, and a
  catch-all differential for every naturally occurring error. Mutation seams,
  synthetic fault hooks, alternate tolerances, impossible branches, and forged
  intermediates are forbidden.
- `Static:` all five `CTDR-IMPL-A-*` and six `CTDR-IB-*` findings are accepted
  and remain `open-until-reimplementation`; the disposition does not falsely
  convert corrected authority into implementation evidence.
- `Static:` shared canonical evaluator node/tail implementation, no mirrored
  arithmetic, exact typed/compact generation/input/caps/frozen/graph/trial/
  coordinate/sign/perturbation/probe/stencil custody, efficient graph reuse,
  authentic independent map/solve/iteration/sweep identities, truthful
  completed/failed lifecycle, conditional presence of only a real
  short-circuit class, and no-audit allocation avoidance are all bound.
- `Static:` B-01 through B-04, B-06, the raw/literal/cfg anti-evasion work,
  BEI classification, revision pins, named-sweep versus release accounting,
  frozen performance protocol, and narrow structural evidence classification
  remain closed. B-05's value/error feasibility correction is sound, but its
  dependency-completeness portion is reopened by CTDR-RRB-003.

### Ran evidence

- Exact ordered eight-file manifest recipe: `PASS`, digest above.
- Both exact structural-parser adversarial tests: `PASS`, 1/1 each.
- Exact revision-31 contract-derived assertion: `PASS`, 1/1.
- Complete LSE authority target: `PASS_WITH_EXPECTED_RED`; 24 passed, exactly
  the seven-symbol revision-31 structural classifier failed, zero ignored.
- Strict Binding Exposure Index: `PASS`, 15 consolidated rows.
- SC unit compliance: `PASS`, no findings.
- `cargo fmt --all -- --check` through `nix develop`: `PASS`.
- Scoped eight-file `git diff --check`: `PASS`.

The passing authority assertion is a string-binding check and does not detect
the direct-edge omissions above. No production behavior or release benchmark
was run; none is appropriate while the implementation remains reverted.

### Exit recommendation

`NO-GO`. Correct CTDR-RRB-003 and obtain fresh dual contract review before a
new production attempt. Preserve all 11 implementation-review findings as
open until a future source-real implementation supplies their executable
closure and passes independent implementation review.

## Final feasibility-corrected re-review — manifest `767bc190...1583`

Evidence mode: `Static + Ran + Expected-red`

Review target: the final ordered eight-file authority manifest. Independent
recomputation produced exactly:

```text
767bc190704e006a51789dc8b6b27f7763f7674e3e2f17a8f2bf57d6b34b1583
```

Verdict: **PASS / GO for a fresh production reimplementation**. This verdict
supersedes the NO-GO for `9fa2a1fc...8986`; no reviewer-B authority finding is
open on the replacement bytes.

### CTDR-RRB-003 closure

The normative direct-edge generator now explicitly binds all ten formerly
implicit source-real relationships at its declared stable-node granularity:

- wet probe to the first routing finalization;
- routing preparation to the second occupancy wet evaluation;
- reciprocal column longwave to every occupancy output;
- terminal ground release and stemflow to final result output;
- lower/ground output to shared heat, shared vapor, and shared tolerance; and
- shared heat and shared vapor to shared tolerance.

The contract-derived revision-31 assertion freezes each relationship rather
than accepting only generic completeness prose. The future independent graph
oracle must still expand and compare every node/direct-edge record and golden
schema hash for `N=1,S=1` and `N=2,S=6`, reject removal/change of every edge,
and include any extra conservative edge explicitly. Thus transitive
reachability alone cannot close `CTDR-IMPL-A-002` or `CTDR-IB-002`.

`CTDR-RRB-003` now appears contiguously inside the historical findings table in
`disposition.md`; the table-breaking blank line in the superseded
`7d158e9c...5915` manifest is gone. It is structurally recorded as
`accepted/closed` before the separate implementation-review table.

### Complete fresh audit

- Production revert: `PASS`. Static searches find none of the seven replay
  graph/base/probe/audit/function declarations in either production source.
  The structural classifier is red on exactly all seven symbols; no partial
  rejected mechanism is retained.
- Feasibility: `PASS`. Current-leaf source-real errors are the presently
  crossable class. Maximum-leaf and the named fallible noncrossable/not-yet-
  established-crossable families require guard-by-guard successful-base plus
  admitted-probe implication proofs and authentic boundary/branch successes.
  Infallible families require exact source-order field parity without invented
  errors. The unmodified differential corpus catches any naturally occurring
  error, while mutation seams, fault injection, forged intermediates,
  alternate tolerances, and synthetic physics entry points are forbidden.
- Canonical evaluation and graph: `PASS`. Complete and replay paths must share
  one canonical node/tail implementation with no mirrored physical,
  tolerance, residual, branch, or output arithmetic. The expanded direct-edge
  set, inclusive closure, conservative unknown-edge handling, exact schema
  identity, source order, duplicated wet routing, all-lower propagation,
  reciprocal longwave, terminal routing, shared reductions/tolerances,
  residual normalization, and result assembly are bound.
- Custody and performance design: `PASS`. The base/probe bind authentic
  generation, every input, exact caps/frozen values, graph, trial,
  map/solve/iteration/sweep, coordinate, sign, perturbation, exact probe, and
  actual stencil. Typed borrows/generations or compact validation-point seals
  replace `Debug`, length/hash-only, cloning, repeated scans, and per-probe
  graph construction.
- Audit truthfulness: `PASS`. Map, solve, iteration, and sweep identities come
  from distinct real lifecycle positions; completed and failed records reflect
  work actually performed. `RejectedBeforeProbe` is a stencil outcome, and a
  `ShortCircuited` lifecycle exists only if the unchanged solver has an
  authentic non-error early-end path. Disabled audit collection need not
  allocate.
- Finding disposition: `PASS`. All historical contract-review findings through
  CTDR-RRB-003 are accepted/closed. All five `CTDR-IMPL-A-*` and six
  `CTDR-IB-*` findings are accepted and remain
  `open-until-reimplementation`; corrected authority is not misreported as
  executable implementation closure.
- Baseline and retention: `PASS`. The exact CPU-0 command, source and binary
  identities, three raw JSON tuples, sorted-middle medians, dual `100000 us`
  improvement ceilings, per-run `rss_kib <= 65536`, science/workload/audit/
  full-solve requirements, and full-revert rule remain frozen and
  unambiguous.
- Prior schema/pin/parser matters: `PASS`. BEI classification and new-ID fact,
  revision-31 index/evidence level, all three owning integration pins, ordered
  eight-file custody, named-sweep versus release accounting, and the complete
  raw/raw-byte/character/byte-character/lifetime/cfg/nesting structural
  anti-evasion posture remain reconciled.

### Ran evidence

- Ordered eight-file digest: `PASS`, exact value above.
- Both structural-parser adversarial tests: `PASS`, 1/1 each.
- Revision-31 contract-derived assertion: `PASS`, 1/1.
- Complete LSE authority target: `PASS_WITH_EXPECTED_RED`; 24 passed, exactly
  the seven-symbol revision-31 structural classifier failed, zero ignored, and
  no unrelated assertion failed.
- Strict Binding Exposure Index: `PASS`, 15 consolidated rows.
- SC unit compliance: `PASS`, no findings.
- `cargo fmt --all -- --check` through `nix develop`: `PASS`.
- Scoped eight-file `git diff --check`: `PASS`.

The build emitted only the unrelated existing two-field dead-code warning in
the hillslope orchestrator terminal request.

### Scope of GO

This GO authorizes a fresh implementation attempt against the corrected
contract. It does not close any of the 11 implementation-review findings or
claim production connectivity, graph execution, behavioral parity, counters,
performance qualification, release retention, verification, or package
completion. Those require new executable evidence and independent review; the
current truthful overall readiness remains `HOLD` until they pass.
