[Parent contract](../SC-LANDSURFACEENERGY-001.md)

## Dependencies
| Target | Required when | Boundary/obligation | Reading extent |
|---|---|---|---|
| interface.md#interface | always | shared authority | whole chapter |
| terminal-support.md#support | physical solve or receipt interval admission, including snow-free work | physical support, zero support and pre-Newton floor | section (entry extent) |
| solve-boundary.md#solve-boundary | solver correctness or implementation | complete ordered physical algorithm and acceptance | whole chapter |
| surface-energy.md#surface-energy | changed evaluator or correctness review | affected radiation/turbulence | whole chapter |
| soil-coupling.md#soil-coupling | changed lower boundary or soil evaluator | regime equations | whole chapter |
| terminal-support.md#terminal-support | represented-snow or terminal solve | regime and support | whole chapter |
| dependency-replay.md#dependency-replay | component-temperature probe reuse | graph/custody/fallibility proof | whole chapter |
| [vapor](water-vapor.md#vapor), [water](water-vapor.md#water), [errors](water-vapor.md#errors), [invariants](water-vapor.md#canonical-invariants), [obligations](water-vapor.md#canonical-obligations) | represented-snow evaluator/error-order requirements review | signed enthalpy, immutable water/ingress and canonical errors; complete water definitions, including all universal P001–004 | section (entry extent) |
| water-vapor.md#water-vapor | active water/ingress implementation or full water-owner audit | complete owner duties | whole chapter |
| ../SC-VEGETATION-001.md#openwepp_c3_woody_v10-nonpositive-assimilation-amendment | V10 gas/evaluator equivalence or full solver correctness | V10 gas branch owner | whole external contract; frozen protocol scope |
| ../SC-VEGETATION-001.md#purpose | cross-contract vegetation scope | vegetation scope | whole external contract; frozen protocol scope |

<a id="nonlinear-solve"></a>
# Nonlinear Solve

A represented-snow reuse correctness review includes both potential and fixed-final solves and the applicability of each V10/V11–13 branch below, together with complete solve-boundary error and acceptance rules.

Current complete ordered covered solver and V10-specific specialization. INV-138 retains the exact admitted closed-bound qualifier; valid-current/one-inadmissible-probe summaries do not broaden that scope. INV-139 accepts only the unchanged current iterate. V10 diagonal scaling remains potential/nonpositive-assimilation-only. Identity-anchor and leaf reuse are distinct limited optimizations; component replay additionally requires dependency-replay and its applicable qualification.

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

<a id="canonical-invariants"></a>
## Canonical invariants
| Invariant ID | Statement | Authority | Evidence | Guard | Failure posture |
|---|---|---|---|---|---|
| <a id="INV-LANDSURFACEENERGY-138"></a> `INV-LANDSURFACEENERGY-138` | Every covered-column Jacobian uses the canonical centered binary64 difference at an interior coordinate and the unique inward one-sided difference only at an exact admitted closed bound; an invalid current iterate or two inadmissible probes rejects. | deterministic numerical-domain authority | `[INFERENCE][Static]` | runtime/test | typed constitutive-domain failure |
| <a id="INV-LANDSURFACEENERGY-139"></a> `INV-LANDSURFACEENERGY-139` | When the current complete covered residual vector passes, the full Newton trial cannot satisfy the existing no-update witness because it is domain-invalid or a governed prospective step exceeds its unchanged threshold, and the first domain-valid halved trial has every governed exact prospective step norm inside those thresholds, the solver accepts the current iterate without installing the trial. | deterministic no-update termination authority | `[INFERENCE][Static]` | runtime/test | accepted current iterate or unchanged strict-decrease/fail-closed path |
| <a id="INV-LANDSURFACEENERGY-131"></a> `INV-LANDSURFACEENERGY-131` | Only a numerically inactive sun-leaf, shade-leaf, or wet-surface temperature coordinate uses `max(T_canopy, 273.15 K)` as its representational anchor; zero physical area and the existing inactive-wet residual predicate guarantee no physical operand or ledger interference. | v31:L1327-L1327 | [INFERENCE][Static] | cold-canopy zero-area phase-domain vector; unchanged active-component residual/closure guards and numerical rejection | cold-canopy zero-area phase-domain vector; unchanged active-component residual/closure guards and numerical rejection |
| <a id="INV-LANDSURFACEENERGY-111"></a> `INV-LANDSURFACEENERGY-111` | V1-to-V2 migration copies every LSE scientific value bit-identically and derives only V2 identity receipts; partial nonpositive-assimilation root supply is unsupported. | v31:L2072-L2072 | [INFERENCE][Static] | [Ordered domain/closure guards](solve-boundary.md#solve) and [typed errors](water-vapor.md#errors) | Typed domain/convergence/closure: [errors](water-vapor.md#errors); no partial state or promotion |
| <a id="INV-LANDSURFACEENERGY-112"></a> `INV-LANDSURFACEENERGY-112` | Only the uncapped active V10 nonpositive-assimilation potential solve uses the declared diagonal coordinate scaling for Jacobian pivot classification and dimensionless Newton solution; physical residuals and steps are unchanged. | v31:L2073-L2073 | [INFERENCE][Static] | [Ordered domain/closure guards](solve-boundary.md#solve) and [typed errors](water-vapor.md#errors) | Typed domain/convergence/closure: [errors](water-vapor.md#errors); no partial state or promotion |
| <a id="INV-LANDSURFACEENERGY-113"></a> `INV-LANDSURFACEENERGY-113` | A store-cap-active V10 nonpositive-assimilation wet coordinate below the canonical water-rate tolerance uses the domain-valid inactive anchor defined by `INV-LANDSURFACEENERGY-131` only when its physical energy residual already passes, without changing liquid or energy ledgers. | v31:L2074-L2074 | [INFERENCE][Static] | [Ordered domain/closure guards](solve-boundary.md#solve) and [typed errors](water-vapor.md#errors) | Typed domain/convergence/closure: [errors](water-vapor.md#errors); no partial state or promotion |

<a id="canonical-obligations"></a>
## Canonical obligations
| Obligation ID | Statement | Applicability | Authority | Enforcement/failure | Test bindings |
|---|---|---|---|---|---|
