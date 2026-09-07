[Parent contract](../SC-LANDSURFACEENERGY-001.md)

## Dependencies
| Target | Required when | Boundary/obligation | Reading extent |
|---|---|---|---|
| interface.md#interface | always | shared authority | whole chapter |
| nonlinear-solve.md#nonlinear-solve | solver implementation or V10/V11–13 branch/algorithm review | complete eligibility, scaling, exact stencils and termination | whole chapter |
| dependency-replay.md#dependency-replay | evaluator/reuse equivalence or optimization review | exact reuse predicates, custody, first errors and qualification | whole chapter |

<a id="solve-boundary"></a>
# Physical solve boundary

Current ordered physical solve and acceptance boundary for physical rule selection
and reconstruction from accepted primitives. Reviewing solver implementation or
V10 eligibility, partial-root, scaling, wet-coordinate or V11–13 numerical branches
also requires nonlinear-solve. Optimization correctness adds dependency-replay;
accepted primitive reconstruction does not itself claim evaluator/reuse equivalence.

<a id="solve"></a>
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


<a id="canonical-invariants"></a>
## Canonical invariants
| Invariant ID | Statement | Authority | Evidence | Guard | Failure posture |
|---|---|---|---|---|---|
| <a id="INV-LANDSURFACEENERGY-108"></a> `INV-LANDSURFACEENERGY-108` | Ordered unknowns, residuals, branches, finite differences, pivots, backtracking, tolerances, and error precedence are deterministic. | v31:L1097-L1097 | [INFERENCE][Static] | [Ordered domain/closure guards](#solve) and [typed errors](water-vapor.md#errors) | Typed domain/convergence/closure: [errors](water-vapor.md#errors); no partial state or promotion |
| <a id="INV-LANDSURFACEENERGY-109"></a> `INV-LANDSURFACEENERGY-109` | LSE-V2 imports exact LSE-V1 physics and accepts only the V10 vegetation owner at the coupled boundary. | v31:L2070-L2070 | [INFERENCE][Static] | [Ordered domain/closure guards](#solve) and [typed errors](water-vapor.md#errors) | Typed domain/convergence/closure: [errors](water-vapor.md#errors); no partial state or promotion |
| <a id="INV-LANDSURFACEENERGY-110"></a> `INV-LANDSURFACEENERGY-110` | Exact FullSupply finalization seeds only coordinates, then reevaluates the complete fixed-final system from immutable beginning owners; a passing initial evaluation accepts at iteration zero without a Jacobian. | v31:L2071-L2071 | [INFERENCE][Static] | [Ordered domain/closure guards](#solve) and [typed errors](water-vapor.md#errors) | Typed domain/convergence/closure: [errors](water-vapor.md#errors); no partial state or promotion |
