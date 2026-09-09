[Parent contract](../SC-LANDSURFACEENERGY-001.md)

## Dependencies

| Target | Required when | Boundary/obligation | Reading extent |
|---|---|---|---|
| interface.md#interface | every task | shared physics, custody and errors | whole chapter and recursive dependencies |
| numerical-methods.md#nonlinear-solve | implementation/review | existing coordinates, normalization, solver and historical experiment | whole chapter and recursive dependencies |
| nonlinear-solve.md#nonlinear-solve | implementation/review | inactive-row replacement and acceptance | whole chapter |
| litter-phase.md#litter-phase | GenericV3 | complete actual residual wrapper and custody | whole chapter |

<a id="inactive-jacobian"></a>
# Version 34 experimental inactive-leaf Jacobian

Static: prospective numerical representation, not production activation.
Scope: package `20260908-stage3-hotpath-jacobian-reference-and-prototype-001`.
Owner-approved primary is computationally executed, physically inactive sun/shade
temperature columns. Positive-area ground temperature is the named alternative;
it requires a prospective complete dependency amendment before implementation.
No adjacent expansion is admitted by this initial cut. The v33 stem method,
0/16 result, thresholds and expiry remain historical and unchanged. They are
not this experiment's acceptance rules. This authority expires at disposition.

## Mathematical target and complete dependency mask

Use the smooth branch-conditioned canonical normalized residual r(x;u,b0), not
the derivative of the discontinuous binary64 rounding map. Retain coordinate and
row order D=10*N+3+S from numerical-methods.md, arbitrary validated N and S.
For occupancy o, j=10*o+6 (sun) or j=10*o+7 (shade), hold the other coordinates,
validated inputs u, caps, support and canonical frozen branches b0 fixed.
Eligibility requires successful ordinary same-state evaluation; authority
admits_nonpositive_assimilation(); the corresponding input leaf-area bits are
positive zero; the successful component area is positive zero; dimensions and
row order agree. Wet-fraction-only zero dry area, positive input area, negative
zero input area and other authorities are outside this initial support class.
Invalid/nonfinite input is an error, never structural inapplicability.

The anchor is a=max(T_canopy,273.15 K), independent of the selected temperature,
including cold/warm/tied canopy. The own residual is r_j=(T_j-a)/s, where s is
the existing positive finite energy_tolerance(1.0) normalization operand
(canonical binary64 1e-6 + 1e-10, approximately 1.0001e-6). No tolerance changes.
For this branch-conditioned column s and a are fixed, hence dr_j/dT_j=1/s.
This is the derivative of the executed residual with its supplied normalization
operand. No implicit leaf derivative or active foliage claim is made.

The complete mask is a consequence of these dependencies, not an area shortcut:

| Residual rows | Dependency on selected zero-input-area leaf temperature |
|---|---|
| Own energy row | Exactly (T_j-a)/s; ds/dT_j=0, da/dT_j=0. |
| All hydraulic and beta rows, all occupancies | Current and maximum gas state are Inactive after ordinary fallible preprocessing; zero dry area multiplies transpiration/demand. Root flows and hydraulic normalization do not read this temperature. |
| Other component energy rows, all occupancies | Zero area gives zero reciprocal-longwave emission weight. Other incident radiation, gas state, heat and latent operands and normalizers are independent. Wet routing reads wet temperature, not the selected coordinate. |
| Shared canopy heat/vapor rows | Selected heat/vapor contribution is zero; other terms and normalizers are independent. |
| Covered ground and every soil row | Radiation is unchanged; shared, ground, soil and frozen boundary operands are unchanged, including represented-snow identity anchors. |
| GenericV3 replacement vapor/surface/soil rows | Complete wrapper consumes unchanged canopy/ground/soil/context/radiation operands; no dependency is reintroduced in Potential or FixedFinal. |

All other entries in the column are structural zeros only with this complete
proof. Successful leaf preprocessing (peaked factors, Arrhenius, saturation,
conductance and gas-domain guards) remains mandatory on the unchanged base.
Source provenance: canonical INV-131, SC-VEGETATION-001 V10 inactive gas branch,
and frozen A solver_covered_evaluation.rs, physics.rs and solver_litter_phase.rs;
source evidence corroborates the canonical equations, not a new physics model.

## Coordinates, assembly and custody

For x=D_scale*y use J_y[:,j]=J_x[:,j]*D_scale[j]. Physical-coordinate assembly
uses 1/s; scaled-coordinate assembly uses D_scale[j]/s. Selected temperature
scales currently equal one; tests also expose non-unit mapping errors. Do not
infer scales from physical replay x: bind actual scaled x and scale bits.
Build the complete canonical matrix first, then retain existing inactive-row
replacement, right-hand-side anchoring and any existing subsequent unit scaling
in exactly their current positions. The consumed identity diagonal is distinct
from the canonical 1/s diagonal. Do not remove unknowns, resize the system,
change pivoting, or move row replacement.

Use one private non-Clone, nonserialized capability scoped to one successful
base/sweep, borrowing exact inputs, trial, evaluation, caps/context/frozen state
and order. No replay graph, hashing, whole-input cloning or corpus lookup is
required at derivative entry. Validated state cannot be supplied by caller flags.
No independent public constructor may pair a foreign evaluation with inputs.
Borrowing/ownership must make mismatched/stale use impossible, or reject it with
typed CoveredJacobianDerivativeIntegrity(reason). Consume each column once;
wrong topology/order/base/caps/context, second use, invalid scale/normalizer,
nonfinite tangent and malformed column/matrix fail typed without partial use.

After ordinary same-state guards, choose one hybrid assembly prospectively:
existing identity columns, direct eligible leaf columns, canonical FD elsewhere.
There is no FD retry after derivative entry or failure. Do not skip ordinary
base evaluation, alter its first typed error or rerun an older solver. Deleted
FD probes and their internal failures are numerical trajectory differences,
not a promise to reproduce A probe rejection histories. Retain source-real
invalid-base tests separately from valid-base/probe-failure classification.
The canonical controller, budgets, domains, step and residual tolerances,
line search, dense solve, event/adaptive policy, receipts and publication remain.

## Candidate-independent reference and comparison policy

Freeze the following before J comparison. B_i=1 normalized residual acceptance
unit, Delta_j=1 K, eta=1e-3, and m=max(1,number of supported leaf columns in the
sweep). Allowed derivative error E_ij=eta*B_i/(m*Delta_j), in K^-1. Thus simultaneous
physical perturbations bounded by 1 K produce at most 0.001 normalized units
of linearized error. In scaled coordinates multiply both uncertainty and E by
the positive coordinate scale. This is not a solver or physical closure limit.
Require abs(J_ij-reference_ij)+U_ij <= E_ij; oversized U is UNRESOLVED, never
an enlarged acceptance bound. Never tune E or reference selection using J.

Route 1: canonical same-family branch-preserving residual perturbations at
h in [0.5,0.25,0.125,0.0625] K in that order. Record represented abscissae and
use their actual separation. Use centered secants when both probes are lawful;
at closed bounds use the unique lawful inward secant with the base. Reject zero
represented movement and any domain/branch/dependency change. An exact affine
column has no truncation term and need not show a nonzero fourfold basin.
For each own-row sample define the conservative evaluation allowance
e(T)=8*epsilon*(abs(T)+abs(a))/s + 8*epsilon*max(1,abs(r(T))).
Secant U=(e(T_plus)+e(T_minus))/abs(T_plus-T_minus)
+ 8*epsilon*max(1,abs(secant)); replace one endpoint by base for one-sided use.
This bounds affine subtraction/division and represented-spacing arithmetic;
overflow, subnormal/underflow hazards or nonfinite terms are UNRESOLVED.
Choose minimum U among lawful samples, ties by the listed order. Off-diagonal
structural-zero reference has U=0 only after the full dependency proof and
exact canonical residual-bit invariance under those probes, not from a plateau.

Route 2: independent affine identity s*dr_j/dT_j=1, with supplied s interpreted
exactly, and complete row-mask proof. Use a separately implemented rational
reference (exact binary64 input import, reciprocal rounded once) or independently
bounded affine residual/normalizer relation. Its bound covers reference rounding;
never call J's helper as oracle. Confirm canonical primal relation separately.
The two routes corroborate, not substitute for complete same-family coverage.
Directional checks perturb all admitted coordinates in positive, negative and
alternating directions at the same lawful steps; bound remainder by endpoint
evaluation allowances plus summed derivative allowance times displacement.
No nonzero second-order remainder or convergence-ratio prerequisite applies.

Route 3 for genuinely unresolved entries only: exact binary64 input import and
128/256/512-bit reference ladder, same branch-conditioned algebra, separately
reconciled primal values and stencil/evaluation error. Two agreeing precision
levels alone are not a rigorous enclosure. No general precision framework or
production dependency is authorized; record supported estimate versus bound.

Reference self-tests retain constant/affine/quadratic/cubic, large-offset
cancellation, near-zero, x-dependent quotient normalization, feasible one-sided
bounds, kink/tie/branch rejection, known implicit sensitivity and deliberate
wrong diagonal/off-diagonal tests. General nonlinear exercises test reference
limits; they do not make active foliage a dependency of this affine candidate.
Keep predecessor stem development results immutable. Cut2 is baseline development;
fresh J states or held-out classes are checked with the frozen policy, not tuned.

## Admission, cost and lifecycle

Same-state primal values and first errors remain bit-identical. Each A/J solve
must independently pass unchanged residual, step, domain and reconstructed
water/energy closure. For cross-algorithm continuous fields, use the dimensional
v33 full-solve comparison table, but no mixed-unit minimum: apply any tighter
canonical same-unit limit only to the corresponding field; compare residuals
and closure separately in their own units. Unlisted fields remain exact.
Discrete IDs, ordering, topology, ownership, rollback, event sequencing and
within-run payload transfer are exact. Each run authenticates its own payload;
cross-algorithm digests may differ only for explicitly admitted payload values.
Iterations/backtracking/probe failures are recorded, not golden trajectories.

Before timing require complete-column and directional evidence, guard/custody
negatives, observation on/off equivalence through actual solvers, Covered
Potential and GenericV3 Potential/FixedFinal solves, and an authentic one-OFE
consumer with nonzero direct-column participation and independent closure.
Keep corrected population counts even if they differ from cut1's 9,844.
Count opportunity is not a runtime fraction or active-foliage evidence.

Measure identical complete column obligations, charging eligibility, allocation,
assembly and retained checks. Report removed complete residual/leaf calls and
replacement operations separately. Local protocol: two warmups, 30 paired
batches, at least 100 ms per arm. Tenfold removed work and twofold local speed
are expansion priorities, not correctness gates. One-OFE: two warmups per
executable, 12 balanced fresh-process pairs; one extension to 24 if decisive.
Report wall/CPU, paired uncertainty, failures and counters, preserving all valid
samples. No 75% coverage prerequisite. Bound or measure nonoverlapping removable
time fraction f and local acceleration s_local before interpreting optimistic
whole-run savings f*(1-1/s_local); no column-count proxy for f.
For a useful admitted candidate retain six memory pairs, three ten-run teardown
processes per arm, max(4 MiB,5% of A) extra matched peak and no newly supported
persistent post-drop growth. Check real J participation before 10/19-OFE scale
series (three pairs each, 19 only after 10 admission). Unqualified scale is not
one-OFE rejection or success. One performance refinement maximum.

Dual prospective review precedes expected-red tests and J. Dual terminal
reviews/verifications, applicable A0/A1/A3 and full correctness regression,
typed error/custody/rollback/restart, formatting/Clippy and recoverable source/
executable/protocol/results remain current obligations. No production activation.
CALIBRATION_NOT_APPLICABLE: no physical parameter or observation mapping changes.

## Canonical invariants

| Invariant ID | Statement | Authority | Evidence | Guard | Failure posture |
|---|---|---|---|---|---|
| <a id="INV-LANDSURFACEENERGY-166"></a> `INV-LANDSURFACEENERGY-166` | Isolated v34 zero-input-area sun/shade columns represent the complete canonical affine residual derivative with exact structural-zero proof, coordinate-aware assembly and unchanged base validation and row replacement. | INV-131/138; SC-VEGETATION-001 V10 inactive gas branch; affine differentiation | [INFERENCE][Static] | private same-sweep capability and independent reference/consumer tests | typed integrity failure, no retry or partial matrix; production HOLD |

## Canonical obligations

| Obligation ID | Statement | Applicability | Authority | Enforcement/failure | Test bindings |
|---|---|---|---|---|---|
| <a id="OBL-LANDSURFACEENERGY-C-022"></a> `OBL-LANDSURFACEENERGY-C-022` | Prove the complete mask, affine/roundoff and scaled assembly, same-state errors, solver observation neutrality, arbitrary topology, stale/foreign/second-use rejection, full solves and actual Covered/GenericV3 consumer closure before charged cost evidence. | Isolated v34 package only | INV-LANDSURFACEENERGY-166 | dual prospective/terminal review, independent verification; missing admission remains HOLD | land_surface_energy_balance_authority_contract; isolated inactive Jacobian tests and authentic runner |
