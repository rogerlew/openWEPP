# Prospective v34 correctness review A

Static: canonical prospective v34, planned isolated write set, and frozen A source.
Ran: read-only source/authority inspection and SHA-256 identification only;
no Rust build, test, capture, reference calculation or measurement by this reviewer.
Role/session: /root/v34_correctness, primary correctness reviewer.
Configured/requested effort: high; effective runtime settings: UNOBSERVED.

## Findings

No blocking prospective correctness finding in the reviewed design. This is
permission to proceed with contract-derived expected-red tests and the isolated
implementation after the other prospective review, not admission of an unseen J.
No finding is deferred or used to waive current package acceptance.

Reviewed canonical chapter:
`docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001/inactive-jacobian.md`,
SHA-256 `e8917d559b9f33d3939c4db4a63567017961751a05984e83d2b4cc83ebf7f6d6`.
Reviewed exact planned write set: package `artifacts/authority-and-protocol.md`,
SHA-256 `9bc597cb0879e3527dab68df8e8c9a067f7145c8f7f73338faa52a04c9e76726`.
Registry/binding integration was still being completed by the parent and is not
certified by these two identities. Source inspected below is in
`/tmp/openwepp-hotpath-jacobian-jmvOZh/A`, not production repository HEAD.

## Independent correctness checks

- `solver_covered_evaluation.rs:1290-1590`: exact input leaf area times the
  unchanged dry fraction supplies component area. The positive-zero component
  branch selects the anchor residual and `energy_tolerance(1.0)` separately.
  The own column is therefore reciprocal-normalizer, with no derivative of
  anchor or normalizer with respect to the selected leaf temperature.
- `solver_covered_evaluation.rs:719-775,1100-1288`: fallible biochemical and
  saturation work precedes the inactive gas branch. The gas branch is Inactive,
  not a promise that every diagnostic is constant: surface humidity and
  stomatal resistance still depend on temperature. Hydraulic/beta residuals
  and their normalizers have no mathematical dependence through the selected
  zero-area flux. Ordinary base evaluation must remain authoritative.
- `physics.rs:340-417`: selected zero emissive weight removes its temperature
  from layer emission and every propagated longwave boundary. This establishes
  the cross-occupancy part of the mask, rather than assuming a local diagonal.
  Physical temperature guards are still executed in the base evaluator.
- `solver_covered_evaluation.rs:2485-2690`: the actual V3 wrapper replaces shared
  vapor, surface and soil rows using canopy, ground, soil, context and radiation
  operands. None reintroduces dependence on the selected inactive coordinate.
  `solver_litter_phase.rs:103-123,200-303` supplies the actual scaled-to-physical
  transform and preserves post-assembly inactive-row adjustment.
- `solver_covered_solve.rs:643-779` and `numerics.rs:180-226`: canonical FD work
  is constructed before inactive-row replacement. Complete canonical columns
  and the subsequently consumed identity rows are different obligations.
  Existing Covered unit multiplication must not be applied twice; the generic
  scaled-coordinate column must use the coordinate scale, not its FD unit.
- The prospective comparison budget has units K^-1 and sums to at most eta
  normalized units under the stated bounded physical perturbations. Actual
  represented spacing, an explicit affine evaluation-roundoff allowance, and
  independent reciprocal/row-mask checks avoid requiring a fictitious
  nonzero truncation basin. Large uncertainty remains unresolved, not a
  retrospective widening. No numerical calculation was run to certify U.
- Borrowed same-sweep capability, unchanged base-error precedence, no FD retry
  after entry, and one shared private column module are appropriate boundaries.
  The planned write set does not require a new solver, physical formula,
  unknown elimination, public selector or serialization of validated state.
  No substantial duplicated Rust implementation exists yet to assess.

## Residual risk and missing evidence

J is not implemented or accepted by this review. Mandatory remaining evidence
includes corrected cut2 capture and reconciliation, actual solver observation
off/on equivalence, expected-red bindings, complete-column and directional
references for Covered Potential and both GenericV3 passes, non-unit mapping,
positive-area/negative-zero/invalid-base negatives, capability misuse and
arbitrary-cardinality checks, independently accepted solves and consumer
water/energy closure, custody/rollback/restart, and charged runner cost.
Probe-only failures may differ lawfully; this never excuses an invalid base.
Bitwise off-diagonal invariance must be tested on canonical residuals, not on
temperature-dependent diagnostic records or a floating-point plateau alone.

The critical numerical change retains full applicable correctness regression
and A0/A1/A3 obligations. Prospective review is not terminal review or stable-cut
verification. No gate is silently deferred. Rust line-count reconciliation and
source/build/fixture identity binding remain implementation/terminal duties;
this review writes only its assigned Markdown artifact.

Verdict: PASS for the bounded prospective design and exact planned write set,
subject to the independent second review and subsequent executable evidence.
Production remains HOLD; the package remains executing.

## Focused prospective write-set refinement

Static: rechecked the protocol after adding the isolated
`crates/openwepp-land-surface-energy/src/solver_inactive_jacobian_oracle.rs`.
Current protocol SHA-256:
`3c4c47b3ad9141901e4521b4ddf72d8647f539cfb70c2041ca5507f9a59ee7c8`.
The canonical v34 chapter retains the previously reviewed SHA-256 above.

No blocking finding. This is a sound separation of already-required evidence:
the independent reference worker owns baseline-only complete-column FD,
affine/directional checks and accepted energy/water operand reconstruction;
the candidate worker retains candidate code/tests and `solver.rs` registration.
The additional file does not authorize candidate-derived expected values,
duplicate production physics, new tolerances, or a precision framework. Its
reference implementation must remain independent of the J column helper and
reconstruct closure from accepted physical operands rather than copying the
producer residual. Those requirements were already mandatory.

Verdict: PASS for this prospective write-set split. Unchanged substantive
design review is retained; no implementation or executed evidence is certified.

## Focused prospective accepted-operand audit refinement

Static: inspected the appended observation-only write set and hook surroundings
in detached J; no builds or tests run. Protocol SHA-256 reviewed:
`f9d1a1b246f11130b57193523592f26f70789c1726e6ee98310d945a1de2658e`.
Canonical v34 math and budgets retain the original reviewed chapter hash.

Finding AUD-PLAN-01 (low, source-description precision): the protocol says all
three hooks follow existing successful operand/receipt validation. In
`transaction_v3_bridge.rs:168-217`, however, the V3 operand set is constructed
without a whole-set validator at that point. Describe this hook as following
accepted-operand construction, with independent observation-time raw-operand
validation. Do not imply an existing complete V3 set check or introduce a new
production validation decision to make that wording true. Parent notified;
wording correction pending at this review cut. This does not block the scoped
observer implementation under the precise interpretation below.

No blocking design finding. Ordinary `transaction.rs:2100` validates its
assembled energy operands; `transaction_v3.rs:183-187` validates the phase
receipt before candidate return. `covered_output.rs:477` reconstructs component
and shared-air operands, while `litter_phase_closure.rs:46` reconstructs mass,
phase-free storage, signed vapor and fusion/ending-capacity relations from
physical primitives. Reusing these existing independent-of-Newton checks is
appropriate; observing normalized residuals or merely comparing stored closure
fields is not sufficient. The V3 bridge requires its advertised independent
audit checks on the constructed raw set, not a presumed whole-set check.

New `solver_accepted_operand_audit.rs`, cfg-only `lib.rs` registration and the
three cfg hooks are within the existing consumer/closure evidence obligation.
Every observed set must be validated, with bounded retained identities plus
whole counts; truncation, overflow, unexpected lifecycle or validation errors
must invalidate evidence. Observer failures must neither mutate nor propagate
into ordinary physical outcomes. Untimed enablement and an inactive fast path
must be verified; timing cannot include active audit/reconstruction work.

Acceptance is bounded to accepted physical candidate construction, not final
publication or independent soil-interior closure. Existing consumer transfer,
within-run custody and remaining closure gates are neither replaced nor
deferred. Rejected later joins can leave an observed candidate, so reports must
preserve this distinction. Required executable follow-up includes observer
neutrality, poison detection, nonzero per-family counts and source-hook coverage.

Verdict: PASS for the bounded prospective evidence plumbing, with the low
source-description correction tracked above. No observer implementation,
whole-run result or timing admission is certified by this static review.

Focused correction re-review: AUD-PLAN-01 is FIXED and independently rechecked.
Current protocol SHA-256:
`8bced9f867048e95dd303c777d2387b525ca814b6088bda2e5fe6b61dacf784c`.
The revised text explicitly distinguishes V3 construction from prior whole-set
validation and places its independent check in the audit, without changing
production validation. Final prospective plumbing verdict: PASS, no open
blocking findings. The scoped cfg-only worker implementation may proceed.

## Separate physical-closure admission review (in progress)

Static: existing detached-J raw-operand validators, their actual transaction
hooks and canonical owner requirements. Ran: source/authority reads only;
no Rust poison execution or actual audit consumer result certified here.
This scope is distinct from prospective derivative and observation-plumbing
approval. It does not claim full snow/soil interior or WB14 qualification.

### Open findings

- CLOS-A01 (medium, evidence admission): `covered_output.rs:558-568`
  reconstructs shared heat/vapor residuals but compares them against supplied
  positive finite tolerance fields, without independently deriving their
  canonical values. Changing reference-air flux, updating the stored residual,
  and inflating the supplied tolerance can evade that isolated check. Audit
  admission must reconstruct source-correct thresholds from raw operands and
  reject this poison. For represented snow the source normalizer uses exact
  zero ground-sensible/final-ground-vapor scale operands, while the residual
  uses actual snow carriers; do not substitute carrier values into its scale.
  Source: `solver_covered_evaluation.rs:2188-2254`. Audit worker notified;
  implementation reported but independent inspection/tests still pending.
- CLOS-A02 (medium, finite-domain evidence):
  `CoveredSurfaceEnergyOperands::validate`, `covered_output.rs:73-84`, checks
  finite inputs but not derived latent/residual/scale. Finite latent heat and
  signed vapor each equal to `1e308` produce infinite product, residual and
  scale; `abs(residual) > energy_tolerance(scale)` is false for infinity versus
  infinity. This is a static standalone-validator counterexample, not a run
  result or proof that a real accepted solver emits it. Audit admission needs
  finite intermediate checks/poisons or a proved enforced valid-record bound.
  Worker notified; no production validator change authorized by this review.

### Reading and claim frontier

For this additional task, SC-VEGETATION-001 was read completely through line
3080; two truncated table portions were separately reread. Current LSE
interface, common-details, surface-energy, soil-coupling, water-vapor,
terminal-support, numerical-methods, nonlinear-solve, solve-boundary and
litter-phase were read in this review session. Complete selected external-owner
reconciliation is still in progress: SC-SURFACELIQUID-001 read through line
220 at this checkpoint; remaining applicable external owners are not claimed
fully read. No blanket science approval follows from prior prose, a validator
name, or its invocation count. Volume is recorded as reading work, not a waiver.

Current disposition: NOT YET ADMITTED for the actual consumer physical-closure
claim. Keep implementing and checking the bounded observer; close these
evidence findings and remaining applicable authority/poison coverage before
using its success to admit timing. Production remains HOLD.

### Completed owner reading and first audit-source review

Static: complete selected external owners now read in this review:
SC-VEGETATION-001 (3080 lines), SC-SURFACELIQUID-001 (2205),
SC-SNOWENERGY-001 (3559), and SC-COUPLEDTIME-001 (1108). Truncated table
segments were separately reread. Also completed LSE surface-custody and
map-custody alongside the physical chapters listed above. Current ADR-0044
overrides were applied; historical 96-map dispatch and old terminal policies
are not revived. WATBAL ingress/partition, rain-temperature provider derivation,
exact surface/soil carry reconstruction, and complete final publication are
outside this bounded component/shared/boundary plus litter-primitive claim.
Their unrelated qualification is not asserted or replaced by these checks.

First audit-source hash reviewed:
`f17d5e79dd27b70566a23334e0de96e22ba02137ce63a0fa526183ed7d690a84`
(`J/crates/openwepp-land-surface-energy/src/solver_accepted_operand_audit.rs`).
No Rust execution performed by this reviewer.

CLOS-A01: the source correction is sound prospectively. It reconstructs
occupancy-ordered component sums and canonical heat/water limits, checks finite
residuals and limits, and requires exact supplied-limit bits. Represented snow
is selected only after the existing authority/lower-boundary validation; its
actual carriers enter residuals while exact-zero ground operands enter scales.
The intentional audit-only formula duplication is independent reconstruction,
not a second production solver or configurable tolerance. Executable heat and
vapor laundering poisons on actual retained vectors remain required; the first
source cut implements only the heat poison.

CLOS-A02: explicit canopy latent/carrier/T^4 and lower-boundary interval
products plus a finite absolute-sum bound address the demonstrated component
overflow, but first-cut litter coverage is incomplete. JSON represents
nonfinite floats as null, so `numeric_bound` alone cannot prove raw finiteness.
In `litter_phase.rs:452-470`, a finite large depth can overflow the warm/cold
energy-mass bound and `min(available)` can conceal that infinity in the actual
transfer. Matching infinite bound fields survive ordinary `PartialEq` replay.
Require explicit finite transfer fields, heat-capacity/sensible-coordinate,
fusion/ice-enthalpy and phase-free energy-rate-times-duration intermediates.
The worker received these bounded audit-only corrections; no production
validator change is authorized. Bind a poison through `Raw::validate`, not only
through the standalone `finite(product)` helper.

Scientific reconstruction assessment: component energy is SW + LW - sensible
- latent; shared air uses the same occupancy/component order and actual ground
carriers; snow-free ground has independently reconstructed surface/latent and
equal-opposite ground-heat joins. The represented-snow boundary check reconstructs
canopy-to-snow energy and receipt joins, not snow interior storage. The V3
adapter reconstructs canopy energy only; serialized `litter_surface` is not
validated by that adapter. Separate litter receipts reconstruct phase-free
storage, liquid/ice mass, signed vapor energy, fusion, U-Lf*ice, ending heat
capacity and temperature. They do not prove later spill/ingress/exact-carry or
final-publication closure. These claim boundaries must remain explicit.

Residual evidence: audit edge-vector observer neutrality is not complete runner
on/off equality; actual runner parity and complete counts remain separate
obligations. Actual records may originate from candidates later rejected by an
outer join, so none can be called final published state solely from this hook.
Verdict remains NOT YET ADMITTED pending focused corrections, independent
re-review and executable poison/runner evidence, not a stop to J work.

### Prospective local-column cost harness refinement

Static: reviewed the parent-proposed cfg(test)-only
`solver_inactive_jacobian_cost.rs` plus `solver.rs` registration write-set
extension; no implementation or timing result reviewed. No mathematical blocker
under the following frozen comparison: identical replay cases/order, fresh base
evaluation per repetition in both arms, complete eligible columns, actual FD
stencil/displacement and unit scales, and J capability/eligibility/direct
assembly costs included. Existing inactive-row replacement must remain in the
same position. Checksums consume full columns and cannot substitute for prior
numerical admission.

Two warmups per arm and calibration select one common repetition count before
30 deterministic balanced AB/BA paired batches. Both arms must reach the
100-ms floor; a capped or short attempt is retained as invalid, not admitted or
silently pooled. Record every pair and refusal without outcome-based trimming.
The harness remains a local column-construction cost experiment, not whole-run
speedup. Actual numerical and runner admission precede measured batches.
Verdict: prospective PASS for this bounded extension; exact-source and
execution review remain pending.

### Existing bounded physical poison coverage (source inspection)

Static: read `covered_output.rs` tests for component area/conductance/humidity,
vapor authorization, omitted shared-ground carrier, SW/LW reconstruction,
represented-snow one-bit joins and finalized-water latent joins. Read
`litter_phase_tests.rs` receipt reseal/raw-humidity and phase-free ice-alias,
vapor-only and producer-residual rejection tests, plus phase mass/enthalpy and
old-capacity/wrong-sign checks. These are suitable existing focused regression
groups for this bounded claim; execution remains a runner obligation. The
old-capacity/wrong-sign test proves mutant values differ, not that a fully
mutated receipt is rejected. New audit finite-domain and both heat/vapor
tolerance-laundering poisons remain separately required.

### Exact J core source review

Static: reviewed direct A-to-J deltas (not repository-HEAD deltas), complete
private derivative implementation, call sites, source dependencies and guard
helpers. No new correctness finding in this core cut. Reviewed SHA256 values:

| J source | SHA256 |
|---|---|
| solver_inactive_jacobian.rs | ae26fe918d113d3ad9adb634fa3795161b7d36b59ff61280fcbc2dd4cf8d81b3 |
| numerics.rs | ceabb0d248d8d6ffd019607afdd783716cf08ffb92653adb83ed7331c63b7225 |
| solver_covered_solve.rs | aa448182cc1fc7d93ab9a91df92bca26284de04159d224e7633f62fa8fbbe8a8 |
| solver_litter_phase.rs | 8c4f80545682e5c6dbd3bbb4f2f4f42cf58563bfc627e296dcc9ddbda945452f |
| error.rs | b5a23770850db97aa0a08b3354741788a7c8e13ede72785ea31e1773673fe3be |

The existing generic sweep is extended through one private provider; the default
provider retains FD and is not an alternate nonlinear solver. Covered base
construction now consumes the actually evaluated borrowed trial, removing the
independent trial argument. V3 owns the freshly evaluated physical coordinates,
borrows exact inputs/caps, retains copied immutable context, and refuses a frozen
probe as a base. Its provider checks every scaled-coordinate product bit and
freshly derived frozen branch state. No independent evaluation constructor,
Clone capability, public selector, or FD retry after integrity failure exists.

Eligibility implements authority plus positive-zero input and successful
component areas; it does not admit wet-fraction-only inactivity. Matrix/scale/
normalizer and affine identity checks precede writes. Complete columns have
D[j]/s on the diagonal and positive structural zeros elsewhere. Wrong order,
shape, repeated use and failed assembly poison the sweep. The typed integrity
error maps to the existing numerical class. Covered and V3 row adjustment,
RHS anchors, subsequent Covered unit scaling, pivots and controller remain
unchanged after complete assembly.

Rechecked complete dependency provenance: fallible gas preprocessing remains
before the inactive branch; selected zero-area demand cannot enter hydraulic
flows or normalizers; reciprocal longwave has zero selected emission weight;
wet routing reads wet temperature; shared carriers are explicit zero; ground
and all soil equations read unchanged radiation/shared/ground operands. V3
replacement vapor, phase-free storage/surface and soil rows introduce no
selected-leaf-temperature dependency. Removing probe-internal failures is the
expressly admitted trajectory difference, not a base-guard waiver.

Residual evidence: source-real guard tests compare six ordinary first errors;
wrong scaled trial/scale/frozen state and sweep lifecycle are executable
negatives. Foreign input/caps/context pairing is excluded by construction,
not independently exercised by the lifecycle counter. Three structural FD
cases directly test the shared eligibility predicate; they are not three
full physical solves. Nonunit scale tests use a real V3 base. Arbitrary matrix
dimensions exercise assembly, not arbitrary-topology full solve qualification.
Independent oracle exact-base replay is currently reported failing and remains
an admission blocker until diagnosed without comparison relaxation. This
reviewer ran no Rust tests. Verdict: source-level PASS, not numerical or terminal
package approval.

### Corrected accepted-operand source re-review

Static: corrected audit SHA256
`22c79f32e2b038f825c2e1e4b8a68ec89bf2be98d7932fdc184f106d86d1da03`.
CLOS-A01 and CLOS-A02 source corrections accepted. Both heat and vapor
laundering poisons independently modify actual retained raw vectors, require the
old supplied-threshold validator to pass, and require the canonical audit to
reject. Raw overflow poisons now enter `Raw::validate`; explicit transfer fields,
beginning/post/ending capacity and sensible products, fusion/ice enthalpy and
vapor-duration products close the identified finite-arithmetic gaps. The finite
absolute-sum guard is evidence arithmetic protection, not new physics or a
caller-tunable acceptance limit. Existing typed validators still supply raw
field/domain checks; JSON-null handling alone is not relied on for transfer
finiteness.

V3 serialization now labels canopy-only energy. The new real litter transaction
poison reconstructs input from a retained receipt, makes its liquid authorization
exceed the available pool, compares off/on first typed errors and immutable
input, and verifies no hooks ran. This is one pre-hook litter failure, not
post-hook rollback or all transaction failure qualification. Edge-record replay
still does not replace complete actual-runner off/on equivalence.

Verdict: source-level PASS for the bounded physical evidence checker; execute
focused poisons, existing reconstruction regressions, complete actual-runner
neutrality and whole-record count reconciliation before physical admission.
Production remains HOLD; no terminal or performance approval is implied.

### Bounded A0/A1/A3 applicability map for exact J diff

Static: direct command selection, not execution or a replacement gate planner.
Read current correctness-authority model and external-authority framework;
inspected the registry and actual Cargo/test sources. Registry SHA256 is
`aab73a15bbef3d3d442225073240dc0c7840fae95314ea1583a98c026ebd7f47`
and matches root and J.

- A0: current canonical v34 INV-166/OBL-C-022 and existing owner dependencies
  remain the authority admission basis. Run in exact J:
  `cargo nextest run --profile full --test land_surface_energy_balance_authority_contract --test solver_architecture_authority_contract --test v10_nighttime_authority_contract`.
  These test registered authority/source bindings; they do not replace numerical
  or physical evidence. Current v34 design already has dual prospective review.
- A1: `cargo nextest run --profile full -p openwepp-land-surface-energy` covers
  touched generic numerical behavior, typed/domain guards, covered component/
  radiation/shared-air/ground joins, litter reconstruction and V3 transactions.
  Relevant source-real tests include `normalized_solver_rejects_stagnation_below_one`,
  `normalized_solver_rejects_increase_remaining_below_one`,
  `normalized_solver_uses_inward_jacobian_probe_at_closed_lower_bound`,
  `covered_component_shared_air_and_radiation_poisons_fail_closed`,
  `stage3_lower_boundary_and_column_joins_reject_one_bit_poison`,
  `authorization_cannot_replace_finalized_water_in_covered_latent_join`,
  `phase_free_energy_closure_rejects_phase_alias_vapor_only_and_producer_mutants`,
  and `potential_and_fixed_final_use_v3_and_return_the_accepted_evaluation`.
- A1 real consumer: run
  `cargo nextest run --profile full --test land_surface_energy_real_hydrology_shadow_contract`.
  Its registered source includes mixed open/covered owner closure, shared-stratum
  closure, canonical seam poison and configuration/state/topology fail-closed
  tests. The planned exact stable-cut full workspace run additionally covers
  affected orchestrator/runner/output/sim-contract regression and custody paths.
- Explicit ignored admission remains separate: authentic independent reference,
  complete-column, lifecycle/error/adjustment, nonunit mapping tests; runner
  `stage3_residual_capture_solver_observation_equivalence` and
  `stage3_untimed_accepted_operand_audit`. Use retained corpus/environment and
  release runner protocol; full profile does not execute ignored tests by itself.
- A3: NOT_APPLICABLE to a separately defined touched-family external suite,
  not PASS. Current Level-4 registry families are soil fc/wp, subhyd lateral
  drainage, watbal percolation and infile soil-producer behavior. None is changed
  by the exact J column-construction/cfg audit diff, and no LSE, vegetation,
  litter or snow-energy Level-4 suite is registered. Unchanged leaf constitutive
  source and its existing frozen-vector/base-error tests remain A1 regression
  and dependency evidence; they are not a newly invented A3 suite. Full
  workspace execution may exercise the existing unrelated A3 targets but is
  not LSE constitutive qualification.

The full profile intentionally excludes named snowbench/PySnobal/SNOTEL/frost
empirical families; do not label it unfiltered every-test execution. Applicable
current-scope A0/A1 evidence and explicit ignored admission may not be deferred
or replaced by passing unrelated suites. This map does not waive other package
lint, doctest, custody, full-correctness or terminal-verification obligations.

### Local-cost first-source correctness review

Static: reviewed all 540 lines of `solver_inactive_jacobian_cost.rs`, SHA256
`cbd396a8e1004e66789c4c0e4b40281727fd1bf17adf7398342559fbdb8456ae`.
No cost measurements reviewed or run.

COST-A01 (medium, cost evidence integrity), `batch` checksum accumulation:
`rotate_left(1) ^ row_checksum` cancels systematically when the fixed corpus
repeats. For 12 records, rotation by 12 has order 16; 32 repetitions XOR two
identical cycles and produce zero for every possible per-record checksum.
Calibration doubles repetitions, making this realistic rather than theoretical.
The subsequent aggregate A/J equality cannot demonstrate consumed matrix/RHS
parity at those repetition counts. Require untimed per-case full consumed
matrix/RHS bit equality and a sequence-bound non-canceling retained checksum;
test wrong matrix/RHS entries and signed-zero changes. Author accepted the
finding and will correct after the build freeze. Current disposition: OPEN;
no timing admission on this source cut.

Other source assessment: FD uses the current production h and 2*h denominators,
not the independent oracle's actual-abscissa denominators; the distinction is
correct for timing the existing numerical method. It preserves minus-before-plus
order and physical/scaled validity, and constructs every selected row. J pays
fresh same-family base, real provider/capability checks, allocation and complete
assembly. Source-order row adjustment and later Covered scaling are applied
after columns in both arms. Current hydraulic coordinates are shared test-local
initial anchors, explicitly not reconstructed original-solve RHS. The copied
FD/adjustment code is justified as a bounded test-only baseline comparison,
not an additional production solver; independent exact parity is still required
to guard this intentionally retained duplication.

Calibration freezes one shared repetition count after both arms reach 100 ms;
two warmups and 30 balanced paired batches follow. Short measured attempts are
emitted and invalidated before restarting at a larger count. Both canonical
and consumed full matrices are read with black-box checksums, and direct
operation counts are independently reconciled with the existing counter.

Scope limitation: `disjoint_columns_ns` includes full-matrix checksums,
allocation, common row adjustment and deallocation. It is not pure removable
FD-probe time. Report that inclusive local-column-obligation ratio explicitly;
combination with any whole-run fraction requires matching nonoverlapping work
populations, not multiplying it by a pure probe-only fraction without further
reconciliation. The isolated selected-column workspace is not a full Jacobian
sweep or full nonlinear solve benchmark.

### COST-A01 corrected-source re-review

Static: corrected cost SHA256
`d739593f11193598d82c5fd2a5295bb5f4be78cd35babaa8d0d8d870d93e2063`.
COST-A01 source fix accepted. Preflight obtains both arms' consumed matrices
and RHS through the actual repetition path and compares shape and every bit
per retained case before calibration. This includes signed zero, and does not
depend on finite hashes. Measured repetitions use a no-op sink, so preflight
cloning is excluded from measured spans. A bytewise accumulator binds repetition,
record and checksum instead of canceling rotate/XOR cycles. The focused test
contains off-diagonal, RHS, signed-zero, shape, sequence and 32-by-12 repeated
record mutants. This reviewer did not execute it. Verdict: corrected-source
PASS; focused integrity test and actual preflight execution remain required.
Previously documented timing population/overhead limits still apply.

### Latest numerical/exclusion evidence inspected

Static evidence inspection: `j-release-numerical-tests2.log` records 13/13;
`j-release-numerical-tests4.log` records 14/14, including the authentic reference
and added provider-exclusion test. This reviewer did not launch those runs.
Read the new source-real exclusion helper: each participating family gets fresh
valid positive-area, exact wet-only inactive dry-area and negative-zero cases.
Actual solver capture reports two completed sun FD columns and zero direct
sun columns for each of nine cases; all explicitly report `accepted_solve=false`.
Thus these demonstrate FD retention through real execution, not modified-fixture
convergence or active-physiology admission. Historical authority is separately
checked for unchanged first failure or all-direct-column exclusion. Prior
predicate-only evidence limitation is upgraded by these tests. Earlier failed
attempts remain retained; exact reference replay cause/disposition and complete
consumer admission are tracked independently of this cost source review.

### Actual J accepted-operand evidence acceptance

Static: inspected the complete test outcome and compactly decoded emitted
records from `j-accepted-audit-admission.log` and its wrapper JSON. Ran: this
reviewer independently decoded exact u64 float payloads with `struct` and
recomputed retained edge component/shared energy, vapor, litter phase-free
energy, total phase mass and fusion residuals; no Rust run was launched here.
An initial Node-based read-only calculation could not launch (Node absent);
the repository Python environment completed the calculation without file writes.

The runner wrapper records exit 0, one passing ignored test, exact unchanged
binary SHA256 `463c8d79ecc03f73d1590f20c5af7c5e23e710c573cfd712ccf9e32d702cab50`,
and log SHA256 `92bcc793b5a6d3f57b1b9055d1ef0f382966d0e342ff66614d8f129924b793a8`
(independently matched). Source index identity `ba34f54ec71b1f80fd7aa403c340cbd20904b00b68b793c4985158d845185c91`
contains the reviewed audit SHA `22c79f32...`; the indexed runner test source
hash is `302498b8994c3ac90304b12d59ce9b755d1a15c44e44cece674df90fa4d92243`.

| Actual family | Checked | Failed | Retained edge transaction IDs |
|---|---:|---:|---|
| Covered | 400 | 0 | 41 / 44 |
| V3CanopyEnergyOnly | 52 | 0 | 41 / 88 |
| Litter | 52 | 0 | 41 / 88 |

All 504 records were validated by the reviewed loop; only first/last raw records
are retained per family. Overflow is false and first_error is null. Covered
edge lower boundaries are represented snow. Direct-column counts reconcile to
8,000 Covered Potential, 916 GenericV3 Potential and 928 GenericV3 FixedFinal;
Covered FixedFinal is zero. These are counts, not measured runtime fractions.

Independent edge recomputation found zero component-energy residuals for the
retained inactive canopy components; shared heat residual magnitudes at most
8.53e-13 W/m² against limits at least 1.0068e-6 W/m²; shared vapor magnitudes
at most 3.77e-19 kg/m²/s against limits at least 1.0005e-12 kg/m²/s. Supplied
shared limit values exactly match canonical reconstructed values on all four
canopy edges. Litter phase-free energy magnitudes are at most 2.82e-12 W/m²
against limits above 1.0149e-6 W/m²; retained phase mass and fusion residuals
are zero. These sampled calculations supplement, not replace, whole-record
validator execution and the earlier complete dependency/owner review.

Source-bound successful test execution covers eight independent heat/vapor
tolerance-laundering poisons across the four canopy edges; six raw overflow
poisons; six existing-validator raw poisons with equal off/on first errors and
immutable operands; and two real pre-hook litter over-pool transaction failures
with unchanged typed errors/input and zero observation hooks. Missing-hook
evidence correctly rejects those failure-only sessions. This is not all failure
branches or post-hook rollback qualification.

The actual two-run off/on assertion checks identical input hash, seven public
output payload hashes, Lane-D closure operands and direct-column counts. It is
whole-run observer neutrality for these explicit surfaces, not a claim of every
unserialized internal diagnostic bit. Existing separate solver-neutrality and
publication/custody gates remain distinct obligations.

Disposition: CLOS-A01 and CLOS-A02 CLOSED for the bounded accepted physical
candidate component/shared/boundary plus V3 litter-primitive audit evidence.
This does not repair or qualify the preexisting production validator gaps in
isolation, assert active foliage, validate V3 `litter_surface` via the canopy
adapter, or claim full snow/soil interior, final publication or whole WB14
scientific qualification. No blocker remains in this bounded audit admission.
Broader regression, cross-arm comparison, cost and terminal gates remain open;
production remains HOLD.

### Broad LSE natural-limit failure: bounded classification

Static: inspected `j-lse-release-full.log` (164/165 passing) and the exact
failing fixture. `covered_natural_iteration_limit_matches_declared_rust_outcome`
uses `HistoricalV8`, positive sun/shade input areas
1.1102678697049466 / 1.5980654636283864 m²/m², changed hydraulic conductances
and a fixed start. The v34 provider is structurally inapplicable by authority;
no direct eligible column should execute. The failure is the exact stored
normalized-residual diagnostic vector after discrete kind/occupancy/bounds and
iteration/backtracking checks. It is not evidence by itself of physical
closure failure, but remains an unwaived required regression failure pending
the parent's matched-A run and disposition. Do not assume inheritance, apply
eligible-column trajectory allowances to this historical path, or update its
golden values without authority. Core source remains frozen.

### Matched-A failure attribution and cost integrity execution

Static evidence inspection: matched A log
`a-lse-natural-limit-differential.log`, SHA256
`c78915324f2f7de2dfc987e7f08eda6896228fe1fb3773607e65dc0d9469b98e`,
fails the same single test at the same exact diagnostic assertion. A read-only
diff of every emitted left/right vector line against J's broad-suite log has
zero differences. Attribution is CLOSED as inherited A/J-identical failure,
not a v34 trajectory allowance. The required regression gate remains FAIL;
no golden update or self-waiver follows. J broad-suite log SHA256 is
`2cc7a6a44c1f9530af56771ab9fe8c05c4d659c6c6f771ae5048bc7f0e93f880`.

That J log directly records passing `inactive_jacobian_cost_output_integrity`,
`finite_input_latent_overflow_is_not_a_closure_pass`, and
`ownership_missing_hooks_and_overflow_fail_closed`. COST-A01 correction and
its focused mutant/cancellation test are now CLOSED; actual corpus preflight
remains mandatory before benchmark samples. This reviewer inspected evidence,
not launched the Rust runs. Broader package/regression acceptance remains open.

### Prospective J authority-reader dependency correction

Static: the full workspace compile exposed a missing dependency of the copied
primary v34 authority test: `tests/integration/support/sc_contract_text.rs` and
its canonical `tools/sc_contract_directory.py` reader. Read both existing
primary files completely (28/544 newline-counted lines) and applicable tests guidance.
Prospective PASS to copy them verbatim into J and declare the shared
`.venv -> /workdir/openWEPP/.venv` as an explicit untracked external environment.
This restores the intended test reader; it neither substitutes a parser nor
relaxes authority assertions, solver math or acceptance.

Retain the failed compile log; prove exact copy hashes, record the link target
and resolved Python/version, amend resolved source composition and preserve a
new bundle. The parser uses only standard-library modules; its authority-input
path/symlink guards remain unchanged. The interpreter link is not authority
input redirection and must not cause the environment to be recursively treated
as source. Run focused J authority-reader validation then the full retry.
No solver/runner byte change is proposed; earlier runner results retain their
exact existing identity, while terminal test-source custody must reflect this
amendment. No gate is waived or inferred passing.

### Refreshed lint-only source review and authority retry disposition

Static: independently diffed each refreshed Rust file directly against retained
`j-source-cut2/resolved-source.tar` members (read-only streaming, no extraction
writes). Provider ownership changes from value `C` to immutable `&C`, with
matching default/V3 call-site borrows; the sweep still borrows its provider/base
and drops before matrix adjustment. `_caps` is renamed `caps` and retains its
actual immutable borrow in every build; only the unused-field lint expectation
is cfg-dependent. Other changes are test helper argument borrowing, equivalent
semicolon/default initialization, diagnostic function relocation with `&str`,
documentation and focused lint expectations. No arithmetic, normalization,
domain/error order, threshold, parser fallback or scientific assertion changed.
Verdict: refreshed source-level PASS, not executable or gate approval.

Reviewed refreshed SHA256 values:

| Source | SHA256 |
|---|---|
| numerics.rs | f96bf864b382c05579e53bdac1fe1724da315bfd98ab338a81474d8b131e0c88 |
| solver_litter_phase.rs | ce2c9a77aac60d9407f8b0c9c9f9f42b9c478d8a7f003e89579e0d0524b2cd56 |
| solver_inactive_jacobian.rs | 1ec0b72d82277373d1fe747ff01ad42a66b4296aa7dcfb00e5371921120b6372 |
| solver_inactive_jacobian_tests.rs | 958854de0ed3e07e5341ab8b3d8c539e4b792e23c663efdbfb18bfc2e837d6aa |
| solver_inactive_jacobian_oracle.rs | f70475864fc1218093e41e49163d622a2806d07ef5a430d9f9d5fc885f193d9e |
| solver_inactive_jacobian_cost.rs | 0217d68d33ba9ad4e431706d6469dda187ad904b370b151bd9df5eb0e3c6c3a9 |
| solver_accepted_operand_audit.rs | 6e3dcb1c029e55cc55bcb4900a1608e4a243f3ce2d4fd7b9a448f0ef5c062c5c |
| solver_residual_corpus_capture.rs | 7d9aef12c8a451f27d8c56d7e26e634ffa5a27480e6d1993b68de804b74df243 |

`j-authority-readers-retry.log`, SHA256
`da32d3a15b7401d7c6da62868acf352f681c62a1c734f394ffffe747fdc42d30`,
records 105/122 passing and 17 FAIL. Read actual failures. Nonredundant
coordination with independent `capture_review` assigned six reader diffs and
doc/index/roadmap comparison; that reviewer confirms their exact current reader
copies and that the failing version/index/roadmap expectations predate this
reader correction. Its source-composition findings are attributed to that
independent review, not a claim this reviewer repeated those comparisons.

| Failure category | Count | Lawful disposition |
|---|---:|---|
| Stale owner-version/index/roadmap expectations | 14 | Static pre-existing mismatch attribution; retain FAIL. No further owner-doc copying is justified by these failures. |
| Historical v31 dependency-replay structural seam | 1 | Deliberate historical expected-red remains FAIL; v34 column assembly does not implement that separate graph/replay seam. |
| V9 runtime oracle/poison admission | 2 | VEG-E-133 rejects libcrypto.so.3 plus libz.so.1 mismatch. Retain runtime-admission FAIL. |

The V9 tests use exact `/usr/bin/python3.14 -I -S -B`, cleared environment and
the digest-pinned verifier. A/J verifier source is identical. Source inspection
of `_verify_runtime` and canonical INV-VEGETATION-133 confirms only no mismatch
or libcrypto-only mismatch is admissible; a second dynamic-object mismatch must
fail. Do not widen the exception, rewrite descriptors/vectors or declare poison
success after early runtime rejection. This is static common-runtime attribution,
not a newly executed A runtime comparison. Importing current SnowEnergy would
also bring unrelated experimental authority and is not a lawful cure here.

Minimal disposition preserves all 17 failures and original assertions while
continuing scoped release refresh, numerical/actual-consumer admission and cost.
Those bounded checks cannot make the broad gate green. Full workspace run and
new source custody remain pending; production and broad package approval stay
HOLD. No terminal/full-green claim is authorized by this source-review PASS.

Reference replay clarification received from the independent oracle author:
the earlier failure was debug replay of a release corpus; matched release
execution passed unchanged strict replay/reference checks without recapture or
tolerance change. Exact original failing field was not independently localized,
so profile mismatch is supported, not a proven field-level causal diagnosis.
The diagnostic now compares float bits (including signed zero); its lint-only
relocation above does not alter admission.

### Cut3 scoped scientific admission refresh

Static: independently inspected the final cut3 source bindings, runner test,
audit poison protocol, emitted evidence and execution wrappers. Ran: no builds
or tests by this reviewer; the following are directly inspected executed logs.
No new correctness finding in this bounded refresh. CLOS-A01/A02 remain CLOSED
only for the accepted-candidate operand audit described above.

| Evidence | Inspected result | Log SHA256 |
|---|---|---|
| `j-release-cut3-numerical.log` | 15/15 PASS | `7f6dddef39ff96944ec24f891595938fde9270ed1d42c59e0254cbe9327353df` |
| `j-cut3-audit-admission.log` | 1/1 PASS | `dffb9f6e6ede8409229c6eba63986ad9a36f86d7463a28f2e98dfa6b69bacf0f` |
| `j-cut3-actual-admission.log` | 1/1 PASS | `811bababf33aa1fadd330a50c8ce05a5b961fe3d7911ed0bd4d6387ce67e4d70` |

These paths are under `/tmp/openwepp-hotpath-jacobian-jmvOZh/`. Both actual
runner wrappers record exit zero and unchanged runner SHA256
`8493e547df4829e28a1cd784d93a51660347d98ac7256796b1e6544f55f9a145`.
Independently hashed its adjacent `.json` sidecar:
`9fe8c69d7d8aaf37bf242e02bc0bdf574bd9e955fce71a1d0247bf393d24ab71`.
Cut3 resolved-source identity is
`7b5133c93de4c827d54945e265b94235834bffef4eb21ec62942b3c377930d57`;
the resolved index binds the reviewed audit/core hashes above and runner source
`35b6c215749475e6eed6054e9ccb6e5ed4d2d4576d830105f45f84ff4517045a`.
The final runner lint cleanup retains equivalent `if let` handling and a
marker-only telemetry guard: its type contains only `PhantomData`, has no Drop,
and `take_release_qualification_telemetry_v1` closes the session explicitly.

The complete emitted cut3 audit JSON is byte-identical to the earlier directly
reviewed `j-accepted-audit-admission.log` audit JSON. Thus all retained primitive
bits, first/last identities and counts match the independently reconstructed
edge evidence above, not merely matching summarized residuals. Checked counts
are Covered 400, V3CanopyEnergyOnly 52, Litter 52: 504 raw sets, all failed counts
zero, no overflow and no first error. First/last transaction identities remain
41/44 for Covered and 41/88 for V3/litter. Every observed set is checked; only
the first/last primitive sets are retained for independent edge reconstruction.

Read the executed test's assertions and current audit protocol: it requires
observer-off/on identical input hash, public output payload hashes, Lane-D
closure operands and direct participation. It also executes eight independent
heat/vapor tolerance-laundering poisons, six raw overflow/nonfinite poisons,
six raw-validator first-error/immutable-input checks and two actual pre-hook
litter over-pool rejection comparisons with zero hooks/missing-hook rejection.
These are not evidence for every post-hook rollback or all internal diagnostics.
Direct columns reconcile to Covered Potential 8,000, GenericV3 Potential 916,
GenericV3 FixedFinal 928 and Covered FixedFinal zero: 9,844, across the same
four accepted microsteps and 56 accepted publication supports. This count is
not a runtime percentage or a claim that every audited candidate was published.

Independently diffed sorted `.common_identity` from A's admission identity and
`j-cut3-admission-identity.json`: no differences. J's identity log/source hashes
match the cut3 evidence above. This is exact public/common-outcome evidence;
the separate operand reconstruction and numerical checks carry the bounded
scientific claim. No new active-foliage, final-publication closure, soil-interior,
snow-interior or full-watershed qualification is inferred.

Residual risk / remaining gates: the broad workspace is non-green; the parent
reports 179 failures including one manually interrupted test. This refresh does
not independently classify all 179, waive the already documented failures, or
turn an interrupted test into a scientific pass. Local cost's exact per-case
preflight and valid measured batches still must execute; no cost result is
admitted here. Source-bundle recoverability verification remains separate.

Verdict: PASS for the frozen cut3 numerical/accepted-consumer prerequisites to
the authorized scoped cost experiment. No blocker to running that experiment;
not terminal package approval, broad regression qualification or production
approval. Production remains HOLD.

### Terminal engineering decision support: measured priority rejection

Static: read the frozen authorization sections 10–11 and current protocol;
directly inspected the four cut3 cost/memory analysis artifacts and independently
recomputed matched sampled-active process maxima and thresholds. Ran: no new
timing, memory observation or source change by this reviewer. QA separately
owns full raw-series validity verification; this review adjudicates the bounded
decision without replacing that verification.

Finding MEM-A01 — medium, engineering-priority FAIL:
`/tmp/openwepp-hotpath-jacobian-jmvOZh/j-cut3-teardown-analysis.json`,
matched process pair 0. Across its ten runs, A's sampled-active maximum is
83,788 KiB and J's is 88,276 KiB. Extra matched peak is 4,488 KiB; the frozen
allowance is `max(4096, 0.05 * 83788) = 4189.4 KiB`. Excess is 298.6 KiB.
The other two matched sampled-active maxima are within their bounds, as are
all six single-run sampled-active pairs. All three teardown lifetime-peak
comparisons are within bounds (A/J: 87800/88724, 87952/87772, 87324/87760 KiB).
These distinct measurements do not cancel the sampled-active failure. No frozen
rule permits selecting lifetime peaks alone, substituting a median, discarding
pair 0, rerunning toward a pass or widening the bound after observation.

Disposition: preserve the measured priority FAIL. It is not proof of a new
physical/numerical defect, causal live-allocation regression or persistent leak.
Sampling and allocator behavior limit attribution; ten complete runs do not
prove lifetime boundedness. No further memory framework or implementation
refinement follows from this review, and no leak-free claim is admitted.

The inspected local analysis reports 9.444662x charged-column-obligation
acceleration and 4.504620x base-inclusive acceleration. Its matching whole-run
fraction remains UNMEASURED. The separate 12-pair actual runner analysis reports
0.422922% median wall savings, 95% bootstrap interval 0.034531–1.246556%, about
0.021091 seconds saved, and median process CPU ratio 0.995961. Preserve that
small measured whole-run gain; do not relabel the local ratio as a whole-run
speedup, derive coverage from column counts, or invoke an inapplicable 30%
target before sufficient modifiable coverage is established. The narrow block
remains a scientifically admitted experimental building block, not an accepted
throughput or memory qualification.

Conditional scope decision: authorization requires 10/19-OFE timing for
**competitive admitted** multi-OFE cases, not every correct prototype. With
the retained memory-priority failure, small demonstrated complete-run benefit
and no authorized new refinement in this disposition, this candidate does not
meet that condition. Further three-pair 10-OFE and then 19-OFE timing is not
warranted or required for this terminal decision. Preserve any already-started
10-OFE admission outcome as precisely bounded execution/topology evidence; a
passing admission alone does not establish competitiveness or scale timing
qualification. The reported topology has only the downstream vegetated OFE,
so even its eventual pass cannot be multiplied into ten-fold target coverage.
Unperformed multi-OFE timing remains NOT QUALIFIED, not PASS or a waived gate.

Inspected analysis SHA256 values:

| Analysis | SHA256 |
|---|---|
| local cost | `278fee699bfaff300d749b6f0e8b5ba52bbc467dae18d02e28772b62a5e17c35` |
| one-OFE timing | `9e79a25b81d8d50ad9036685fefa58e008b35115cb69801ab9e56875aca28dbb` |
| six-pair memory | `840ef9054ca1e6022343f3ffa5b8ecd232ecc4413cb562bc25608be856ec21e1` |
| teardown | `5a70c156aa092cd08d5d76483d82d7f2f9fe14be6f4a5545c4e4f2701d90a19e` |

Residual obligations: retain every valid sample and failure; complete independent
QA/raw-evidence and source-recoverability handoff checks. Broad gates remain
FAIL/non-green and production remains HOLD. No blocker to ending optimization
at this measured engineering-priority rejection and delivering its recoverable
experimental handoff; this is not scientific self-waiver or full-green approval.

Formal engineering disposition under authorization section 11:
`REJECTED_IMPLEMENTATION`. This actual evaluated candidate fails the frozen
matched-peak engineering cost criterion. Its valid affine derivative and useful
coverage-limited local result remain preserved; the rejection does not reject
the science or larger architecture. `USEFUL_BUT_COVERAGE_LIMITED` alone would
omit the decisive measured memory failure and is not the terminal engineering
label. The stop-tuning rule applies after this actual implementation evaluation;
the requirement to attempt alternatives after an inadequate reference does not
apply to this adequate reference and measured candidate rejection.

The parent subsequently reports both 10-OFE admission runs PASS with exact
common identity. Retain that separately as reported admission evidence, not a
newly independently rechecked scale claim in this review. No three-pair scale
timing was required by the unsatisfied competitive-candidate condition.
Engineering `REJECTED_IMPLEMENTATION`, package executed-HOLD, and broad-gate
FAIL are separate statuses; none supplies a waiver for either other status.

### One prospective architecture action, outside this package

Recommend a separately authorized experiment with reduced linear-system
assembly for the already proven inactive sun/shade coordinates, retaining and
reconstructing the complete physical state and canonical anchors. This is a
new authority decision: v34 explicitly prohibits unknown removal, resizing and
pivot changes. It is not an implementation task authorized by this handoff and
does not reverse the current engineering rejection.

Earliest decisive test: frozen same-input full-system versus reduced-system
solves in Covered Potential and GenericV3 Potential/FixedFinal, comparing the
reconstructed full-coordinate scaled Newton step, inactive anchors, controller
outcome, typed errors and accepted raw operands. Preserve base preprocessing
and invalid/stale/positive-area exclusions. Any numerical/controller allowance
must be prospective, not fitted after seeing differences. Before expanding,
measure the exclusive remaining matrix/linear-solve cost and actual-run CPU and
memory effect; this package supplies no matching runtime fraction or predicted
speedup for compaction. Stop the proposal if its reconstruction/solve semantics
cannot be established or its measured cost opportunity is not useful.

Read `next-architecture-physical-evidence-note.md`: the reported physical-evidence
span includes real physical evaluation, and the physical-only split and one
shared physical prefix per batch already exist. Those nested totals neither
demonstrate duplicate same-state work nor justify claiming removable evidence
overhead. No new physical-evidence reuse is recommended from those totals.

### Terminal substantive review — evidence refresh in progress

Static: reviewed current package, implementation/parity, treatment, gate,
finding, kernel checklist, next-action and source-manifest claims. Directly
inspected terminal format diff, exact-A patch manifest/application log, all 16
current prototype file hashes, focused timeout log and five matched-A runtime
failures against structured J JUnit records. Ran: no builds/tests by this
reviewer. This section is not final custody approval while the terminal bundle
and independent verifications are pending.

Findings first:

- MEM-A01 remains a decisive measured engineering-priority FAIL, with
  `REJECTED_IMPLEMENTATION`, not a failed affine-science claim. Preserve the
  broad gate failures and production/package HOLD independently.
- TERM-A01, medium, current-artifact evidence contradiction: implementation/
  parity still labels full consumer comparison and accepted audit NOT RUN;
  finding disposition labels executed local preflight NOT RUN; source/build
  manifest status says refresh/timing pending with no measurement entries;
  kernel checklist describes completed exclusion/full-workspace execution as
  future work. Requested current-state reconciliation or explicitly labeled
  historical sections. No new source, numerical rule or test run is needed to
  correct these statements. OPEN pending corrected artifact review.
- No new numerical, domain, serialization, guard-precedence or production
  mutation finding in this source refresh. Prior intentional test-only
  reference/audit duplication remains bounded and independently checked; no
  second production solver or authority implementation was added.

Source-cut distinction: `j-terminal-format.log` records an exact 27-path scoped
rustfmt check PASS and contains the actual cut3-to-cut4 diff: assertion wrapping
only, no changed assertion strings or logic. Primary/J owning authority-test
SHA256 is `ddce942406d6f84477fdab66cb7f958822d00118bc8e6ab03f452bc4e4865acf`.
Independently checked all 16 current prototype Rust files against the exact
J-cut3 hashes in `j-prototype-cut3-patch-manifest.json`: 16/16 match. Its manifest
declares exact A-cut2b archive inputs and J-cut3 archive outputs, all applied
outputs bit-identical. The application log lists those 16 files without fuzz;
this reviewer did not reapply the patch. It is not a patch against repository
HEAD. Measured source remains cut3; formatting-only cut4 has not been timed.

Direct gate evidence: the formerly interrupted runner case's focused full-profile
retry naturally records TIMEOUT at 720.022 seconds (0 passed, 1 timed out,
241 skipped). This does not erase the original manual SIGINT, supply a complete
workspace rerun, or classify the timeout as a numerical error.

The five A triage tests all failed. Independently compared each full typed-error
payload against its matching J JUnit record:

| Case | Same A/J failure payload |
|---|---|
| latest accepted state / next WB14 proposal | canonical covered dependent-output instability |
| interior terminal event / snow-free remainder | canonical covered evaluation budget |
| ordinary reuse / forced double evaluation | support-liquid receiver publication support identity |
| SIMIMPL18 multi-day storage mutation | pressure_pa zero snow conductivity guard, with identical full emitted snow-state diagnostic |
| R7E direct production manifest | pending V11 parent-finalization record source identity |

This supports inherited attribution for these five selected failures only.
All remain FAIL; other cases are not attributed by shared test counts, labels
or a narrow runner PASS. A log SHA256:
`227aa2e2680cb77637a3040dc47197b94b7e01f8f8bbfdd734710634a98d5e5e`;
J structured full2 SHA256:
`251e920ca264b1bdbbc83f24ea17e0b8f618d4137e8378aeeb978dda2489d450`;
focused timeout log SHA256:
`d37d934e2f128e91caf3b29e95eeef3505e6a41b8ceb92b94ff54d95b490d922`;
terminal format log SHA256:
`d6eb759c7b18b2452f9e4235ec3d52682ea632ae20c566706c65ba581e18b2e4`.

Remaining evidence/approval boundary: reconcile TERM-A01; capture and verify
terminal results/cut4 custody without relabeling cut4 as measured; complete
dual terminal verifications. No safe new implementation or optional scale
campaign is required after the supported engineering rejection. The next
reduced-system action correctly requires separate authority. No broad failure
is deferred, waived or silently accepted by this bounded handoff review.

### Focused terminal corrections and reading applicability

TERM-A01 CLOSED: reread corrected implementation/parity, finding disposition,
kernel checklist and manifest current status/measurements. They now distinguish
the executed cut3 15/15 numerical, actual consumer/audit, cost and memory results
from historical pending stages. The early gate list is explicitly historical.
No numerical or acceptance rule changed to resolve this narrative finding.

Terminal custody now exists, rather than being merely planned. Independently
hashed `j-terminal-bundle/manifest.json`:
`17fbc3a85c9c53088734975f19999890e61d9cf52ba7cebe280e512f46f04764`.
Inspected the capture/verification records, selected terminal results and the
explicit `terminal-results/custody-status.txt` supplement. It preserves cut3
source and measured binaries plus the one-file formatting overlay, final timeout,
five-case A triage, structured full2 and all four analysis artifacts. Overlay
source identity is
`0c582dd73d12434faf2d0394f320c5247633f1966538b347844dbe78bb9df70a`.
Historical premeasurement metadata is explicitly superseded for current status
by the terminal supplement, not rewritten. These are custody-record checks,
not this reviewer's independent all-blob recovery verification; the assigned
two terminal verifiers retain that obligation. Later final review records are
not retrospectively claimed to have existed inside this earlier checkpoint.

Reading applicability finding READ-A01 — medium, pending parent expansion:
`required-reading-map.md` records the parent reading SC-SNOWENERGY only through
1355. The current claim includes independent admission of represented-snow
Covered shared-carrier residuals/normalization and lower-boundary authority.
`soil-coupling.md` Dependencies explicitly selects the whole external
SC-SNOWENERGY contract for represented-snow boundary review;
`terminal-support.md` likewise selects it for regime-rule selection or the
represented-snow boundary. This trigger does not require a full snow-interior
qualification claim or a SnowEnergy source edit. The parent should finish the
selected owner from 1356 through EOF, record the actual complete reads, and
report any conflicting obligation. This reviewer already read that selected
owner completely, but that does not change the parent's truthful read record.

SC-WATBAL remains conditionally UNTRIGGERED for the stated canopy/shared/ground
boundary plus litter primitive reconstruction scope. `water-vapor.md` selects
it for ingress partition/closure or cross-contract water scope. The present
Lane-D claim is unchanged common-outcome/operand identity, not a new independent
WB14 ingress-partition or watershed-water qualification; the LSE litter mass/
fusion/enthalpy checks do not transfer ownership of WB14 water partition.
Maintain that explicit boundary. Any later claim to the named WB14/full-water
mechanisms would trigger the whole owner then, not permit a partial read.

Final substantive approval remains pending READ-A01 disposition and the
assigned final verification records. No new implementation work is requested.

READ-A01 CLOSED after focused record/scope review. The parent now explicitly
records complete SC-SNOWENERGY reading through line 3559/EOF, including the
previous 1356–3559 gap and separate rereads of truncated INV-062/063, mirrored
INV-059/060 and change-log 53–49 passages. The record truthfully dates this
completion to terminal review rather than retroactively claiming preimplementation
completion. Parent reports no newly conflicting obligation; this is consistent
with this reviewer's earlier complete-owner reading and the unchanged bounded
represented-snow carrier/ground-boundary admission. No source, science rule,
acceptance threshold or gain claim changed to close the finding.

The reading map now explicitly leaves SC-WATBAL unread and UNTRIGGERED only
while Lane-D evidence remains common output/control/operand identity, not
independent ingress partition or cross-owner water qualification. That bounded
disposition matches the applicable dependency triggers. There is no remaining
reading blocker to the current substantive review. Final narrative/custody
supplements and assigned independent verifications remain separately pending.
