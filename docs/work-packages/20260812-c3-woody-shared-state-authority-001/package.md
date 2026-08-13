# Admit C3 Woody V4 Shared-State Authority

Status: `COMPLETE / OPENWEPP_C3_WOODY_V4 shared-state implementation authority released`

Date: `2026-08-12`

Package ID: `20260812-c3-woody-shared-state-authority-001`

Plan class: `Critical contract-first scientific-authority correction`

This ExecPlan is a living document maintained under
`docs/codex_exec_plans.md`.

## Objective

Admit `OPENWEPP_C3_WOODY_V4` under `SC-VEGETATION-001@8`, selecting displayed
leaf carbon and nitrogen as the sole LAI and leaf-capacity owners and removing the unconsumed
`previous_leaf_offset_flux` and `previous_root_offset_flux` fields from the
executable shared-stratum state. Bind the exact successor schema, migration,
definition identity, and independent shared-state fixtures without editing
production Rust or changing immutable V1/V2/V3 definition bytes.

## Progress

- [x] Freeze intake, required reading, predecessor identity, and protected bytes.
- [x] Amend canonical authority to V8 before implementation work.
- [x] Generate deterministic independent V4 shared-state fixtures and definition.
- [x] Complete two independent science reviews and disposition every finding.
- [x] Run focused, authority, anti-evasion, documentation, and critical heavy gates.
- [x] Reconcile exact terminal bytes and complete two independent verifications.
- [x] Archive the active kickoff prompt byte-for-byte and release the implementation handoff.

## Scope

Included:

- exact V4 shared-stratum schema and canonical state serialization obligations;
- displayed-leaf-carbon-only LAI and derived leaf/stem/root area identities;
- exact V3-to-V4 migration with removal of only two unconsumed fields;
- deterministic independent fixtures, poisons, definition bytes, digests, and
  authority evidence;
- dual independent science review, finding disposition, critical gates, dual
  terminal verification, and implementation handoff.

Excluded:

- Rust or runtime changes;
- edits to V1/V2/V3 definition or fixture bytes;
- new phenology, turnover, allocation, or offset-flux equations;
- runtime activation, selector changes, deployment, publication, PRs, remote
  branch creation, or pushes unless separately directed;
- calibration, identifiability, empirical validation, transferability, canopy
  snow, soil transformations, or real-consumer cutover.

Protected identities:

- V1 `003107043e8eb5bda6d9d6476e3ea01690815e3280ac98daf169317ce4d09157`;
- V2 `38e1bb90abd3ff82879f7d9c80b0377bb510a3b97fdd2b6f07c12b7c42b80dc3`;
- V3 `7768657ca3d03603b66f5cd6677f032ee630fdd46d6ffadf214c713065f73852`.

## Intended Write Set

- this package tree;
- `docs/specifications/science-contracts/contracts/SC-VEGETATION-001.md`;
- `docs/specifications/science-contracts/index.md`.
- `docs/work-packages/README.md`.

No production crate, integration test, predecessor authority artifact, runtime
selector, or consumer is in the write set.

## Canonical Selections

1. `LAI_s=tissues[leaf].display.carbon*SLA_s` exactly, and positive-LAI
   `Nleaf_area=tissues[leaf].display.nitrogen/LAI_s` exactly.
2. Leaf storage/transfer C/N supplies no exposed area or leaf capacity until
   accepted transfer to display.
3. `SAI_s=LAI_s*sai_relation_s` and
   `RAI_s=(LAI_s+SAI_s)*root_to_leaf_area_s`; serialized areas are integrity caches.
4. V4 shared state has the exact field set in the V8 contract amendment and no
   previous leaf/root offset-flux field.
5. V3-to-V4 migration validates V3, removes only those two fields, reconstructs
   exact areas, copies everything else exactly, rebinds V4 identity, and rejects
   source cache mismatch without partial output.
6. Removal invents no material, energy, water, phenology, or numerical semantics.

## Phase Plan

1. Record exact HEAD/worktree, instruction discovery, required reading, and protected digests.
2. Amend the canonical contract and registry.
3. Generate the independent fixture and canonical V4 definition deterministically.
4. Run focused generator/digest/schema/Markdown/authority gates and preserve all results.
5. Obtain two independent science reviews, fix accepted findings, and repeat to PASS.
6. Run the critical terminal command set, reconcile exact bytes, and obtain two independent verifications.
7. Archive the kickoff prompt byte-for-byte and publish a bounded implementation handoff.

## Review and Delegation Authorization

Subagent authorization: this package explicitly authorizes and requires
spawning/delegating to two independent science reviewers for shared-state
schema, LAI ownership, migration, digest binding, fixture independence, and
historical-byte preservation; one `comparator_suite_runner` for heavy batch
closure; and two independent terminal verifiers for final exact-byte
verification. Expected outputs are bounded package review/verification
artifacts and compact gate metrics/log paths. Reviewers and verifiers are
read-only except for their named package artifact; the comparator has bounded
write access only to package gate logs/results. The parent owns authority and
fixture corrections and finding disposition.

## Validation and Acceptance

Required evidence:

- deterministic Python regeneration with byte-identical fixture and definition;
- exact V1/V2/V3 byte and digest preservation;
- V4 canonical JSON, imported V3 digest, V8-section digest, fixture digest, and
  generator digest identity;
- exact shared-schema, area, migration, digest-mutation, and poison checks;
- science-contract admission, unit compliance, A0 authority, AUTH11
  anti-evasion, package Markdown, formatting, and diff hygiene;
- warnings-denied workspace Clippy, full-workspace nextest, workspace doctests,
  and dependency audit because this is a Critical kernel-authority amendment;
- dual PASS science reviews, complete disposition, exact terminal reconciliation,
  and dual PASS verifications.

Any `FAIL`, `BLOCKED`, or unjustified `NOT RUN` prevents completion. Failed
attempts remain in `artifacts/gate-results.md`.

## Exit Criteria

The package closes only when V8 and V4 bind the exact selected schema and
equations, independent fixtures distinguish every named poison, predecessor
bytes remain unchanged, every gate passes, both reviewers and both verifiers
PASS, and the active prompt is archived byte-for-byte.

Terminal status is:

`COMPLETE / OPENWEPP_C3_WOODY_V4 shared-state implementation authority released`

It continues to state
`calibration_evidence_status=NOT_CALIBRATION_READY`,
`identifiability_status=NOT_ASSESSED`, no runtime activation, no real consumer
cutover, no canopy snow, no soil transformations, and no empirical validation
or transferability claim.

## Idempotence and Recovery

All writes are flat files. The reference calculator is deterministic. Failed
commands and reviews are appended. No history is rewritten. If the exact
shared-state authority or independent evidence cannot be established, the
package remains HOLD and the existing implementation stays fail-closed.

## Transient Heavy-Gate Failure Audit

The first required Critical full-workspace attempt failed. Comparator Clippy
passed, but full nextest could not compile concurrently modified
`crates/openwepp-vegetation/src/occupancy_solver/potential.rs` because several
`StageAEvaluation` initializers omit `active_water_caps`. Rust is outside this
authority package's declared write set and owned by the active implementation
campaign. The implementation owner corrected the transient defect, so a safe
in-scope heavy retry is active. Focused V4 authority gates and both science
reviews pass; V1/V2/V3 bytes remain protected. Rerun the complete heavy command
set and both terminal verifiers against stable bytes; retain the failed attempt.
