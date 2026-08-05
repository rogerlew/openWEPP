# Jennings Dryad Hourly Archive Git LFS Vendoring

Status: `executing / awaiting independent review`

This ExecPlan is a living document governed by `docs/codex_exec_plans.md`.

## Purpose

Vendor the exact Jennings et al. hourly precipitation-phase observation CSV
from Dryad DOI `10.5061/dryad.c9h35`, version 1 published 2019-01-31, into its
existing
source-native fixture path through Git LFS. Preserve DOI, original UCAR
`ds464.0` lineage, CC0 redistribution authority, byte identity, and an opt-in
heavy-data validation posture.

## Frozen Intake

- User authority: direct instruction on 2026-08-05 to vendor the archive.
- Source DOI: `https://doi.org/10.5061/dryad.c9h35`.
- Source file:
  `jennings_et_al_2018_file2_ppt_phase_met_observations.csv`.
- Existing local path:
  `tests/fixtures/precip_phase_observed/jennings2018/jennings_et_al_2018_file2_ppt_phase_met_observations.csv`.
- Size: `1,206,721,342` bytes.
- SHA-256:
  `0cc82fbc5211c2c24b19653c4711d63a88fc4ed7bd90fc39cce84913d071f3a1`.
- Expected rows: `17,810,806` including the header.
- Rights: Dryad datasets are published under CC0; scholarly citation and the
  provider-requested original UCAR `ds464.0` citation remain required
  provenance practice.

## Objective

Replace the local-only/gitignored custody of the exact hourly CSV with a
tracked Git LFS pointer and object, add a checksum manifest and accurate
fixture/reference ledgers, prove the staged pointer binds the frozen bytes,
retain ordinary test behavior without requiring the 1.2 GB object in unrelated
profiles, and document Git LFS clone bandwidth plus the no-smudge opt-out.

## Implementation Intent

Intent: external observed-data authority vendoring and custody only. No kernel,
runtime, science-contract, calibration, result, default, assurance report, or
public-output behavior changes.

## Included Scope

- Scope one Git LFS rule to the exact hourly CSV.
- Stop ignoring that exact file.
- Track the current frozen bytes through Git LFS.
- Add or update checksum, DOI/version, CC0, UCAR lineage, size, cadence,
  columns, and applicability documentation.
- Update the reference bibliography, rights ledger, work-package catalog, and
  roadmap.
- Run LFS pointer/object, checksum, row-count, CSV-shape, documentation,
  focused consumer, anti-evasion, review, and verification checks.

## Excluded Scope

- Re-downloading or transforming the already frozen matching local file.
- Editing observation values, missing-data semantics, column names, or order.
- Adding derived outputs or a second 1.2 GB copy under `references/`.
- Making the Jennings corpus calibration authority, independent validation for
  a claim it already informed, or a required ordinary-CI download.
- Kernel/runtime/default/report/publication/release changes.
- Pushing the Git LFS object or commits without separate user authority.

## Intended Write Set

- `.gitattributes`
- `docs/ROADMAP.md`
- `docs/work-packages/README.md`
- `docs/work-packages/20260805-jennings-dryad-hourly-archive-lfs-vendoring-001/**`
- `references/annotated_bibliography.md`
- `references/rights_classification_first_pass_2026-05-11.md`
- `tests/fixtures/precip_phase_observed/jennings2018/.gitignore`
- `tests/fixtures/precip_phase_observed/jennings2018/README.md`
- `tests/fixtures/precip_phase_observed/jennings2018/SHA256SUMS`
- `tests/fixtures/precip_phase_observed/jennings2018/VENDORED_ARCHIVE.md`
- `tests/fixtures/precip_phase_observed/jennings2018/jennings_et_al_2018_file2_ppt_phase_met_observations.csv`

Everything else is read-only.

## Progress

- [x] (2026-08-05) User authorized exact DOI-based vendoring.
- [x] (2026-08-05) Confirmed local size, checksum, existing provenance, and
  Git LFS availability.
- [x] (2026-08-05) Scaffolded package, active prompt, intake, reading map, and
  queue pointers before fixture/LFS edits.
- [x] (2026-08-05) Installed the exact LFS rule, staged pointer, checksum
  manifest, and provenance updates without rewriting the archive.
- [x] (2026-08-05) Ran byte/object/pointer/CSV checks, two deterministic
  current-consumer runs, and both external-authority anti-evasion gates.
- [ ] Complete dual review, dual terminal verification, and disposition.

## Phase Plan

### Phase 1 — Scaffold And Rights Freeze

Freeze source identity, rights, write set, validation requirements, and
protected no-physics/no-publication boundaries. Commit the scaffold before
fixture or LFS changes.

### Phase 2 — LFS Installation And Provenance

Add the exact path-scoped LFS rule before staging the CSV, remove its local
ignore rule, add `SHA256SUMS`, and reconcile fixture/reference documentation.

### Phase 3 — Validation And Review

Prove the working-tree bytes, staged LFS pointer, local LFS object, row count,
header, checksum manifest, DOI/license records, and existing consumer. Run
applicable focused checks and external-authority anti-evasion guards. Complete
dual independent review and disposition findings.

### Phase 4 — Terminal Verification And Disposition

Reconcile the exact diff/write set, run dual terminal verification, update
catalog/roadmap state, archive the prompt, and commit the stable increment.

## Validation Requirements

1. `git check-attr filter diff merge text -- <hourly-path>` reports the exact
   LFS attributes.
2. `sha256sum -c SHA256SUMS` passes and the hourly file remains exactly
   `1,206,721,342` bytes and `17,810,806` lines.
3. The header and representative CSV parsing prove the documented ten columns
   without rewriting the source.
4. `git lfs status` and `git show :<hourly-path>` prove a valid pointer with
   OID `sha256:0cc82f...` and size `1206721342`.
5. The corresponding local `.git/lfs/objects` object exists and matches the
   frozen SHA-256.
6. The existing Jennings consumer/harness focused check passes without
   changing scientific results.
7. `bash tools/release/check_authority_suite_antievasion.sh` and
   `cargo nextest run --test auth11_required_suite_obligation_guards_contract`
   pass because external-authority fixture custody changes.
8. Documentation lint/validation, American-English preview, path checks, and
   `git diff --check` pass.
9. Dual review, finding disposition, dual terminal verification, and exact
   write-set reconciliation pass.

No full-workspace correctness run is selected: the intended diff changes only
external-data custody, LFS metadata, checksums, and prose. Repeated current
consumer runs must retain identical row counts, confusion matrices, and
headline accuracies. Aggregate threshold floats and JSON bytes are
observational because the pre-existing consumer reduces station values from
an unordered `HashMap`.
The retained June report predates the hydrometeor solver bisection fallback in
commit `62063495`; it is historical evidence rather than the expected output
of current `HEAD`.

## Subagent Authorization

Subagent authorization: this package explicitly authorizes subagent
spawning/delegation to two read-only reviewers and two read-only terminal
verifiers for rights/provenance, LFS/pointer integrity, consumer evidence,
write-set reconciliation, and disposition. Expected outputs are compact
findings returned to the parent and retained as package artifacts. No subagent
may edit files, push Git/LFS objects, reinterpret observations, or create
scientific/publication authority.

Subagent requirement: none for heavy batch execution because no heavy batch or
campaign/release boundary is selected.

## Exit Criteria

- The exact frozen hourly CSV is tracked by a path-scoped Git LFS pointer.
- Checksum, size, rows, DOI/version, CC0, UCAR lineage, and consumer purpose are
  explicit and verified.
- Ordinary profiles are not silently made dependent on the 1.2 GB download.
- No observation, science, runtime, result, assurance, or public surface
  changes.
- All selected validation, review, and verification requirements pass.

## Decision Log

- Decision: track the existing source-native fixture rather than duplicate it
  under `references/vendorable/`. Rationale: this path is the real Snowbench
  consumer input, while bibliography and rights ledgers can bind it without a
  second 1.2 GB object. Date/Author: 2026-08-05 / Codex.
- Decision: use one exact-path LFS rule rather than a global CSV rule.
  Rationale: ordinary text CSV fixtures should remain diffable; this source is
  exceptional because it is 1.2 GB. Date/Author: 2026-08-05 / Codex.

## Outcomes And Retrospective

Execution installed exact-path Git LFS custody for the unchanged Dryad bytes.
Integrity and current-consumer checks pass. Independent review and terminal
verification remain pending.

## Idempotence And Recovery

Do not stage the hourly CSV until the path-scoped LFS rule exists. If staging
is interrupted, verify attributes, reset only the path from the index without
discarding its working-tree bytes, and restage. Never rewrite or normalize the
CSV. Preserve the Dryad bytes and checksum as the recovery authority.
