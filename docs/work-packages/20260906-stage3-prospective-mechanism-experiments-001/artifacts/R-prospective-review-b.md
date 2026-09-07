# R prospective runtime/API review B

Static: independent prebehavior review, 2026-09-06. No builds, tests, comparator
or measurements ran in this review. Review A was not consulted before the first
finding submission. Isolated R base is `2b56e6ebc9c8e8073fb68b89626d4d4e2b4602e3`;
the reviewed initial source delta was the new replay-test helper and its two
wiring additions in `solver_tests.rs`. Later inert definitions/graph tests are
not claimed as reviewed behavior. Canonical authority approval remains intact.

## Findings and disposition

| ID / severity | Path | Finding / disposition |
| --- | --- | --- |
| RB-R1 / P2, resolved prospectively | `artifacts/R-implementation-plan.md:28` | The original API summary did not explain authentic scope/generation custody beyond borrowing. The appended intrinsic custody proof now ties validated input/caps/graph to the real solve, capture/trial/frozen values to the iteration's sweep, and each consumed capability to that exact base. An expected-base pointer join rejects a foreign live base with the existing integrity error. Optional observer identities never authorize reuse. This is a viable structural design; executable negative-capability and foreign-base evidence remain admission obligations. |
| RB-R2 / Low, planned correction | isolated `crates/openwepp-land-surface-energy/src/solver_component_dependency_replay_tests.rs:17` | The seed compares `Vec<f64>` using `PartialEq`, which permits `+0.0/-0.0` changes. It cannot establish the contract's bitwise parity. `R-runtime-test-plan.md` explicitly requires float-bit comparisons in the full corpus; retain that requirement and do not relabel this seed as bitwise evidence. |

## Design assessment

One shared canonical evaluator with successful capture publication fits
INV-LANDSURFACEENERGY-164/C-020. The proposed capture keeps route preparation,
liquid ledgers and four source-ordered leaf states; only graph-unreachable
successful values may copy. Reexecuting the remaining canonical tail is lawful
and its cost must remain visible. The plan does not introduce alternate physics,
new tolerances, a derivative rule, altered finite-difference order or fallback.

The canonical pair constructor retains the original minus/plus arithmetic and
stencil choice. A non-Clone capability binds the exact base and its coordinate,
sign, perturbation and stencil and is consumed by evaluation. The expected-base
join and scope containment must work with observation disabled. Invalid
shape/multicoordinate/noncomponent probes select the unchanged complete evaluator
before replay; integrity faults and errors after replay starts return directly.

The initial helper is actually called by the represented-snow fixture at
`solver_tests.rs:1184`, after its lower-boundary and optical receipts are built.
On A, its first lawful component probe increments the complete-dispatch counter,
so its zero-counter assertion is statically expected red. No executed failure
is claimed. Zero complete counters plus residual equality alone do not prove
component replay connectivity, full node parity or any performance result.

The separate graph plan identifies exact node/direct-edge records, independent
all-node-pair predicates and hash serialization, golden N=1/S=1 and N=2/S=6
identities, and per-edge deletion/change checks. Two explicit terminal-custody
edges are included in its versioned descriptor/hash. Unknown reads cannot be
treated as independent; implementation review must reconcile the actual source
read set with this graph. A hash comparison alone cannot discover an omitted
source dependency.

## Required admission matrix

These are existing C-020 obligations and named owner responsibilities, not new
acceptance rules. Their current prospective status is NOT RUN. Completing them
after the reviewed API is implemented is legitimate; claiming admission before
they pass is not.

| Required evidence | Owner / concrete proof |
| --- | --- |
| Graph completeness | r_graph: exact direct records and independently serialized golden hashes for N1/S1 and N2/S6; every required edge mutation fails; unknown coordinate/topology/read cannot authorize copying. |
| Full differential results | r_contract_tests + implement_r: every captured/node/evaluation field and raw/tolerance/normalized float bit, full Jacobian/RHS/linear diagnostics, branches and iteration/backtracking trajectory for both potential and fixed-final caps; complete solve/owner parity. |
| Physical boundary corpus | r_contract_tests + implement_r: authentic two-occupancy/six-soil reciprocal longwave, upper-to-every-lower wet routing and terminal descendants; source-real current-leaf first errors and byte-exact rollback; each noncrossable matrix family's named-guard implication plus its boundary/branch success and exact fields. No synthetic physical-error hooks. |
| Private custody and selection | implement_r + r_contract_tests: successful independent bases, wrong/foreign bindings, single-use and scope containment (runtime rejection or compile-time negative capability as appropriate); exact typed error precedence, zero mutation and no post-start complete fallback. |
| Real consumer | Parent: nonzero completed replay on the primary workload; independent lawful-stencil/classification reconstruction, actual lifecycle/record-loss reconciliation, exact output/control identity except authorized work counts and independent closure reconstruction. |
| Critical runtime cut | Parent/comparator: current full correctness, applicable authority/guards/restart, warnings-denied lint/format, exact source/command identities, line-count disposition, dual immutable implementation reviews and terminal verification. |

The reviewed prospective test/API scope is sufficient to proceed with R
implementation while retaining this matrix as current-scope admission work.
It does not waive missing source-real errors, noncrossability proofs, exact
custody, full output parity or critical-cut validation.

## Reviewed identities

Retained package artifact hashes:

| Artifact | SHA-256 |
| --- | --- |
| `R-implementation-plan.md` including intrinsic custody addendum | `11af0b6af17f4766920e898e67f00ee3d62bc42d46f23786ea58b74ee8314386` |
| `R-implementation-evidence.md` | `6149ed0fd312e802012e81afade3c0a8f623709c6f0ca816f14b678c423c4895` |
| `R-required-reading.md` | `fb983bf5fc6f826c559667c1b9bc98f395bc2a4a3ff82d2a018c61b1049ca38b` |
| `R-runtime-test-plan.md` | `f2b8a8192f4d63b733a152c413b257c189021084519d9d0fa2438435b194e8ac` |
| `R-graph-plan.md` | `06a9ab9a3fad8925207ca8528e7c2d6adbebb83f50b4c7f5523c84a301d5c71c` |
| isolated initial `solver_component_dependency_replay_tests.rs` | `f43f6a9d27f1c04fee1378b32a3d17e0615eebd6b64b1416ae0538ac5825615c` |
| isolated initial `solver_tests.rs` | `f7c31caa00a21f0f80c7de1825ca9b253e8afdddc5cb918491317b33d0a20a83` |

QA decision: GO for the prospective R test/API design. Runtime behavior,
scientific admission, measurements and package closure are not approved by this
static review.
