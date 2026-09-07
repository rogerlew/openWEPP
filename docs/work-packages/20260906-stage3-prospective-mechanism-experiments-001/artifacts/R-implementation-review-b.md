# R implementation QA review B

Static: independent review of isolated R against common A
`2b56e6ebc9c8e8073fb68b89626d4d4e2b4602e3`, 2026-09-06. Inspected six modified
and four new Rust files, their runtime consumers, current authority and retained
graph/runtime-test/implementation plans. Review A was not consulted before the
first source findings were submitted. Ran: `git diff --check`, PASS. No build,
Rust test, compiler-negative test, lint, simulation or measurement was run here.

## Findings

| ID / severity | Path / line at inspected cut | Finding and disposition |
| --- | --- | --- |
| RIB-1 / P2, resolved statically | L `solver_component_dependency_replay.rs:134`; L `solver_covered_solve.rs:719` | The initial canonical pair minted a component-replay capability for every lawful coordinate, including non-Stage-3/noncomponent/graph-ineligible probes. That contradicted INV164's selection-before-capability rule. Corrected pair members are non-Clone generic signed probes. Their consumed checked entry mints the separate component capability only after represented-snow graph eligibility and forced-complete selection; ordinary probes retain canonical complete/anchor dispatch. Exact h/vector/stencil arithmetic is unchanged. |
| RIB-2 / P2, resolved statically | L `solver_component_dependency_replay_tests.rs:223` | The initial full-field node oracle substituted a recomputed frozen-base capture for the actual dispatcher's production natural-branch capture. Corrected source takes `base.capture.as_ref()` and passes that exact capture to full-field replay. It separately compares the frozen re-evaluation and capture against the original base evaluation/capture. This closes the oracle-source mismatch, not the still-unexecuted bitwise runtime gate. |
| RIB-3 / P2, scientific-admission HOLD | retained `R-runtime-test-plan.md`, Exact leaf guard disposition; `SC-LANDSURFACEENERGY-001.md:3007` | The guard table truthfully leaves conditional IEEE/carbon/Brent maximum implications unresolved. Exact operands prove skipped-node guards identical, and shared execution proves reachable-node parity/order; neither alone proves the stronger successful-base-plus-admitted-probe implication that C020 requires for its noncrossable rows. Complete the named-guard implications over existing admitted premises, or establish authentic crossability before an authorized classification disposition. Do not silently substitute ordinary finite fixtures, invented physiological bounds, or global-range assertions. This does not block compiling/testing the candidate, and is not a finding that R changes its physical equations. |

L means `crates/openwepp-land-surface-energy/src/` in isolated R. RIB-3 also
retains the stated route/wet/longwave/hydraulic/lower boundary-success obligations;
none is waived by this source review or by an exact output from one fixture.

## Implementation and custody assessment

The graph is constructed once in the validated evaluation-input object, outside
the Newton/probe loop. Exact lexical nodes/direct edges, schema/N/S hash and
inclusive closure reduce to routing/current/maximum masks. Unknown endpoints or
coordinates cannot prove independence. Runtime uses no graph strings, ordered
map rebuilding, graph hashing or closure walk inside a signed probe. A future
new evaluator read still requires explicit authority/graph review; this static
descriptor does not discover reads automatically.

Only routing preparation/first wet/finalization and selected leaf current/maximum
results are copied. Every other scientific expression executes in the same
canonical evaluator body and source order. Sun/shade/stem probes preserve route
inputs; wet probes rerun their route and all lower routes. Leaf function inputs
exclude wet-adjusted area, so downstream wet-area changes do not authorize
unnecessary leaf recomputation or change a copied leaf argument. The graph
current-to-maximum edge prevents copying a maximum whose current leaf changed.

For a skipped route, both original wet calls use identical preparation, wet T,
canopy T/q and column operands; the frozen wet branch is taken from that same
successful occupancy evaluation. It equals the original natural branch (or the
unchanged natural selection is used if no branch entry exists). Thus the copied
route trio retains exact guard inputs and first-finalizer values. The second wet
call/finalizer still runs the canonical functions. This exact skipped-node proof
is narrower than an unproved global noncrossability assertion for changed inputs.

The successful iteration evaluation owns capture. Its immutable sweep base owns
trial/frozen/capture and borrows validated column/caps/graph, preserving exact
generation and input identity without hashes as proxies. Canonical minus/plus
construction owns unchanged vectors and mints consumed signed proofs. Foreign
live bases reject through pointer identity; coordinate/sign/h/stencil and exact
probe coordinate bits are checked before physical work. Optional observation
context is attribution, not an independent grant of scientific custody. The real
local construction sites bind solve/iteration/sweep lifetimes. Capture is dropped
when the base is consumed after sweep completion, before LU/backtracking.

The regression adapter for externally supplied vectors is test/test-support
only; its vector scan is not the production capability proof. Ordinary malformed
or multicoordinate input retains complete-evaluator selection/error precedence.
After recognized replay begins, reachable-node errors return directly; no
complete evaluator or alternative solver is invoked as recovery. Probe order,
canonical h/units, finite-difference stencil/value formulas, row order, scaled
matrix/RHS operations, LU, convergence and backtracking arithmetic match A.

## Test and gate quality

The independent graph oracle enumerates all node pairs using normative direct-read
predicates, independently serializes fields and pins N1/S1 and N2/S6 golden hashes.
It compares exact production edge records, deletes/changes every direct edge,
and tests unknown-read fail-closed behavior and compact selectors. It does not
mistake transitive reachability for a direct edge.

The runtime corpus compares explicit scalar/vector bits for complete evaluation,
liquid preparation/ledgers, four ordered leaf states, longwave, raw/tolerance/
normalized residuals, complete dense Jacobian, matrix/RHS/linear diagnostics and
successful trial trajectories. Accepted/rejected solve fields additionally receive
explicit bit comparisons. Potential and fixed-cap, exact beta, dry/capacity,
signed-zero PAR and inactive fixture requirements cannot pass via an ignored
matched early error: success callers now assert full-sweep completion.

Current source-real error tests require an actually successful complete base and
unchanged canonical probe. The VPD/Ci and bounded gb/Rd/activation/subnormal cases
are source input constructions, not mutation seams or forged intermediate errors.
Every bracket mismatch fails rather than silently hunting or accepting another
error family. These tests remain NOT RUN and cannot establish an unexecuted
surface, phenotype validity or a universal numeric bound.

The later separately bounded maximum-compensation investigation is diagnostic,
not a noncrossability gate: it brackets PAR, inspects a fixed local temperature/
beta/sign corpus, and admits only actually successful complete bases. A witness
requires the exact existing error, replay/complete first-error parity and leaf-call
counts identifying current-then-maximum execution. Zero witnesses is explicitly
not a theorem. Unlike the named-error brackets above, this exploratory bracket
can stop on an unrelated complete-evaluator error. The corrected diagnostic logs
that identity and exact parameter bits, plus bracket-error/rejected-base counts,
so its result is not mistaken for complete surface coverage.

Foreign-base and wrong-coordinate/sign/h/stencil tests exercise real consumed
constructor objects. The retained compiler-negative plan compiles actual private
types in a disposable copy, requires positive compilation first and named E0382/
E0502/E0505 failures for second-use/mutation/drop. It now uses the generic consumed
`evaluate_for` API, not a stale direct call with a mismatched capability type.
That gate still must run; inaccessible/stub symbols or unrelated compiler errors
are not custody evidence.

The runner has separately named forced-complete and N2/S6 node-oracle tests.
They reuse the authenticated production consumer. Normal measurement does not
enable either scoped flag. Oracle code exists only under test/test-support;
ordinary callbacks see disabled state and return. Node-oracle injection records
extra physical work honestly and is explicitly excluded from lifecycle/timing
claims. Both actual potential and fixed-final postures must complete before its
wrapper succeeds. Ordinary measured replay still needs nonzero actual completed
replay, independent leaf/physical counters and the corrected stencil/lifecycle
audit; class labels alone are not elimination evidence.

## Non-blocking QA follow-ups

- Retain explicit failure context (occupancy, coordinate, sign and guard) when
  extending the large runtime corpus; a field-only bit assertion can otherwise
  make a rare boundary failure hard to reproduce.
- Document non-nesting or preserve previous state for the node-oracle activation
  wrapper; the forced-complete wrapper already restores its prior flag. The
  present dedicated one-session test path does not nest this API.
- The evaluation file is WARN at 2700 lines; replay 408, graph 384 and separate
  test modules keep new details out of the solver body. Reconcile exact final
  line counts, formatting and warning-denied lint after corrective edits.

## Inspected implementation identities

Source root `/tmp/openwepp-controlled-mechanisms-Hb6uS2/R`; L paths relative to
land-surface-energy `src/`, U relative to runner `src/hillslope/tests03/`.
Hashes capture the capability-corrected cut, with the runtime test hash updated
for the inspected RIB-2 repair and bounded maximum investigation before rustfmt.

| File | SHA-256 |
| --- | --- |
| L `solver.rs` | `feb47dd48b7d9028a16d8cc7f21f8cc76e4c33ff2b6944bfafc59676f69f1402` |
| L `solver_covered_evaluation.rs` | `c23618821c6c2e3a9bc954959585ba42ab93afe452015859d88747cee6e637a2` |
| L `solver_covered_solve.rs` | `9d63b2427445978a65b64cc1c321bc766d8db3db07613e0936d491d33500dcd6` |
| L `solver_mechanism_audit.rs` | `9bbdcc5dd7b301e8949b297ccf1d45a1848bb5a09471c97ee22a5b1be9aadb36` |
| L `solver_tests.rs` | `af1ab088b9e1e7db73fcf5ac8cd06663ae90641add48f155db69c495a017e566` |
| L `solver_component_dependency_graph.rs` | `20212e27ad05685abfb51deeb639a7abfcfcbd164e329bf88508b34b9d61f5bc` |
| L `solver_component_dependency_graph_tests.rs` | `0990374f174d2a20bdcfe9ca3056b6b7786f4eef6dcf408ea4b3fb158de4bd4f` |
| L `solver_component_dependency_replay.rs` | `cfd9fd90e2da6859caa51a0de7511b2730beba88ea1ac94f514113b8115a026d` |
| L `solver_component_dependency_replay_tests.rs` | `6f593bc09ca0158f5af33e13a1d824d026212508087111a1f129fa78fa83ffd8` |
| U `controlled_mechanism_experiments.rs` | `43c3abec0aca0c57097d72bc7610657c87a35f645076b780dba008dddd0ecc7a` |

QA decision: GO toward serial executable verification and in-scope corrections.
HOLD scientific/measurement admission for RIB-3 and unexecuted exact-cut
runtime, authority, full correctness, format/Clippy/deny, closure/output/restart,
compiler-negative custody and terminal dual-verification gates. This review does
not claim a measured failure, speedup, scientific qualification or promotion.

## Prospective proof-closed admission proposal

Static authority judgment before edits: GO to author a separately identified
experimental condition limiting sun/shade replay to the affected class's exact
binary64 beta one, with wet/stem graph replay unchanged. This is not yet approval
of a concrete amended authority/test/behavior cut. INV164 permits conservative
preselection, and the normative text explicitly requires unchanged complete
evaluation for unproved component probes (`SC-LANDSURFACEENERGY-001.md:3027`).
The proposal narrows optimization admission, not the physical domain or solver.

At exact beta one, the successful current call at the actual signed probe is the
maximum helper's exact result; no conditional maximum arithmetic theorem is
needed. Wet/stem leave every leaf input unchanged. Other sun/shade cases must
select complete evaluation before minting the replay capability or starting
replay, never recover after a replay error. Raw base beta bits must independently
drive expected signed-probe classes; producer labels cannot be the oracle.

Required prospective tests include exact one, one ULP below one, zero and other
lawful beta, both classes and applicable occupancies/cap postures, unchanged
stencils/anchors, real replay error cases, no replay for excluded classes, and
complete field/Jacobian/solve parity. Retain historical assertions/results and
independently derive the new counts without manipulating the real workload.
Nonzero actual real replay and every other C020 fallibility/custody/scientific
obligation remain mandatory. This is a bounded correctness admission repair,
not a new physical approximation, proof waiver or performance-driven refinement.

## Corrective PC1 and SG1 exact-cut follow-up

Static: inspected production PC1 predicate and both consumers. Replay file
`0c557403ded163aa67867a28d3eecdfd9444908da3666ab3469670f4befdb7ce`, solve file
`fdb5ed651eabd847e18076cc74004282af29613a4d976dc29e7c28d48e6ec200`.
Graph eligibility is checked first; only component6/7 reads its own beta at
coordinate−2 and requires exact binary64 one. Wet/stem8/9 remains eligible.
The generic consumed signed probe checks this predicate before minting the
component capability; the diagnostic supplied-vector dispatcher applies the
same predicate. Unproved leaf probes select the unchanged complete path before
replay. No h, stencil, physical domain, solver arithmetic or error-retry change.
Corrected tests additionally alternate exact-one/next-down beta by class and
rank in both cap postures, preventing any-class/wrong-rank proof shortcuts.

RIB-3 maximum assurance is resolved prospectively by the separately reviewed
PC1 authority and concrete selector, not by the198-base zero-witness result.
The remaining wet-family assurance is addressed by the exact proposed SG1
text/source/rollback cut reviewed in `R-shared-guard-review-b.md`; canonical
adoption and actual execution remain distinct pending parent-owned gates.
That review binds final formatted tests638de1d0..., actual-capture bit rollback,
and source proof48e5dcfe.... All other C020 graph/custody/invariant obligations
remain unchanged. GO to serial verification; no measurement or terminal PASS.

## N2 optical fixture correction

Static: bounded GO for `solver_tests.rs` SHA-256
`ee516d65a81f0c4a4c0e4184c351d3444141148bbcd2f0261f0e84ce183e1b8f`.
The added N2 PC1 hook had copied the N1 optical receipt although the actual
N2 fixture changes terminal radiation and its bound shortwave column. The
existing absorption guard correctly rejected that inconsistent setup.

Corrected lines1876–1907 use the unchanged canonical
`partition_ground_shortwave` on N2's actual terminal bands and albedos, create
the typed optical receipt with the retained fixture identity/provenance, and
update the lower-boundary absorbed scalar and optical digest to that receipt.
The N2 fixture remains N2; no physical guard, assertion, probe construction,
replay source or baseline fixture is relaxed. Existing lower albedos/lineage
remain consistent with the unchanged fixture values. Required potential/fixed
and mixed-class PC1 assertions still execute through the same hook once
admission succeeds. Parent-owned rerun remains required; this source review
does not claim an executed corrected fixture or supersede the retained FAIL.
