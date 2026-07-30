# Review Agent B

Evidence class: `Ran + Static`.

Final verdict after correction re-review: `PASS`.

Initial verdict: `FAIL`.

Ran:
`.venv/bin/python docs/work-packages/20260729-snow-surface-eb-01-reconciliation-factorial-design-001/tools/validate.py`
returned `PASS` for 11 rectangular CSVs, three accessible SVGs, sidecar
pairing, local links, and byte-for-byte generator determinism.
`git diff --check` also passed.

Those checks establish artifact shape and reproducibility of the current
hard-coded outputs. They do not establish scientific provenance, a valid
control-volume closure, or package completion. The source reconciliation
correctly identifies the single entangled `SnowMeltModel`, the separate Stage
A/B mass sink, the typed surface-energy helpers, and the missing longwave
runtime/authority operands. The initial review found the following
closure-blocking issues.

## Findings

### B-01 — High — The mass/energy control volume and exact-one coupling are not yet executable

`factorial-design.md` reconstructs mass using `retained_rain_change`, but that
operand is absent from `mass-energy-operand-lineage.csv`. The ledger contains
`rain_retained` as a positive input and no explicit before/after solid storage,
before/after retained-liquid storage, total-pack storage definition, duration,
or area/normalization fields. It is therefore ambiguous whether `SWE_before`
and `SWE_after` include liquid water and whether retained rain is an external
input or an internal storage change.

The latent pair is also not algebraically frozen. The shared meteorology
contract states that positive flux adds energy or vapor mass to the surface;
sublimation therefore produces a negative signed mass flux and negative latent
energy. The package instead records a non-negative `vapor_mass_loss` with a
minus sign and a signed latent-energy term, but never states the conversion
between those conventions. It also does not bind the phase-appropriate latent
heat returned by `latent_heat_for_surface_temperature`. A successor could
invert the sign, use vaporization rather than sublimation heat, or count
retained liquid twice while still satisfying the prose.

Evidence:
`crates/openwepp-meteorology/src/surface_energy.rs:1-5,482-506,565-579`;
`factorial-design.md`, “Independent Closure”; and
`mass-energy-operand-lineage.csv`.

Action: define one control volume and either total-pack storage or separate
solid/liquid before-and-after storage. Add every equation operand to the
lineage with source path, denominator/normalization, area basis, time basis,
and exact duration. Freeze the signed conversion, for example loss-positive
vapor mass against a negative surface mass/latent flux, with the admitted
phase-dependent latent heat. Require an independent consumer to reconstruct
both storage closure and latent/mass equivalence from raw operands.

### B-02 — High — Generated evidence is deterministic but not reproducibly derived

`tools/generate.py` hard-codes observation counts, candidate scores,
sublimation totals, classifications, and all three figure inputs. It reads the
retained Stage B JSON but asserts only
`stage_b_robust_fail_count == 15`; the sidecar nevertheless says all plotted
values “are read from” that JSON. The dependency manifest hashes observation
manifests, not the normalized CSVs from which the counts supposedly derive.
After an input change, running the generator would update dependency hashes
while preserving stale hard-coded science facts, and `validate.py` would pass.

This is already visible in Harvard: the generated `total_rows` values are
`448`, `449`, and `390`, but the installed normalized file contains `821`
rows for each of hardwood, hemlock, and open. Those generated values are the
non-null snow-depth counts.

Evidence:
`tools/generate.py:132-190,233-320`;
`tests/fixtures/cancov_forest/observations/sites/harvard_hf237_strata.csv`;
and the retained Stage B JSON `summary`, which contains the four candidate
failure/score fields used by the figure.

Action: derive all available counts and prior outcomes from the named retained
inputs, include normalized data/provenance files in the dependency manifest,
and fail closed on identity, units, binding, period, or schema changes. For
facts not present in a machine-readable dependency, cite the exact retained
source and do not claim the generator read them.

### B-03 — High — The selector ledger misstates the active default carrier

`selector-composition-ledger.csv` labels the `surface_energy_carrier` current
default as “shortwave-only Stage 3,” and the implementation ledger calls the
Stage 3 absorbed-shortwave operand `implemented_active`. The real no-environment
Stage 3 selector returns `SnowStage3LiquidRoutingModel::Disabled`. The current
no-environment melt default is `CoeLiquidHoldingCapacityV1`; Stage 3's
shortwave-only sum exists only when its independent opt-in selector is enabled.

This distinction is load-bearing for the factorial baseline. Saying Stage 3 is
the current default could cause `B` to change the carrier relative to the
retained default before either mechanism is enabled.

Evidence:
`00c_day_input_builder_impl.rs:1457-1475,1516-1540` and
`runoff_reconciliation.rs:329-369,770-845`.

Action: separate `production_default`, `available_opt_in`, and
`factorial_requirement` in the selector and implementation ledgers. State
explicitly that the production default has Stage 3 disabled and that any common
factorial carrier requires prospective authority and identical selection in
all four cells.

### B-04 — High — The calibration-readiness deliverable violates binding status governance

`calibration-readiness-matrix.md` has one free-form disposition column rather
than the required orthogonal
`science_implementation_status`,
`calibration_evidence_status`, and
`identifiability_status` fields. It also does not classify every applicable
science-contract-spec obligation as `PASS`, `BLOCKED`, or `NOT_APPLICABLE`
with evidence and rationale.

Evidence: `docs/work-packages/AGENTS.md`, “Science Implementation And
Calibration Readiness,” and package exit criterion 9.

Action: rebuild the matrix with the canonical vocabularies and reconcile every
current-scope `BLOCKED` row under the Validation Evidence Non-Deferral Rule.
Keep longwave science authority, warm-maritime transfer data, and
sublimation-composition implementation as separate axes.

### B-05 — High — The binding one-round stop-loss is not machine-readable or falsifiable

Package exit criterion 8 requires machine-readable stop-loss and successor
admission decisions. `stop-loss.md` is numbered prose and uses undefined terms
including “improves,” “targeted,” and “material protected-lane regression.”
`response-operator-ledger.csv` does not define the persistence window for
snow disappearance, the runoff evaluation window, uncertainty/tie handling,
protected lanes, or promotion direction and thresholds.

Action: add a deterministic decision-rule ledger with hard closure gates,
primary and protected responses, lane/period aggregation, directionality,
threshold or authority owner, uncertainty/tie handling, and exact dispositions
for improvement, neutral, tradeoff, and regression. If thresholds require a
successor authority decision, make freezing them a pre-result gate rather than
calling the present rule binding.

### B-06 — Moderate — Catalog state claims completion before required closure evidence exists

The package remains `executing`, its terminal gate rerun and finding
disposition are pending, and dual terminal verification has not occurred.
Nevertheless, the campaign roadmap marks EB-01 “complete,” and the work-package
catalog says `COMPLETE` and claims the independent ledger prevents double
counting. That claim is stronger than the current B-01 evidence.

Action: keep roadmap/catalog state `executing` until accepted findings are
corrected, re-reviewed, terminal gates pass, and both verifiers approve the
exact final tree. Only then synchronize package, roadmap, and catalog state.

## Positive Evidence

- The source trace correctly proves that Stage 3 passes absorbed shortwave and
  zeros other surface-balance operands, while Stage A/B removes bounded vapor
  mass in a separate path.
- The two-factor interaction
  `Y(LS) - Y(L) - Y(S) + Y(B)` is correct.
- `rejected-formulas.md` rejects the main longwave, latent-energy, vapor/liquid,
  duration, depth/SWE, and producer-self-consistency aliases.
- The successor decisions preserve Stage A/B nonpromotion and do not treat
  generic Stefan-Boltzmann arithmetic as sufficient canopy-longwave authority.
- The exact diff currently remains inside the declared documentation and
  package-local analysis write set; no Rust, contract, fixture, selector, or
  production file is modified.

That was the initial closure conclusion. The correction re-review below
supersedes it.

## Correction Re-Review

Re-review date: `2026-07-30`.

Ran:

- `.venv/bin/python
  docs/work-packages/20260729-snow-surface-eb-01-reconciliation-factorial-design-001/tools/validate.py`
  returned `PASS` for 14 rectangular CSVs, three accessible SVGs, sidecar
  pairing, links, and deterministic generation;
- `markdown-doc lint --path <package>` returned 25 files, zero errors, and zero
  warnings;
- `markdown-doc validate --path <package>` returned 25 files and zero errors;
  and
- `git diff --check` passed.

### Finding Resolution

| Finding | Resolution | Evidence |
| --- | --- | --- |
| `B-01` | `RESOLVED` | `factorial-design.md` now defines a whole-pack ice-plus-retained-liquid control volume, explicit before/after storage, signed solid/liquid/vapor/outflow terms, a common ground-area basis, exact step duration, signed thermal storage, phase-change mass, and the phase-appropriate latent heat. `mass-energy-operand-lineage.csv` names every operand, source, authority, consumer, time basis, area basis, and visibility. The independent identities are dimensionally and sign consistent: sublimation is negative in both the vapor and latent terms, and a loss-positive counter cannot become a second debit. |
| `B-02` | `RESOLVED` | `generate.py` derives candidate failure counts, ordinal scores, and Stage A/B sublimation magnitudes from the retained JSON. It derives Marcell, Harvard, SNOTEL, and Sleepers counts and periods from normalized CSVs. The dependency manifest now hashes those normalized inputs. Schema, units, binding states, periods, and manifest file identities are asserted. Harvard correctly reports 821 total rows per stratum while preserving the distinct non-null depth/SWE/density counts. Figure values derive from the reconciled ledgers. |
| `B-03` | `RESOLVED` | `selector-composition-ledger.csv` separates production default, available opt-in, and factorial requirement. It records the bulk CoE carrier with Stage 3 disabled as the production default and shortwave-only Stage 3 as opt-in. `current-implementation-ledger.csv` likewise marks absorbed shortwave and cold content as opt-in rather than default-active. The factorial requires one prospectively admitted identical carrier in all cells. |
| `B-04` | `RESOLVED` | `calibration-readiness-matrix.md` carries the three canonical status fields and exactly the ten obligation rows required by `science-contract-spec.md`, each with `PASS`/`NOT_APPLICABLE`, evidence, and rationale. Prospective `NOT_CALIBRATION_READY` mechanism states remain distinct from EB-01's completed reconciliation obligation. |
| `B-05` | `RESOLVED` | `decision-rules.csv`, `stop-loss.csv`, and `successor-admission-decision.csv` provide machine-readable operators, owners, hard failures, and outcomes. Values that require successor authority are explicitly unresolved and force `HOLD_EB04`; the prose no longer claims those thresholds are already operational. |
| `B-06` | `RESOLVED` | `package.md` is `reviewing`; the campaign roadmap says `executing / review correction`; the canonical roadmap and package catalog require correction closeout and terminal verification. None claims EB-01 complete before the terminal gates. |

## Final Assessment

All initial Review B findings are resolved. The corrected package truthfully
distinguishes current defaults from opt-ins, admitted helpers from missing
process authority, and EB-01 reconciliation completion from EB-02/03/04
science prerequisites. The mass/energy design prevents vapor/liquid aliasing
and latent double debit through an independently reconstructable signed
identity. The generated factual evidence is source-derived and snapshot-bound.

Review B passes the corrected tree. Package completion still requires the
declared terminal validation, exact-diff refresh, dual terminal verification,
and final disposition; this review does not pre-approve those later gates.
