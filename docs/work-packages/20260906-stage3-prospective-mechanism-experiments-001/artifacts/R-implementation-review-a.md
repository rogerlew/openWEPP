Static: independent implementation review A of isolated R versus A commit
`2b56e6ebc9c8e8073fb68b89626d4d4e2b4602e3`. Source root:
`/tmp/openwepp-controlled-mechanisms-Hb6uS2/R`. No build or Rust/runtime test
execution by this reviewer. Review B findings were not consulted.

## Findings on first implementation cut

MEDIUM RI-A1, `crates/openwepp-land-surface-energy/src/solver_component_dependency_replay.rs:218`:
`CoveredCanonicalProbePair::new` creates the named component-replay capability
for every lawful signed coordinate, including hydraulic, beta, shared, anchor
and non-Stage-3 coordinates. Selection of complete evaluation occurs only in
the subsequent dispatcher. The SC-LANDSURFACEENERGY-001 eligibility table
requires ordinary noncomponent/non-Stage-3/unknown selection before replay
capability creation. A generic canonical signed-probe proof may own unchanged
probe construction for every column; component replay evidence must be minted
only after its represented-snow/graph/component eligibility and oracle posture
are established. Implementer accepted this bounded correction. No derivative,
domain, h, probe order or numerical policy change is needed.

Initial decision: HOLD static implementation admission for RI-A1 only. Other
current-arm runtime/authority gates remain pending, not waived.

## Shared arithmetic, graph and custody assessment

The occupancy/column extraction retains one canonical expression body. Optional
leaf reuse wraps the original calls without changing sun-current, shade-current,
sun-maximum, shade-maximum order. The routing helper contains the original
prepare/first-wet/finalize sequence and operands; the second wet/finalization and
route-match still execute in the original occupancy tail. Longwave, hydraulics,
ground, reductions, tolerances, residual normalization and output assembly are
not copied into a parallel physics implementation. Recomputing other tails even
when graph-unreachable is conservative, with no claim that those calls vanished.

Only successful iteration evaluation publishes capture into its immutable
sweep base. The base owns exact trial/frozen/capture data and borrows one
validated input/caps/graph object. Canonical minus and plus vectors use the
original binary64 arithmetic and trial predicates, with minus executed first.
Consumed proof values compare exact base identity, coordinate, sign, h bits,
stencil and graph/lifecycle binding; component reuse then borrows that same base.
The base is consumed and capture dropped before LU/backtracking; no replay
result persists across iteration, solve, map or retry. Observation identities
describe the actual scopes but do not authorize physics reuse.

The graph is built once per validated represented-snow solve, outside its probe
hot path. Its complete normative direct-record generator, lexical/hash schema
and compact route/current/maximum masks were reviewed against INV-164. For wet
probes, every lower route is reachable; other component probes retain unchanged
route operands. Only the changed sun/shade current and maximum leaves are
reachable; wet-area changes do not alter the leaf-state function's fixed leaf
operands. The separate all-pairs graph test oracle compares all records and
golden N1/S1 and N2/S6 hashes, plus removal/change sensitivity and unknown-edge
conservative selection. No graph test execution is claimed here.

Production replay is recorded before the shared Evaluation wrapper executes.
Both complete and replay probes enter that wrapper exactly once; anchors enter
none. A post-start evaluator error returns through `?` without a complete retry.
Reduction in leaf calls is distinct from shared Evaluation counts. The runtime
node oracle adds diagnostic work transparently in a separate ignored runner
entry; it must never be used as production lifecycle or timing evidence.

## Guard assurance and remaining runtime evidence

For graph-unreachable copied leaf nodes, exact equality of every call operand,
branch and successful captured current/maximum state proves deterministic
guard/output equality; no global physiological bound is needed for that scoped
implication. This is stronger than merely asserting that a node once succeeded.
For affected leaves, the original guard chain runs in one shared function and
source order. That establishes the parity mechanism but does not by itself
prove the separately required noncrossability classification of an affected
maximum call whose temperature changes. Keep the guard-by-guard implication or
authentic counterexample/reclassification and paired source-real vector explicit.
Do not infer all current leaf errors are tested from a selected error sample.

The expanded runtime corpus concretely compares capture/evaluation float bits,
dense finite-difference entries, accepted/rejected result fields, matrix/RHS/
linear diagnostics and successful trial traces. It binds wrong base/coordinate/
sign/h/stencil, malformed vectors, potential/fixed caps, exact-beta/dry/capacity/
zero-PAR boundaries and source-real current-leaf errors. It is still being
authored/formatted; assertions and search-derived boundary vectors are NOT RUN.
The real N2/S6 callback observes actual admitted inputs/caps and requires complete
potential and fixed-final corpus success rather than counting an early error
prefix as full coverage. Compile-negative/single-use/borrow checks, complete
named-guard and byte-rollback evidence, actual R lifecycle/replay counts, forced
complete scientific outputs/closure and all critical correctness/lint gates
remain before measurement. No new scope or blanket physiological proof is added.

## First-cut identities

Paths are relative to `crates/openwepp-land-surface-energy/src/`, except runner.

| Path | SHA-256 |
| --- | --- |
| `solver.rs` | `feb47dd48b7d9028a16d8cc7f21f8cc76e4c33ff2b6944bfafc59676f69f1402` |
| `solver_component_dependency_graph.rs` | `20212e27ad05685abfb51deeb639a7abfcfcbd164e329bf88508b34b9d61f5bc` |
| `solver_component_dependency_graph_tests.rs` | `0990374f174d2a20bdcfe9ca3056b6b7786f4eef6dcf408ea4b3fb158de4bd4f` |
| `solver_component_dependency_replay.rs` | `f0435e488ce708db371da4861f725998c116d8f953851a3a258db3a178e69d74` |
| `solver_component_dependency_replay_tests.rs` | `e1c9f6172d94f93bb4ad8ce6da74ca654201874d1e0f6c95653664e82ea44f7c` |
| `solver_covered_evaluation.rs` | `c23618821c6c2e3a9bc954959585ba42ab93afe452015859d88747cee6e637a2` |
| `solver_covered_solve.rs` | `9d63b2427445978a65b64cc1c321bc766d8db3db07613e0936d491d33500dcd6` |
| `solver_mechanism_audit.rs` | `9bbdcc5dd7b301e8949b297ccf1d45a1848bb5a09471c97ee22a5b1be9aadb36` |
| `solver_tests.rs` | `af1ab088b9e1e7db73fcf5ac8cd06663ae90641add48f155db69c495a017e566` |
| `crates/openwepp-runner/src/hillslope/tests03/controlled_mechanism_experiments.rs` | `43c3abec0aca0c57097d72bc7610657c87a35f645076b780dba008dddd0ecc7a` |

## Bounded correction: GO static implementation

Static: reread the corrected constructor, generic signed-proof dispatcher and
component capability consumer. `ValidatedCoveredSignedProbe` now owns canonical
construction for arbitrary lawful columns but authorizes no replay. Only the
guarded non-forced, represented-snow eligible-component branch creates
`ValidatedCoveredComponentProbeReplay`. Ordinary complete/anchor selections
never mint that capability. Exact header/base checks, perturbation arithmetic,
bound predicates, ordering and shared evaluation remain unchanged. RI-A1 is
RESOLVED.

Corrected replay module SHA-256:
`cfd9fd90e2da6859caa51a0de7511b2730beba88ea1ac94f514113b8115a026d`.
Solver remains `9d63b2427445978a65b64cc1c321bc766d8db3db07613e0936d491d33500dcd6`;
evaluator remains `c23618821c6c2e3a9bc954959585ba42ab93afe452015859d88747cee6e637a2`.
The foreign-base runtime test now consumes the generic proof through its
checked `evaluate_for` API. Runtime test file was
`138396b9292277cf92e48fa091fdee2a1c0df9d9ef38ecfe662593b1a9c6c40f`
at this check and remains subject to ongoing test authoring/execution, not a
measurement-frozen test cut. Ran: `git diff --check` PASS. No build/test run.

Decision: GO / no remaining static implementation blocker on the corrected
production source. This does not close the named-guard implication evidence,
authentic paired error/rollback vectors, exact corpus/compiler gates, actual
N2/S6 and forced-complete workload results, or critical-cut validation. Those
are current-arm premeasurement requirements. In particular, the affected
maximum proof compares against the successful current call at the same signed
probe temperature; generic global finite/Brent claims are not established by
this source approval or by a finite sample of passing runtime vectors.

## Activated R-PC1 bounded source review

Static: no production numerical, source-order, capability or duplication defect
found in the activated exact-beta selector. No Rust build/test run by reviewer A.

Medium, assurance classification only: the retained C-020 wet-family implication
is not established by successful base plus bounded temperature alone.
`solver_covered_evaluation.rs:651` computes saturation humidity with denominator
`pressure - 0.378 * es(T)`; `covered_wet_flux` at line 1021 uses that changed
quantity, and `covered_liquid.rs:177` rejects nonfinite signed vapor. The current
source proof states no admitted pressure margin against that denominator and
therefore does not establish the blanket finite/noncrossable claim. Canonical
C-020 lines 3016–3020 requires the implication for each named existing guard,
including replay-executed nodes, not merely for bypassed nodes. This is not an
executed counterexample or a demonstrated replay-versus-complete mismatch.

Every affected wet-routing call and lower descendant actually executes the same
canonical code at the same source position in both modes. Thus the implementation
already preserves such an error if it occurs; no runtime restriction, changed
domain, error retry or speculative parameter hunt is prescribed by this finding.
A separately reviewed canonical experimental assurance supplement can resolve
the classification as described below; a package-only label cannot override it.

### Exact reviewed cut

Paths are relative to `crates/openwepp-land-surface-energy/src/` in isolated R.

| Path | SHA-256 |
| --- | --- |
| `solver_component_dependency_replay.rs` | `0c557403ded163aa67867a28d3eecdfd9444908da3666ab3469670f4befdb7ce` |
| `solver_covered_solve.rs` | `fdb5ed651eabd847e18076cc74004282af29613a4d976dc29e7c28d48e6ec200` |
| `solver_covered_evaluation.rs` | `c23618821c6c2e3a9bc954959585ba42ab93afe452015859d88747cee6e637a2` |
| `solver_component_dependency_replay_tests.rs` | `36bf6a8cbe4e8faaa9e093097a91d458d970822337ebaf8d2ee5312cbc445036` |
| `solver_tests.rs` | `1e9dfdddc36b37fa2087c06b24ceadd0ac872b78ae3d7320ac1be7b0662bd668` |

Runtime plan hash at inspection:
`14622f98720a4c4cc26f584a6a8cea1cf346d147f5d12e47195ce9f86a747ccb`.
Canonical R-PC1 authority remains
`c993020e96aec0dac934acd5ccfa3050f8173a90bcd7f860b88b8da7405418cd`.

### Source and test disposition

`covered_component_replay_is_proved` at replay-module line 277 first requires
the authenticated represented-snow component graph, then checks the affected
sun/shade beta at `coordinate - 2` by exact one bits; wet/stem remain eligible.
The generic signed dispatcher at line 186 calls this predicate before creating
component replay authority. The ordinary branch reaches complete evaluation;
the shared observed dispatcher uses the same predicate. There is no duplicate
selection formula, tolerance, normalization or post-error fallback.

At admitted beta one, the changed current leaf executes before its canonical
maximum helper, which returns that same successful current state. The other
current/maximum states retain identical operands. Wet/stem probes change no
leaf-call input; routed wet area is applied only in the subsequently recomputed
vapor, hydraulic and energy expressions, not passed to the leaf-state function.
Capture is published only by a successful production iteration evaluation and
borrowed from that exact immutable base. Failed capture prefixes are not
published. Base lifetime, canonical h/sign/bounds, frozen branches, identity
anchors, shared Evaluation count and complete error returns are unchanged.

Unchanged hydraulic potentials, roots, caps and frozen branches retain all
root-law/cap-presence guard operands. The lower-boundary resistance inputs are
unchanged. Both liquid finalizations use one shared canonical function with the
same preparation, wet-flux inputs and pass, preserving route-match by
construction. Reciprocal longwave and all remaining tails still execute the
canonical expressions; their existing proof/boundary requirements are not
relaxed by PC1 or the proposed wet-family correction.

The final test cut adds alternating exact-one/next-lower beta patterns across
sun/shade and occupancy ranks, reversing the pattern, under potential and fixed
caps. It asserts independent raw-beta dispatch counts and compares actual base
capture fields, every full evaluation field/residual, and canonical Jacobian
entries. Preparations, both liquid-ledger observations and all four leaf states
have explicit bit assertions; the real N2/S6 callback still requires successful
full potential and fixed-final corpora before recording coverage. This source
review is not an execution claim for those assertions.

### Bounded authority resolution and remaining gate

Prospective GO in principle for a canonical, separately identified EXP-only
supplement limited to `route.prepare`, `route.wet`, `route.finalize`, `occ.wet`
and `occ.liquid` when actually recomputed, including affected lower descendants.
Its assurance is exact shared-function/operand/source-order result-or-first-error
equivalence, not unproved noncrossability or a claim that an authentic error
witness exists. Every copied route/leaf node must retain exact-operand
successful-base proof. Keep the hydraulic, longwave, route-match, lower-boundary
and PC1 maximum obligations, all natural-error paired first-error/no-later-work/
no-fallback/rollback tests, and historical text unchanged. A concrete canonical
supplement and derived tests require dual prospective review. This corrects the
same mechanism's scientific assurance without changing physics or forgiving an
error; it is not authorization for a blanket gate waiver.

Decision: activated PC1 source GO / no implementation blocker. Scientific
admission remains pending the precise wet-family authority/proof resolution,
runtime and authentic output/trace checks, and the existing full validation
gates. F's separately reported frozen identity was noted only; no F execution
or new F-source approval is claimed by this R review.

## Concrete SG1 proposal/source proof and compiler-negative harness review

Static: GO for the exact proposed canonical text in
`R-shared-guard-authority.md` hash
`856ff186550815276e2baf205f7d1a4f127bfa07e63ed6d43a4155e53ff7131d`
and the five-family source proof in `R-shared-guard-source-proof.md` hash
`48e5dcfe7ea904e19bf434023a87e87da0efde87900879cb4718a719551c72de`.
The proposal does not assert an authentic crossing witness or historical
noncrossability PASS. It explicitly replaces only the experimental implication
for actually recomputed wet-routing instances, retaining successful-base exact
operand proof for skipped predecessors and every other named obligation.
The source proof correctly binds each shared call, its operands and order,
including every lower routing descendant and the distinct second wet/liquid
calls. No runtime or physical modification is requested by this approval.

Adoption of the complete SG1 cut is still pending corrected derived-test review:
the source proof truthfully identifies that the natural Err/Err catch-all
currently asserts beginning state by PartialEq, not an exhaustive bit snapshot.
The test owner's bounded explicit recursive float-bit plus typed-structure,
base/capture/caps/frozen/graph identity snapshot design received prospective A
GO; the concrete implementation and execution have not yet been reviewed here.

Compiler-negative design: the actual private-type snippet
`reproduction/replay_borrow_negative.rs` hash
`86ec7d5d90a19bbcc33b3693530e31a2c551c5ae91447bee3b59207297993941`
tests a second consumed signed capability, mutation while the base is borrowed,
and dropping the borrowed base. Required Rust codes are E0382/E0502/E0505 with
primary spans inside the corresponding actual functions, not generic compiler
failure. The harness reconstructs fresh verified source and requires the same
explicit positive test compile before injection. It retains commands, statuses,
source bytes, structured compiler messages and unrelated errors.

One bounded evidence correction was requested on harness hash
`5d384a6f582de9318c6dcb2386c6097944858ea5fe6fb269c1f59b82e1841111`:
after negative compilation, seal the injected target bytes and all other
manifest rows again, not only after the positive compile. Pending that check,
the harness is not execution-admitted by this reviewer. Ran: six read-only
diagnostic-classifier controls passed (expected errors, wrong code, wrong
function span, wrong file, missing required error, positive-cut unexpected
errors). No compiler, source reconstruction or scratch injection was invoked.

Compiler-negative correction: GO for harness source
`8b3f9e03ae446f7ee1d82c8f49b376a621c28e6ccb1d179099c734b88ca8660c`.
After negative compilation it now verifies all manifest rows again, overriding
only the named injected target's expected byte count/hash while retaining its
mode/symlink checks and every other row/deletion check. This resolves the
post-compile source-seal finding. Actual positive and three targeted negative
compiler outcomes remain NOT RUN by this reviewer and must be retained by the
parent’s serial execution before that gate can be called PASS.

## Final SG1 derived rollback cut: prospective adoption GO

Static: the corrected formatted runtime test cut is
`638de1d0e5d04c32f4e8a5e4f10a9fe2fca2f2f964ed2515499b061ed162e2e2`;
its source-field coverage plan is
`70d64ad77e98323ea16d48cd7fb3f46711f98b107f10d460765c705f1c3b0aa3`.
Reviewed the explicit traversal against declared column, ground, geometry,
bare-soil, soil-node, occupancy, leaf, biochemical, root, shortwave, lower-
boundary, optical and cap float fields. All such input floats have bit checks;
typed clones separately preserve IDs, enum/Option tags, cardinality and order.
The snapshot owns the actual base evaluation/capture/trial and compares their
existing exhaustive bit encodings, frozen maps, lifecycle and base/validated/
column/caps/boundary pointer joins. The graph digest is not misrepresented as
an exhaustive private-mask snapshot: immutable-borrow source proof and the
independent graph tests remain separate obligations.

The natural Err/Err branch now asserts this snapshot before returning, as do
the existing named paired first-error comparisons and supporting probes. This
resolves the prospective assertion limitation identified in the SG1 source
proof without new physical vectors, a fault seam, Debug proxies or production
changes. No claim is made that these assertions have already executed.

Decision: GO for concrete SG1 adoption when combined with proposed canonical
text `856ff186550815276e2baf205f7d1a4f127bfa07e63ed6d43a4155e53ff7131d`
and source proof `48e5dcfe7ea904e19bf434023a87e87da0efde87900879cb4718a719551c72de`,
subject to independent B approval and parent-controlled canonical application.
Actual compiler, runtime, real-consumer, output/trace and full validation gates
remain required before scientific/measurement admission. No historical wet
noncrossability PASS is inferred.

## Bounded N2 optical-fixture correction: source GO

Static: reviewed `solver_tests.rs` hash
`ee516d65a81f0c4a4c0e4184c351d3444141148bbcd2f0261f0e84ce183e1b8f`.
The existing N2 fixture already changes terminal radiation and rebinds column
shortwave, so copying the N1 optical receipt into its new replay test wrapper
was inconsistent. The correction uses the canonical ground partition and typed
receipt constructor with the actual N2 terminal bands and unchanged albedos,
then binds the corresponding lower absorbed total and optical receipt digest.
It preserves the existing OFE/tile/provenance operands and all validators.
No production physics, canonical perturbation or baseline fixture forcing is
adjusted to manufacture replay coverage. The replay assertion source remains
`638de1d0e5d04c32f4e8a5e4f10a9fe2fca2f2f964ed2515499b061ed162e2e2`.
GO for this test-only correction. Parent-reported prior 149/150 outcome remains
a failed cut; corrected runtime PASS has not been verified by reviewer A.

## Five-path Clippy correction: concrete source GO

Static: independently compared each current file against the retained R-source-04 changed-source snapshot. No builds or Rust tests run. All changes are the prospectively reviewed mechanical scope; no new correctness finding.

- `solver.rs` — `3337c2a392e851264b1ec7f4e0592492774abf7ecd1a206bc4f534d65b4b756e`: explicit same-parent imports for the non-test test-support oracle; test glob remains cfg(test). No visibility or runtime call change.
- `solver_component_dependency_graph.rs` — `4fca5d95b4fd764aa41ef1accbf1ea3a2723443fa68cea0c2d115c1d645cb135`: if-let preserves the missing-node `complete=false`/reachable=true branch; `is_none_or` preserves the previous unknown-mask semantics. No graph edges, traversal order, masks or eligibility changes.
- `solver_component_dependency_graph_tests.rs` — `36af591efe4f77618e0d30598c11f392982b880b024b22b104367751d0ba255c`: identical direct-edge predicates merged, filter/map retains pair order, and String `write!` retains lowercase two-digit hash bytes.
- `solver_component_dependency_replay.rs` — `ba07446fe108bedd96a5ded2fcac6fb493c6fe739f87b6827e699afd3ff45941`: panic documentation and lexical local Restore type placement only. Begin, guard construction, closure execution and cleanup retain their original order.
- `solver_component_dependency_replay_tests.rs` — `d606fb02f874fb53538c295282ba1c3ca45af129bd42a72d3ab70c47ea8393a2`: checked conversions of counts bounded by two and class offsets from the existing `[6,7]` loop; statement semicolon; Restore declarations moved before guards without moving TLS snapshot/guard construction. No first-error, rollback or comparison assertion was removed.

GO for this concrete five-path correction. The separately owned common natural-limit two-path observer is reviewed in diagnostic-parity-review-a.md. Actual current-cut compiler/Clippy/regression outcomes and exact source freeze remain parent-owned gates; source review is not execution PASS.

Compiler-driven import correction: parent R Clippy found `Digest` unused in the explicit non-test oracle import. Final `solver.rs` hash `7d3837a5dfdea1f35b37384637561c59947ca9f320865448c6463f11e264b460` removes only that import; test glob and top-level sha2 trait remain. A read-only inverse insertion reconstructs exact previously reviewed `3337c2a...` bytes. No runtime or API change. Concrete source GO; no reviewer build and no current-cut lint PASS inferred.
