[Parent contract](../SC-LANDSURFACEENERGY-001.md)

## Dependencies
| Target | Required when | Boundary/obligation | Reading extent |
|---|---|---|---|
| numerical-methods.md#nonlinear-solve | solver implementation, numerical-branch algorithm or evaluator-equivalence review | complete V10 scaling/partial-root and V11–13 stencil/witness algorithms and termination | whole chapter |
| dependency-replay.md#dependency-replay | evaluator/reuse equivalence or optimization review | exact reuse predicates, custody, first errors and qualification | whole chapter |

<a id="solve-boundary"></a>
# Physical solve boundary

Current ordered physical solve and acceptance boundary for physical rule selection
and reconstruction from accepted primitives. Complete numerical-methods WITH its required admission dependency satisfies accepted-output admission duties.
The standalone accepted-primitive route requires the complete nonlinear-solve
interface plus its unique definitions, guards and tests. Independent implementation, numerical-branch algorithm (including V10
scaling/partial-root and V11–13 stencil/witness computation), or evaluator-equivalence
review requires whole numerical-methods. Optimization correctness adds dependency-replay;
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
| <a id="INV-LANDSURFACEENERGY-108"></a> `INV-LANDSURFACEENERGY-108` | Ordered unknowns, residuals, branches, finite differences, pivots, backtracking, tolerances, and error precedence are deterministic. | v31:L1097-L1097 | [INFERENCE][Static] | [Solve guards](#solve) and [typed errors](water-vapor.md#errors) | [Typed failures](water-vapor.md#errors); no partial state or promotion |
| <a id="INV-LANDSURFACEENERGY-109"></a> `INV-LANDSURFACEENERGY-109` | LSE-V2 imports exact LSE-V1 physics and accepts only the V10 vegetation owner at the coupled boundary. | v31:L2070-L2070 | [INFERENCE][Static] | [Solve guards](#solve) and [typed errors](water-vapor.md#errors) | [Typed failures](water-vapor.md#errors); no partial state or promotion |
| <a id="INV-LANDSURFACEENERGY-110"></a> `INV-LANDSURFACEENERGY-110` | Exact FullSupply finalization seeds only coordinates, then reevaluates the complete fixed-final system from immutable beginning owners; a passing initial evaluation accepts at iteration zero without a Jacobian. | v31:L2071-L2071 | [INFERENCE][Static] | [Solve guards](#solve) and [typed errors](water-vapor.md#errors) | [Typed failures](water-vapor.md#errors); no partial state or promotion |

<a id="b01-grid40-original-start-experiment"></a>
## Isolated B01 original-start strict-decrease grid experiment

`EXP-B01-LSE-GRID40-20260919` applies only to the owner-adopted diagnostic
experiment in `20260911-b01-wb14-verified-cadence-repair-001/package.md` at the
original-start proposal published in commit
`d574b65b5b11948a48a7be1bc51da964c34fca59`. Evidence `[DIRECT][Static]` is the
owner's2026-09-19 adoption; `[INFERENCE][Static]` is the finite-grid/domain
obstruction recorded in the reviewed first-trial diagnosis. No convergent
result, root-existence claim, physiological parameter or new physical equation
is implied. This experimental applicability requires prospective independent
correctness and QA approval before implementation; ordinary production
algorithm/authority and all paused experiments remain unchanged.

### Exact input, identity and confinement

One immutable private replay case authorizes exactly one baseline process and
one treatment process. Each consumes the complete `solver_input` and original
29-coordinate `initial_trial` from trace SHA-256
`527208c3e5ad07f1744f1bd92c9e4ac17cf7efd9469968a97d7964c5db482dfd`.
It is R0, V11SnowCovered, represented snow, uncapped Potential, transaction41,
OFE`ofe-1`, tile`forest`, support`[0,60000000000)`ns, two occupancies and six
soil nodes. Exact binary64 bits including signed zeros, all input/owner/
configuration identities, topology and original warm start must be preserved.
The source predecessor is tree8cd4866f66b06d6cf96403753cdb3d41ba75f6ca87652341ebe082a86a1b4bc5,
with its separately bound build-support inputs; a main-checkout SHA is not that
executable's identity. Authenticate the trace/case before evaluation and reject
wrong or mutated inputs at the private entry. An iteration6 restart is not a
substitute. Existing typed input/domain/owner guards still run.

The treatment has a separate non-persisted experimental identity:
`EXP-B01-LSE-GRID40-20260919;strict=0..40;witness=0..20;updates=50`.
Baseline uses the same experiment tag with`strict=0..20`. R0 selects original
physical/numerical thresholds; it does not identify the changed strict-search
range. Original configuration and support receipts remain immutable provenance;
no receipt is resealed and treatment is never represented as execution of its
unchanged b0..20 policy. This amendment authorizes only private solve-input
replay, not a receipt-bearing runtime consumer. Its numerical result is a
disposable diagnostic candidate even if the existing constructor succeeds.
Preserving and publishing diagnostic evidence/source/recovery is permitted;
persisting the candidate as owner/restart state, consuming it in runtime,
resealing original receipts or presenting it as production-policy output is
forbidden. No native owner installation, fixed-final solve, physical interval
advance, day staging, reader/model run, production selector or downstream
continuation may consume it. The entry is private and test-only, outside the
real runner, and inaccessible to production at disposition.

### One changed numerical choice

| Predicate | Baseline | Treatment |
| --- | --- | --- |
| strict-decrease ordered factors | `2^-b`, b0..20 | `2^-b`, b0..40 |
| full no-update witness | existing b0 | unchanged |
| halved no-update witness | first domain-valid b1..20 only | unchanged |
| completed Newton updates |50|50|

Only the existing covered strict-decrease range and its exhaustion contribution
to cumulative backtracking change in the disposable experimental build. Use the
same solver body, domain checks, evaluator ordering, strict norm-decrease test
and accepted-candidate constructor. `MAX_BACKTRACKING_HALVINGS`, generic/open/
litter solvers and other regime consumers are unchanged. No failure-triggered
solver dispatch or fallback is allowed. Forty is a fixed engineering study
limit (41 factors versus21), not a proven convergent cap or production default.
Earlier choices remain identical until baseline exhaustion. After update50,
evaluate the final base and preserve acceptance-before-iteration-limit order;
no51st update or extra Jacobian follows.

INV108 retains deterministic ordering, binary64 arithmetic, finite differences,
LU/pivot rules, strict decrease and first-error precedence except the named
experimental search range/count. INV112/138 retain scaling applicability and
canonical derivative stencils. INV139 and the complete accepted-solve admission
remain unchanged: halved witness availability/order, all finite current residuals
passing, governed step thresholds and acceptance of current state only. Preserve
all equations, raw/normalized residuals and scales, inactive coordinates,
constitutive domains, supports, owner guards and physical branch selection.
No clipping, extra support, changed warm start, tolerance/pivot relaxation,
domain-aware alternative or post-result cap escalation is admitted.

### Prerequisites, outcomes and computation limits

Before numerical implementation, contract-derived expected-red checks must bind
unchanged earlier choices; lawful strict decrease beyond20; failure through40;
exact-bound outward direction; nonfinite/evaluation failure; duplicate-rounded
trials; unacceptable residuals despite tiny steps; unchanged first-valid
no-update witness and no trial installation; wrong-case/non-production
confinement. Applicable A0/A1/A3 and affected component/quality checks remain
mandatory. Reviewers inspect the actual detached shared-constant consumers,
call sites and count mapping before measurement.

One source/input/protocol-bound original-start baseline replay must reproduce
the recorded six-update/iteration6 E034 refusal and ordinary numerical trajectory
before one treatment replay may launch. Compare relevant input, coordinates,
residuals, Jacobian/RHS/direction, branches, exponents and step fields explicitly;
new harness metadata is not numerical equivalence. Baseline mismatch invalidates
the comparison. No retries or replacement pair. Disable the prior four
post-refusal diagnostic evaluator probes in both arms; extra probes=0.

Per arm: at most50 updates,51 base evaluations,50 Jacobian assemblies,
29 coordinates and2900 signed Jacobian probe starts, retaining nested component
limits. Baseline caps are1201 ordinary/4101 total complete-residual calls;
treatment caps2201 ordinary/5101 total, including at most2050 strict-search and100
witness calls. Identity shortcuts reduce actual calls. The20000 domain-predicate
ceiling counts every invocation of `covered_trial_is_valid`, charged at function
entry, including initial/base-evaluator, stencil/observation/probe, witness and
strict-search callers. Scalar comparisons inside that function are not separate
invocations. Other constitutive/domain guards and nested component limits remain
unchanged and are reported separately; this counter never exempts their checks.
A complete-residual call consumes its category ceiling immediately before entry
to the canonical complete evaluator, whether it completes or returns an error.
The ordinary categories are iteration-base, strict-search trial and prospective
no-update witness. Their conservative treatment bound is
`51 + 50*41 + 50*2 = 2201`; baseline is`51 + 50*21 + 50*2 = 1201`.
The witness bound covers at most one full and the first domain-valid halved
trial per iteration, never20 evaluations of successively smaller witnesses.
The separate Jacobian signed-probe-start counter increments before invoking
the signed-probe residual helper. Each start is classified identity-anchor or
complete-evaluator; only the latter additionally consumes the Jacobian
complete-residual-call counter, including error attempts. At most`50*29*2=2900`
signed starts permits at most2900 such complete calls. Total complete attempts
are exactly ordinary plus Jacobian complete attempts, bounded by5101/4101;
identity shortcuts and nested component calls are not added again. For each
category reconcile attempted=completed+error+in-flight-at-interruption; retain
actual error and incomplete classifications. The named `covered_trial_is_valid` counter increments
before every invocation, including repeated validator/stencil invocations.
Updates count only installed strict-decrease steps; base/assembly attempts are
counted before entry, including errors. Report nested component work separately
under its unchanged own limits. A denied next operation is recorded as a budget
stop and is not executed or counted as an evaluator start; preserve the
consumed count and denied operation. No counter may silently saturate or wrap.
Each result-bearing process has120s wall,16GiB address-space,64MiB stack and1GiB
output limits; the pair has240s total wall limit. Enforce expired-launch refusal
and live limits with current host resource checks and review/preservation reserve.

Every first-solve terminal outcome (Accepted, Rejected, Error or resource stop)
retains its source/input/policy-bound trace and costs. Preserve interruption
partials as incomplete; failed serialization or missing evidence cannot pass.
Private diagnostics identify actual branch-specific row units, raw values,
applied normalizers, base iteration and exact step provenance, without changing
public/persisted schemas or historical raw bytes. Retain full trajectory,
accepted exponents, final coordinates/domain distances/residuals/steps,
candidate-construction result and first obstruction.

Scientific success requires the existing accepted-candidate constructor to
succeed, the complete residual vector finite with every normalized absolute
value<=1, unchanged governed steps/domain/branch/identity requirements, cost
compliance and independent reconstruction from decisive primitive operands.
Potential rates/requests are not finalized fluxes or global closure evidence.
`Ok(Rejected)`, b21 acceptance, a lower norm or tiny steps are not success.
A valid typed refusal is a negative fixed-policy result, not proof of root
nonexistence. Resource/counter/time abort and missing/integrity/infrastructure
evidence are incomplete/invalid as applicable; private budget refusal cannot
be reported as canonical convergence. A valid treatment refusal ends the
experiment. No b41/b60 or different method/input follows.

### Guard map, readiness and disposition

| Binding | Required guard/evidence | Failure posture |
| --- | --- | --- |
| INV108 experimental search/count applicability | exact private case/policy; source confinement; expected-red/runtime vectors; baseline trajectory; counter reconciliation | typed canonical refusal or private invalid/budget result; no downstream use |
| INV112/138/139 unchanged | byte/diff and focused scaling/stencil/witness/acceptance vectors; all original thresholds | failed prerequisite blocks measurement/acceptance |
| original owner/domain and candidate admission | existing validated input/evaluator/candidate construction, independent primitive reconstruction | no accepted endpoint or owner installation on failure |
| complete evidence and limits | every-outcome recorder, partial-on-interruption, finite counters and external live resource limits | missing or over-budget evidence remains incomplete |

Distinct independent correctness and QA reviewers approve substantive authority,
then assess affected implementation/fixes and terminal evidence. Reuse those
reviewers; the method author/adviser cannot provide independent acceptance.
Preserve exact source/recovery/raw evidence and leave the disposable entry
production-inaccessible. This authority expires for new execution at study
disposition; historical outcomes and broader HOLDs are not rewritten.
The owner's 2026-09-20 adoption of the Grid40 measurement continuation
prospectively reactivates this same study once after its earlier premeasurement
INCOMPLETE disposition, solely for remaining final QA, concrete launch freeze,
the still-unspent original baseline/treatment pair and independent result review.
The earlier disposition and zero executed arms remain historical evidence.
All numerical predicates, input identities, run limits and cumulative budget
remain unchanged. Carry135809.983578 charged seconds under140479.464696;
the conservatively anchored resumption deadline is2026-09-20T02:42:19.481118Z,
with the existing1200-second review/preservation reserve. Both independent
scopes verify this narrow lifecycle continuation before measurement. New
execution expires again at continuation disposition or that hard deadline;
neither unused arms nor a later session renew it.
Calibration/identifiability: NOT_APPLICABLE, no estimated physical parameter;
observations are DIAGNOSTIC_ONLY. No reader/E008/RQ1/A-001, full scientific/
conservation/restart, production or cadence qualification follows.

Change log:2026-09-19, non-versioned isolated B01 grid40 experiment under
INV108/112/138/139; prospective dual-review gate, no production authority change.
