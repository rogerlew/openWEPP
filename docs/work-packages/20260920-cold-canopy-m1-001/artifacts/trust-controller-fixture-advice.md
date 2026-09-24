Static: this is a design recommendation for the named blocker, not independent acceptance. I inspected draft04’s numerical-methods section (SHA-256 `3931d006…bb63`), the failed Stage2 declarations/controls, and the retained body16 Stage1 implementation. Ran: only source inspection and a small independent arithmetic calculation; no Rust, physical evaluator, or target run.

The initial controller group can use the following fixtures without changing any frozen method constant. Physical phase, hydraulic, materialization, and adapter obligations remain a separately reviewed integration increment.

### 1. Use one controller with interchangeable evaluators

Replace the two-coordinate `ControllerModel` and input-selected `AdmissionPath` with a private, full-21-coordinate evaluator boundary. The controller owns the actual iteration, proposal selection, radius, acceptance, and accounting.

The evaluator supplies operations and operands:

- Evaluate a specified coordinate vector, returning raw residuals `[f64;21]`, dynamic normalizers `[f64;21]`, and the associated complete evaluation payload.
- Assemble the raw Jacobian and phase metadata at the current base.
- Select/check the phase from the actual Stage1 direction and execute the selected-side probe.
- Evaluate candidate admission and reconstruct complete step operands.
- Materialize the specified current evaluation, preserving owning errors.

No callback receives a requested final outcome or admission path. Observed dispositions can remain enums in the returned trace.

Use the existing Stage1 numerical component directly. Remove or wrap its unused physical input/trial arguments; do not reconstruct its SVD/BVLS implementation. Expose its existing generic weighting/merit arithmetic through a narrow internal interface.

**Necessary merit integration correction:** `frozen_weighted_merit` presently forms `rho=actual/predicted` before returning. Split its existing arithmetic so finite `pred<=0` can produce the prescribed nonterminal rejection before division. Preserve ordered products, sums, and subtraction. A real stationary step produces `pred=0`; routing its `0/0` through “nonfinite merit” would violate draft04.

Tests may inspect one actual controller transition, using the same transition method called by its run loop. That is preferable to demanding eventual convergence from every fixture. If the implementation exposes only a complete run, inspect the returned success-or-refusal trace’s first transition without assuming final success.

### 2. Common full-rank embedding

Let `b` be a valid coordinate-box base and define scaled coordinates

\[
z_j=(x_j-b_j)/S_j .
\]

Use the actual prescribed `S` and coordinate bounds. Convenient bases are temperature coordinates `300 K`, humidity `0.01`, and drainage `0`; choose finite M/H coordinates. These are mathematical fixtures, not physically authenticated M1 inputs.

Define an analytic dimensionless residual `F(z)` and positive normalizers `t(z)`. Supply

\[
R_i(x)=t_i(b)\,F_i(z),\qquad
J^\mathrm{raw}_{ij}(x)=t_i(b)\,\frac{\partial F_i}{\partial z_j}/S_j
\]

when the fixture specifies fixed row multipliers. Dynamic acceptance still computes `R_i(x)/t_i(x)`.

Every unused row has its own nonzero diagonal derivative and zero residual at the starting base. Do not pad the problem with zero Jacobian columns or fix all unused coordinates. This supplies 21 independent columns initially and, after one active bound, 20 independent free columns.

For the small-residual witness fixtures below, choose the fixed raw multiplier `t_i=2^-60` in every row. This keeps raw operands finite and small without making the *weighted* system ill-conditioned. These synthetic normalizers do not replace or alter `m1_tolerances` in the M1 adapter.

Synthetic derived observables can be explicit constant functions: wet temperatures `300`, hydraulic potentials `0`, beta `1/2`, and ci `20`. Reconstruct norms from these operands plus actual coordinate differences; do not supply “governed-pass” booleans. Humidity and dry-temperature differences remain actual coordinate differences.

### 3. Ordinary update and radius decisions

Use coordinate `0`, fixed `t=1`, and

\[
F_0(z)=-2+z_0+c z_0^2,\qquad F_j(z)=z_j\quad(j\ne0).
\]

At `z=0`, the real weighted Stage1 inputs are `f=(-2,0,…)`, `A=I`, `Delta=1`. Its solution is

\[
p_0=1,\quad p_{j\ne0}=0,\quad\lambda=1,\quad pred=3/2.
\]

The following expectations follow from evaluating the actual polynomial at the candidate:

| `c` | Candidate `F0` | `ared` | `rho` | First transition |
|---|---:|---:|---:|---|
| `0` | `-1` | `3/2` | `1` | Install; radius `2` |
| `-1/2` | `-3/2` | `7/8` | `7/12` | Install; radius `1` |
| `-7/8` | `-15/8` | `31/128` | `31/192` | Install; radius `1/4` |
| `-1` | `-2` | `0` | `0` | Reject; same base; radius `1/4` |

The first three candidates strictly decrease the maximum dynamic normalized residual. Their temperature step is `1 K`, so these controls also prove that ordinary progress does **not** require the root-admission step threshold.

All initial singular values are `1`. The positive-lambda solution lies exactly on the ball, with no rank or ball-accuracy obstacle. Assert actual operands and the resulting first state transition; do not assert convergence of the complete nonlinear run.

### 4. Genuine nonzero constrained stationarity

Use drainage coordinate `5`, starting exactly at its lower bound:

\[
F_5(z)=2+z_5,\qquad F_j(z)=z_j\ (j\ne5),\qquad A=I.
\]

The first free solve wants a negative `p5`. Stage1 must:

1. Start coordinate `5` free.
2. Reach its lower bound at `theta=0`.
3. Activate that bound.
4. Solve the remaining 20-column, full-rank face.
5. Return `p=0`, with `g5=h5=2`.

This is a constrained stationary point with maximum normalized residual `2`, not a root. The remaining free residual component in row `5` is orthogonal to all remaining columns.

Consequently `pred=0`; record `TrustRegionNoPredictedReduction`, retain the base, and shrink. Each proposal restarts from the free face, so this same two-factorization sequence repeats.

With uninterrupted quarter-shrinking, the actual radii are

\[
1,4^{-1},\ldots,4^{-10}=2^{-20}.
\]

There are **11 proposals**, followed by refusal of proposed radius `4^-11=2^-22`. Expect typed radius exhaustion, no installation, and no materialization. The separate 21-proposal cap is not reached. In this no-phase-change fixture, the numerical trace contains 22 entered face factorizations.

### 5. Distinguish raw Jacobian and frozen merit

At the base, use raw residuals and derivatives

\[
R_0=-6/5+z_0,\quad R_1=-1+z_1,\quad J^\mathrm{raw}_{00}=J^\mathrm{raw}_{11}=1,
\]

with `S0=S1=1`. Let

\[
t_0(z)=1+z_0/2,\qquad t_1(z)=1/2.
\]

Other rows have `Rj=zj`, `tj=1`. Require positive `t0` through the synthetic evaluator’s domain check.

Then

\[
f=(-6/5,-2,0,\ldots),\quad A=\operatorname{diag}(1,2,1,\ldots).
\]

At `Delta=1`, the solution is

\[
p=(3/5,4/5,0,\ldots),\qquad\lambda=1.
\]

The candidate has nonzero raw residuals `(-3/5,-1/5)` and dynamic normalizers `(13/10,1/2)`. Independent values are:

| Quantity | Expected value |
|---|---:|
| Frozen base merit | `68/25` |
| Frozen candidate merit | `13/50` |
| `pred = ared` | `123/50` |
| Frozen `rho` | `1` |
| Moving candidate merit | `788/4225` |
| Incorrect moving-weight `ared` | `10704/4225` |

Thus frozen and moving actual reductions demonstrably differ. Candidate dynamic maximum is `6/13`, below the base maximum `2`; the first update is accepted and expands the radius.

The derivative of the *normalized* first residual at the base is `8/5`, whereas the required weighted raw derivative is `1`. Substitution of that wrong derivative cannot yield the same KKT solution: at the proposed `(3/5,4/5)`, the two coordinates require different lambda values. This fixture therefore distinguishes actual raw-J use, not merely recorded matrix labels.

The singular values are `2,1,…,1`, and the desired lambda endpoint is `1`; the rank and ball guards are comfortably reachable. Compare continuous derived values with a declared tight arithmetic tolerance, while retaining exact comparisons for counters, coordinate identity, and event order.

### 6. Three genuinely reachable admission paths

**Full no-update witness.** Use fixed `t=2^-60`, `A=I`, and

\[
F_0(z)=z_0-2^{-28},\qquad F_j(z)=z_j.
\]

Current maximum normalized residual is `2^-28`. Stage1 returns the interior step `p0=2^-28`; the candidate is a root and its temperature change is approximately `3.7253e-9 K`, below `1e-8`.

After complete candidate evaluation, materialize **the current base `b`**, not the candidate. Expect zero installed updates, one proposal, and one materialization whose coordinate operand is exactly `b`.

**Post-update admission.** Use fixed `t=2^-60` and

\[
F_0(z)=2^{29}z_0-2,\qquad F_j(z)=2^{29}z_j\ (j\ne0).
\]

Initially the residual maximum is `2`, so no initial no-update admission is available. Stage1 has `A=2^29 I` and returns `p0=2^-28`, with all other components zero. This is an interior, full-rank solve; its condition number is one.

The candidate residual is zero, `pred=ared=2`, and `rho=1`. Install it. At the next post-update check, residuals and the immediately preceding complete step norms pass. Materialize the installed point before another assembly. This distinguishes the admitted coordinate from the full no-update fixture and proves the second assembly is absent.

**First-valid replacement after governed excess.** Use fixed `t=2^-60` and

\[
\begin{aligned}
F_0(z)&=-2^{-27}+z_0/4,\\
F_3(z)&=-1/2+z_3/4-z_3^2/4,\\
F_j(z)&=z_j/4 \quad(j\notin\{0,3\}).
\end{aligned}
\]

At the initial base, `A=I/4`, current normalized maximum is `1/2`, and every residual passes.

At `Delta=1`, Stage1 produces:

- `lambda=1/16`;
- `p0=2^-26`, exceeding the temperature threshold;
- `p3=1`, supplying the substantial **ungoverned mass-coordinate** part of the radius;
- candidate `F3=-1/2`.

The full candidate is actually rejected: its maximum residual does not strictly decrease and `rho` is approximately zero. Ordered binary64 arithmetic gives a tiny positive `ared`, about `2.78e-17`, but `rho≈2.96e-16`, still decisively below `0.1`.

At the retained base and `Delta=1/4`, Stage1 produces:

- `lambda=7/16`;
- `p0=2^-28`, now below the temperature threshold;
- `p3=1/4`;
- candidate `F3=-29/64`.

This first later domain-valid proposal receives complete evaluation and step reconstruction. It becomes the replacement witness, materializing **the original current base** without installing either proposal. All this happens far above `Delta_min`.

The mass coordinate is essential: a standalone thermal step near `2e-8` would remain interior throughout the permitted radius range.

**Domain-only replacement variant.** Set `F0=0`, make row `3` affine,

\[
F_3=-1/2+z_3/4,
\]

retain `A=I/4`, and give the synthetic model the explicit pre-evaluator domain `z3<=1/2`. The full proposal has `p3=1` and is domain-invalid; the next has `p3=1/4` and is valid. All governed norms are zero. Again, materialize the unchanged current base after the first complete valid replacement.

For first-valid error precedence, inject an owning evaluation error at that latter candidate’s **actual evaluator entry**, after recording the attempt. It must terminate with that owning error and cannot try a later witness. A missing norm must prevent admission; do not manufacture a new error kind merely to label that test.

### 7. Work-cap witnesses without fictitious executed work

Put the shared attempted/entered guard immediately around each actual owned operation, including the Stage1 face-factorization entry. Do not add returned Stage1 totals afterward.

A legitimate boundary test can initialize a **clearly identified synthetic incoming budget snapshot** at the fixed ceiling, then attempt the next real operation:

1. Record the injected starting snapshot separately from observed work.
2. Increment attempted work at the real guard.
3. Refuse before entering the operation.
4. Assert observed attempted delta `1`, observed entered delta `0`, unchanged entry sentinel, and unchanged owner/state.
5. Assert the refusal identifies the actual operation and fixed ceiling.

Use the implemented ceiling constants, not literals copied into an outcome factory. For a `cap-1` snapshot, permit one real entry, including an injected owning error after entry; that failed entry remains charged. The next entry must be denied.

This proves the actual pre-entry boundary. It does **not** prove that 6,500 evaluators or 47,300 factorizations executed; those absolute numbers belong to the injected starting state and must be described that way. Separate unseeded controls establish actual call counts and composed accounting.

### Acceptance implications

These fixtures support a coherent first controller increment: real Stage1 invocation, actual nonlinear candidate evaluation, radius rules, stationary rejection, raw/frozen weighting, and distinct admission paths.

They do not establish physical M1 convergence, all 21 physical raw acceptance obligations, phase/capacity coupling, hydraulic closure, real materialization, or release-harness reachability. Keep those obligations pending for the real adapter increment and its independent reviewers. No new owner decision or method amendment is needed for these bounded synthetic controls; the orchestrator can sequence them under the existing authorization.

### Attributable arithmetic appendix — /root/trust_controller_fixture_adviser

The adviser role’s developer instruction is **read-only**, so I cannot create the artifact directly. The parent can save my previous final response verbatim and append the following evidence.

Draft04 numerical-methods SHA-256, directly observed:

```text
3931d006af19db61f321abd23e0990d088770b297eb23cc56a004c704df5bb63
```

Inspected immutable Stage1 source:

```text
/home/roger/openwepp-experiments/cold-canopy-m1-trust-body16-reconstruction-20260924/crates/openwepp-land-surface-energy/src/m1_trust_region_stage1.rs
```

I did not calculate that file’s individual hash. Its identity was supplied through the retained reconstruction; no independently measured file hash should be attributed to this advice.

**Ran: independent arithmetic only.** Executed with `/workdir/openWEPP/.venv/bin/python`; no Rust or physical evaluator ran. Exact calculation:

```python
from fractions import Fraction as F
import math
for c in (F(0), F(-1,2), F(-7,8), F(-1)):
    r=-1+c; pred=F(3,2); ared=(4-r*r)/2
    print('ordinary', c, 'candidate',r,'pred',pred,'ared',ared,'rho',ared/pred)
base=F(68,25); frozen_trial=F(13,50)
moving_trial=(F(6,13)**2+F(2,5)**2)/2
print('weighted', 'base_phi',base,'pred_ared',base-frozen_trial,'moving_trial',moving_trial,'moving_ared',base-moving_trial)
for d,l in ((1,1/16),(.25,7/16)):
    p0=.25*(2**-27)/(.25**2+l); p3=.25*.5/(.25**2+l)
    f0=-2**-27+.25*p0; f3=-.5+.25*p3-.25*p3*p3
    lin3=-.5+.25*p3
    b2=(2**-27)**2+.25
    pred=.5*(b2-(f0*f0+lin3*lin3)); ared=.5*b2-.5*(f0*f0+f3*f3)
    print('witness',d,'p',p0,p3,'norm',math.sqrt(p0*p0+p3*p3),'ftrial',f0,f3,'pred',pred,'ared',ared,'rho',ared/pred,'temp_pass',p0<=1e-8)
print('radii', [4.0**(-k) for k in range(11)], 'next', 4.0**-11)
```

Exact stdout:

```text
ordinary 0 candidate -1 pred 3/2 ared 3/2 rho 1
ordinary -1/2 candidate -3/2 pred 3/2 ared 7/8 rho 7/12
ordinary -7/8 candidate -15/8 pred 3/2 ared 31/128 rho 31/192
ordinary -1 candidate -2 pred 3/2 ared 0 rho 0
weighted base_phi 68/25 pred_ared 123/50 moving_trial 788/4225 moving_ared 10704/4225
witness 1 p 1.4901161193847656e-08 1.0 norm 1.0 ftrial -3.725290298461914e-09 -0.5 pred 0.09375000000000003 ared 2.7755575615628914e-17 rho 2.96059473233375e-16 temp_pass False
witness 0.25 p 3.725290298461914e-09 0.25 norm 0.25 ftrial -6.51925802230835e-09 -0.453125 pred 0.029296875 ared 0.0223388671875 rho 0.7625 temp_pass True
radii [1.0, 0.25, 0.0625, 0.015625, 0.00390625, 0.0009765625, 0.000244140625, 6.103515625e-05, 1.52587890625e-05, 3.814697265625e-06, 9.5367431640625e-07] next 2.384185791015625e-07
```

This calculation supports the fixture derivation; it is not Stage1 execution, controller verification, or independent acceptance.
