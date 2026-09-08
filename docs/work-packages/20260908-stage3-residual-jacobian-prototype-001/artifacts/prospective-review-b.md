# Independent prospective QA/science review B

Static: reviewed the primary-checkout v33 diff from scaffold commit
`827a7470e6357c431b00757674cc7990d9feeb1c`, the package and handoff,
`authority-and-protocol.md`, the active kickoff, the complete reviewer procedure,
testing-strategy sections 7--10 and 17--18, applicable science obligations, the
LSE interface/solve/numerical-method authority, and the canonical primal source
at `solver_covered_evaluation.rs`, `solver_covered_solve.rs`, and `physics.rs`.
No candidate implementation or result was reviewed. Reviewer A's result was not
read.

Ran: attempted
`cargo nextest run --test land_surface_energy_balance_authority_contract version_thirty_three_binds_experimental_residual_jacobian_policy`
from `/workdir/openWEPP`; NOT RUN because `cargo` is unavailable in this review
environment (`bash: cargo: command not found`, exit 127). All findings below are
static.

Role/session: prospective reviewer B (secondary QA/science); configured/requested
effort: secondary QA review; effective runtime setting: UNOBSERVED.

Reviewed identity: dirty primary-checkout authority/test diff relative to
`827a7470e6357c431b00757674cc7990d9feeb1c`; no detached A/J source manifest,
corpus manifest, or expected-red evidence exists in the package. Assigned scope:
target/order/units/normalization, stem dependency completeness, branches/kinks /
zero-area, independent FD/Taylor oracle, numerical bounds, comparison classes,
cost protocol, isolation/write set, and staged preimplementation gates.

## Findings

### B-H1 — HIGH — The authoritative stem dependency map is not the executed reciprocal-longwave graph

Location: `docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001/numerical-methods.md:101-118`; mirrored by
`docs/work-packages/20260908-stage3-residual-jacobian-prototype-001/artifacts/authority-and-protocol.md:14-20`.

Evidence: v33 declares every positive-area component-energy row in every
occupancy affected by a stem-temperature column and says occupancy never bounds
the dependency. In the canonical primal, however,
`physics.rs:386-409` computes a layer component's incident term from
`downward[index] + upward[index+1]`; neither boundary contains that same layer's
emission. Therefore `T_stem,o` affects components in layers above/below `o`, its
own stem row through its direct `-2*w*(1-tau)*sigma*T^4` emission (and sensible
heat), and the ground, but it does not affect the sun/shade/wet energy rows in
the same occupancy through longwave. Those same-layer cross-component entries
are exact zeros. The present statement conflicts with the exact-zero claim and
makes the required independent sparsity oracle, omission perturbations, and
charged "complete affected-column" denominator non-reproducible.

Required disposition: amend the canonical authority and protocol with an
index-relative row map (`i<o`, `i=o`, `i>o`) derived from the executed downward /
upward recurrence, explicitly separating structural candidate rows from
mathematically nonzero entries. Bind same-layer exact-zero and both-direction
cross-occupancy tests. Re-review the corrected stable cut.

### B-H2 — HIGH — The FD/Taylor oracle is under-specified and can classify disagreement as “unsupported”

Location: `docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001/numerical-methods.md:168-204`; mirrored by
`docs/work-packages/20260908-stage3-residual-jacobian-prototype-001/artifacts/authority-and-protocol.md:42-52`.

Evidence: nine estimates are requested, but the text never defines which
estimate or extrapolation is `J_oracle`; “resolvable truncation/roundoff basin”
has no numerical predicate; and the roundoff term uses an unbound `h` after a
nine-step sweep. The Taylor rule likewise does not freeze direction vectors,
their units/norm, initial `t`, row/vector remainder norm, or the precise
floor-reaching stop test. Finally, “otherwise the state/row is unsupported or
fails” leaves the candidate-comparison disposition discretionary. Independent
implementations can select different oracle values, uncertainty bounds, and
pass populations from identical bytes.

Required disposition: freeze a deterministic basin-selection and tie-breaking
algorithm, selected `h`/estimate (or extrapolant), uncertainty arithmetic, Taylor
directions/scaling/initial steps/norm/stop rule, and a baseline-only eligibility
mask. Once candidate bytes exist, a baseline-admitted comparison that disagrees
must fail rather than become unsupported. Add oracle self-tests using synthetic
smooth, roundoff-dominated, kink-crossing, and deliberately wrong derivative
functions. Re-review before expected-red/candidate implementation.

### B-H3 — HIGH — Normalization branch semantics and admission do not completely match the source formulas

Location: `docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001/numerical-methods.md:120-145` and
`docs/work-packages/20260908-stage3-residual-jacobian-prototype-001/artifacts/authority-and-protocol.md:22-30`.

Evidence: the contract mentions absolute-value signs and “shared maximum-based
tolerances,” but the affected source normalizers are not one generic maximum.
Component energy uses
`energy_tolerance(sum(abs(shortwave,longwave,sensible,latent)))`
(`solver_covered_evaluation.rs:1443-1477`); shared heat uses
`energy_tolerance(max(abs(canopy_sensible)+abs(ground_sensible)+abs(reference_heat),1))`
(`:2203-2220`); ground energy uses
`energy_tolerance(sum(abs(surface_operands)))` (`:2309-2321`); and
`energy_tolerance` itself applies another `max(scale,1)` (`physics.rs:585-588`).
The text neither specifies the below-floor zero derivative nor explicitly
certifies that each affected sum stays on one side of the `1 W m^-2` floor. Its
unique-maximizer language does not describe shared heat's sum-of-absolute-values
formula. Also, “positive area” does not bind eligibility to the executed dry
stem area `(1-wet_fraction)*stem_area` (`solver_covered_evaluation.rs:1273-1279`),
which can be zero while raw stem area is positive.

Required disposition: enumerate `R_i`, `s_i`, and `ds_i/dT_stem` by affected row
class, including every abs sign, the `1 W m^-2` floor branch, wet-inactive
predicate stability, and exact-zero derivative cases. Define active dry stem as
positive finite executed `dry_stem` (or state a different source-backed rule),
and define the certified interval quantitatively relative to temperature bounds
and all kink surfaces. Add floor-below/floor-above/floor-tie, abs-near-zero,
wet-fraction-one, and wet-inactive-transition vectors.

### B-H4 — HIGH — The required preimplementation cut is not actually frozen and sequencing is contradictory

Location: `docs/work-packages/20260908-stage3-residual-jacobian-prototype-001/package.md:68-75,88-90`;
`artifacts/preimplementation-contract-gate.md:8-20`;
`artifacts/required-reading-map.md:22-25`;
`artifacts/authority-and-protocol.md:54-66,104-112`.

Evidence: package authority requires the corpus, protocol, contract-derived
tests, and captured expected-red evidence before dual prospective review.
Instead, the protocol describes a future corpus without state/fixture identities,
the gate says expected-red follows accepted review, the binding test is
implemented but not run, and mandatory LSE interface/numerical/solve/dependency
reading is still recorded pending after the authority was declared frozen.
`A-source-05` is named but has no package-local exact source-kit path, base/patch
identity, detached implementation file/function write set, or build-input
manifest. Thus the reviewers have neither the frozen corpus nor the concrete
initial write set the kickoff requires them to approve, and the current
`PASS-STATIC` dependency/oracle/cost rows overstate the evidence.

Required disposition: complete and truthfully record mandatory reading; bind the
exact A source/build inputs, detached path, implementation files/functions,
fixture/corpus records and baseline-only masks; execute the binding test and
capture expected-red behavior before prospective approval in the package's
declared order. Mark incomplete rows `NOT RUN`/`BLOCKED`, then present one stable
substantive cut for both independent reviews.

### B-M1 — MEDIUM — Accepted-result bounds are not a dimensioned, enumerated comparison contract

Location: `docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001/numerical-methods.md:206-217` and
`artifacts/authority-and-protocol.md:68-77`.

Evidence: `2e-7 + 2e-9*max(|a|,|b|)` is assigned generically “in each published
field's declared units,” without enumerating compared fields, their units/bases,
absolute terms, alias treatment, or the applicable existing closure/publication
limit. A single unitless-looking literal cannot be independently applied to W
m^-2, kg m^-2 s^-1, kg m^-2, or receipt operands. “No looser than” also does not
define the operative minimum when publication resolution and closure thresholds
apply to different quantities.

Required disposition: add a field-level comparison table with units, area/time
basis, exact-vs-toleranced class, absolute and relative term, governing closure /
resolution bound, and final combination rule. Keep independent A0 reconstruction
separate from A/J agreement.

### B-M2 — MEDIUM — The local work-count and wall-attribution criteria lack unambiguous counter arithmetic

Location: `docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001/numerical-methods.md:219-240` and
`artifacts/authority-and-protocol.md:79-91`.

Evidence: the tenfold criterion combines complete residual evaluations and
nested leaf/constitutive solves without defining whether heterogeneous/nested
events are summed, separately gated, or weighted; candidate derivative,
validation, assembly, and scratch work is said to be “counted” without a common
count unit. Likewise, the >=75% broad-coverage gate does not freeze an exclusive
timing attribution method for overlapping base evaluation, Jacobian assembly,
and solver overhead. Two conforming collectors could compute different admission
ratios.

Required disposition: define named monotonic counters and exact formulas for
each call-count ratio (reporting nested events separately), and freeze mutually
exclusive timing spans plus the weighted aggregation over corpus classes. Retain
the already-clear paired local time criterion as a separate decision axis.

### B-M3 — MEDIUM — The binding test is prose-fragile and does not validate the new binding relationship

Location: `tests/integration/land_surface_energy_balance_authority_contract.rs:6-53`.

Evidence: the test asserts line-wrapped prose fragments and mere global presence
of the invariant, obligation, and exposure IDs. It does not assert the exact
binding-index row maps `LSE-V33-EXPERIMENTAL-RESIDUAL-JACOBIAN` to INV-165/C-021,
that both IDs resolve to the numerical-method anchors, or that lifecycle/status
and production-HOLD wording are coherently bound. Benign reflow will fail while
several material misbindings can pass.

Required disposition: retain only stable semantic tokens in the method-body
check and add exact row/anchor mapping assertions through the canonical directory
parser. Execute the test under the package's declared Rust toolchain before the
gate can pass.

## Non-blocking debt / follow-ups

- `artifacts/worker-handoff.md:26-35` and `artifacts/required-reading-map.md:22-25`
  are stale relative to the frozen v33 diff. Reconcile them when the corrected
  review cut is prepared; do not treat this documentation drift as evidence that
  required source/authority reading occurred.
- Record the exact Rust line-count disposition once the detached implementation
  write set exists. The current primary diff has no Rust production change, so a
  production-source line-count judgment is premature rather than passed.
- Before any heavy gate or measurement, record the required
  `comparator_suite_runner` execution identity or the one permitted genuine
  service-failure fallback. No heavy gate was attempted in this review.

## Recommendation

**HOLD.** The prospective authority is not implementation-ready: the dependency
map disagrees with the executed longwave recurrence, the normalization and
independent-oracle algorithms are not deterministic enough for separate
implementations to agree, and the package has not supplied the frozen corpus,
source/write-set identity, expected-red evidence, or successfully executed
binding gate required at this stage. Production HOLD remains correct. Resolve
B-H1 through B-H4 and B-M1/B-M2, strengthen and run the binding test, then obtain
focused independent re-review of one corrected stable cut before experimental
Rust implementation.
