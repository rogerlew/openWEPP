[Parent contract](../SC-LANDSURFACEENERGY-001.md)

## Dependencies
| Target | Required when | Boundary/obligation | Reading extent |
|---|---|---|---|
| interface.md#interface | all solver work | authority, owner and qualification boundaries | whole mechanism chapter |
| surface-energy.md#surface-energy | changed evaluator or equivalence/correctness review | all affected radiative/turbulent physics | whole mechanism chapter |
| soil-coupling.md#soil-coupling | changed lower boundary or soil evaluator | regime-specific equations | whole mechanism chapter |
| water-vapor.md#water-vapor | changed evaluator/error-order or full solver review | water/enthalpy ordering | whole mechanism chapter |
| terminal-support.md#terminal-support | represented-snow or terminal solves | admission and regime | whole mechanism chapter |
| dependency-replay.md#dependency-replay | component-probe reuse review | graph and error/custody proof | whole mechanism chapter |
| ../SC-VEGETATION-001.md#openwepp_c3_woody_v10-nonpositive-assimilation-amendment | V10 gas/evaluator equivalence or full solver correctness | V10 gas branch owner | named section plus destination scope; retain applicable single-file whole-contract/frozen-kickoff requirements |
| ../SC-VEGETATION-001.md#purpose | cross-contract vegetation scope | vegetation scope | named section plus destination scope; retain applicable single-file whole-contract/frozen-kickoff requirements |

<a id="nonlinear-solve"></a>
# Nonlinear Solve
Current complete ordered covered solver and V10-specific specialization. INV-138 retains the exact admitted closed-bound qualifier; valid-current/one-inadmissible-probe summaries do not broaden that scope. INV-139 accepts only the unchanged current iterate. V10 diagonal scaling remains potential/nonpositive-assimilation-only. Identity-anchor and leaf reuse are distinct limited optimizations; component replay additionally requires dependency-replay and its applicable qualification.

<a id="ordered-numerical-algorithm-active-branches-and-error-precedence"></a>
### Ordered numerical algorithm, active branches, and error precedence

For each OFE, tiles sort by typed tile ID. Within each tile, canopy occupancies
sort top-to-bottom by rank and then typed occupancy ID. The exact unknown order
is: every V8 occupancy unknown block in that order (with the V7 within-block
order), the shared tile `(T_c,q_c)` when covered, `T_s`, and soil thermal
`T_1..T_N`. The exact residual order is: matching V8 gas/component-energy/
hydraulic blocks, shared canopy-air heat, shared canopy-air vapor, surface
energy, then soil thermal layers `1..N`. Open tiles omit only the two shared-air
unknowns/residuals and use reference-air fluxes.

The nonlinear unknown bounds are `200<=T<=350 K`,
`0<=q_c<=0.1 kg kg^-1`, and the unchanged V8 bounds for `ci`, hydraulic
potentials, and `beta`. Configuration/domain validation precedes numerical
bounds; an invalid physical input is not reported as nonconvergence.

The joint system may be monolithic or a mathematically equivalent nested solve
only when every inner solve converges at every outer residual evaluation and
the complete ordered residual vector passes. Residuals are normalized by the
dimensional thresholds below before the infinity norm is formed. Starting from
the complete caller warm start, each Newton iteration uses
`delta_i=sqrt(epsilon)*max(abs(x_i),unit_scale_i)`. An interior coordinate
evaluates the minus point and then the plus point and uses the exact centered
difference. At an admitted closed bound where exactly one of those canonical
probes violates the existing covered-trial domain, the Jacobian uses the
unique inward one-sided difference between the valid current iterate and the
admitted probe. An invalid current iterate, or two inadmissible probes, rejects
with a typed constitutive-domain failure. The solve then uses deterministic
partial-pivot LU. First apply the existing full-trial no-update witness: when
the `b=0` trial is domain-valid, evaluate its exact prospective component
steps and accept the current iterate if the current complete residual vector
and every governed step norm pass. If that full-trial witness fails because
the trial is domain-invalid or any governed full-trial step exceeds its
unchanged threshold while the current complete residual vector passes, the
solver examines the same ordered factors `2^-b`, `b=1..20`, until the first
domain-valid halved trial. That trial is evaluated prospectively without
installing it. The exact applied hydraulic, beta, temperature, humidity, and
derived `ci` step norms are reconstructed; every norm with an existing
coordinate threshold must pass. The `ci` norm remains diagnostic because this
contract defines no independent `ci` step threshold. If those predicates pass, the
solver accepts the current iterate with no update. The accepted solution,
evaluation, state, branch, ledger, and owner candidates are exclusively those
of the current iterate; the diagnostic step norms and the exact examined
exponent contribution added to the existing cumulative backtracking count
record the prospective witness. No separate persisted or public exponent field
is authorized. This witness is unavailable when the full trial is
itself a passing existing no-update witness, the current residual vector does
not pass, the first domain-valid halved trial cannot be completely evaluated,
or any governed prospective coordinate step fails.

Otherwise the solve accepts the first factor `2^-b`, `b=0..20`, producing a
strict decrease in normalized infinity norm and installs that trial as the
next iterate. Equal pivot magnitudes choose the lowest row. Failure to decrease
through `b=20` is backtracking limit. A pivot below
`64*epsilon*matrix_inf_norm` is singular. The iteration limit is 50 completed
Newton updates. Unit scales are `1 K`, `0.001 kg kg^-1`, `1 Pa`, `1000 mm`,
and `1` for beta.

Each energy residual threshold is
`1e-6 W m^-2+1e-10*max(1,sum(abs(component operands)))`. Each water/vapor
residual threshold is `1e-12 kg m^-2 s^-1+1e-9*scale`. Accepted temperature
step is at most `1e-8 K`; humidity step at most `1e-12 kg kg^-1`; accepted
hydraulic step is at most `1e-7 mm`; and beta step at most `1e-10`.
Convergence requires all residual and step criteria on the same accepted
iterate or the exact no-update witness above, whose residuals come from that
accepted current iterate and whose prospective step norms come from the first
domain-valid halved trial. Active branches are evaluated in this order: typed domain; surface
class; covered/open turbulence; positive-surface-water versus dry-soil vapor;
vapor sign; water cap (`cap<=law`, equality cap-active); then numerical solve.
Identity, unit, basis, owner, layer, band, direction and D/A/F inequalities are
exact and cannot be tolerance-repaired.

Error precedence is: malformed serialization; model/configuration/state/
transaction identity; missing/duplicate topology or owner; nonfinite operand;
unsupported snow/frozen/thawing/calm/nonneutral/domain branch; constitutive
domain; request/authorization identity or bound; singular pivot;
backtracking limit; iteration limit; accepted-step/residual failure;
component closure; control-volume closure; cross-owner join. Only the first
error is returned, with diagnostics accumulated up to that point.

Every failure includes typed model/configuration/state/transaction/OFE/tile/
occupancy/pass/solve identity, ordered residuals, iteration and backtracking
counts, step, bounds/caps, bracket/pivot/matrix evidence and rollback hashes.
No failed iterate or partial owner candidate is usable.

<a id="version-11-inactive-liquid-vapor-coordinate-domain-amendment"></a>
## Version 11 inactive liquid-vapor coordinate domain amendment

The sun-leaf, shade-leaf, and wet-surface temperature coordinates use the
covered solver's liquid-vapor saturation law, whose admitted phase domain
begins at `T_ref = 273.15 K`. When one of those component coordinates is
numerically inactive because its physical component area is exactly zero, or
when the existing `INV-LANDSURFACEENERGY-113` wet-coordinate predicate has
already proven its physical energy residual inside the canonical tolerance,
its deterministic representational anchor is

```text
T_inactive_liquid_vapor - max(T_canopy, T_ref) = 0.
```

The dry-stem inactive anchor remains exactly `T_stem - T_canopy = 0` because
that coordinate does not invoke the liquid-vapor law. The amended target only
keeps an otherwise unconstrained numerical coordinate inside its existing
constitutive domain. For an exactly zero-area component the Newton row is the
direct deterministic anchor row; it does not inherit the nondifferentiable
phase-boundary slope from a finite difference through `max`. An exactly
zero-area component contributes no physical
radiative, sensible, latent, mass, enthalpy, or ledger operand, and the
existing V10 inactive-wet predicate still requires the unanchored physical
wet-energy residual to pass before the row substitution. Active components,
physical residual equations, tolerances, ledgers, receipts, events, the exact
60-second raw fallback, backtracking limits, rollback, and fail-closed behavior
are unchanged.

| ID | Binding rule | Guard/failure |
|---|---|---|

<a id="version-12-exact-closed-bound-finite-difference-amendment"></a>
## Version 12 exact closed-bound finite-difference amendment

The covered-column nonlinear domain already contains closed coordinates,
including `0 <= beta <= 1`, `273.15 <= T_liquid-vapor <= 350 K` for canopy
components and liquid-bearing ground, the remaining temperature bounds, and
`0 <= q_canopy <= 0.1 kg kg^-1`. A valid current
iterate exactly at one of those bounds cannot admit both canonical centered
probes. This is a property of the existing domain, not constitutive failure at
the current physical state.

For every covered-column authority and for both owner-uncapped potential and
fixed-authorization final solves, retain the exact perturbation
`delta_i=sqrt(epsilon)*max(abs(x_i),unit_scale_i)`, unit scales, ordered minus
then plus evaluation, normalized residuals, and frozen active branches. When
both probes satisfy the existing covered-trial domain, use the exact centered
difference

```text
J[:,i] = (R(x + delta_i e_i) - R(x - delta_i e_i)) / (2 delta_i).
```

When the current iterate is valid and exactly one probe violates that domain,
use only the unique inward one-sided difference:

```text
lower bound: J[:,i] = (R(x + delta_i e_i) - R(x)) / delta_i
upper bound: J[:,i] = (R(x) - R(x - delta_i e_i)) / delta_i.
```

An invalid current iterate rejects before Jacobian construction. If neither
probe is admissible, Jacobian construction rejects with the covered
constitutive-domain error; it does not shrink `delta_i`, clamp a probe, infer a
derivative, or continue. Exact `beta=0`, `beta=1`, and exact `273.15 K` active
or zero-area liquid-vapor coordinates, and exact `273.15 K` liquid-bearing
ground coordinates are ordinary boundary cases under this rule. Values outside
their existing domains remain poisons.

This amendment changes no constitutive equation, closed bound, coordinate or
residual scale, branch predicate, branch-freezing order, diagonal scaling
authority, pivot rule, backtracking, convergence threshold, ledger, receipt,
event, custody, rollback, or fail-closed requirement. In particular, the
diagonal coordinate scaling admitted by `INV-LANDSURFACEENERGY-112` remains
exclusive to the uncapped active V10 nonpositive-assimilation potential solve;
the inward derivative rule is not scaling authority for any other solve.

| ID | Binding rule | Guard/failure |
|---|---|---|
| `INV-LANDSURFACEENERGY-138` | Every covered potential/final Jacobian retains centered differences for two valid probes and uses the unique inward one-sided difference only when a valid current iterate has exactly one inadmissible canonical probe. | beta lower/upper, active/zero-area canopy and liquid-bearing-ground vapor lower bound, interior centered, potential/final, invalid-current, and neither-probe poisons; typed `covered_jacobian_bound` rejection |

<a id="version-13-first-domain-valid-no-update-termination-amendment"></a>
## Version 13 first-domain-valid no-update termination amendment

The covered solver already admits a no-update termination when the current
complete normalized residual vector passes and a valid full Newton trial proves
that every governed prospective component step is inside its unchanged
threshold. A closed phase bound can make that full trial invalid, while an
otherwise valid full trial can exceed one unchanged governed step threshold.
In either case a deterministic halving can produce a domain-valid governed
step too small to produce an observable strict residual decrease in binary64.

For both owner-uncapped potential and fixed-authorization final solves, first
retain the existing no-update witness on a domain-valid full `b=0` Newton
trial. If and only if the current complete residual vector passes and that
full-trial witness cannot accept because the full trial is outside the existing
covered-trial domain or any governed full-trial prospective step exceeds its
unchanged threshold, examine the existing ordered backtracking sequence from
`b=1` until its first domain-valid halved trial. A
complete prospective evaluation of that trial supplies the exact applied
hydraulic, beta, temperature and humidity norms and the diagnostic derived
`ci` norm. When all four governed norms pass their unchanged thresholds,
accept the current iterate without applying, projecting, or publishing any
part of the trial. Record the prospective norms and add the exact examined
exponent to the existing cumulative backtracking-count diagnostic; do not add a
separate public or persisted field. Retain the current solution, evaluation,
active branches, state, water requests/uses, ledgers, owner candidates, and
closure operands exactly.

The halved witness is refused when the current residual vector has any nonfinite or
out-of-tolerance member; when the full trial itself passes the existing
no-update witness; when the first domain-valid halved trial cannot be
completely evaluated; or when any governed prospective step norm fails. After
refusal, that same first domain-valid trial
and all later factors remain eligible only under the unchanged strict-residual-
decrease rule for an actual installed update. Exhaustion remains the existing
typed backtracking-limit failure with exact rollback. The solver may not skip
the first domain-valid candidate to obtain a smaller no-update witness.

This amendment changes no closed bound, constitutive equation, residual or
step threshold, finite-difference rule, pivot rule, iteration/backtracking
limit, active branch, event chronology, 60-second floor, mass/energy ledger,
receipt, custody, topology, publication, or rollback rule. It admits no trial
clamp or projected state and does not turn strict decrease into a tolerance;
strict decrease remains mandatory for every actual update.

| ID | Binding rule | Guard/failure |
|---|---|---|
| `INV-LANDSURFACEENERGY-139` | After the full Newton trial fails the existing no-update witness by domain invalidity or a governed step excess, a passing current residual vector and passing governed step norms on the first domain-valid halved trial accept only the unchanged current iterate. | positive full-outside and full-step-excess/first-valid witnesses; residual/nonfinite, already-passing-full, per-thresholded-coordinate, prospective-evaluation and skip-first poisons; unchanged strict-decrease update/backtracking-limit/rollback vectors |

Required real-consumer vectors are the two interior terminal-event paths that
previously reached `FinalFixedCap` iteration 4 and exhausted 20 halvings. They
must complete with current-state acceptance, unchanged owner closure, and no
trial installation. Existing oracle backtracking-limit and genuine strict-
decrease vectors remain required and must retain their prior disposition.

<a id="openwepp_snow_free_lse_v2-v10-coupling-amendment"></a>
## `OPENWEPP_SNOW_FREE_LSE_V2` V10 Coupling Amendment

V2 imports every V1 control volume, tolerance, owner, rollback rule, and
positive-PAR accepted result. It requires the V10 vegetation identity and
admits its exact-zero-PAR and respiration-dominated positive-low-light
branches. It does not recompute, clamp, or relabel V10 gas states.

When every positive final water authorization is identity- and amount-equal to
its potential request with `FullSupply`, and every canonical zero request
retains its exact identity and zero amount, V2 uses the accepted potential
coordinates as the deterministic fixed-final initial iterate. It rebuilds the
complete fixed-final evaluation from immutable beginning owners and exact
per-resource caps. No potential flux, candidate state, branch, receipt, or
diagnostic is copied.

If that initial evaluation satisfies every residual tolerance, active-branch
inequality, domain and bound, `F<=A<=D`, identity, and owner check, V2 accepts
at iteration zero with exact-zero step norms and zero backtracking without
constructing a Jacobian. Every actual solver step retains V1 strict-decrease
and convergence rules. Copying the potential candidate without complete final
reevaluation is forbidden. A residual outside tolerance or any branch,
identity, owner, or amount mismatch cannot use this acceptance path.

Nonpositive-assimilation partial positive root authorization is typed unsupported in V2.

It does not invoke hydraulic attenuation, conductance or vulnerability floors,
plant capacitance, or authorization donation.

Every covered potential and final solve uses the exact closed-bound derivative
rule in `INV-LANDSURFACEENERGY-138`. This general numerical-domain rule does
not broaden the V10-only coordinate-scaling authority below.

For that same uncapped active V10 nonpositive-assimilation potential solve only, the
Newton linear system is expressed in the declared coordinate units. With
`x = D y`, where `D` is the exact finite-difference unit-scale diagonal, the
solver forms `J_y[:,j] = J_x[:,j] * D[j]`, applies the canonical pivot test to
`J_y`, solves `J_y delta_y = -r`, and maps
`delta_x[j] = D[j] * delta_y[j]`. This deterministic nondimensionalization
changes neither the residual equations nor the represented physical Newton
direction. It is forbidden for V1, V8/V9-derived behavior, positive-PAR V10,
and fixed-final solves. Reducing the pivot multiplier, accepting a rejected
pivot, regularization, larger iteration/trust bounds, or physiological floors
is not an equivalent implementation.

When the uncapped V10 nonpositive-assimilation potential evaluation selects the
canopy-liquid store-cap branch and the preliminary store rate is no larger
than the canonical water residual tolerance, the wet-surface temperature is a
numerically inactive coordinate: the wet-energy residual is already below its
admitted closure scale and cannot determine that temperature. V2 replaces
that one row with the existing inactive-component anchor
`T_wet - max(T_canopy, 273.15 K) = 0`. Liquid mass, enthalpy, longwave area, and every owner
ledger remain evaluated normally; no liquid amount is clamped or discarded.
The predicate additionally requires the unanchored physical wet-energy
residual already satisfy its canonical energy tolerance. It is unavailable to
V1, positive-assimilation V10, condensation, or a constitutive-law wet flux.

V1-to-V2 migration validates complete V1 and V10 owner identities, copies all
LSE scientific values bit-identically, and changes only the LSE identity and
transitively derived receipts. V1 remains immutable and is not a V2 alias.

| ID | Binding rule |
|---|---|
| `LSE-E-109` | V8/V9 vegetation identity, mixed V1/V2 receipts, or any owner alias rejects before V2 physics. |
| `LSE-E-110` | Missing, duplicated, mutated, or locally recomputed V10 nighttime gas state rejects the coupled owner envelope. |
| `LSE-E-111` | Partial or value-mutating V1-to-V2 migration rejects without a V2 state. |

This amendment remains default-off. It authorizes no production selector,
default/output change, cutover, snow handoff, deployment, calibration, or
empirical claim.

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


<a id="canonical-invariants"></a>
## Canonical invariants
| Invariant ID | Statement | Authority | Evidence | Guard | Failure posture |
|---|---|---|---|---|---|
| <a id="INV-LANDSURFACEENERGY-138"></a> `INV-LANDSURFACEENERGY-138` | Every covered-column Jacobian uses the canonical centered binary64 difference at an interior coordinate and the unique inward one-sided difference only at an exact admitted closed bound; an invalid current iterate or two inadmissible probes rejects. | deterministic numerical-domain authority | `[INFERENCE][Static]` | runtime/test | typed constitutive-domain failure |
| <a id="INV-LANDSURFACEENERGY-139"></a> `INV-LANDSURFACEENERGY-139` | When the current complete covered residual vector passes, the full Newton trial cannot satisfy the existing no-update witness because it is domain-invalid or a governed prospective step exceeds its unchanged threshold, and the first domain-valid halved trial has every governed exact prospective step norm inside those thresholds, the solver accepts the current iterate without installing the trial. | deterministic no-update termination authority | `[INFERENCE][Static]` | runtime/test | accepted current iterate or unchanged strict-decrease/fail-closed path |
| <a id="INV-LANDSURFACEENERGY-162"></a> `INV-LANDSURFACEENERGY-162` | In a validated represented-snow covered solve, the ground and soil temperature equations are exact identity anchors and those coordinates affect no other normalized residual. Their canonical minus-then-plus Jacobian probes may therefore reuse the current complete residual vector, replacing only the probed anchor residual with the exact full-evaluator expression. Trial-domain admission, finite-difference stencil and arithmetic, dense Jacobian bits, LU/pivot/backtracking order, convergence, diagnostics, accepted result, and first-error precedence remain identical; every other coordinate and every non-Stage-3 regime uses the complete evaluator. | `INV-LANDSURFACEENERGY-108/138/154/159` | `[INFERENCE][Static]` | private same-solve validated Stage-3 anchor proof, exact full-evaluator differential oracle, evaluator-call counter, boundary/domain poisons | any unproved dependency or identity mismatch uses the complete evaluator; no analytic derivative, approximation, fallback, or error suppression |
| <a id="INV-LANDSURFACEENERGY-163"></a> `INV-LANDSURFACEENERGY-163` | Within one covered-occupancy evaluation, the internal beta-one maximum-demand leaf state may reuse the already successful current leaf state only when every call operand is bit-identical because current beta is exact binary64 `1.0`, or when the returned private gas branch is `Inactive` or `ExactZeroPar` and that branch provably does not read beta. Sun-before-shade and current-before-maximum error precedence, equations, tolerances, residuals, branches, Jacobian, solver, and results remain bit-identical. | `INV-LANDSURFACEENERGY-108/138/162` + deterministic V10 leaf-gas branch authority | `[INFERENCE][Static]` | private same-evaluation `LeafTrialState`, exact beta predicate, beta-independent branch classifier, exhaustive-call differential oracle and call counter | every other branch or beta executes the complete beta-one call; no cross-evaluation cache, approximation, fallback, or suppressed error |
| <a id="INV-LANDSURFACEENERGY-108"></a> `INV-LANDSURFACEENERGY-108` | Ordered unknowns, residuals, branches, finite differences, pivots, backtracking, tolerances, and error precedence are deterministic. | [Original authority](https://github.com/rogerlew/openWEPP/blob/b932db101cce07d0860b45b5ecaa8ddb7f455b58/docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001.md#L1097-L1097) | [INFERENCE][Static]; [original rule/context](https://github.com/rogerlew/openWEPP/blob/b932db101cce07d0860b45b5ecaa8ddb7f455b58/docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001.md#L1097-L1097) | Existing coupled domain/closure guards in [ordered solve](nonlinear-solve.md#ordered-numerical-algorithm-active-branches-and-error-precedence) and [typed errors](water-vapor.md#independent-closure-and-errors); [source](https://github.com/rogerlew/openWEPP/blob/b932db101cce07d0860b45b5ecaa8ddb7f455b58/docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001.md#L1097-L1097) | Existing typed domain/convergence/closure failure: [error map](water-vapor.md#independent-closure-and-errors); no partial state or promotion |
| <a id="INV-LANDSURFACEENERGY-131"></a> `INV-LANDSURFACEENERGY-131` | Only a numerically inactive sun-leaf, shade-leaf, or wet-surface temperature coordinate uses `max(T_canopy, 273.15 K)` as its representational anchor; zero physical area and the existing inactive-wet residual predicate guarantee no physical operand or ledger interference. | [Original authority](https://github.com/rogerlew/openWEPP/blob/b932db101cce07d0860b45b5ecaa8ddb7f455b58/docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001.md#L1327-L1327) | [INFERENCE][Static]; [original rule/context](https://github.com/rogerlew/openWEPP/blob/b932db101cce07d0860b45b5ecaa8ddb7f455b58/docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001.md#L1327-L1327) | cold-canopy zero-area phase-domain vector; unchanged active-component residual/closure guards and numerical rejection | cold-canopy zero-area phase-domain vector; unchanged active-component residual/closure guards and numerical rejection |
| <a id="INV-LANDSURFACEENERGY-109"></a> `INV-LANDSURFACEENERGY-109` | LSE-V2 imports exact LSE-V1 physics and accepts only the V10 vegetation owner at the coupled boundary. | [Original authority](https://github.com/rogerlew/openWEPP/blob/b932db101cce07d0860b45b5ecaa8ddb7f455b58/docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001.md#L2070-L2070) | [INFERENCE][Static]; [original rule/context](https://github.com/rogerlew/openWEPP/blob/b932db101cce07d0860b45b5ecaa8ddb7f455b58/docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001.md#L2070-L2070) | Existing coupled domain/closure guards in [ordered solve](nonlinear-solve.md#ordered-numerical-algorithm-active-branches-and-error-precedence) and [typed errors](water-vapor.md#independent-closure-and-errors); [source](https://github.com/rogerlew/openWEPP/blob/b932db101cce07d0860b45b5ecaa8ddb7f455b58/docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001.md#L2070-L2070) | Existing typed domain/convergence/closure failure: [error map](water-vapor.md#independent-closure-and-errors); no partial state or promotion |
| <a id="INV-LANDSURFACEENERGY-110"></a> `INV-LANDSURFACEENERGY-110` | Exact FullSupply finalization seeds only coordinates, then reevaluates the complete fixed-final system from immutable beginning owners; a passing initial evaluation accepts at iteration zero without a Jacobian. | [Original authority](https://github.com/rogerlew/openWEPP/blob/b932db101cce07d0860b45b5ecaa8ddb7f455b58/docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001.md#L2071-L2071) | [INFERENCE][Static]; [original rule/context](https://github.com/rogerlew/openWEPP/blob/b932db101cce07d0860b45b5ecaa8ddb7f455b58/docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001.md#L2071-L2071) | Existing coupled domain/closure guards in [ordered solve](nonlinear-solve.md#ordered-numerical-algorithm-active-branches-and-error-precedence) and [typed errors](water-vapor.md#independent-closure-and-errors); [source](https://github.com/rogerlew/openWEPP/blob/b932db101cce07d0860b45b5ecaa8ddb7f455b58/docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001.md#L2071-L2071) | Existing typed domain/convergence/closure failure: [error map](water-vapor.md#independent-closure-and-errors); no partial state or promotion |
| <a id="INV-LANDSURFACEENERGY-111"></a> `INV-LANDSURFACEENERGY-111` | V1-to-V2 migration copies every LSE scientific value bit-identically and derives only V2 identity receipts; partial nonpositive-assimilation root supply is unsupported. | [Original authority](https://github.com/rogerlew/openWEPP/blob/b932db101cce07d0860b45b5ecaa8ddb7f455b58/docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001.md#L2072-L2072) | [INFERENCE][Static]; [original rule/context](https://github.com/rogerlew/openWEPP/blob/b932db101cce07d0860b45b5ecaa8ddb7f455b58/docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001.md#L2072-L2072) | Existing coupled domain/closure guards in [ordered solve](nonlinear-solve.md#ordered-numerical-algorithm-active-branches-and-error-precedence) and [typed errors](water-vapor.md#independent-closure-and-errors); [source](https://github.com/rogerlew/openWEPP/blob/b932db101cce07d0860b45b5ecaa8ddb7f455b58/docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001.md#L2072-L2072) | Existing typed domain/convergence/closure failure: [error map](water-vapor.md#independent-closure-and-errors); no partial state or promotion |
| <a id="INV-LANDSURFACEENERGY-112"></a> `INV-LANDSURFACEENERGY-112` | Only the uncapped active V10 nonpositive-assimilation potential solve uses the declared diagonal coordinate scaling for Jacobian pivot classification and dimensionless Newton solution; physical residuals and steps are unchanged. | [Original authority](https://github.com/rogerlew/openWEPP/blob/b932db101cce07d0860b45b5ecaa8ddb7f455b58/docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001.md#L2073-L2073) | [INFERENCE][Static]; [original rule/context](https://github.com/rogerlew/openWEPP/blob/b932db101cce07d0860b45b5ecaa8ddb7f455b58/docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001.md#L2073-L2073) | Existing coupled domain/closure guards in [ordered solve](nonlinear-solve.md#ordered-numerical-algorithm-active-branches-and-error-precedence) and [typed errors](water-vapor.md#independent-closure-and-errors); [source](https://github.com/rogerlew/openWEPP/blob/b932db101cce07d0860b45b5ecaa8ddb7f455b58/docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001.md#L2073-L2073) | Existing typed domain/convergence/closure failure: [error map](water-vapor.md#independent-closure-and-errors); no partial state or promotion |
| <a id="INV-LANDSURFACEENERGY-113"></a> `INV-LANDSURFACEENERGY-113` | A store-cap-active V10 nonpositive-assimilation wet coordinate below the canonical water-rate tolerance uses the domain-valid inactive anchor defined by `INV-LANDSURFACEENERGY-131` only when its physical energy residual already passes, without changing liquid or energy ledgers. | [Original authority](https://github.com/rogerlew/openWEPP/blob/b932db101cce07d0860b45b5ecaa8ddb7f455b58/docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001.md#L2074-L2074) | [INFERENCE][Static]; [original rule/context](https://github.com/rogerlew/openWEPP/blob/b932db101cce07d0860b45b5ecaa8ddb7f455b58/docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001.md#L2074-L2074) | Existing coupled domain/closure guards in [ordered solve](nonlinear-solve.md#ordered-numerical-algorithm-active-branches-and-error-precedence) and [typed errors](water-vapor.md#independent-closure-and-errors); [source](https://github.com/rogerlew/openWEPP/blob/b932db101cce07d0860b45b5ecaa8ddb7f455b58/docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001.md#L2074-L2074) | Existing typed domain/convergence/closure failure: [error map](water-vapor.md#independent-closure-and-errors); no partial state or promotion |

<a id="canonical-obligations"></a>
## Canonical obligations
| Obligation ID | Statement | Applicability | Authority | Enforcement/failure | Test bindings |
|---|---|---|---|---|---|
| <a id="OBL-LANDSURFACEENERGY-C-017"></a> `OBL-LANDSURFACEENERGY-C-017` | requires every represented-snow ground/soil anchor probe residual vector and dense Jacobian column to equal the complete covered evaluator bit-for-bit for centered and admitted inward stencils. It must prove unchanged minus-then-plus trial construction and domain admission, zero complete constitutive reevaluations for those anchor probes, complete reevaluation for every other coordinate and regime, dependency-invalidating poisons, unchanged first-error precedence, and authentic runner output/count parity. | Represented-snow ground/soil identity-anchor probe consumers; retain the Statement and linked source model/regime, failure and qualification limits | [Original clause](https://github.com/rogerlew/openWEPP/blob/b932db101cce07d0860b45b5ecaa8ddb7f455b58/docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001.md#L529-L536) | Existing local mechanism guards and [coupled error map](water-vapor.md#independent-closure-and-errors); [original obligation](https://github.com/rogerlew/openWEPP/blob/b932db101cce07d0860b45b5ecaa8ddb7f455b58/docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001.md#L529-L536) | [Shared test-vector obligations](interface.md#test-vector-obligations) and [owning detailed requirements](nonlinear-solve.md#stage-3-identity-anchor-jacobian-amendment), including their named fixture/test and real-consumer requirements; [original source](https://github.com/rogerlew/openWEPP/blob/b932db101cce07d0860b45b5ecaa8ddb7f455b58/docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001.md#L529-L536) |
| <a id="OBL-LANDSURFACEENERGY-C-018"></a> `OBL-LANDSURFACEENERGY-C-018` | requires every field of the reused private leaf state, the complete covered evaluation, normalized residuals, frozen branches, and full solve outcome to equal an exhaustive beta-one-call oracle bit-for-bit. It must cover exact beta one, exact-zero-PAR, inactive, positive-PAR beta one ULP below one, centered and inward beta probes, call elimination only for the admitted cases, unchanged call order and typed-error precedence, and authentic runner output/count parity. | Covered leaf dependency-reuse consumers; retain the Statement and linked source model/regime, failure and qualification limits | [Original clause](https://github.com/rogerlew/openWEPP/blob/b932db101cce07d0860b45b5ecaa8ddb7f455b58/docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001.md#L538-L544) | Existing local mechanism guards and [coupled error map](water-vapor.md#independent-closure-and-errors); [original obligation](https://github.com/rogerlew/openWEPP/blob/b932db101cce07d0860b45b5ecaa8ddb7f455b58/docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001.md#L538-L544) | [Shared test-vector obligations](interface.md#test-vector-obligations) and [owning detailed requirements](nonlinear-solve.md#covered-leaf-maximum-demand-exact-reuse-amendment), including their named fixture/test and real-consumer requirements; [original source](https://github.com/rogerlew/openWEPP/blob/b932db101cce07d0860b45b5ecaa8ddb7f455b58/docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001.md#L538-L544) |
