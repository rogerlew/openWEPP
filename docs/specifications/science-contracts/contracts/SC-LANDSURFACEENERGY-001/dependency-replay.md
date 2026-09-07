[Parent contract](../SC-LANDSURFACEENERGY-001.md)

## Dependencies
| Target | Required when | Boundary/obligation | Reading extent |
|---|---|---|---|
| interface.md#interface | always | shared authority | whole chapter |
| [nonlinear-solve](nonlinear-solve.md#nonlinear-solve), [surface-energy](surface-energy.md#surface-energy), [qualification](qualification.md#qualification) | component-temperature replay correctness | canonical stencils, leaf reuse and errors; radiation/turbulent dependencies; historical versus experimental limits | whole chapter |
| [validated-in-memory-lse-custody-handoff-amendment](map-custody.md#handoff), [covered-nonfinal-physical-only-map-amendment](map-custody.md#pending) | replay custody/error-order review | original validation positions and pending-map identity | section (entry extent) |
| [vapor](water-vapor.md#vapor), [water](water-vapor.md#water), [errors](water-vapor.md#errors), [invariants](water-vapor.md#canonical-invariants), [obligations](water-vapor.md#canonical-obligations) | represented-snow evaluator/error-order requirements review | signed enthalpy, immutable water/ingress and canonical errors; complete water definitions, including all universal P001–004 | section (entry extent) |
| water-vapor.md#water-vapor | active water/ingress implementation or full water-owner audit | complete owner duties | whole chapter |

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


<a id="identity-anchor"></a>
<a id="stage-3-identity-anchor-jacobian-amendment"></a>
## Stage-3 Identity-Anchor Jacobian Amendment

`INV-LANDSURFACEENERGY-162` applies only after the immutable covered-column
inputs and the represented-snow lower boundary have passed their complete
same-solve validation. In that regime the ground-temperature residual is
exactly

```text
(T_ground - T_snow) / 1e-9
```

and soil-temperature residual `i` is exactly

```text
(T_i - T_i,beginning) / 1e-9.
```

The represented-snow evaluator uses the boundary snow temperature—not the
ground coordinate—for reciprocal longwave, uses the boundary sensible and
vapor fluxes for the shared-air equations, and performs no ground vapor,
storage, or soil-conduction solve. Consequently each ground or soil coordinate
changes only its matching identity-anchor residual.

For one such Jacobian column, the canonical solver still constructs the exact
current-derived minus trial first and plus trial second using the unchanged
`sqrt(epsilon)*max(abs(x_i),unit_scale_i)` perturbation. It validates the
current and both trials through the existing covered-trial domain rule and
selects the unchanged centered or unique inward stencil. For each admitted
probe it copies the current complete normalized residual vector and replaces
only the matching anchor entry with the exact expression above, evaluated in
the same subtraction-then-division order as the complete evaluator. The
existing finite-difference function consumes those values; an analytic
derivative, sparse solve, changed operation order, or tolerance shortcut is
not authorized.

Every non-anchor coordinate, every ordinary or snow-free regime, every base
evaluation, and every prospective or backtracking evaluation retains the
complete evaluator. Missing or unvalidated Stage-3 boundary authority cannot
select the anchor path. If any residual dependency is introduced or cannot be
proved exactly, the implementation must use the complete evaluator rather
than approximate, cache across mutation, suppress an error, or install a
fallback. Unknown/residual ordering, dense Jacobian layout and bits, pivot
classification, LU, backtracking, convergence, diagnostics, output, receipts,
and first-error precedence remain those of `INV-LANDSURFACEENERGY-108/138`.

`OBL-LANDSURFACEENERGY-C-017` — Compare the optimized probe residual vectors
and resulting dense Jacobian columns with forced complete-evaluator results
bit-for-bit for ground and every soil coordinate, centered and admitted inward
stencils, multiple current iterates, and Stage-3 boundary/soil-anchor poisons.
Prove canonical minus-then-plus trial construction and domain rejection,
complete-evaluator call elimination only for admitted anchor probes, complete
reevaluation for all other coordinates/regimes, unchanged solver outcome and
diagnostics, and authentic runner output, closure, map-count, and publication
parity.

| Profile surface | Binding |
| --- | --- |
| algorithm step | For a validated represented-snow ground/soil column, construct and admit the canonical probes, replace only the matching exact anchor residual in the current complete vector, then use the existing finite-difference operation. |
| branch/guard | Private same-solve Stage-3 proof and exact anchor index are mandatory; all other columns/regimes use the complete evaluator, and any unproved dependency forbids reuse. |
| invariant guard map | `INV-LANDSURFACEENERGY-162` -> private validated boundary proof, ground/soil anchor classifier, exact probe-residual assembler, canonical stencil and dense-Jacobian path. |
| test vector | `OBL-LANDSURFACEENERGY-C-017`: full-evaluator residual/Jacobian bit parity, centered/inward/domain vectors, dependency poisons, evaluation-call counts, authentic runner parity. |
| binding exposure | `LSE-V28-STAGE3-ANCHOR-JACOBIAN`, active, `new-INV`, IDs `162/C-017`, dual review/verification. |
| change log | 2026-09-04, contract 28: exact represented-snow ground/soil identity-anchor probe reuse; unchanged equations, probes, Jacobian, solver, errors, outputs, and custody. |

<a id="leaf-reuse"></a>
<a id="covered-leaf-maximum-demand-exact-reuse-amendment"></a>
## Covered Leaf Maximum-Demand Exact-Reuse Amendment

`INV-LANDSURFACEENERGY-163` applies only within one invocation of the covered
occupancy evaluator after its current sun or shade `leaf_trial_state` call has
succeeded. The internal maximum-demand call uses the same leaf inputs,
biochemical constants, temperature, canopy humidity, gas environment,
boundary conductance, minimum conductance, and Medlyn parameter, changing only
beta to exact binary64 `1.0`.

The successful private `LeafTrialState` may be copied as the maximum-demand
state only when either current beta is bit-identical to exact `1.0`, so every
operand is identical, or its returned gas branch is `Inactive` or
`ExactZeroPar`. Those two admitted branches complete before beta participates
in any operation and therefore return the same state for the beta-one call.
`RespirationDominated`, `PositiveAssimilation`, and every unclassified branch
retain the complete beta-one evaluation unless beta itself is exact one.

Current sun and shade calls remain first and keep their existing order. Each
maximum result remains logically ordered sun then shade; a non-reused maximum
executes the unchanged function in that position. A failed current call is
never reused. The proof is the successful private `Copy` state in that same
stack evaluation; it is not a public capability and cannot cross an evaluator,
trial, Newton iteration, mutation, restart, serialization, or publication
boundary. Leaf equations, arithmetic, gas branches, maximum demand, hydraulic
residuals and tolerance, all other normalized residuals, finite differences,
dense Jacobian, LU/backtracking/convergence, results, diagnostics, and typed
first-error precedence remain bit-identical. Approximation, tolerant beta
comparison, additional branch admission, persistent caching, and fallback are
forbidden.

`OBL-LANDSURFACEENERGY-C-018` requires a forced-exhaustive test oracle that
always performs the beta-one calls and compares every private leaf-state field,
complete covered evaluation and normalized residual, frozen branch, and full
solve result bit-for-bit. It covers exact beta one, inactive,
exact-zero-PAR, positive-PAR beta one ULP below one, centered and inward beta
probes, success and typed-error precedence, exact call counts, and authentic
release output, closure, map-count, and publication parity.

| Profile surface | Binding |
| --- | --- |
| algorithm step | After a successful current leaf call, copy its private state for the adjacent beta-one maximum only for exact beta one or an admitted beta-independent branch; otherwise execute the complete maximum call. |
| branch/guard | Exact `to_bits` beta predicate or returned `Inactive`/`ExactZeroPar` branch is mandatory; all other cases perform the existing call, with no cache or fallback. |
| invariant guard map | `INV-LANDSURFACEENERGY-163` -> private same-evaluation leaf state, exact classifier, exhaustive-call oracle, invocation audit. |
| test vector | `OBL-LANDSURFACEENERGY-C-018`: every-field and complete-evaluation/solve bit parity, exact-beta and branch matrix, centered/inward probes, call order/count, typed-error precedence, authentic runner parity. |
| binding exposure | `LSE-V29-LEAF-MAXIMUM-EXACT-REUSE`, active, `new-INV`, IDs `163/C-018`, dual review/verification. |
| change log | 2026-09-04, contract 29: exact same-evaluation reuse of already successful bit-identical or beta-independent leaf states for internal beta-one maximum demand; unchanged equations, solver, outputs, errors, and custody. |


<a id="validation"></a>
<a id="carrier-parent-static-and-same-map-validation-once-amendment"></a>
## Carrier Parent-Static and Same-Map Validation-Once Amendment

This version extends the already admitted private validation-once custody of
`INV-LANDSURFACEENERGY-159`; it creates no new invariant and no solver version.
Within one already admitted terminal parent, one private non-Clone, non-wire,
generation-bound structural plan may retain only successful semantic
validation and deterministic indexes for immutable LSE and surface
configuration plus authenticated OFE/tile/occupancy topology. It never attests
to structural V8 state or to the distinct native resident's V3 LSE
configuration/state or V2 surface configuration/owner. It owns no mutable
runtime state, cannot be constructed from digests alone, and is absent from
restart, checkpoint, serialization, receipt, publication, and external APIs.

Plan construction is lazy at the first structural validation that an admitted
charged map reaches. If the parent has no charged map, no plan is minted and no
new validation occurs. Each plan join occurs at the exact configuration,
topology, or index check it replaces, after every existing carrier guard that
precedes that check; no plan join is hoisted ahead of support, duration,
transaction, joint, or forcing errors. On first use, the canonical full
validation at that position executes in its unchanged order. Failure returns
the same first typed error and leaves the parent and every owner byte-identical.
A successful plan binds the exact live parent generation, configuration,
topology, and index-source objects. Later maps may omit only those immutable
checks while every binding remains exact. Generation change, replacement,
equal-digest substitution, mutation, or transfer rejects at the original check
position without reconstructing a plan or falling back.

Every charged map retains the source-real order:

1. Run every existing carrier child/joint, support, duration, transaction,
   vegetation, receipt, boundary, prepared-input, and soil-read guard in its
   current position. Join the structural plan only when the first replaceable
   immutable structural check is reached.
2. At the existing forcing-validation position before V8, validate and
   canonically normalize that map's exact forcing. That first validation may
   mint a private move-only map proof bound to the live forcing allocation,
   transaction, support, generation, complete semantic digest, and normalized
   values. Equal digest with a different allocation is not authority.
3. Execute V8 projection in its current position. It freshly validates all
   current-map structural state and dynamic vegetation, LSE, surface, BGC,
   soil, hydrology, lower-boundary, and join surfaces. At V8's later validation
   of the pointer-identical forcing, consume the forcing proof instead of
   repeating only that validation. V8 neither receives nor attests to the
   distinct native resident's V3 LSE or V2 surface objects.
4. Derive the ingress schedule in its current fallible position after V8 and
   before native projection.
5. Only in a native regime, at the existing native-validation position, join
   the exact `FrozenLitterV3Resident` to its private
   `ValidatedFrozenLitterV3ResidentRevisionV1`. The revision must still match
   the resident's complete validated configuration/state/envelope digest,
   topology, transaction/predecessor/support, publication-prefix count/head/
   tail/chain, and exact V3-LSE/V2-surface references. That successful join may
   mint one borrowed, pointer-, revision-, parent-generation-, and map-bound
   proof, consumed immediately to omit only the repeated
   `lse_beginning.validate(lse_configuration)` and
   `surface_beginning.canonical_bytes(surface_configuration)` calls.
6. Continue every remaining native solver-ready, topology, rebinding,
   lower-boundary, residual, solver, output, and owner validation and every
   physical operation exactly as before. Ordinary maps mint and consume no
   resident proof.
7. Consume the final map through existing finalization and atomic parent
   commit. Rejected, history, or failed maps expose no plan/proof and mutate no
   owner. Restart discards ephemeral authority and reconstructs a fully
   validated resident revision through the canonical restore path.

The role order remains Initial, zero or more history candidates, and one final
candidate; each adaptive attempt retains direct before composed, and composed
retains Half1 before Half2 with Half2 beginning from the authenticated Half1
ending. Lazy plan creation, plan joins, forcing proof consumption, and resident
revision joins occur only at the checks they replace. Thus support, duration,
transaction, joint, forcing, V8, ingress-schedule, native-resident, subsequent
dynamic/solver, and output failures retain their present relative order, and
only the first error is returned. A stale plan paired with an earlier support,
duration, transaction, or joint poison returns that earlier error; a native
resident poison paired with an ingress-schedule poison returns the ingress
error. Malformed restart input still fails at its existing boundary.

The plan or proof must not contain or cache a
`ValidatedV8RuntimeInputProjection`, projected column, solver-ready tile,
hydrology snapshot, physical result, or dynamic owner candidate. It must not
use `Arc<DirectV10...>` or another shared owning handle to extend the lifetime
of a dynamic or complete DirectV10 input. A canonical digest may accompany
pointer/generation identity as evidence but can never independently admit an
object. The persistent resident revision is private validated custody and may
remain with an unchanged resident across maps; it is not the ephemeral map
proof. Its existing `Clone` implementation is authorized only as an inseparable
private clone of the exact whole immutable resident and never as independently
transferable admission. Every accepted resident successor is fully validated
before its revision advances atomically, and the resident may not mutate while
a borrowed proof exists. The plan, forcing proof, and resident map proof have no `Clone`,
serde, wire, public or unchecked constructor, cross-map/cross-parent transfer,
persistence, or restart restore. Second consumption and transfer are rejected;
alternate solver selection and silent full-validation fallback are prohibited.

`OBL-LANDSURFACEENERGY-C-019` requires an executable forced-full-validation
oracle against the admitted path. On the retained authentic terminal-parent
workload whose carrier performs 52 maps, audit evidence must report exactly one
parent-static validation, 52 exact normalized-forcing validations, and 52 fresh
dynamic-map validations. For each applicable regime independently, the oracle
must enumerate and compare every required Initial/history/final and
direct/Half1/Half2 role/path, with byte-for-byte physical and final-owner parity
and exact call order. Native and native-multilane maps exercise the real native
consumer; ordinary maps prove zero resident-proof mint/consume and zero native
physical execution.

Independent poisons distinguish structural versus native LSE configuration
and state, structural versus native surface configuration and owner,
generation, topology, index, support, duration, transaction, joint, forcing
pointer, same digest/different allocation, ingress schedule, resident revision,
proof second-use, cross-map, cross-parent, restart restoration, dynamic
vegetation/surface/soil-hydrology state, native solver/residual, and output
validation. Competing-poison vectors cross each ordered boundary through
dynamic validation, solver/residual, and output validation and require the same
first typed error on full and admitted paths. Every rejection has zero fallback and
publication plus byte-exact rollback. Counters and order records must originate
at the real carrier, first forcing validator, V8 projection, ingress scheduler,
resident-revision join, native V3 consumer, dynamic validators, and final owner;
fabricated outcomes, manually incremented fixture counters, or source scanning
alone cannot satisfy the obligation.

This is validation/custody architecture only. It adds no dimensional symbol,
conversion, scalar exception, constant, empirical parameter, tolerance,
equation, physical branch, solver, residual, output, publication field, or wire
format. All existing units, aliases, numeric guards, closure thresholds,
calibration posture, and constitutive-suite obligations remain unchanged.
Calibration and identifiability are therefore `CALIBRATION_NOT_APPLICABLE` for
this amendment; the contract-level fields remain unchanged.

| Profile surface | Binding |
| --- | --- |
| state surface | Private non-Clone/non-wire parent structural plan, per-map exact-forcing proof, existing resident validated revision, and borrowed non-Clone resident map proof; no cached dynamic state, result, owner candidate, restart, or publication representation. |
| algorithm step | Retain existing early carrier guards; join the lazy structural plan only at each replaced immutable check; validate forcing once before V8 and consume its proof at V8's duplicate forcing check; run V8 and fallible ingress; then join the exact resident revision and consume its proof only for the two repeated native V3/V2 validations. |
| branch/guard | Exact pointer, revision, parent generation, map, transaction, support, configuration, topology, index, and semantic identity are mandatory as applicable. Changed, reused, or transferred authority rejects at its original validation position with no fallback; restart and every trust boundary perform canonical full validation. |
| invariant guard map | `INV-LANDSURFACEENERGY-159` -> parent-static plan, exact-forcing proof, resident-revision-sourced native proof, authentic call-site audit, forced-full oracle, paired poison/error-order matrix, and rollback gate; `INV-LANDSURFACEENERGY-161` and `SC-COUPLEDTIME-001#INV-COUPLEDTIME-030` retain role/disposition custody. |
| alias/unit/constant/tolerance | No new aliases, dimensional values, conversions, constants, parameters, tolerances, or numeric normalization. Existing contract tables remain authoritative. |
| calibration | `CALIBRATION_NOT_APPLICABLE`: no parameter, observation, objective, calibration evidence, or identifiability claim changes. |
| test vector | `OBL-LANDSURFACEENERGY-C-019`: authentic 1/52/52 audit, per-applicable-regime bitwise role/path parity, ordinary zero-native proof, exact order, structural/native identity and proof-custody poisons, paired precedence, no cache/Arc/wire/fallback surface, and byte-exact rollback. |
| binding exposure | `LSE-V30-CARRIER-PARENT-STATIC-VALIDATION-ONCE`, active, `maps-to-existing-INV`, IDs `159/C-019`, dual review/verification. |
| change log | 2026-09-04, contract 30: admitted parent-static, source-ordered forcing, and resident-revision-sourced native validation-once custody only; structural V8 and resident V3/V2 objects remain distinct; no process physics, solver, tolerance, output, publication, or wire change. |


<a id="canonical-invariants"></a>
## Canonical invariants
| Invariant ID | Statement | Authority | Evidence | Guard | Failure posture |
|---|---|---|---|---|---|
| <a id="INV-LANDSURFACEENERGY-164"></a> `INV-LANDSURFACEENERGY-164` | Within one validated represented-snow Jacobian sweep, a canonical sun-leaf, shade-leaf, wet-surface, or dry-stem temperature probe may reuse private successful same-iteration node results only when a topology-generic static transitive dependency graph proves those nodes unreachable from the sole changed coordinate. Every reachable node executes one shared canonical evaluator node/tail implementation in the exact complete-evaluator arithmetic and source order. Canonical probe construction/admission, residual order and bits, dense Jacobian/LU/pivots/bounds/backtracking, errors, trajectory, diagnostics, and output remain bit-identical. | `INV-LANDSURFACEENERGY-101/108/138/154/162` + reciprocal-longwave and liquid-routing authority | `[INFERENCE][Static]` | immutable sweep base, single-use signed-probe capability, versioned/hashed topology graph, forced-complete oracle, scoped and aggregate counters, normative fallibility/crossability matrix | ordinary ineligibility or conservatively unknown edges select complete evaluation before replay; private integrity mismatch fails typed; any post-start error returns directly; no synthetic fault hook, duplicated physics math, analytic/AD derivative, coloring, sparse solve, cache, approximation, recovery fallback, or error suppression |
| <a id="INV-LANDSURFACEENERGY-162"></a> `INV-LANDSURFACEENERGY-162` | In a validated represented-snow covered solve, the ground and soil temperature equations are exact identity anchors and those coordinates affect no other normalized residual. Their canonical minus-then-plus Jacobian probes may therefore reuse the current complete residual vector, replacing only the probed anchor residual with the exact full-evaluator expression. Trial-domain admission, finite-difference stencil and arithmetic, dense Jacobian bits, LU/pivot/backtracking order, convergence, diagnostics, accepted result, and first-error precedence remain identical; every other coordinate and every non-Stage-3 regime uses the complete evaluator. | `INV-LANDSURFACEENERGY-108/138/154/159` | `[INFERENCE][Static]` | private same-solve validated Stage-3 anchor proof, exact full-evaluator differential oracle, evaluator-call counter, boundary/domain poisons | any unproved dependency or identity mismatch uses the complete evaluator; no analytic derivative, approximation, fallback, or error suppression |
| <a id="INV-LANDSURFACEENERGY-163"></a> `INV-LANDSURFACEENERGY-163` | Within one covered-occupancy evaluation, the internal beta-one maximum-demand leaf state may reuse the already successful current leaf state only when every call operand is bit-identical because current beta is exact binary64 `1.0`, or when the returned private gas branch is `Inactive` or `ExactZeroPar` and that branch provably does not read beta. Sun-before-shade and current-before-maximum error precedence, equations, tolerances, residuals, branches, Jacobian, solver, and results remain bit-identical. | `INV-LANDSURFACEENERGY-108/138/162` + deterministic V10 leaf-gas branch authority | `[INFERENCE][Static]` | private same-evaluation `LeafTrialState`, exact beta predicate, beta-independent branch classifier, exhaustive-call differential oracle and call counter | every other branch or beta executes the complete beta-one call; no cross-evaluation cache, approximation, fallback, or suppressed error |


<a id="canonical-obligations"></a>
## Canonical obligations
| Obligation ID | Statement | Applicability | Authority | Enforcement/failure | Test bindings |
|---|---|---|---|---|---|
| <a id="OBL-LANDSURFACEENERGY-C-020"></a> `OBL-LANDSURFACEENERGY-C-020` | requires an exact forced-complete differential oracle for every admitted component-temperature probe. It compares all replayed and retained node values, raw/tolerance/normalized residual vectors, dense Jacobian bits, branch identities, first errors, full potential/final solves, diagnostics, accepted output and rollback. The matrix covers centered and inward bounds, all wet/gas/zero-area branches, the normative fallibility/crossability classes, every naturally occurring first error, a real two-occupancy/six-soil- node Stage-3 fixture, reciprocal longwave between every component, and upper- wet-temperature effects routed into every lower occupancy. It requires one shared canonical evaluator tail, an independently enumerated complete direct- edge graph oracle, exact no-proxy custody, and truthful map/solve/sweep audit identities and lifecycle semantics. Crossable errors use source-real paired error/rollback vectors; noncrossable fallible nodes use implication proofs and authentic boundary successes; infallible nodes never receive synthetic errors. That fixture's full interior centered sweep reports exactly 58 ordered logical probes: 14 existing synthesized identity-anchor probes, 16 component dependency replays, and 28 complete probe evaluations. The eight hydraulic, four beta, and two shared-canopy-air columns retain complete evaluation. | Component-temperature dependency-replay consumers | v31:L546-L563 | [Local guards/errors](water-vapor.md#errors) | [Tests](common-details.md#tests); [Detail](#replay); named fixtures/tests and real consumers |
| <a id="OBL-LANDSURFACEENERGY-C-017"></a> `OBL-LANDSURFACEENERGY-C-017` | requires every represented-snow ground/soil anchor probe residual vector and dense Jacobian column to equal the complete covered evaluator bit-for-bit for centered and admitted inward stencils. It must prove unchanged minus-then-plus trial construction and domain admission, zero complete constitutive reevaluations for those anchor probes, complete reevaluation for every other coordinate and regime, dependency-invalidating poisons, unchanged first-error precedence, and authentic runner output/count parity. | Represented-snow ground/soil identity-anchor probe consumers | v31:L529-L536 | [Local guards/errors](water-vapor.md#errors) | [Tests](common-details.md#tests); [Detail](#identity-anchor); named fixtures/tests and real consumers |
| <a id="OBL-LANDSURFACEENERGY-C-018"></a> `OBL-LANDSURFACEENERGY-C-018` | requires every field of the reused private leaf state, the complete covered evaluation, normalized residuals, frozen branches, and full solve outcome to equal an exhaustive beta-one-call oracle bit-for-bit. It must cover exact beta one, exact-zero-PAR, inactive, positive-PAR beta one ULP below one, centered and inward beta probes, call elimination only for the admitted cases, unchanged call order and typed-error precedence, and authentic runner output/count parity. | Covered leaf dependency-reuse consumers | v31:L538-L544 | [Local guards/errors](water-vapor.md#errors) | [Tests](common-details.md#tests); [Detail](#leaf-reuse); named fixtures/tests and real consumers |
| <a id="OBL-LANDSURFACEENERGY-C-019"></a> `OBL-LANDSURFACEENERGY-C-019` | on the authentic 52-map terminal-parent workload, prove exactly one parent-static configuration/topology/index validation, exactly 52 exact normalized-forcing validations, and exactly 52 fresh dynamic-map validations. Prove full-versus-admitted bitwise physical and final-owner parity for every applicable Initial/history/final and direct/Half1/Half2 role/path in ordinary, native, and multilane regimes; ordinary maps must mint and consume zero native-resident proofs. Preserve exact source call and first-error order. Independently poison structural and native LSE configurations/states, structural and native surface configurations/owners, generation, topology, index, support, duration, transaction, joint, forcing pointer, same-digest/different-allocation, resident revision, proof second-use, cross-map, cross-parent, and restart; add competing-poison vectors across all ordered boundaries. Every rejection has zero fallback/publication and byte-exact rollback. Executable evidence must exercise the real carrier, first forcing validator, V8 structural seam, ingress schedule, resident revision, and native-V3 consumer; fabricated counters or source scanning alone cannot discharge this obligation. | Carrier parent-static/same-map validation reuse consumers | v31:L378-L394 | [Local guards/errors](water-vapor.md#errors) | [Tests](common-details.md#tests); [Detail](#validation); named fixtures/tests and real consumers |
