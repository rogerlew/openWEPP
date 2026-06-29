# FROST STEP 2 Sleepers Attribution

Status: complete — `EXECUTED-COMPLETE-DIAGNOSTIC-ATTRIBUTION`.

Package type: diagnostic-only work package.

Objective: attribute frost-depth residuals at the two Step 1 unblocked Sleepers
sites using the `INV-SNOWFREEZE-050` forcing-robust rubric and the Step 1
current-snow comparison reports.

Primary gap: `GAP-SNOWFREEZE-002`.

## Scope

Included sites:

- `site1_sleepers_south_field_vt`
- `site2_sleepers_w9_hardwood_vt`

Excluded sites:

- `site4_ggd498_morris_mn`: Step 1 `BLOCKED`.
- `site3_scan_mandan_nd`: Step 1 `INCONCLUSIVE-NO-PAIRED-SNOW`.
- `site5_reynolds_creek_us_rls_id`: Step 1 `INCONCLUSIVE-NO-PAIRED-SNOW`.

Included analysis:

- Per-water-year onset, thaw, and frozen-duration timing verdicts.
- Sign-coherence check against Step 1 snow over-prediction.
- Frost-depth magnitude residual summary, tagged forcing-limited and
  sign-coherence-aware, without converting magnitude-only residuals into
  frost-model defects.
- Step 3 candidate pointers for timing failures not explained by snow forcing.
- Updated package-local `GAP-SNOWFREEZE-002` disposition input.

Excluded:

- No frost-model changes.
- No snow-model changes.
- No `Qwet`, frozen-K, SFCC, impedance, residue, or heat-flow implementation.
- No contract ratification.
- No default activation, output-schema change, fixture edit, or selector change.

## Required Reading

- `docs/planning/snow-frost-fidelity-strategy.md` section 11.
- `docs/work-packages/20260629-frost-step1-current-snow-control-rerun-001/`.
- `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
  `INV-SNOWFREEZE-047`, `INV-SNOWFREEZE-048`, `INV-SNOWFREEZE-050`,
  `TOL-SNOWFREEZE-009`, and `GAP-SNOWFREEZE-002`.
- `docs/decisions/0017-re-pin-operational-distrust-comparator-is-flag-not-target.md`.
- `docs/decisions/0028-observed-data-admission-authority.md`.
- `docs/work-packages/20260624-snowfreeze-observed-frost-depth-harness-001/`.

## Intended Write Set

- `docs/work-packages/20260629-frost-step2-sleepers-attribution-001/**`
- `docs/work-packages/README.md`
- `docs/planning/snow-frost-fidelity-strategy.md`

## Execution Plan

1. Consume Step 1 Sleepers comparison reports and routing evidence.
2. Reconstruct frost-depth residual distribution from the already-produced WAT
   files referenced by those reports and the normalized observation corpus.
3. Score onset, thaw, and frozen-duration residuals against the 14-day timing
   tolerance.
4. Apply sign coherence: Step 1 snow is modeled deeper than observed, so snow
   forcing can explain shallower, later-onset, earlier-thaw, shorter-duration
   frost; opposite-sign timing failures become candidate frost-model defects.
5. Record per-site tables, Step 3 pointers, review, verification, and
   disposition.

## Exit Criteria

- Only the two Step 1 `FORCING-LIMITED` Sleepers sites are analyzed.
- Per-site timing tables are emitted with pass/fail and sign-coherence
  classifications.
- Magnitude residuals are reported as forcing-limited and not used alone as
  defect verdicts.
- Candidate frost-model defects, if present, point to Step 3 candidate families
  without implementing them.
- `GAP-SNOWFREEZE-002` remains open or is narrowed truthfully.
- Markdown validation passes for touched docs.

## Disposition

The package consumed the Step 1 Sleepers reports already captured under
`20260629-frost-step1-current-snow-control-rerun-001/` and did not rerun the
harness. Both Step 1 `FORCING-LIMITED` Sleepers sites still expose
verdict-bearing timing failures whose direction is not explained by modeled
snow over-prediction:

- `site1_sleepers_south_field_vt`: `4` candidate frost-model timing cells,
  all thaw-late cells. `7` timing failures are forcing-attributable to deeper
  modeled snow, and `85/96` timing signatures pass.
- `site2_sleepers_w9_hardwood_vt`: `14` candidate frost-model timing cells,
  including early-onset and thaw-late cells. `8` timing failures are
  forcing-attributable, and `49/75` timing signatures pass.

Magnitude remains forcing-limited and non-verdict-bearing. South Field has a
mixed-sign magnitude profile; W9 has sign-incoherent over-deep frost magnitude,
but this is a magnitude signal, not a standalone model-defect verdict.

`GAP-SNOWFREEZE-002` remains open and is narrowed to Step 3 investigation of the
timing candidate defects at the two Sleepers sites. Primary pointer:
residue-lifecycle handoff (`static` vs dynamic `resdep`), with the legacy-
envelope outlier flag as comparator context. The absent `Qwet` evaporative term
is not the primary pointer from this sign-coherence pass.

## Security / Production Impact

Diagnostic-only. No runtime, physics, fixture, output-schema, default, or
contract authority changes.
