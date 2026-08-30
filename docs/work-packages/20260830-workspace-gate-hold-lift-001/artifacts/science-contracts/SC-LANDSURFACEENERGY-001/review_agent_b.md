# Independent review B — WGHL-FULL-001F

Recommendation: `HOLD`

Scope: preserved `SC-LANDSURFACEENERGY-001@13`, SHA-256
`922917e963788ae10faae699ab8c6eb95180748d53a94b15aa484a34eeadfede`,
specifically `INV-LANDSURFACEENERGY-139`, its implementation in
`solver_covered_solve.rs`, and 001F evidence. The separately owned version-14
frozen-litter successor authority was read for context but is not reviewed or
dispositioned here.

Static: reviewed the applicable root, work-package, crate, and science-contract
instructions; the science-contract authoring procedure and kernel profile; the
complete current contract; the 001F package authority/evidence; the covered
solver; its focused tests; the authority source-scan; and the numerical norm
helper.

Ran:

- `nix develop -c cargo nextest run -p openwepp-land-surface-energy -E
  '<the three INV-139 predicate tests plus covered frozen-oracle, natural
  failure/no-publication, and transaction rollback-lineage tests>'`: `6/6`
  pass, run `22872cbb-d516-4e11-9561-40c580891d1d` before the final typed-
  refusal/member-finiteness refinement;
- exact-source controller/predicate vectors: reviewer `5/5` pass, run
  `30d46382-50eb-48cc-8ff4-ab5b21c89a27`; producer retained `5/5` pass, run
  `baaf9f04...`;
- exact-source complete LSE crate: producer retained `87/87` pass, run
  `dcd3e84b...`;
- `nix develop -c cargo nextest run --test
  land_surface_energy_balance_authority_contract`: `9/12` pass, run
  `03260332-a1b6-4d55-8b72-596fec39839e`; the stale LSE lifecycle-row assertion
  and two separately owned version-14 production/adoption assertions fail.

## Findings

### `LSE-001F-B-001` — BLOCKER — required real-consumer proof is not passing

`SC-LANDSURFACEENERGY-001.md:1240-1244` requires both unchanged interior
terminal-event vectors to *complete* with current-state acceptance, unchanged
owner closure, and no trial installation. The retained post-fix evidence at
`contract-test-implementation-evidence.md:63-68` instead records both tests
stopping at `qualification terminal snow-free successor chronology`. Clearing
the earlier `LSEB-E-034` establishes reachability past the original failure,
but it does not establish completion, accepted-current operand identity, owner
closure, or absence of trial publication in the real consumer.

Impact: the package's explicit consumer-path and non-deferral gates remain
open. Proposed disposition: `accepted`; rerun both unchanged consumers after
the chronology correction, require PASS, and retain assertions/evidence for
accepted-current identity, primitive owner closure, and no trial installation.

### `LSE-001F-B-002` — CLOSED AFTER CORRECTION — ordering/classification controller coverage

The corrected source extracts the installed `b=1..20` ordering into private
controller `covered_first_domain_valid_halved_no_update_witness`
(`solver_covered_solve.rs:68-95`) and calls that exact controller from the
production solve (`solver_covered_solve.rs:577-615`). Its probe result
distinguishes domain-invalid, evaluation-incomplete, and completely evaluated
step metadata. The controller returns only `(exponent, CoveredStepNorms)`, so
it cannot return or install a prospective trial.

Static: direct controller tests at `solver_covered_solve.rs:882-935` prove
ordered skipping of domain-invalid factors to the first complete candidate;
immediate refusal on an evaluation-incomplete or step-failing first domain-
valid candidate; no later-candidate skip; and no probing without a typed full-
trial refusal. Adjacent predicate tests retain both typed refusal positives,
complete-member nonfinite/out-of-tolerance poisons, and per-governed-coordinate
including NaN refusal. The unchanged actual-update loop remains separately
strict-decrease-only.

Disposition recommendation: `accepted-and-closed`. Exact-source controller
vectors pass `5/5` in both the producer and independent reviewer runs, and the
complete exact-source LSE crate passes `87/87` in the retained producer run.

### `LSE-001F-B-003` — HIGH — canonical source binding does not protect INV-139

The package failure inventory declares an exact authority-test/index/impact-map
binding for 001F, but `land_surface_energy_balance_authority_contract.rs`
currently contains no `INV-LANDSURFACEENERGY-139` or first-domain-valid
assertion. Its version-12 lifecycle-row assertion also fails after the current
row correctly advanced to version 14. No exact-path impact-map entry names
`solver_covered_solve.rs` or this authority test.

Impact: canonical admission is red and the preserved v13 invariant can be
dropped from a successor contract without the declared source-bound guard.
Proposed disposition: `accepted`; add a successor-safe INV-139 text/invariant
assertion, reconcile the lifecycle-row assertion to the current row, add the
prospectively declared exact critical binding, then run the focused authority,
A0, and anti-evasion gates. Version-14 scientific content remains outside this
finding.

### `LSE-001F-B-004` — CLOSED AFTER AUTHORITY CLARIFICATION — cumulative diagnostic

Canonical version 14 now expressly requires adding the exact examined exponent
to the existing cumulative backtracking-count diagnostic and prohibits a
separate public or persisted field (`SC-LANDSURFACEENERGY-001.md:1213-1217`).
The controller returns the exact exponent and production supplies
`backtracking_count + exponent` to unchanged candidate construction
(`solver_covered_solve.rs:577-615`). Prospective norms remain those of the
witness while solution/evaluation remain current.

Disposition recommendation: `authority-clarified-and-closed`. Production now
matches the prospective canonical diagnostic semantics exactly and does not
introduce persistent microstepping telemetry.

## Preserved behavior observed statically

- The current-vector gate now checks every member finite and `abs <= 1`; the
  prior scalar-infinity-norm NaN masking risk is closed in the reviewed source.
- Exact inclusive step thresholds are unchanged; step NaN refuses and `ci`
  remains diagnostic.
- Typed `DomainInvalid` and `GovernedStepThresholdExceeded` are the only new
  full-trial classification routes; a domain-valid full passing witness retains
  priority.
- The private controller owns the dedicated `b=1..20` witness order and
  precedes the unchanged `b=0..20` strict-decrease loop. It does not skip the
  first domain-valid candidate for a smaller witness; evaluation failure or
  step refusal ends witness consideration, after which actual-update search
  remains unchanged. Its return type contains only exponent and step metadata,
  never a prospective trial.
- Witness acceptance passes current `x` and current `detail` to candidate
  construction; no prospective trial value is installed.
- Every actual update still passes `is_strict_residual_decrease`; the
  iteration/backtracking ceilings and typed failure/rollback paths are
  otherwise unchanged.
- No debug print or package-specific persistent telemetry was found in the
  reviewed production source.

Final recommendation: `HOLD` on external evidence only. B-001 chronology and
B-003 canonical binding remain parent-owned closure gates. B-002 and B-004 are
closed; this review has no undispositioned 001F contract/production-code
finding. A later verification should use
the corrected exact source and passing real-consumer evidence rather than this
review's current failing state.
