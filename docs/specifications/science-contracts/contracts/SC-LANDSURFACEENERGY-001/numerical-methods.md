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

<a id="liquid-canopy-temperature-roles"></a>
The [owning vegetation support clarification](../SC-VEGETATION-001.md#liquid-canopy-supported-domain-clarification) applies here.
For this existing component-temperature domain, active leaf and ordinary wet surface temperatures are distinct from reference air, shared canopy air, dry stems, ground snow, and zero-area numerical anchors. The anchor rule in `INV-LANDSURFACEENERGY-131` remains a zero-area representational rule only. An invalid initial coupled coordinate retains `LandSurfaceEnergyError::ConstitutiveDomain("covered_initial_trial")` before Jacobian work; an invalid liquid-saturation operand retains `LandSurfaceEnergyError::ConstitutiveDomain("liquid_saturation_polynomial")`; a rejected trial retains existing trial/line-search handling. These `Domain` seams are not interchangeable with the ordinary-release unsupported-domain seam and do not classify air temperature or select ice, supercooling, a solver, or a fallback.

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

<a id="cold-canopy-m1-trust-region-experiment"></a>
## COLD-CANOPY-M1 bounded trust-region experiment

This section authorizes only the isolated diagnostic experiment named
`COLD-CANOPY-M1-TR-SVD-BVLS-01`, at the source and input identities frozen by
the adopted COLD-CANOPY-M1 experiment authorization.  It expires when that
experiment receives a bounded disposition.  It is neither a production solver
amendment nor a physical-regime selector, and it cannot publish, advance, or
retain an owner state.  Its result can only be the stated local positive,
negative, or invalid measurement; it cannot establish M1 acceptance or global
physical infeasibility.

At each admitted base `x_k`, retain the complete ordered 21-coordinate M1
residual.  Let `R_k` be the **raw** vector, `t_k` the positive finite 21-entry
normalizer vector returned by the existing `m1_tolerances` evaluation at that
same base, and `Jraw_k` the raw Jacobian returned by the already authorized
phase-selected assembly.  Define

```text
W_k = diag(1/t_k),       f_k = W_k R_k,
A_k = W_k Jraw_k S,      Phi_k(x) = 0.5 ||W_k R_raw(x)||_2^2.
```

`S` is diagonal in the existing coordinate order.  Its entries are `1 K` for
the fourteen temperature coordinates `0,1,2,6,7,8,12,14..20`,
`1e-2 kg m^-2` for M coordinates `3,9`, `1e3 J m^-2` for H coordinates `4,10`,
`1e-5 kg m^-2 s^-1` for D coordinates `5,11`, and `1e-3 kg kg^-1` for
`qcan` coordinate `13`.  The trust subproblem is exactly

```text
minimize 0.5 ||f_k + A_k p||_2^2
subject to x_lower <= x_k + S p <= x_upper,  ||p||_2 <= Delta.
```

The exact 21-coordinate box is
`[263.15,373.15] K` at `0,1,2,6,7,8`; unboxed at M/H coordinates `3,4,9,10`;
`[0,+infinity) kg m^-2 s^-1` at `5,11`; `[258.15,373.15] K` at `12`;
`[0,0.1] kg kg^-1` at `13`; and unboxed at ground/soil `14..20`.  Unboxed is
not unconstrained physical admission: M/H, ground and soil retain the original
phase inversion, capacity, saturation, hydraulic and owning domain predicates
on every base, probe and candidate.

The raw Jacobian is required here.  `normalized_jacobian` differentiates the
state-dependent normalizer and therefore cannot be divided, multiplied, or
otherwise transformed to recover `Jraw_k`.  The existing assembly already
returns both matrices from the same probes; this experiment uses that returned
raw matrix and `t_k`, with no third assembly or derivative reconstruction.
`W_k` remains frozen for predicted and actual reduction.  The existing
recomputed normalized residuals remain the sole physical root-acceptance
surface and are not this merit function.

First assemble the natural current-state tangent and solve this experiment's
trust subproblem to obtain its **predictor** `p_p`; the old Newton linear
direction is not computed or retained as a predictor.  Select the phase and
capacity tangent from `S*p_p` by the existing exact join rules.  If that set
differs from the natural set, reassemble exactly once with the selected tangent
and solve this same trust subproblem once more for the final `p`; otherwise
`p=p_p`.  The final phase/capacity normals from `S*p` must select that same
set, and the existing selected-side representable probe must pass before a
candidate is considered.  A nonfinite normal, a selection disagreement, or
the owning selected-side domain error retains its existing typed failure with
no third assembly, phase loop, candidate retry, radius shrink, or alternate
solver.  Thus a base has at most two assemblies and 22 subproblem solutions:
one natural predictor, one selected-tangent replacement when needed, and at
most 20 later radius proposals using that same selected tangent.  At most 21
of these are candidate proposals.

Notwithstanding the production selected-drainage affine-elimination
representation below, that representation is superseded solely within this
expiring diagnostic arm: both D coordinates and every one of their raw coupling
columns remain in the full problem, the selected capacity row remains its
selected raw affine row, and `D>=0` is its ordinary coordinate bound.  No
solve-then-overwrite, derived drainage coordinate, or post-solve clipping is
permitted.  The experiment must prove radius-limited D steps are not forced to
`dD=-D`, retain both D coupling columns, and cover drainage/capacity ties.  This
does not amend the primal identity or production authorization.

### Deterministic bounded subproblem

Use the following one bounded-variable active-set algorithm for every proposed
step.  It is not dogleg, scalar damping, a line search, or a fallback.

1. Set `Delta=1` at the first base, with `Delta_min=2^-20` and
   `Delta_max=8`; set `p=0` and make every non-fixed coordinate free.  A
   coordinate exactly on a physical bound is still free and may only move
   inward.  Every radius proposal restarts from this exact `p=0` face.  No
   library feasibility nudge is allowed.
2. On a face, hold its active coordinates at their present exact `p` values.
   Form the free residual after those fixed contributions in ascending
   coordinate order.  Compute a one-sided Jacobi SVD of the resulting dense
   `A_free`: begin with `V=I`; visit column pairs `(i,j)` lexicographically in
   each sweep; rotate a pair only when
   `abs(dot(a_i,a_j)) > 2^-48*sqrt(dot(a_i,a_i)*dot(a_j,a_j))`; use the
   following exact binary64 rotation.  With `alpha=dot(a_i,a_i)`,
   `beta=dot(a_j,a_j)`, and `gamma=dot(a_i,a_j)`, set
   `tau=(beta-alpha)/(2*gamma)`,
   `t=sign(tau)/(abs(tau)+hypot(1,tau))`, taking `t=1` when `tau=0`, then
   `c=1/hypot(1,t)` and `s=c*t`; apply
   `(a_i,a_j)=(c*a_i-s*a_j,s*a_i+c*a_j)` and the identical rotation to columns
   of `V`.  A nonfinite intermediate is a refusal.  Run at most 64 sweeps and
   require one complete no-rotation sweep.  Sort singular values descending,
   breaking equal binary64 values by their original free-coordinate index.
   Any nonfinite operand, missing convergence, or
   `sigma_min <= 2^-40*sigma_max` is a typed subproblem refusal.
3. First form `r2=Delta*Delta-sum_active(p_i*p_i)` in ascending active-index
   order.  A nonfinite or negative `r2` is
   `TrustRegionDegenerateFace`; it is never clamped to zero.  With no free
   coordinate, return `TrustRegionDegenerateFace` before SVD; never return an
   unchecked active corner.  With a free coordinate and exact `r2=0`, return
   `TrustRegionDegenerateFace`; lambda is deliberately undefined for that face.
   Otherwise set `r=sqrt(r2)` and,
   with those SVD factors, solve the free trust-region least-squares step by
   `p_free(lambda)=-V diag(sigma_j/(sigma_j^2+lambda)) U^T y`, where
   `y=f_k+A_active*p_active` and `U_j=A_free*V_j/sigma_j`.  It is evaluated
   directly from singular values, never by forming `A^T A`.  Use `lambda=0`
   when its norm fits the remaining radius.  Otherwise begin at `1`, multiply
   by four at most 48 times until the norm fits, then bisect that bracket 48
   times; return the feasible upper-lambda endpoint.  For positive remaining
   radius `r`, require `||p_free||<=r` and
   `r-||p_free|| <= max(2^-40*r,64*epsilon*max(1,r))`.  A nonfinite value,
   absent bracket, or failed ball-accuracy check
   is a typed refusal.  The active coordinates contribute to the radius
   exactly; no free-coordinate radius is substituted.
4. If the face solution crosses a box bound, move from the present `p` to the
   first crossing by the smallest finite `theta` in `[0,1]`; exact equal
   `theta` values activate the smallest coordinate index first.  Keep that
   coordinate at the reached bound and resolve the new face.  If it is box
   feasible, compute `g=A_k^T(f_k+A_kp)` and `h=g+lambda*p`.  Release the
   active lower bound with `h_i<0`, or active upper bound with `h_i>0`, having
   largest absolute violation; exact ties use the lowest coordinate index.
   Otherwise return this `p`.  At most 42 face pivots occur in one proposal;
   reaching the cap is a typed refusal.

The initial radius, every radius change, face start, pair order, SVD tolerance,
rank threshold, bracket, bisection count and tie rule are frozen above.
No random ordering, robust loss, rank repair, regularization retry, or hidden
inner iteration is allowed.

All subproblem and merit arithmetic is ordered binary64: form each `W_k*R_i`
and `(W_k*Jraw_ij)*S_j` in ascending row/column order; form every SVD dot,
norm, `A_free*V`, `U^T*y`, `A*p`, residual, `r`, `g`, `h`, and squared norm by
ascending index accumulation from `0`; and form `pred`, `ared`, and `rho` in
the written order.  No fused multiply-add, reassociation, compensated/reduction
tree, or overflow-to-finite replacement is allowed.  A nonfinite operand or
intermediate is a typed numerical refusal before candidate acceptance.

For a finite candidate, set
`pred=0.5*(||f_k||_2^2-||f_k+A_kp||_2^2)` and
`ared=Phi_k(x_k)-Phi_k(x_k+Sp)`.  It is accepted as an update only if all
unchanged domain, phase, capacity, hydraulic, finite and full-trial predicates
pass, `pred>0`, `ared` is finite, `rho=ared/pred>=0.1`, and
the existing recomputed maximum normalized residual strictly decreases.  On a
rejection, including `TrustRegionNoPredictedReduction`, retain the same base,
raw Jacobian, `t_k`, and phase selection; shrink `Delta` by `0.25`.  After an
accepted update, shrink by `0.25` when
`rho<0.25`, enlarge to `min(2*Delta,8)` when `rho>0.75` and
`||p||_2>=0.99*Delta`, otherwise retain it.  A proposed radius below
`Delta_min`, or 21 proposals at one base, is a typed radius-exhaustion result.
There are at most 50 accepted nonlinear updates.  Small step, constrained
stationarity, small gradient, lower merit, or radius exhaustion cannot accept
a root.  Only all 21 existing normalized residual predicates, raw residual
checks, hydraulic blocks, materialization, independent closure, and failure
atomicity can do so.

At the top of every post-update iteration, before a new assembly, materialize
the current base exactly when all 21 dynamically normalized residuals pass and
the immediately preceding installed candidate has complete unchanged governed
step norms and every unchanged governed step threshold passes.  Separately,
after assembly and also at an initial base, apply the
prospective no-update witness.  If the current residuals pass and the complete
first proposal has complete governed step norms and every unchanged governed
step threshold passes, materialize the current base without installing that
proposal.  Only when that full proposal is
pre-evaluator-domain-invalid or has governed-step excess may the first later
pre-evaluator-domain-valid proposal be its replacement witness; that proposal
must receive complete evaluation and have complete governed step norms and
every unchanged governed step threshold passes before it materializes the
current base.  The first domain-valid proposal is never skipped or substituted.
Governed step thresholds are not required for an
ordinary non-root update, only for these existing no-update/root admission
paths.  Existing
base/input, phase, saturation, M/H, capacity, hydraulic, nonfinite, and
selected-side errors remain terminal in their original precedence and never
shrink a radius.  A candidate refused solely by its original trial-domain
predicates is a recorded domain-refused proposal and shrinks its radius; a
candidate error that is not an ordinary trial-domain refusal is terminal.  No
rejection changes inputs, normalizers, bounds, seed, selected set, or an owner
state.

Every experimental numerical refusal is `LSEB-E-034` with one stable kind and
reason: `TrustRegionSvdNonFinite`, `TrustRegionSvdNoConvergence`,
`TrustRegionRankDeficient`, `TrustRegionLambdaBracket`,
`TrustRegionBallAccuracy`, `TrustRegionFacePivotLimit`,
`TrustRegionRadiusExhausted`, `TrustRegionWorkCap`, or
`TrustRegionDegenerateFace`.  `TrustRegionNoPredictedReduction` is a
nonterminal recorded proposal-rejection reason; it returns no `LSEB-E-034` at
that point and shrinks radius.  A later terminal record retains prior proposal
reasons.  Existing `PhaseActiveSetInconsistent`,
`PhaseActiveSetUnrepresentable`, `VEG-E-141`, `VEG-E-142`, hydraulic, and
materialization errors retain their owning kinds and precede these new kinds.
Every refusal records the nonlinear iteration, proposal ordinal, natural and
selected phase/capacity sets, radius, active lower/upper/free coordinate masks,
raw and dynamically normalized residual maxima when available, predicted and
actual reductions when available, and all monotonic work counters.  Missing
values are null only when their owning evaluation did not occur.

### COLD-CANOPY-M1-TR-SVD-BVLS-02 finite-precision active-face amendment

**Status: ACCEPTED / premeasurement diagnostic authority only.** Distinct
independent correctness and QA review accepted its exact formula, operation
model, scope, and controls before body work. It supersedes the
exact-sign release paragraph of `COLD-CANOPY-M1-TR-SVD-BVLS-01` only in the
revised detached treatment build. The `01` source, wording, controls, result,
and its `FacePivotLimit` observation remain historical evidence; they are not
relabelled as a defective implementation. All other `01` rules in this section
remain unchanged unless this amendment states otherwise. This is one revised
finite-precision policy from the first face of every subproblem, never an
old-policy attempt followed by a rescue policy.

For the revised policy only, the exhaustive `01` `LSEB-E-034` taxonomy is
extended by `TrustRegionOptimalityIndeterminate`; the historical `01` wording
and its recorded refusal remain unchanged.

The primal problem, exact box/radius feasibility, 21-coordinate order,
binary64 SVD, pair/sweep/rank/ball/lambda rules, frozen raw-J and merit rules,
phase rules, work caps, nonlinear acceptance, governed-step predicates,
materialization, physical residuals, and typed physical-domain failures are
unchanged. It adds no factorization, evaluation, precision mode, regularizer,
pivot expansion, clipping, fallback, or physical/model uncertainty term. The
only added work is the bounded scalar/vector optimality assessment below.

Let `u=2^-53` be binary64 unit roundoff and
`gamma_q=q*u/(1-q*u)`. This amendment uses `q=87`; `87*u<1`. Its implemented
coefficient is the upward-rounded binary64 value
`Gamma87=upward(87/(2^53-87))`, whose positive denominator is the exact
rewriting of `1-87*u`. At a finite face
solution returned by the unchanged SVD/ball procedure, use the exact stored
binary64 operands `A`, `f`, `p`, and nonnegative `lambda` and retain the
existing ordered calculation

```text
r_hat = fl(f + A*p)
g_hat = fl(A^T*r_hat)
h_hat = fl(g_hat + fl(lambda*p)).
```

Here every product and addition is separately rounded binary64, all sums begin
at zero and accumulate ascending index, and no FMA, reassociation,
compensation, reduction tree, extended precision, or overflow-to-finite
substitution is allowed. For coordinate `i`, define the nonnegative exact
operand scale

```text
C_i = sum_r |A_ri| ( |f_r| + sum_j |A_rj| |p_j| ) + |lambda| |p_i|,
E_i = gamma_87 C_i.
```

There are 43 rounded operations in each residual component (21 products, 21
accumulations, and the addition of `f`), 42 in each gradient component (21
products and 21 accumulations), and two in `lambda*p` plus its addition to the
gradient. Their serial composition is bounded by `gamma_(43+42+2)` times the
displayed absolute operand scale, under the normal finite arithmetic model
below. Thus `E_i` is a rounding envelope for evaluating the KKT quantity at
the returned `p`; it is not a forward-error bound for the SVD solution and is
not a physical tolerance.

Compute an upper binary64 enclosure `Cbar_i >= C_i` in ascending index order.
After guarding every operand as stated below, use exactly
`up_mul(a,b)=0` if `a==0` or `b==0`, otherwise
`next_up(fl(a*b))`; and `up_add(a,b)=a` if `b==0`, otherwise
`next_up(fl(a+b))`, including when `a==0`. Thus every nonzero explicit term
into a zero running sum receives its required upward step. Apply these
functions in the written order of `C_i`, including `up_add(|f_r|,inner_r)`,
which skips only when `inner_r` is zero and otherwise evaluates and steps even
when `|f_r|` is zero. Compute `tau_i` by upward-rounded
evaluation of `Gamma87*Cbar_i`, again moving the result once toward
`+infinity`. The checked tolerance is exactly

```text
tau_i = upward(Gamma87 * Cbar_i).
```

The check accepts only normal finite nonzero intermediate magnitudes in this
enclosure calculation. Only explicit zero products and additions with a zero
right addend may be skipped; a nonzero
subnormal operand, product, partial sum, `Cbar_i`, `tau_i`, or KKT operand,
and every NaN, infinity, overflow, invalid `next_up`, negative lambda, or
otherwise nonfinite intermediate makes the numerical status indeterminate and
returns typed `TrustRegionOptimalityIndeterminate`. This is fail closed rather
than silently treating an underflowed scale as a zero tolerance. If the
enclosure is exactly zero, require the corresponding computed `h_hat_i` to be
exact zero (either signed zero) and set `tau_i=0`; any nonzero `h_hat_i` is the
same typed refusal. The implementation records the reason and does not
materialize, mutate, shrink a radius, or invoke another solver.

In particular, before a zero product is treated as a skipped zero term, the
check tests whether both finite operands were nonzero; that condition is an
underflow-to-zero refusal. Before that test, every nonzero operand is checked
normal, so a nonzero subnormal cannot be hidden by a later zero skip. It also
rejects every nonzero subnormal output of a checked multiplication or
accumulation. Before a checked addition producing signed zero is accepted, its
finite inputs must either both be signed zero or be exact binary64 additive
opposites; otherwise it is an indeterminate zero-addition refusal. This is a
prospective conservative guard, not a claim that an unequal binary64 finite
addition has been observed to round to zero under gradual underflow. Signed
cancellation satisfying that exact-opposites guard remains representable, but
cannot waive normal finite guards on its operands or the independent `Cbar`
enclosure. All `r_hat`, `g_hat`, `lambda*p`, and `h_hat` products, partial
sums, additions, and final values are guarded.

The stable `TrustRegionOptimalityIndeterminate` reason token is exactly one of
`optimality_nonfinite`, `optimality_negative_lambda`,
`optimality_non_normal_operand`, `optimality_subnormal_intermediate`,
`optimality_underflow_to_zero`, `optimality_zero_addition`,
`optimality_upward_enclosure`, `optimality_release_ratio`, or
`optimality_free_stationarity`. Each records the coordinate when one applies;
the existing refusal record retains the masks, radius, lambda, and work
counters. No implementation may substitute a generic numerical error or a
success result for one of these reasons.

The returned `p` is assessed as an approximate KKT point, rather than assumed
accurate because the SVD completed. Every free coordinate must satisfy
`abs(h_hat_i) <= tau_i`; this is the explicit acceptance of the finite
SVD/face-solve accuracy relevant to this decision. Every lower-active
coordinate must satisfy `h_hat_i >= -tau_i` and every upper-active coordinate
must satisfy `h_hat_i <= tau_i`. The old sign-only release is replaced by:

```text
lower active: release only if h_hat_i < -tau_i
upper active: release only if h_hat_i >  tau_i
free:         return only if abs(h_hat_i) <= tau_i.
```

Among genuinely violating active bounds, rank
`abs(h_hat_i)/tau_i` descending when `tau_i>0`; calculate that division only
after its nonzero normal finite numerator and denominator have been checked,
then require a normal finite result. An overflow, subnormal, zero, NaN, or
infinite ratio returns `TrustRegionOptimalityIndeterminate` with
`optimality_release_ratio`; it is never silently ranked. A true-sign violation
at `tau_i=0` ranks above every finite ratio; exact ties select the lowest
coordinate index. The selected bound is released and the unchanged face loop
continues. If no active bound is releasable but any free-coordinate test fails,
return `TrustRegionOptimalityIndeterminate` with
`optimality_free_stationarity`; an unchecked constrained corner is never
returned.

A coordinate with `lower_i==upper_i` is classified `fixed` before every face:
its `p_i` is exactly that scaled bound, it contributes to the existing radius
calculation, and it is outside the lower-active, upper-active, and free
optimality/release tests. Any other inverted or nonfinite box retains the
existing typed refusal. A return is permitted only after the existing ordered
binary64 checks establish finite `p`, `lower_i <= p_i <= upper_i` for every
coordinate (including fixed coordinates), and `||p||_2 <= Delta`; it returns
the already strictly checked box/radius-feasible constrained-face `p`. A
signed-zero crossing remains an exact activation under the existing crossing
rule; this amendment neither normalizes its sign nor permits an infeasible
all-free or clipped direction.

For normal finite checked arithmetic, the directed enclosure gives
`|h_hat_i-h_i(p)| <= tau_i`, where `h_i(p)` is the real-arithmetic KKT value at
the returned binary64 `p`. Consequently a release (`h_hat_i < -tau_i` at a
lower bound or `h_hat_i > tau_i` at an upper bound) is a resolvable strict
sign violation of `h_i(p)`, while a returned face has free
`|h_i(p)| <= 2*tau_i`, lower `h_i(p) >= -2*tau_i`, and upper
`h_i(p) <= 2*tau_i`. This is the accepted a-posteriori stationarity accuracy of
the unchanged face solve. It deliberately makes no forward-error claim about
the Jacobi SVD's `p`: the complete free-coordinate test measures the quantity
needed for an approximate KKT decision, and the unchanged finite/rank/ball
guards remain separate conditions for accepting the solve. A last-dot-product
rounding estimate is therefore not represented as a certificate for the SVD
output.

For fixed dimension 21, constructing one `Cbar_i` and `tau_i` takes at most
`21*(21*4+2+2+2)+4+2 = 1896` bounded binary64 arithmetic or upward-step
operations: the inner absolute products/sums, the row contribution and outer
sum, then the lambda term, final sum, and coefficient multiplication. All 21
coordinate thresholds take at most 39,816 such operations. The existing KKT
vector calculation is reused; the amendment performs no additional SVD,
evaluator, hydraulic block, radius proposal, or solver iteration.

#### Contract-derived 02 controls and recording

Before any revised-body or result-bearing execution, tests using the shared
implementation and real numeric operands must establish all of the following:

1. independent ordered arithmetic reconstructs `Cbar`, `tau`, `r_hat`,
   `g_hat`, and `h_hat` without calling the candidate helper; the retained
   cancellation-sensitive face records all of those values and either returns
   only after the complete all-coordinate test or retains its refusal. Its
   observed multiplier is not an input to the formula or a threshold choice;
2. resolvable lower and upper violations release, an active coordinate within
   its uncertainty band remains bound, a materially nonstationary free
   coordinate refuses, and an exact feasible bound is retained. Include a
   signed-zero crossing with an independently established descent obligation so
   a zero-length crossing cannot lock a bound;
3. a coordinate permutation and mathematically corresponding diagonal scaling
   preserve the applicable dimensionless release/return predicates, while a
   real negative drainage step remains inadmissible. These are numerical
   equivalence checks, not unjustified bit-identity requirements across changed
   finite operation orders;
4. fixed lower-equals-upper coordinates stay outside active/free tests while
   retaining their exact box/radius contribution; zero-scale terms, nonzero
   subnormal operands/intermediates, underflow-to-zero products, unchecked
   zero additions, ratio overflow/subnormal/nonfinite results, overflow, NaN,
   infinity, and invalid upward-enclosure operations fail closed with the
   corresponding stable typed reason; and
   and
5. the check adds no SVD factorization, evaluator, hydraulic block, radius
   proposal, solver path, or materialization. The original exact feasibility,
   full residual/root, phase, hydraulic, owner, closure, and failure-atomicity
   predicates remain operative after a local return.

The treatment records masks, `lambda`, `p`, `Cbar`, `tau`, all three KKT
vectors, the per-coordinate classification, selected release ratio/tie data
when any, and the typed indeterminate reason when any. These records are
diagnostic evidence, not a new acceptance surface. The retained authenticated
`01` ordinary-positive failure is pre-body historical custody, not a redundant
physical replay. The discriminating 02 controls above precede any 02 body
change. After the reviewed 02 body, run the unchanged original60 plus
coordinate-20 `+1e-4 K` non-target ordinary-positive control before any
full-feature or target work; all source/input/counter custody remains required.

### Contract-derived experiment controls

Independent expected values, never the candidate helper, must cover: an
admissible bounded root for which scalar Newton damping stalls; a bounded
root-free or constrained-stationary case with nonzero residual that cannot
pass; every closed-box start without a feasibility nudge; row and coordinate
scaling and frozen-weight raw-merit arithmetic; nonfinite, SVD nonconvergence,
rank deficiency, lambda/ball/no-free/degenerate-face and pivot-cap refusals; exact
phase joins, capacity ties, selected-side probe precedence, radius-limited D
steps and both retained D coupling columns; each work-cap exhaustion with its
attempt counted before entry; ordinary update versus both no-update witnesses;
and exact unchanged-state/owner rollback for every refusal.  Controls also
prove one/two assembly and predictor/selected/proposal probe counts from the
source call graph.  They must distinguish an ordinary trial-domain refusal
that shrinks radius from a terminal owning error, and must retain all 21 raw and
dynamically normalized residual acceptance checks.  These controls are
non-target mathematical/phase/domain/guard checks; neither source construction
nor a control execution authorizes an additional support-1 target arm.

### Fixed work and custody caps

The experimental call graph bounds one base at 43 `core` entries for its
natural assembly, plus 44 for an optionally selected assembly because
`evaluate_core_with_jacobian(selected_probe=Some)` includes its selected-side
probe for the predictor, not for the final selected treatment step.  Each of
the 21 proposed treatment steps therefore requires its own selected-side probe;
the maximal base count is `43+44+21=108` selected-assembly/probe entries, plus
at most 21 candidate `core` entries.  There are at most 50 bases; every
accepted update except the first has one top-of-loop current entry; and terminal
testing has one.  The complete-evaluator ceiling is therefore exactly
`50*(108+21) + 49 + 1 = 6500` entries.  It includes all finite-difference
probes, selected probes, rejected and domain-refused trials, but no unexecuted
materialization.  The Jacobian-assembly cap is 100.  The
hydraulic-block cap is exactly `2*(50 + 50*21 + 1)=2202`: two base blocks per
base, two per candidate, and two final materialization blocks.  The SVD cap is
`50*22*43=47300` face factorizations; each has at most 64 sweeps, 48 bracket
expansions and 48 bisections.  One final materialization is allowed.  Every
counter is monotonic, records its refused work, and returns typed exhaustion
before any extra evaluator, hydraulic block, factorization, or materialization.
Increment each attempted-work counter before entering its evaluator, hydraulic
block, SVD face factorization, or materialization, so guard and error paths are
counted as well as successful returns.

### Frozen target measurement

Before either support-1 target arm, freeze the release build command/toolchain,
source tree and binary hash for a canonical baseline diagnostic and for this
treatment diagnostic, their authenticated support-1 clone or immutable prefix
handoff, method ID and all counters.  The canonical support-0 prefix remains
the unchanged native path in both arms; it is neither a treatment nor a timing
denominator.  The baseline must reproduce its canonical support-1 refusal and
the treatment must begin from the same complete 21-coordinate seed and physical
input, never from a terminal Newton base.  The detached harness receives an
immutable clone of the actual prepared `M1CoupledColumnInput` at the real
prefix-to-solver boundary, after the native prefix and before either policy is
chosen or invoked; it may not reconstruct input from JSON traces, an
iteration-15 state, or a guessed replay.  The retained debug baseline is
custody evidence only: the compared target results use release-mode binaries.
Different implementation hashes/provider IDs are recorded and their physical
support-1 projection is compared explicitly.

Time each release target solve with monotonic wall and process CPU clocks.  The
timed kernel interval begins immediately before the support-1 solver entry and
ends at its return; separately report support-0 reconstruction, provider
initialization, manifest/authentication and instrumentation overhead.  The
observer-off/observer-on correspondence is a named **NON-TARGET** control: it
uses only the frozen original60 or injected local control path and may not
invoke authenticated support-1, consume either target arm, or produce a target
outcome.  It must show identical physical input, result class, raw/normalized
residual verdict and owner nonpublication before observed target counts are
used.  Observer output is bounded to the frozen counters and final diagnostic:
no duration-proportional numerical history is permitted.  Baseline failure time
is reported as time-to-refusal, not a speedup denominator.

The numerical rationale is the bound-constrained trust-region treatment and
dense SVD subproblem discussion in SciPy `v1.15.2` `least_squares` documentation
and source `scipy/optimize/_lsq/trf.py` at tag `v1.15.2`
(`316d5e12c24cb349a291d0d5ba0c2a86489b6499`, source SHA-256
`1255479c1d949f779a4388c9f0a1c7a7e8705df607332a674ad85f44edf73fdd`); that
source is rationale only, is not a dependency, and supplies no acceptance
criterion.

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
