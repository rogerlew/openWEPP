# Numerical tolerance authority

Status: `TERMINAL AUTHORITY RETAINED — BOUNDED CORRECTNESS ONLY`

Evidence mode: `Static + retained measured captures`

## Authority separation

The replacement distinguishes five independent questions:

1. **Nonlinear convergence**: are continuous iterate changes and physical
   residuals small enough that another map is immaterial?
2. **Temporal truncation**: is the accepted support sufficiently resolved?
3. **Physical ledger closure**: do independently reconstructed mass/energy
   operands close?
4. **Constitutive domain**: are inputs finite and inside the physical model's
   domain?
5. **Discrete identity/custody**: are owners, order, topology, events,
   transactions, receipts, and exact-one transfers identical?

Passing one class never substitutes for another. In particular, an exact
receipt digest cannot require bit-exact continuous temperatures, heat, vapor,
or state payloads. The receipt envelope and its discrete lineage remain exact;
continuous payload admission uses the dimensional policy below.

## Canonical continuous stopping policy

The values are maximum absolute/relative differences between each canonical
outer coordinate in the exact charged candidate `x_k` and the corresponding
fresh authentic map output `F(x_k)`. A consecutive earlier output
`F(x_(k-1))` is not the candidate and cannot substitute at the stopping seam.
Both values must be finite. A comparison uses
`abs(a-b) <= abs_tol + rel_tol * max(abs(a),abs(b))`.

| Quantity | Absolute | Relative | Rationale |
| --- | ---: | ---: | --- |
| canopy/snow temperature | `1e-5 K` | `1e-9` | At least 1,000 times below the `0.01 K` adaptive state scale and 10,000 times below typical `0.1 K` forcing precision. |
| top-soil temperature | `1e-8 K` | `0` | Retains the active absolute-only endpoint scale. Soil energy is not inferred from this comparison; its exact high-plus-carry coordinate and independent ledger guard remain separate. |
| heat flux | `1e-5 W m^-2` | `1e-8` | At most `0.036 J m^-2` over one hour; far below a `1 W m^-2` process signal. |
| vapor flux | `1e-10 kg m^-2 s^-1` | `1e-6` | At most `3.6e-7 kg m^-2` over one hour, below the `1e-6 kg m^-2` mass-ledger guard. |
| snow water/storage | `1e-6 kg m^-2` | `1e-9` | Existing covered mass-closure scale; no lifecycle deletion is inferred from it. |
| snow/soil/CN energy coordinate | `1e-6 J m^-2` | `1e-10` | Retains the existing physical energy residual authority. |
| snow density | `1e-6 kg m^-3` | `0` | Numeric density is continuous, but its named constitutive branch, layer cardinality, and ordering compare exactly. No tolerance can cross a density-model branch. |
| represented thickness | `1e-9 m` | `1e-9` | Retains existing derived-depth closure. |
| specific humidity | `1e-12 kg kg^-1` | `1e-8` | Retains the existing LSE/vegetation carrier scale. |

The canonical algorithm may stop only when every applicable continuous
coordinate satisfies this policy **and** the physical residual/ledger,
constitutive, and discrete guards independently pass. A sub-tolerance
non-descent or stagnation is a valid convergence outcome only at that same
fully guarded state; it is not permission to publish a failed residual.

The outer iterated lane coordinates are snow-surface temperature, total
represented water, cold-content energy, density, and derived thickness. Depth
is not an independent degree of freedom: its residual participates in the
multisecant coefficient and convergence guard, while every trial reconstructs
depth from its selected authentic ice mass and proposed density under the
unchanged mass--depth closure. The other four outer coordinates use the stated
multisecant proposal formula directly.
Top-soil temperature, heat/vapor fluxes, and specific humidity complete their
candidate/output closure inside the one authentic carrier map and retain an
additional consecutive-map stability guard. That dependent-output guard is
not an outer fixed-point residual. Numeric per-layer density uses the density
tolerance only while the density-model branch, layer cardinality/order, and
stored settling chronology compare exactly.

## Physical closure and temporal truncation

Physical closure remains independently reconstructed from produced operands.
The following are separate predicates and cannot substitute for one another:

- covered aggregate water/mass closure: `<=1e-6 kg m^-2`, while layer
  lifecycle remains at its existing `1e-9 kg m^-2` represented-fragment
  boundary and vapor/SWE obligations retain their contract-declared units;
- covered physical energy-ledger closure: `<=1e-6 J m^-2`;
- snow/soil transfer custody: the exact same binary64 transferred debit is
  credited with the opposite sign exactly once; there is no tolerance on
  custody identity;
- installed-endpoint CN reconstruction: `<=1e-9 J m^-2`, solely for nonlinear
  termination roundoff and never for changing the transferred debit/credit;
- represented SWE/thickness closure: `<=1e-9 m`;
- Lane D internal transfer and hillslope closure: the existing
  `SC-OFEROUTE-001` / `SC-WATBAL-001` `1e-11 mm` element/adjacent and
  `1e-9 mm` hillslope bounds;
- exact CFL `Cr<=1` remains a domain/step guard, not a tolerance.

Adaptive truncation retains the current scale-aware Stage 3 policy: depth
`1e-9 m + 5e-3 relative`, mass `5e-6 kg m^-2 + 5e-3 relative`, snow energy
`1e-6 J m^-2 + 5e-3 relative`, temperature `1e-2 K + 1e-8 relative`, soil
energy `1e-6 J m^-2 + 1.5e-2 relative`, and LSE energy `1e-6 J m^-2 + 5e-3
relative`. Truncation acceptance cannot waive an A0 ledger failure.

## Exact boundaries

Exact equality remains mandatory for schema versions; lane/OFE/tile/layer and
owner IDs; support start/end ticks; ordering/cardinality/topology; phase and
event identity; transaction/predecessor/target identity; duplicate detection;
exact-one transfer custody; rollback bytes; receipt schema and ordered operand
lineage; and exact dyadic high-plus-carry reconstruction.

The following are explicitly continuous and not bit-exact admission surfaces:
temperatures, density, thickness, liquid/ice/water mass, enthalpy, CN heat,
fluxes, vapor, residual values, and receipt payload fields derived from them.
Their authenticated payload is selected from one fresh authentic accepted
physical evaluation. The producer constructs a new immutable receipt from
that exact payload and canonical discrete envelope; it does not edit or repair
an earlier receipt. The consumer verifies the envelope, reconstructs every
continuous ledger from the sealed payload under its named tolerance, and then
requires byte-identical transfer of that already accepted receipt. A replay
reconstructs the physics/ledgers from immutable inputs and compares continuous
results under the same named tolerance while comparing the discrete envelope
exactly. Receipt schema, field order, operand identity, units, support, branch,
topology, transaction, predecessor, and digest framing remain exact.

Density-model identity is part of the exact envelope. A tolerance-equivalent
density value is admissible only when both authentic evaluations selected the
same exact constitutive branch and layer topology; the accepted density bits
come from the one accepted authentic evaluation and are sealed once.

## Physical-evaluation exhaustion

One accepted support receives at most eight charged authentic physical-map
evaluations: one initial map, no more than six safeguarded trial maps, and one
final authentic accepted map. The first trial may be exactly one bounded
authentic fixed-point predictor from the initial map; every later trial uses
analytic constitutive derivatives or multisecant information from already
charged authentic maps. Multisecant residuals are dimensionless: every
coordinate uses its applicable tolerance scale over the two proposal/map
pairs, and raw mixed-unit Euclidean products are forbidden. The depth-one
coefficient is `alpha_raw=-(r_k dot (r_k-r_{k-1}))/||r_k-r_{k-1}||^2` with
proposal `F_k+alpha*(F_k-F_{k-1})`. Every finite nonzero coefficient is
safeguarded as `alpha=max(-0.75,min(alpha_raw,1))`. Negative values are strict
convex contractions of two charged authentic map outputs; positive values are
capped at `1`. Before a charge, the complete proposal must be finite,
domain-valid, and distinct from both endpoint proposals. Zero, nonfinite
coefficients, degenerate denominators, and repeated endpoints fail typed
without charging another map. The negative saturation is part of the one
canonical depth-one controller,
not a fallback, and cannot be re-entered after a failed later trial;
coordinate finite differences cannot fit this budget and are prohibited. A
prospective preflight must reserve the complete next bounded step; no partial
Jacobian, lattice, receipt-cycle, or second physical replay tail is allowed.
Independent receipt/ledger reconstruction is arithmetic over the accepted
payload and does not invoke the physical map.
Exhaustion returns the canonical same-regime adaptive-support request above the
60-second floor, or a typed `EvaluationBudget`/nonconvergence error at the
floor. It never dispatches another solver.

The release CLI01 qualifying capture at exact-floor support `7980..8040 s`
recorded dimensionless `dot=362742.313896017557`,
`norm=325841.810884757957`, and
`alpha_raw=-1.11324667915103870`. The saturated `alpha=-0.75` step changes the
local squared residual model relative to `alpha=0` by
`2*alpha*dot+alpha^2*norm=-360827.452672354`, so it is strictly improving.
The value is the canonical negative multisecant safeguard, not a solver
fallback or a convergence inference. Tests bind exact `-0.75`, its adjacent
binary64 values, raw `-1`, values below `-1`, arbitrarily large finite negative
values, zero, nonfinite inputs, degenerate denominator, exact
`0.25*F_k+0.75*F_(k-1)` construction, endpoint distinctness, and charge
accounting.

## Sensitivity obligations

Before production admission, contract-derived vectors must pass through the
actual production stopping/admission seam and perturb every
continuous coordinate to `0.5x`, `1x`, and next-binary64-above the bound in
snow-free, strictly frozen, mixed-phase, thaw/refreeze, wet-canopy, and 10+-OFE
Lane D cases. They must show:

- both perturbation signs and both absolute-dominated and relative-dominated
  comparisons are exercised using an independent binary64 oracle;
- `0.5x` and exact-bound perturbations can converge only with every independent
  closure/guard passing;
- above-bound perturbations continue or fail typed;
- discrete one-bit identity/order/event/custody changes always fail;
- accepted output changes remain below one tenth of the smallest protected
  process/reporting signal for the fixture;
- tightening every continuous tolerance by 10x and loosening by 2x changes no
  discrete branch/event/owner outcome and keeps independent closure within its
  unchanged bound.
- wrong-field and wrong-tolerance-ID substitution, cross-coordinate
  substitution, nonfinite values, one-bit discrete poisons, and an uncharged
  physical-map call all fail.

The Phase-2 contract test is expected red until this production seam exists.
The complete matrix must pass in Phase 3 before any representative or
publication claim; moving the dynamic run after implementation is sequencing,
not deferral or waiver.

Retained r54 evidence is the motivating anti-exactness vector: a one-ULP snow
temperature difference (`263.204229777162197` to
`263.204229777162254 K`) changed receipt Q by approximately
`1.6e-11 J m^-2` while the physical root was already tolerance-closed. Searching
for an exact receipt cycle or Q lattice was therefore numerical work below any
represented physical signal. This measured capture supports removing exact
continuous replay; it does not by itself satisfy the full terminal sensitivity
matrix, which remains a required pre-production contract/test gate.
