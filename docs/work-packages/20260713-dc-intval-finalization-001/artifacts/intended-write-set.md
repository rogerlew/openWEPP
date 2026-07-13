# Intended Write Set

Status: `IN-PROGRESS-FIRST-BATCH`

Initial implementation writes are the five missing integration-test targets,
their Cargo registrations, one all-active-required-bindings guard, and this
package evidence. Before any later mechanism-specific correction, record the
new files, applicable guidance, authority, and protected boundaries here.

## Revision 1: INTVAL-AUTH-BIND-001

Evidence class: **Static**.

Authorized implementation paths:

- `Cargo.toml` — register the five restored integration targets.
- `tests/integration/auth11_required_suite_obligation_guards_contract.rs` —
  generic guard requiring every active required/hard-fail registry binding to
  exist and have an explicit Cargo target registration.
- `tests/integration/auth06_fixture_provenance_hash_enforcement_contract.rs` —
  narrow lint annotation on the predecessor package's unchanged provenance
  verification test, exposed after first-candidate lint correction.
- `tests/integration/auth05_level4_constitutive_authority_hardening_contract.rs`.
- `tests/integration/hphys0224_wb19_withdrawal_soilwater_cap_contract.rs`.
- `tests/integration/hphys0225_wb19_layer_pool_withdrawal_cap_contract.rs`.
- `tests/integration/hphys0226_wb19_lateral_saturated_thickness_response_contract.rs`.
- `tests/integration/hphys0227_wb19_fcwp_coca_watyld_authority_contract.rs`.
- this package tree, plus terminal campaign/catalog/roadmap evidence required
  by `package.md`.

Applicable guidance: root `AGENTS.md`, `tests/AGENTS.md`,
`docs/work-packages/AGENTS.md`, science-contract governance, DC-ExecPlan
guidance, and the seven owning external-authority suite documents.

Protected boundaries: do not revive the deleted symbol-map runtime; do not
alter suite posture, fixture values, thresholds, tolerances, canonical
physics, or production behavior during this first binding-restoration batch.

## Revision 2: INTVAL-EROSION-TOE-001

Evidence class: **Ran + Static**.

Exact release candidate 2 reached the full stability matrix and exposed a
production runtime defect at the accepted slope-profile compatibility seam.
Before correction, this revision authorizes:

- `docs/specifications/science-contracts/contracts/SC-SED-001.md` — add the
  pinned `profil.for` terminal-station normalization authority and guard/test
  obligation before runtime edits.
- `tests/integration/erod16_wave1_continuity_fixture_conservation.rs` — add the
  existing fixture-level EROD16 surface if needed during validation.
- `crates/openwepp-hillslope-orchestrator/src/tests/tests_mod/direct_runtime_wave1_continuity.rs`
  — add the contract-derived near-terminal compatibility regression before
  runtime edits, adjacent to the existing `profil.for` normalization vector.
- `crates/openwepp-hillslope-orchestrator/src/direct_runtime/erosion_continuity.rs`
  — correct only Wave-1 slope-station normalization after the contract and red
  test gates are recorded.
- this package tree — preimplementation gate, defect, command, review, and
  validation evidence.

Applicable guidance: root `AGENTS.md`, `crates/AGENTS.md`, `tests/AGENTS.md`,
`docs/work-packages/AGENTS.md`,
`docs/specifications/science-contracts/AGENTS.md`, and the science-contract
authoring procedure and kernel profile.

Protected boundaries: preserve the parser's existing compatibility endpoint
tolerance and declared physical hillslope length; do not clamp normalized
stations, loosen the runtime toe guard, change erosion equations, or treat
comparator agreement as authority. The pinned baseline's terminal station is
the normalization denominator.

## Revision 3: Candidate-3 Stability Families

Evidence class: **Ran + Static**.

Candidate 3 closed `INTVAL-EROSION-TOE-001` family-wide and exposed exactly
three remaining stability mechanisms. Before implementation, this revision
authorizes:

- `docs/specifications/science-contracts/contracts/SC-PLANT-001.md` and
  `crates/openwepp-hillslope-orchestrator/src/direct_runtime/growth.rs` —
  restore the baseline perennial root-cap-before-increment branch, including
  valid `rtmmax=0`, with contract-derived tests in the same Rust module.
- `crates/openwepp-hillslope-orchestrator/src/direct_runtime/subsurface.rs` and
  `crates/openwepp-hillslope-orchestrator/src/tests/tests_mod/direct_runtime_r4mo.rs`
  — consume every strictly positive hourly same-pass infiltration increment
  under existing `SC-PERC-001#INV-PERC-017` authority.
- `docs/specifications/science-contracts/contracts/SC-PERC-001.md` and
  `docs/specifications/science-contracts/contracts/SC-INFILE-SOIL-001.md` —
  reconcile exact-zero restrictive conductivity as a valid impermeable
  boundary before runtime edits.
- the same subsurface source and R4MO tests — implement and test daily/hourly
  exact-zero restrictive conductivity while retaining `slflag=1`.
- this package tree for the cumulative gates and evidence.

Protected boundaries: no closure-tolerance change, no positive-input epsilon
drop, no disabling an active restrictive layer, no division-by-zero exposure,
no input fixture changes, and no fallback. Negative/non-finite domains remain
typed failures; positive behavior remains unchanged.

## Revision 4: INTVAL-FROST-THAW-CLEAR-001

Evidence class: **Ran + Static**.

Focused reruns of all eight zero-restriction watchlist cases progressed beyond
the corrected impermeable-boundary branch and exposed one downstream runner
mechanism. Before contract, regression, or runtime edits, this revision
authorizes:

- `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md` —
  bind the no-final-frost hydrology projection to the post-`frwatc` liquid
  target and the residual store restored over newly unfrozen thickness.
- `crates/openwepp-runner/src/hillslope/03_tests.rs` — add a contract-derived
  thaw-complete vector with prior frozen depth and a post-handoff target.
- `crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/01_frost_and_layer_helpers.rs`
  — bypass the synthetic no-final-frost clear only when the computed outcome
  carries the authoritative material `frwatc` projection, after the contract
  and red-test gates are recorded.
- this package tree for diagnosis, preimplementation, command, and acceptance
  evidence.

Protected boundaries: do not lower residual theta, debit residual storage,
change the exact-zero restrictive boundary, relax closure tolerances, or invent
a storage correction. A material outcome must retain its original layer basis
until R4W applies its authoritative post-`frwatc` projection exactly once; the
existing stale-clear remains for genuinely nonmaterial outcomes.

## Revision 5: INTVAL-CONTRACT-VERSION-BIND-001

Evidence class: **Ran + Static**.

Candidate 4 reached the full workspace suite and exposed 32 test-only contract
marker guards that still require `SC-SNOWFREEZE-001` header version 115. The
contract already contains a version-116 revision-history row and revision 117
correctly reconciles its header. Before the mechanical test correction, this
revision authorizes:

- the 32 integration-test files returned by
  `rg -l 'contract_version: 115' tests/integration` — change only the exact
  expected marker from 115 to 117; and
- this package tree for candidate-4 failure and focused/full validation
  evidence.

Protected boundaries: no scientific assertion, package marker, fixture,
tolerance, suite posture, or production behavior changes. The tests continue
to require the canonical contract header and now bind its current revision.

## Revision 6: INTVAL-EROSION-CLASS-FRACTION-001

Evidence class: **Ran + Static**.

Candidate 5 passed every prerequisite and 1,183/1,185 stability cases, exposing
exactly OR-H0081 and OR-H0204 at the Wave-1 publication nonnegative class-
fraction guard. Before contract, regression, or production edits, this
revision authorizes:

- `docs/specifications/science-contracts/contracts/SC-SED-001.md` — bind the
  enrichment label-50 reproportion to nonnegative class masses when the
  absolute legacy `1e-15` per-class floor would exceed the routed total.
- `crates/openwepp-hillslope-orchestrator/src/tests/tests_mod/direct_runtime_wave1_continuity.rs`
  — add the contract-derived sub-floor total-load reproportion vector.
- `crates/openwepp-hillslope-orchestrator/src/direct_runtime/erosion_enrichment.rs`
  — correct only the per-class floor/reproportion degeneracy after contract and
  red-test gates.
- this package tree for GDB operand evidence, candidate logs, and acceptance.

Protected boundaries: retain total routed load as mass authority, the legacy
analytic enrichment equations, class availability caps, the bounded label-50
loop, TOL-SED-005/006, and the publication nonnegative hard guard. Do not clamp
negative published fractions, loosen a tolerance, change fixtures, or replace
the enrichment physics with a heuristic composition.
