# Admit Six-Tissue Storage-to-Transfer Phenology Authority

Status: `complete / OPENWEPP_C3_WOODY_V7 storage-transfer phenology authority released`

Date: `2026-08-13`

Package ID: `20260813-c3-woody-storage-transfer-phenology-authority-001`

Plan class: `Narrow critical contract-first constitutive authority amendment`

This ExecPlan is a living document maintained under
`docs/codex_exec_plans.md`.

## Objective

Release `OPENWEPP_C3_WOODY_V7` under `SC-VEGETATION-001@11` by admitting the
missing seasonal-deciduous E19-to-E20 storage/transfer preparation and
six-tissue onset deployment. V7 imports V6 unchanged outside this amendment
and preserves all V1--V6 definition bytes.

## Authority Trigger

`GAP-VEGETATION-027` correctly blocks persistent C/N execution because E19
credits noncurrent growth to tissue storage while E20 consumes tissue transfer
and no admitted equation connects them. CLM5 sections 2.19.4--2.19.5,
2.20.1, 2.20.2, and 2.20.3 supply the reference-model rule: on seasonal onset
initiation, move `0.5` of each of six tissue C/N storage pools into its matching
transfer pool, then deploy all six transfer pools with the admitted declining
onset rate. openWEPP selects no second growth-respiration debit.

## Scope

Included:

- canonical V7 amendment, binding exposure, invariants, typed failures, and
  `GAP-VEGETATION-027` disposition;
- independent six-tissue C/N vectors and calculator;
- evergreen exact-zero storage/transfer posture;
- identity-only V6-to-V7 migration rules;
- contract-derived tests, review, verification, and implementation handoff.

Excluded:

- production Rust implementation in this authority package;
- changes to V1--V6 definitions or accepted E01--E19/E21--E22 physics;
- runtime activation, selector change, deployment, publication, consumer
  cutover, calibration, validation, or transferability claims.

## Intended Write Set

- this package tree;
- `docs/specifications/science-contracts/contracts/SC-VEGETATION-001.md`;
- `docs/specifications/science-contracts/index.md`;
- `docs/work-packages/README.md`;
- the canonical model-stack authority copy of the V7 definition; and
- `tests/integration/vegetation_boundary_authority_contract.rs`; and
- the two warnings-denied clone assignments in
  `tests/integration/c3_vegetation_implementation_contract.rs`.

Subagent authorization: this package explicitly authorizes subagent
spawning/delegation to two independent science reviewers and two independent
terminal verifiers, plus one comparator suite runner for the heavy terminal
gates; expected outputs are the named package review and verification
artifacts; write access is bounded to this package tree.

## Progress

- [x] Confirm clean base `79bf36a7b01610c3f7c6702497286dcf02a3b1ad`.
- [x] Verify the load-bearing CLM5 reference equations and branches.
- [x] Freeze V7 event ordering, conservation, migration, and evergreen rules.
- [x] Generate independent fixtures and immutable V7 definition.
- [x] Amend `SC-VEGETATION-001` to Version 11 and update registry lifecycle.
- [x] Add contract-derived tests and pass the focused pre-review authority gates.
- [x] Complete dual independent review and disposition all findings.
- [x] Pass heavy workspace and post-promotion admission gates.
- [x] Complete dual terminal verification and archive the prompt byte-for-byte.
- [x] Issue the existing implementation-package handoff.

## Exit Criteria

Complete only when V7 is canonical and digest-bound; V1--V6 bytes are proven
unchanged; all six tissue C/N preparation/deployment, ordering, evergreen,
migration, conservation, poison, and rollback-vector obligations pass; both
reviews and both terminal verifiers pass; admission, unit, anti-evasion,
Markdown, digest, and applicable workspace gates pass; and the final
disposition makes no implementation or runtime claim.

## Decision Log

- Decision: select CLM5 seasonal onset `f_stor_xfer=0.5` for all six tissue C/N
  pools, adding to existing transfer and consuming beginning storage only.
  Rationale: this exactly closes the owner gap while retaining the existing
  openWEPP onset event and prevents same-interval allocation recycling.
  Date/Author: 2026-08-13 / Codex.
- Decision: storage/transfer relabeling charges no additional growth
  respiration. Rationale: openWEPP already debits growth respiration when E19
  allocates tissue growth; a second debit would violate carbon closure.
  Date/Author: 2026-08-13 / Codex.
