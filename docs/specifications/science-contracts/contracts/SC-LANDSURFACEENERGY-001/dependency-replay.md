[Parent contract](../SC-LANDSURFACEENERGY-001.md)

## Dependencies
| Target | Required when | Boundary/obligation | Reading extent |
|---|---|---|---|
| interface.md#interface | every task | universal scope, owners, failure and qualification | whole mechanism chapter |
| nonlinear-solve.md#nonlinear-solve | component-temperature replay correctness | canonical stencils, leaf reuse and errors | whole mechanism chapter |
| surface-energy.md#surface-energy | component-temperature replay correctness | radiation/turbulent dependencies | whole mechanism chapter |
| qualification.md#qualification | component-temperature replay correctness | historical versus experimental limits | whole mechanism chapter |
| map-custody.md#handoff | replay custody/error-order review | original validation positions and pending-map identity | section (entry extent) |
| map-custody.md#pending | replay custody/error-order review | original validation positions and pending-map identity | section (entry extent) |
| map-custody.md#validation | replay custody/error-order review | original validation positions and pending-map identity | section (entry extent) |
| water-vapor.md#vapor | represented-snow evaluator/error-order requirements review | signed enthalpy, immutable water/ingress and canonical errors | section (entry extent) |
| water-vapor.md#water | represented-snow evaluator/error-order requirements review | signed enthalpy, immutable water/ingress and canonical errors | section (entry extent) |
| water-vapor.md#errors | represented-snow evaluator/error-order requirements review | signed enthalpy, immutable water/ingress and canonical errors | section (entry extent) |
| water-vapor.md#water-vapor | active water/ingress implementation or full water-owner audit | complete owner duties | whole mechanism chapter |

<a id="dependency-replay"></a>
# Dependency Replay
Current INV-164/C-020 scientific implementation obligations. Historical revision-31 retention and prospective EXP-R/PC1/SG1 qualifications remain separate in qualification. No implementation, experiment, production retention or selector is activated by this migration.

<a id="replay"></a>
<a id="component-temperature-jacobian-dependency-replay-amendment"></a>
## Component-Temperature Jacobian Dependency-Replay Amendment

`INV-LANDSURFACEENERGY-164` applies only inside one already validated
represented-snow covered solve and one current Jacobian sweep. It changes no
coordinate, residual, constitutive equation, physical branch, or derivative.
The canonical coordinate order remains ten coordinates per occupancy (four
hydraulic potentials, two beta values, then sun-leaf, shade-leaf, wet-surface,
and dry-stem temperatures), followed by shared canopy-air temperature and
humidity, ground temperature, and the configured soil temperatures. Canonical
probe perturbations, minus-before-plus construction, bounds admission, and
centered or unique inward finite differences remain unchanged.

One immutable `ValidatedCoveredComponentReplaySweepBase` owns the successful
current evaluation for one Jacobian sweep. It binds the exact validated-input
generation and every referenced input, the exact potential or fixed-final caps
values and posture, frozen-branch values, current-trial bits, authentic map,
solve, Newton-iteration, and sweep identities, and the graph version/hash. It
may be borrowed by all signed probes in that sweep but is dropped before the
next iteration, solve, map, or retry. For each admitted minus or plus probe the
canonical probe constructor mints a fresh non-Clone
`ValidatedCoveredComponentProbeReplay` bound to that base, coordinate index,
sign, perturbation bits, exact probe bits, and the actually selected stencil.
The constructor, not a later whole-vector scan, proves the sole-coordinate
difference and sign/stencil/bound relation. That capability is consumed exactly
once by success or error and cannot be transferred, restored, serialized, or
reused. Dropping either object publishes and mutates nothing.

Custody is exact, not a costly proxy check. Production may use immutable borrows,
typed generations and compact canonical seals minted where the corresponding
object is already fully validated, but it must compare every bound identity and
value bit exactly. A `Debug` string, digest length, allocation-independent hash,
reconstructed approximation, whole-probe clone, or repeated full-vector scan is
not custody. A seal must include the caps, frozen branches, graph, trial,
coordinate, sign, perturbation, probe and stencil facts above and must be joined
to the same successful base evaluation. The static graph descriptor is built or
validated once for its authenticated topology/configuration generation and then
borrowed; production must not rebuild strings, ordered maps, transitive closure,
or a whole-input hash in every sweep or signed probe. This bounded descriptor is
not a cross-sweep result cache: no base evaluation, probe, branch result,
residual, solver result, or mutable map state survives its named boundary.

`CoveredComponentTemperatureDependencyGraph` uses schema version
`covered-component-temperature-dependency-v1`. Stable node IDs and direct-edge
families are normative below; `[o]` is authenticated occupancy topology rank,
`[k]` is a component in canonical `sun,shade,wet,stem` order, `[s]` is soil
rank, and `[j>o]` means every lower occupancy. Expanded node and edge records are
sorted lexically, duplicate-free, length-prefixed, and SHA-256 hashed with the
schema version, `N`, and `S` by the contract oracle. A production
representation may be a compact equivalent whose exact edge-set/version/
topology join has been established outside the probe hot path. Inclusive
transitive closure is computed from those exact directed edges in stable node
order. The sweep base and probe capability carry and compare the same
version/hash or an equally exact typed graph identity. An unrecognized
evaluator node/read or missing edge marks the affected coordinate conservatively
ineligible; unknown never proves independence.

| Stable node family | Source-order computation and owned values |
|---|---|
| `probe[o,k]` | sole signed component-temperature coordinate change |
| `route.prepare[o]` | top-to-bottom preliminary liquid preparation from exact incident rain |
| `route.wet[o]` | first/source-distinct wet-flux evaluation from `trial[o,wet]` |
| `route.finalize[o]` | first liquid finalization; throughfall, both drainages, stemflow |
| `route.incident[o+1]`, `route.stemflow[o+1]` | ordered lower-occupancy rain and stemflow-prefix custody |
| `longwave.layer[o]`, `longwave.column` | all areas/temperatures, reciprocal component nets and ground-facing net |
| `occ.leaf.current[o,sun&#124;shade]` | current surface humidity, resistance, CI/carbon and gas branch |
| `occ.leaf.maximum[o,sun&#124;shade]` | beta-one maximum state in exact current-before-maximum order |
| `occ.vapor[o,sun&#124;shade]` | current/maximum vapor rates and conductances |
| `occ.wet[o]` | second/source-distinct wet-flux evaluation during occupancy evaluation |
| `occ.hydraulic[o]` | source/root loop, six hydraulic residuals and tolerances |
| `occ.sensible[o,k]`, `occ.energy[o,k]`, `occ.tolerance[o,k]` | component sensible, energy/anchor residual and exact tolerance arithmetic |
| `occ.liquid[o]`, `occ.route_match[o]` | second liquid finalization and equality with `route.finalize[o]` |
| `occ.output[o]` | all occupancy fields including CI/carbon, branches, water and component arrays |
| `shared.heat`, `shared.vapor`, `shared.tolerance` | occupancy-order reductions, lower boundary and reference air |
| `lower.ground_output` | ground-facing longwave plus represented-snow ground/soil outputs |
| `residual.raw`, `residual.tolerance`, `residual.normalized` | canonical occupancy/shared/ground/soil rows and normalization division |
| `result.ground_release`, `result.ground_stemflow`, `result.output` | terminal incident rain, stemflow sum and complete evaluation fields |

The complete direct-edge generator is normative:

| From | Required direct targets |
|---|---|
| `probe[o,k]` | `longwave.layer[o]` and the matching component families below |
| `probe[o,sun&#124;shade]` | matching `occ.leaf.current`, `occ.leaf.maximum`, `occ.vapor`, `occ.hydraulic`, `occ.sensible`, `occ.energy`, `occ.tolerance`, `occ.output` |
| `probe[o,wet]` | `route.wet[o]`, `route.finalize[o]`, `occ.wet[o]`, matching sensible/energy/tolerance, `occ.liquid[o]`, `occ.output[o]` |
| `probe[o,stem]` | matching sensible/energy/tolerance and `occ.output[o]` |
| `route.incident[o]` | `route.prepare[o]`; `route.stemflow[o]` and `route.finalize[o]` feed `route.stemflow[o+1]` |
| `route.prepare[o]` | `route.wet/finalize[o]`, `longwave.layer[o]`, `occ.wet[o]`, and every occupancy vapor/hydraulic/sensible/energy/tolerance/liquid/output node that reads area, wet fraction, store or branch |
| `route.wet[o]` | `route.finalize[o]`; finalization feeds `route.incident/stemflow[o+1]`, `occ.route_match[o]`, and at terminal `o`, both ground-release/stemflow results |
| `route.incident[o+1]` | the same routing chain for every `j>o` by closure; no adjacent-only truncation |
| `longwave.layer[o]` | `longwave.column`; the column feeds all component energy/tolerance nodes, every `occ.output[o]`, `lower.ground_output`, and `result.output` |
| each `occ.leaf.current` | matching maximum, vapor, hydraulic, energy/tolerance and output nodes |
| each `occ.leaf.maximum` | matching vapor, hydraulic and output nodes |
| each `occ.vapor` | matching hydraulic, energy/tolerance and output plus `shared.vapor` |
| `occ.wet[o]` | hydraulic, wet energy/tolerance, liquid and output plus `shared.vapor` |
| each `occ.sensible[o,k]` | matching energy/tolerance and output plus `shared.heat` |
| `occ.hydraulic/energy/tolerance/liquid[o]` | `occ.output[o]`; both liquid finalizations feed `occ.route_match[o]` |
| `occ.route_match[o]` | `occ.output[o]`; mismatch returns here before later occupancies/shared work |
| each `occ.output[o]` | ordered raw/tolerance rows, shared reductions and final output |
| `lower.ground_output` | `shared.heat`, `shared.vapor`, `shared.tolerance`, matching raw/tolerance rows and `result.output` |
| `shared.heat` | `shared.tolerance`, matching raw/tolerance rows and `result.output` |
| `shared.vapor` | `shared.tolerance`, matching raw/tolerance rows and `result.output` |
| `shared.tolerance` | matching tolerance/normalized residual rows and `result.output` |
| `result.ground_release`, `result.ground_stemflow` | `result.output` |
| raw/tolerance rows | matching normalized row; every residual node feeds final output |

Additional conservative edges may cause more replay; omitting any edge above is
forbidden. An independent graph oracle expands the exact normative node and
direct-edge records for at least `N=1,S=1` and the real `N=2,S=6` topology,
compares every record and the golden schema hash, and proves that removing or
changing any required edge fails. Any additional conservative edge is explicit,
versioned, and present in that comparison; reachability-only tests do not close
direct-edge completeness.

The complete evaluator and replay walker call one shared canonical
implementation for every common node and evaluator tail. Factoring may expose
typed intermediate values but must not duplicate, mirror, translate, or reorder
any physical, tolerance, residual, branch, or output arithmetic. The replay
walker preserves exact source order: trial admission;
top-rain guard; top-to-bottom `route.prepare -> route.wet -> route.finalize ->
incident/stemflow`; reciprocal longwave; then per occupancy sun current, shade
current, sun maximum, shade maximum, vapor, second wet, hydraulic/root,
sensible, energy/tolerance, second liquid finalization, route-match, output;
lower/ground work; shared heat then vapor and tolerances; ground/soil rows;
normalization; result assembly. Every reachable node executes its existing
expression and operation order; only unreachable successful base nodes copy.

<a id="eligibility-integrity-mismatch-and-error-outcomes"></a>
### Eligibility, integrity, mismatch, and error outcomes

| Trigger at its source position | Outcome |
|---|---|
| non-Stage-3, non-component coordinate, inadmissible/multi-coordinate probe, or coordinate disabled for an unknown edge | Select the canonical complete evaluator before capability creation; this is ordinary selection, not error recovery. |
| recognized component probe with every base/graph/probe join exact | Mint one per-probe capability and begin replay. |
| graph version/hash or topology mismatch found before capability creation, with identical complete-evaluator operands | Select the complete evaluator before replay and preserve its first error. |
| stale/foreign base, transfer, wrong coordinate/sign/probe binding, or second consumption | Reject directly with `LandSurfaceEnergyError::ConstitutiveDomain("covered_component_dependency_replay_integrity")`; no complete evaluation and no mutation. |
| fallible reachable node or route-match fails after replay begins | Return that existing error at its source-real position; never run the complete evaluator or another solver afterward. |
| replay succeeds | Assemble the existing result once; consume/drop the probe capability with zero publication. |

<a id="normative-fallibility-and-canonical-crossability-matrix"></a>
### Normative fallibility and canonical-crossability matrix

Here, *canonically crossable* means that a source-real input can produce a
successful immutable base evaluation and the unchanged canonical infinitesimal
one-coordinate component-temperature probe can then reach that existing typed
error surface. A test-only mutation, fault injector, alternate tolerance,
noncanonical perturbation, impossible branch, forged intermediate, or direct
private-node call does not establish crossability.

| Node/error family | Classification for a successful represented-snow base plus an admitted canonical component probe | Required assurance |
|---|---|---|
| `occ.leaf.current[o,sun&#124;shade]` existing leaf-domain errors | fallible and canonically crossable | For every authentic crossable leaf error and both applicable component/occupancy positions, run the same source-real base and signed probe through replay and forced-complete modes; require the same first typed error at the same source-order position, no later node/complete fallback, and byte-exact beginning/custody rollback. |
| `occ.leaf.maximum[o,sun&#124;shade]` existing leaf-domain errors | fallible but not currently established crossable | A successful current-leaf base, unchanged beta-one maximum operands/branch, and admitted component probe must imply maximum-call validity. Require authentic exact-beta/branch boundary successes and exact leaf/evaluation fields. Reclassify as crossable and add the paired source-real error vector only if an authentic successful-base plus admitted-probe counterexample is first established. |
| `route.prepare[o]`, `route.wet[o]`, `route.finalize[o]`, `occ.wet[o]`, `occ.liquid[o]` | fallible but noncrossable from an admitted replay | Prove from the successful base, immutable rain/store/area/caps/frozen inputs, admitted temperature bounds and unchanged branch that every existing domain precondition remains true. Exercise authentic zero/wet/dry, exact-capacity and routing boundary successes and compare all liquid, wet-flux, drainage, throughfall and stemflow fields exactly. |
| `longwave.column` | fallible but noncrossable from an admitted replay | Prove successful-base finite areas/temperatures plus admitted finite bounded component temperature imply every longwave operation remains valid. Exercise authentic reciprocal multi-occupancy/zero-area/boundary successes and compare every component and ground-facing net bit exactly. |
| `occ.hydraulic[o]` | fallible but noncrossable from an admitted replay | Prove the successful leaf/wet predecessors, immutable root/soil/caps/frozen inputs and admitted temperature bounds preserve the hydraulic/root-loop domain. Exercise authentic active/inactive, limiting-root and tolerance-boundary successes and compare all six residual/tolerance fields exactly. |
| `occ.route_match[o]` | fallible consistency guard but noncrossable from correct replay | Prove both calls consume the same immutable routing inputs and one shared canonical liquid-finalization implementation, so equality follows by construction. Exercise authentic upper-to-every-lower routing, zero/nonzero wet drainage, and terminal routes; compare both finalizations and the match fields exactly. A forged mismatch is an integrity test, not a physical poison. |
| `lower.ground_output` existing under-canopy resistance/domain errors | fallible but noncrossable from an admitted replay | Prove the successful base and unchanged lower-boundary/caps/frozen operands retain domain validity; exercise authentic represented-snow resistance and ground/soil boundary successes and compare every lower/ground/soil evaluation field exactly. |
| replay trial-shape, top-rain, coordinate/bound admission, graph/topology and capability-integrity guards | pre-admission or private integrity, not a reachable physical node error | Use separate source-real malformed/boundary selection vectors for the unchanged complete-evaluator guards and real constructor/lifetime operations across authentic generations, bases and probes for private stale/foreign/wrong-coordinate/wrong-sign/wrong-perturbation/wrong-probe/wrong-stencil/second-use vectors. Never mutate private fields to create them. Require exact first error, no replay/fallback, and byte-exact rollback. |
| `probe[o,k]`; `route.incident[o+1]`; `route.stemflow[o+1]`; `longwave.layer[o]`; `occ.vapor[o,sun&#124;shade]`; `occ.sensible[o,k]`; `occ.energy[o,k]`; `occ.tolerance[o,k]`; `occ.output[o]`; `shared.heat`; `shared.vapor`; `shared.tolerance`; `residual.raw`; `residual.tolerance`; `residual.normalized`; `result.ground_release`; `result.ground_stemflow`; `result.output` | infallible computations or assembly under their already validated predecessors | Never invent an error. Execute or copy only as the graph authorizes, in exact source order, and compare every node value, branch, residual/tolerance, terminal route and complete evaluation/output field bit-for-bit against forced complete evaluation. |

For every fallible-but-noncrossable row, the obligation is both a reviewable
successful-base-plus-admitted-probe implication proof over each named existing
guard and executable authentic boundary/branch success vectors with exact field
parity. A generic assertion that the node was successful once is insufficient.
The differential corpus is also a catch-all: whenever any unmodified canonical
input in the branch/bound corpus naturally makes replay or forced-complete
evaluation return an error, both modes must return the identical first existing
typed error and leave beginning/custody bytes identical. Production and tests
must not add mutation seams, fault-injection hooks, synthetic error branches, or
test-only physics entry points to manufacture an otherwise unreachable error.

Every hydraulic-potential, beta, shared-canopy-air, non-Stage-3, malformed,
multi-coordinate, and unproved component probe executes the unchanged complete
evaluator. Existing `INV-LANDSURFACEENERGY-162` synthesis remains exclusive to
represented-snow ground and soil identity anchors. No analytic or automatic
derivative, graph coloring, simultaneous perturbation, sparse Jacobian or LU,
changed pivoting, approximate reuse, cross-sweep/iteration/map/retry cache,
memoization, fallback, hardcoded two-occupancy/six-soil logic, or alternative
solver path is authorized.

The contract-first structural expected-red has one deliberately narrow claim:
it classifies whether the seven named replay graph/evidence/audit/function
declarations exist as unconditional top-level Rust items. It cannot establish
dispatcher invocation, control-flow reachability, graph/evidence consumption,
counter provenance, or any numerical behavior. Empty, skeleton, token-only,
dead-code, or disconnected declarations may turn that source classifier green
but are insufficient for implementation readiness and cannot satisfy
`OBL-LANDSURFACEENERGY-C-020`. Only post-implementation executable tests that
exercise the real dispatcher, observe authentic sealed sweep/run counters, and
pass the forced-complete node/residual/Jacobian/full-solve oracle establish
connectivity, consumption, and behavior.

For `N` occupancies and `S` soil nodes, one full interior centered sweep retains
`2*(10*N+3+S)` ordered logical probes. Exactly `2*(1+S)` represented-snow
ground/soil probes use existing identity-anchor synthesis, `8*N` component-
temperature probes use dependency replay, and `12*N+4` probes use the complete
evaluator. Thus the real `N=2`, `S=6` fixture must report `58 = 14 + 16 + 28`.
Its eight hydraulic, four beta, and two shared-canopy-air columns are the 14
columns whose 28 probes remain complete. This is one named
`N=2,S=6,fully-centered-interior` sweep observation, never a release-run total.

Each sweep resets a local audit before its first column and seals it as
`Completed` only after every required column/probe finishes, or `Failed` at the
source-real first typed error with counts limited to work actually attempted.
`ShortCircuited` exists and is reported only if the unchanged canonical solver
has a real non-error path that ends an already-started sweep before all probes;
if no such path exists, the variant, counter and claimed population are absent,
not fabricated as an always-zero state. `RejectedBeforeProbe` is a per-column
stencil/admission outcome and is not a sweep short-circuit. The record binds
`Potential|FixedFinal`, `N`, `S`, every column's
`Centered|InwardLower|InwardUpper|RejectedBeforeProbe` stencil, admitted signed
logical-probe count, and disjoint anchor/replay/complete buckets. Centered
columns count two signs, inward columns only their admitted sign, and rejection
counts only probes actually attempted. Every record must satisfy
`logical=anchor+replay+complete`.

Map, solve, Newton-iteration, and sweep identities are independent authentic
identities from the real caller/lifecycle: a map ID joins all of that map's
potential/fixed-final solves; a solve ID identifies one actual solve within the
map; iteration and sweep IDs identify their actual nested events. One ordinal
copied into another field, a locally invented release label, or an address/hash
proxy is forbidden. If the library seam lacks map context, the audited path
receives a private typed map identity from the authenticated caller rather than
guessing it. Ordinary production with audit collection disabled need not
allocate records or construct histograms.

A separately reset release aggregator retains every sealed source-real sweep
record without coalescing potential/final, centered/inward, completed/failed, or
any genuinely reachable short-circuit identity. It reports the actually
supported disjoint lifecycle class counts, a per-sweep histogram, and aggregate
bucket sums; it must not require or emit a lifecycle state the canonical solver
cannot enter. Each solve/map aggregate reconciles exactly to its retained sweep
records, including failed and inward-bound sweeps, and reset boundaries forbid
cross-run accumulation. Release acceptance requires complete aggregation and at
least one authentic named fully centered `N=2,S=6` completed record with
`58/14/16/28`; the whole release aggregate does not equal those fixture values.

`OBL-LANDSURFACEENERGY-C-020` binds a test-only mode that forces the complete
evaluator for those same component probes. For potential and fixed-final
solves, the admitted and forced paths compare every node/evaluation field,
raw/tolerance/normalized residual bit, complete dense Jacobian, pivot and matrix
norm, branch, iteration/backtracking trajectory, diagnostic, accepted owner and
typed error bit-for-bit. It proves both modes consume one shared canonical
evaluator node/tail implementation, and an independently enumerated oracle
compares the complete normative direct-edge graph rather than only selected
reachability. Exact custody vectors cover input generations and fields, caps,
frozen branches, graph, trial, coordinate, sign, perturbation, probe and stencil
without `Debug`/length/hash-only or repeated whole-input/probe proxy checks.

The normative fallibility/crossability matrix governs error evidence. Every
canonically crossable typed-error surface receives a source-real paired
replay-versus-forced-complete first-error and rollback vector; currently that is
`occ.leaf.current`, while `occ.leaf.maximum` joins this class only after an
authentic counterexample proves crossability. Every fallible-but-noncrossable
family receives the stated successful-base implication proof plus authentic
boundary/branch success and exact-field parity. Infallible families receive
source-ordered value/evaluation-field parity and never a synthetic error.
Separate real pre-admission and private-integrity vectors cover trial shape,
bounds, graph/topology identity, stale/foreign evidence, wrong bindings and
second use. The unmodified differential corpus catches any naturally occurring
error in either mode and requires the same first typed error, no post-replay
complete evaluation or alternate solver fallback, and byte-exact beginning/
custody rollback. Mutation and fault-injection hooks are forbidden.


<a id="canonical-invariants"></a>
## Canonical invariants
| Invariant ID | Statement | Authority | Evidence | Guard | Failure posture |
|---|---|---|---|---|---|
| <a id="INV-LANDSURFACEENERGY-164"></a> `INV-LANDSURFACEENERGY-164` | Within one validated represented-snow Jacobian sweep, a canonical sun-leaf, shade-leaf, wet-surface, or dry-stem temperature probe may reuse private successful same-iteration node results only when a topology-generic static transitive dependency graph proves those nodes unreachable from the sole changed coordinate. Every reachable node executes one shared canonical evaluator node/tail implementation in the exact complete-evaluator arithmetic and source order. Canonical probe construction/admission, residual order and bits, dense Jacobian/LU/pivots/bounds/backtracking, errors, trajectory, diagnostics, and output remain bit-identical. | `INV-LANDSURFACEENERGY-101/108/138/154/162` + reciprocal-longwave and liquid-routing authority | `[INFERENCE][Static]` | immutable sweep base, single-use signed-probe capability, versioned/hashed topology graph, forced-complete oracle, scoped and aggregate counters, normative fallibility/crossability matrix | ordinary ineligibility or conservatively unknown edges select complete evaluation before replay; private integrity mismatch fails typed; any post-start error returns directly; no synthetic fault hook, duplicated physics math, analytic/AD derivative, coloring, sparse solve, cache, approximation, recovery fallback, or error suppression |

<a id="canonical-obligations"></a>
## Canonical obligations
| Obligation ID | Statement | Applicability | Authority | Enforcement/failure | Test bindings |
|---|---|---|---|---|---|
| <a id="OBL-LANDSURFACEENERGY-C-020"></a> `OBL-LANDSURFACEENERGY-C-020` | requires an exact forced-complete differential oracle for every admitted component-temperature probe. It compares all replayed and retained node values, raw/tolerance/normalized residual vectors, dense Jacobian bits, branch identities, first errors, full potential/final solves, diagnostics, accepted output and rollback. The matrix covers centered and inward bounds, all wet/gas/zero-area branches, the normative fallibility/crossability classes, every naturally occurring first error, a real two-occupancy/six-soil- node Stage-3 fixture, reciprocal longwave between every component, and upper- wet-temperature effects routed into every lower occupancy. It requires one shared canonical evaluator tail, an independently enumerated complete direct- edge graph oracle, exact no-proxy custody, and truthful map/solve/sweep audit identities and lifecycle semantics. Crossable errors use source-real paired error/rollback vectors; noncrossable fallible nodes use implication proofs and authentic boundary successes; infallible nodes never receive synthetic errors. That fixture's full interior centered sweep reports exactly 58 ordered logical probes: 14 existing synthesized identity-anchor probes, 16 component dependency replays, and 28 complete probe evaluations. The eight hydraulic, four beta, and two shared-canopy-air columns retain complete evaluation. | Component-temperature dependency-replay consumers | v31:L546-L563 | Local guards; [errors](water-vapor.md#errors) | [Shared tests](common-details.md#tests) and [Mechanism tests](dependency-replay.md#replay); named fixtures/tests and real consumers |
