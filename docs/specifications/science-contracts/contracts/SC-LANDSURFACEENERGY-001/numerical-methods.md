[Parent contract](../SC-LANDSURFACEENERGY-001.md)

## Dependencies
| Target | Required when | Boundary/obligation | Reading extent |
|---|---|---|---|
| terminal-support.md#support | physical solve or receipt interval admission, including snow-free work | physical support, zero support and pre-Newton floor | section |
| solve-boundary.md#solve-boundary | solver correctness or implementation | complete ordered physical algorithm and acceptance | whole chapter |
| surface-energy.md#surface-energy | changed evaluator or correctness review | affected radiation/turbulence | whole chapter |
| soil-coupling.md#soil-coupling | changed lower boundary or soil evaluator | regime equations | whole chapter |
| terminal-support.md#terminal-support | represented-snow or terminal solve | regime and support | whole chapter |
| dependency-replay.md#dependency-replay | component-temperature probe reuse | graph/custody/fallibility proof | whole chapter |
| [vapor](water-vapor.md#vapor), [water](water-vapor.md#water), [errors](water-vapor.md#errors), [invariants](water-vapor.md#canonical-invariants), [obligations](water-vapor.md#canonical-obligations) | represented-snow evaluator/error-order requirements review | signed enthalpy, immutable water/ingress and canonical errors; complete water definitions, including all universal P001–004 | section |
| water-vapor.md#water-vapor | active water/ingress implementation or full water-owner audit | complete owner duties | whole chapter |
| ../SC-VEGETATION-001.md#openwepp_c3_woody_v10-nonpositive-assimilation-amendment | V10 gas/evaluator equivalence or full solver correctness | V10 gas branch owner | whole external contract; frozen protocol scope |
| ../SC-VEGETATION-001.md#purpose | cross-contract vegetation scope | vegetation scope | whole external contract; frozen protocol scope |
| nonlinear-solve.md#nonlinear-solve | numerical-method correctness or accepted-solve admission | complete V2 ownership/FullSupply, inactive coordinates, no-update refusals, diagnostics and tests | whole chapter |
| audit-details.md#numerical-presentation-reference | original numerical wording, source interpretation or provenance adjudication | retained normative V11/V13/V2 presentation; complete operative rules are in methods and required admission | section |

<a id="nonlinear-solve"></a>
# Current numerical methods

Solver correctness (including represented-snow replay, potential and fixed-final)
requires this whole chapter, whole admission and solve-boundary. All numerical,
owner, refusal, diagnostic and test duties apply. Original wording remains
normative in the reference; uncertainty expands reading.

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

<a id="version-33-experimental-residual-jacobian-representation"></a>
## Version 33 experimental residual/Jacobian representation

This amendment authorizes one isolated, non-production experiment that replaces
canonical finite-difference columns with explicit derivatives of the *same*
covered-column numerical residual. It does not activate a production algorithm.
The mathematical target is

```text
r(x; u, b0) = R_raw(x; u, b0) / s(x; u, b0),
J_ij = partial r_i / partial x_j at the admitted base (x, u, b0).
```

Here `x` has exactly the existing order `10*N+3+S`: each occupancy contributes
`psi_sun, psi_shade, psi_stem, psi_root, beta_sun, beta_shade, T_sun,
T_shade, T_wet, T_stem`, followed by `T_canopy, q_canopy, T_ground` and the `S`
soil temperatures. Residual rows have that same occupancy order, followed by
shared canopy heat, shared canopy vapor, ground energy and soil energy rows.
Hydraulic rows are normalized water-rate residuals, component/shared/ground/soil
rows are their existing normalized energy or vapor residuals, and every unit,
row order and coordinate unit scale remains as declared by the complete solve
boundary. `u` is the complete already validated input/configuration/forcing /
support/owner state. `b0` is only the existing sweep-base frozen root, wet and
ground water-branch data. Gas branches, longwave, turbulence, tolerances and all
other within-branch values remain functions of `x`; they are not frozen.

### Initial block and affected rows

The first supported block is every active dry-stem temperature coordinate
`x_(10*o+9)=T_stem,o`, for arbitrary validated `N` and `S`. At a fixed smooth
base its complete affected row set is:

1. every positive-area component energy row in occupancy `i != o` through
   reciprocal longwave and its tolerance: upward propagation for `i<o` and
   downward propagation for `i>o`;
2. its own stem row through direct fourth-power emission, sensible heat and its
   tolerance; same-layer sun/shade/wet rows are exact zeros because neither
   incident boundary contains layer `o` emission and their direct terms do not
   read `T_stem,o`;
3. the shared canopy-heat row through stem sensible heat and its tolerance; and
4. the ordinary non-represented-snow ground-energy row through reciprocal
   longwave and its tolerance. Represented-snow ground and soil rows retain the
   exact identity-anchor authority; no soil row is otherwise stem-dependent.

Every other row is an independently proved exact zero for this block. Existing
validation rejects nonfinite executed
`dry_stem=(1-wet_fraction)*stem_area` before applicability. A target with exact
zero executed `dry_stem` retains canonical identity handling and is not
derivative-treated. A zero-area receiving component has its existing
identity row and an exact-zero derivative. A static mask is
only an assembly consequence of this complete dependency statement; it is not
permission to omit a reachable row. In particular, adjacency, occupancy, or
the component's own equation never bounds the longwave dependency.

For every normalized row `r_i=R_i/s_i`, with the existing positive finite
normalizer `s_i`, the implementation must evaluate

```text
dr_i/dT = (dR_i/dT)/s_i - R_i*(ds_i/dT)/(s_i*s_i).
```

The second term is mandatory. With
`energy_tolerance(z)=a+b*max(z,1 W m^-2)`, the exact rules are:

| Row | `dR/dT_stem,o` | `z` and `dz/dT_stem,o` |
|---|---|---|
| positive-area component, `i != o` | propagated `dLW` | `z=abs(SW)+abs(LW)+abs(H)+abs(LE)`; only `LW` varies, so `dz=sign(LW)*dLW` when nonzero |
| own stem | `dLW_direct-dH_stem` | `z=abs(SW)+abs(LW)+abs(H)`; `dz=sign(LW)*dLW_direct+sign(H)*dH_stem` for nonzero varying operands |
| own sun/shade/wet or zero-area receiver | exact zero | no operand varies; structurally constant zeros contribute zero |
| shared heat | `dH_stem` | let `y=abs(H_canopy)+abs(H_ground)+abs(H_reference)` and `z=max(y,1)`; only `H_canopy` varies |
| ordinary ground | propagated `dLW_ground` | existing absolute surface-operand sum; only longwave varies |

For the shared-heat row, `ds=0` when `y<1`, `ds=b*dy` when `y>1`, and exact
`y=1` is unsupported. For rows without this floor, `ds=b*dz`. A
derivative-active absolute operand at zero is unsupported; a
structurally constant zero contributes zero. The baseline-only oracle and
directional probes below certify the local interval: executed branch IDs,
positive target `dry_stem`, shared-heat `y<1`/`y>1` classification, and signs of all
derivative-active absolute operands must match the base at every sample.
Otherwise the column is structurally unsupported before candidate execution
and canonical FD is selected.

### Branch, domain and lifecycle policy

A private, immutable, non-Clone same-base derivative capability binds the exact
validated topology, `u`, trial `x`, `b0`, primal evaluation, row/coordinate
order and normalization operands. It is valid for one Jacobian assembly only.
Stale, mutated, foreign, differently ordered or differently frozen input is a
typed integrity failure; after derivative execution begins there is no FD retry,
alternate solver or suppressed error. Structural inapplicability is decided
after unchanged current-trial/base-evaluator guards and before derivative entry
from finite interior stem temperature, finite positive executed `dry_stem`, and
the frozen branch/sign/floor mask. It uses canonical centered FD. Exact closed
stem bounds are FD-only and retain the existing inward rule. Invalid/nonfinite
current trials reject before applicability and are never fallback eligibility.

| Ordered v33 guard | Disposition / typed error |
|---|---|
| existing input, current-trial and base-evaluator validation | unchanged source-real `LandSurfaceEnergyError`; no capability minted |
| structural area/bound/branch/sign/floor inapplicability | canonical FD before derivative entry; not an error |
| capability generation/topology/order/trial/frozen-base mismatch, foreign or second use | new `LandSurfaceEnergyError::CoveredJacobianDerivativeIntegrity(&'static str)`; discard the partial Jacobian |
| nonfinite tangent, zero/nonfinite normalizer, overflow or affected-mask inconsistency | same new variant with stable reason token; discard the partial Jacobian |
| existing fallible canonical calculation reached after entry | propagate its original typed error in source order |

Applicability is pure and read-only over validated records and the baseline mask.
After entry no error may execute FD, publish a partial column/matrix, mutate the
base, or continue to LU.

The analytic formulas differentiate the declared executed smooth algebra at the
matched base. No derivative crosses the safeguarded iterative leaf `ci` solve,
wet cap/routing/finalization, phase boundary, closed-domain endpoint, min/max
tie, exact-zero operand or physical branch in the initial block. The sole
eligible expansion in this experiment is the paired active sun/shade
leaf-temperature block. It requires, before implementation, a separately
reviewed implicit derivative of each defining converged leaf equation, a finite
inner-solve residual/error contribution, gas-branch and VPD/Medlyn domain
admission, and updated affected-row/oracle bindings. Differentiating the finite
iteration program is not admitted. Wet-temperature/routing, hydraulic/beta,
shared-air, ground and soil expansion is outside this amendment.

The existing outer nonlinear controller, dense storage, LU/pivot rule,
coordinate scaling applicability, line search, iteration/support budget,
stopping thresholds, event/adaptive policy and accepted-result construction are
unchanged. A new trajectory may visit different lawful states/active sets; each
new Jacobian base must mint its own capability using its current predicates and
`b0`. The experiment is a single hybrid assembly: exact identity columns where
already authorized, admitted v33 derivative columns, and canonical FD columns
elsewhere. It is never a recovery cascade and has no history/topology hardcode.

### Independent oracle and numeric acceptance

Before candidate comparison, freeze authentic reconstructible sweep states with
their exact `u`, `x`, `b0`, primal evaluation and lifecycle. The oracle calls
only the canonical primal evaluator and independently enumerates affected rows.
For each interior candidate column use centered steps `h_k=2^k*h0`, `k=-4..4`,
where `h0=sqrt(binary64 epsilon)*max(abs(T_stem),1 K)`, rejecting samples that
change a physical branch or the base sign/floor mask. Let `D_k` be the exact
`(r(x+h_k e_j)-r(x-h_k e_j))/(2h_k)` in source minus-then-plus order. For each
consecutive triple `(k-2,k-1,k)`, set
`d_f=abs(D_(k-2)-D_(k-1))` and `d_c=abs(D_(k-1)-D_k)`. It is a basin exactly
when all samples are admitted, `d_f>0`, and `2.5<=d_c/d_f<=5.5`. Choose the
basin minimizing `d_f`; ties use smallest `abs(k-2)`, then smaller `k-2`. With
`q=k-2`, define `J_oracle=(4*D_q-D_(q+1))/3` and
`U=max(d_f/3,64*epsilon*max(1,abs(r_i))/h_q,2e-9 K^-1)`. No basin in baseline A
freezes that entry unsupported before J exists.
The candidate passes an entry only when

```text
|J_candidate-J_oracle| <= 8*U + 2e-7 K^-1
                            + 5e-5*max(|J_candidate|,|J_oracle|).
```

The `64 epsilon/h` term covers subtraction roundoff at the observed residual
scale; the eight-uncertainty multiplier covers the two-sided estimate and basin
selection; `2e-7 K^-1` is below the smallest normalized change resolvable by the
canonical `h0` sweep while avoiding a false exact-zero claim; and `5e-5` limits
scale-relative derivative error to 0.005%, far below the existing unit-order
normalized convergence surface. These constants are frozen before J outcomes
and may not be widened post hoc. Every baseline-admitted disagreement fails and
cannot be reclassified unsupported. Oracle self-tests cover an exact smooth
quadratic/cubic derivative, roundoff-dominated constant, kink-crossing absolute
function, and deliberately wrong candidate derivative.

Independent directional checks use eight vectors cyclically over N stem
coordinates, each with infinity norm one and units K: `e0`, `-e0`, `e1`, `-e1`,
all `+1`, alternating `(+1,-1,...)`, `e0+e1`, and `e0-e1`. For `N=1`, map
`e1` to `e0`; map each two-coordinate expression to `+e0` or `-e0` according
to its first nonzero signed term, so every resulting trial remains norm one;
retain all eight as separately identified repeated trials. Set
`t0=min(0.25 K,0.25*min_j(T_j-200 K,350 K-T_j))` over nonzero entries and use six
halvings. Per normalized row,
`E_m=abs(r(x+t_m v)-r(x)-t_m*(Jv))`, `t_m=t0/2^m`, and
`F_m=128*epsilon*max(1,abs(r_i),abs(t_m*(Jv)_i))`. Before `E_m<=F_m`, two
consecutive `E_m/E_(m+1)` ratios must lie in `[3.2,4.8]`; no ratio is required
after the first floor hit. Baseline branch/sign/floor mismatch freezes the
direction unsupported; an admitted candidate failure fails. Elementary stem
sensible-heat and reciprocal Stefan-Boltzmann fourth-power derivatives provide independent
sign/unit checks. Every plausible cross-occupancy longwave edge receives a
perturbation that would expose its omission.

Same-state ordinary primal residuals and matched-input validator/error precedence
remain bit-identical. Full-solve continuous values use the symmetric table
below. Each predicate uses the minimum of its comparison bound and one tenth of
an applicable canonical residual/closure tolerance. Unlisted continuous fields
must match exactly or are ineligible for an equivalence claim.

| Field class | Units and basis | Symmetric A/J comparison bound |
|---|---|---|
| component/canopy/ground/soil temperatures | K, existing tile/OFE basis | `2e-8 K + 2e-10*max(abs(A),abs(J))` |
| component/shared/ground/soil energy fluxes | W m^-2, declared tile or ground basis | `1e-7 W m^-2 + 1e-11*max(abs(A),abs(J))` |
| vapor/root/liquid water rates | kg m^-2 s^-1, declared tile or ground basis | `1e-13 kg m^-2 s^-1 + 1e-10*max(abs(A),abs(J))` |
| integrated energy operands/closure | J m^-2, declared interval/area basis | energy-flux bound times exact shared interval; independent A0 closure also passes |
| integrated water/storage operands/closure | kg m^-2, declared interval/area basis | water-rate bound times exact shared interval; independent A0 closure also passes |

The energy/water terms are one tenth of the canonical absolute and relative
residual tolerances. Temperature is further tightened by any applicable Stage-3
`1e-8 K` identity-anchor residual. No stopping/closure threshold changes. Owner IDs,
topology/order/cardinality, exact-one transfers, rollback, receipt schema and
within-run producer-to-consumer payload transfer remain exact. Cross-algorithm
payload-derived digests may differ only where their authenticated continuous
payload differs within the preceding bounds. Iterations, probes, backtracking
and numerical diagnostics are observations rather than golden trajectories.

### Cost and experiment lifecycle

The charged local denominator is the complete affected-column obligation for
all admitted stem columns at the same bases. Named monotonic counters remain
separate: `complete_residual_calls`, `leaf_current_calls`, `leaf_maximum_calls`,
`wet_flux_calls`, `longwave_primal_calls`, `longwave_tangent_calls`,
`normalization_tangent_rows`, and `stem_columns_assembled`. The tenfold rule is
`A.complete_residual_calls/J.complete_residual_calls >= 10` for the identical
stem obligation; zero J calls report A's eliminated count, not infinity. Nested
counters are never summed or weighted. Validation, propagation, assembly and
scratch have no invented work unit and are charged by exclusive elapsed time.
Local admission also requires a paired release-mode median block-time ratio at most `0.50`,
with the upper endpoint of a descriptive paired-bootstrap 95% interval below
`0.67`. Use at least two warmups and 30 paired batches, each batch long enough
for at least 100 ms per arm. Report work counts separately from time.

Exclusive stack-top spans are `base_primal`, `jacobian_identity`,
`jacobian_fd_stem`, `jacobian_v33_stem`, `jacobian_fd_other`, `linear_solve`,
`line_search`, and `remaining_solver`, assigning every instant once. In A,
`jacobian_fd_stem` is the time spent constructing exactly the stem columns that
J would support; in J this span is zero and their replacement is charged to
`jacobian_v33_stem`. Baseline modifiable whole-run fraction
`f=sum(A.jacobian_fd_stem)/sum(A.complete_runner)` uses predeclared frozen-corpus
class multiplicities. Broad-coverage fraction is
`sum(A FD-column construction time for columns supported by J) /
 sum(A time constructing all non-identity Jacobian columns)` on those same
weighted records. Overlapping spans and equal-case reweighting are forbidden.
The broad-coverage end-to-end priority criterion remains 30% median wall saving
only after the derivative representation covers at least 75% of baseline
Jacobian-construction wall time on the frozen authentic one-OFE corpus. It uses
12 balanced fresh-process pairs, two warmups per executable, an interval
supporting gain, and candidate median CPU ratio at most `1.03`. Material memory
non-regression means candidate process HWM and matched sampled-active peak are
each no more than `max(4 MiB, 5%)` above A and no new monotone post-drop growth
across the declared ten-run lifetimes. These are engineering priority rules,
not production qualification or scientific tolerances.

This authority expires for implementation use at package disposition. Retained
source/evidence remains historical research custody; no selector or alternate
solver may remain production reachable. `CALIBRATION_NOT_APPLICABLE`: no
physical parameter, equation, constant, unit, output or observation mapping is
introduced.

## V10 coordinate scaling

#### V10 unit scaling

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

<a id="canonical-invariants"></a>
## Canonical invariants
| Invariant ID | Statement | Authority | Evidence | Guard | Failure posture |
|---|---|---|---|---|---|
| <a id="INV-LANDSURFACEENERGY-138"></a> `INV-LANDSURFACEENERGY-138` | Every covered-column Jacobian uses the canonical centered binary64 difference at an interior coordinate and the unique inward one-sided difference only at an exact admitted closed bound; an invalid current iterate or two inadmissible probes rejects. | deterministic numerical-domain authority | `[INFERENCE][Static]` | runtime/test | typed constitutive-domain failure |
| <a id="INV-LANDSURFACEENERGY-165"></a> `INV-LANDSURFACEENERGY-165` | In the isolated v33 experiment, every structurally admitted active dry-stem temperature column is the derivative of the actual normalized canonical residual `r(x;u,b0)`, including all reciprocal-longwave cross-occupancy rows, local/shared sensible heat, ordinary ground coupling, and x-dependent normalization; same-state primal science and all non-Jacobian controller/custody semantics remain unchanged. | `INV-LANDSURFACEENERGY-010/032/108/138/154/162/164` plus smooth analytic differentiation of the declared residual | `[INFERENCE][Static]` | private same-base capability, prospective smooth-branch admission, independent multistep FD/Taylor/elementary oracle, real-consumer and closure gates | typed integrity failure after entry; canonical FD chosen only for pre-entry structural inapplicability; any oracle/physics/custody failure is hard experimental `HOLD`; never production activation |
| <a id="INV-LANDSURFACEENERGY-139"></a> `INV-LANDSURFACEENERGY-139` | When the current complete covered residual vector passes, the full Newton trial cannot satisfy the existing no-update witness because it is domain-invalid or a governed prospective step exceeds its unchanged threshold, and the first domain-valid halved trial has every governed exact prospective step norm inside those thresholds, the solver accepts the current iterate without installing the trial. | deterministic no-update termination authority | `[INFERENCE][Static]` | runtime/test | accepted current iterate or unchanged strict-decrease/fail-closed path |
| <a id="INV-LANDSURFACEENERGY-131"></a> `INV-LANDSURFACEENERGY-131` | Only a numerically inactive sun-leaf, shade-leaf, or wet-surface temperature coordinate uses `max(T_canopy, 273.15 K)` as its representational anchor; zero physical area and the existing inactive-wet residual predicate guarantee no physical operand or ledger interference. | v31:L1327-L1327 | [INFERENCE][Static] | cold-canopy zero-area phase-domain vector; unchanged active-component residual/closure guards and numerical rejection | cold-canopy zero-area phase-domain vector; unchanged active-component residual/closure guards and numerical rejection |
| <a id="INV-LANDSURFACEENERGY-111"></a> `INV-LANDSURFACEENERGY-111` | V1-to-V2 migration copies every LSE scientific value bit-identically and derives only V2 identity receipts; partial nonpositive-assimilation root supply is unsupported. | v31:L2072-L2072 | [INFERENCE][Static] | [Solve guards](solve-boundary.md#solve) and [typed errors](water-vapor.md#errors) | [Typed failures](water-vapor.md#errors); no partial state or promotion |
| <a id="INV-LANDSURFACEENERGY-112"></a> `INV-LANDSURFACEENERGY-112` | Only the uncapped active V10 nonpositive-assimilation potential solve uses the declared diagonal coordinate scaling for Jacobian pivot classification and dimensionless Newton solution; physical residuals and steps are unchanged. | v31:L2073-L2073 | [INFERENCE][Static] | [Solve guards](solve-boundary.md#solve) and [typed errors](water-vapor.md#errors) | [Typed failures](water-vapor.md#errors); no partial state or promotion |
| <a id="INV-LANDSURFACEENERGY-113"></a> `INV-LANDSURFACEENERGY-113` | A store-cap-active V10 nonpositive-assimilation wet coordinate below the canonical water-rate tolerance uses the domain-valid inactive anchor defined by `INV-LANDSURFACEENERGY-131` only when its physical energy residual already passes, without changing liquid or energy ledgers. | v31:L2074-L2074 | [INFERENCE][Static] | [Solve guards](solve-boundary.md#solve) and [typed errors](water-vapor.md#errors) | [Typed failures](water-vapor.md#errors); no partial state or promotion |

<a id="canonical-obligations"></a>
## Canonical obligations
| Obligation ID | Statement | Applicability | Authority | Enforcement/failure | Test bindings |
|---|---|---|---|---|---|
| <a id="OBL-LANDSURFACEENERGY-C-021"></a> `OBL-LANDSURFACEENERGY-C-021` | Bind arbitrary-cardinality active dry-stem columns to the complete affected-row map and derivative-through-normalization; prove bit-identical same-state primal/error precedence, independent nine-scale FD basins, eight feasible directional Taylor checks, elementary sign/unit formulas, zero/sparsity claims, smooth/inapplicable/closed/branch/tie/nonfinite/stale/foreign/mutated/second-use negatives, hybrid no-recovery assembly, full potential/final solves, actual one-OFE consumer participation, independent accepted-output water/energy/mass closure and exact within-run custody. Freeze authentic early/late potential/final and boundary/transition corpus before J comparison. Charge all removed and replacement work. Admit local architecture only at >=10x complete-evaluation/leaf-solve reduction and the frozen local timing bound; broader 30% end-to-end adjudication only after >=75% Jacobian-time coverage. | Isolated v33 residual/Jacobian prototype only | `INV-LANDSURFACEENERGY-165` | typed tests, independent oracle, authenticated counters/receipts, controlled measurement and dual review; unresolved mandatory evidence is `HOLD` | package `20260908-stage3-residual-jacobian-prototype-001`; contract-derived unit/integration tests and detached real-consumer evidence |
