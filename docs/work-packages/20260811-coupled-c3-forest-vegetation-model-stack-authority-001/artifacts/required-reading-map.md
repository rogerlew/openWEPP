# Required Reading Map

Status: `complete / intake and triggered reading measured`

Evidence mode: `Static + Ran`

Ran: `wc -c` over the exact worktree files after package scaffolding on
2026-08-11. Core totals `795845` local bytes and therefore has disposition
`WARN` (`>400000` and `<=800000`). Conditional totals `148873` bytes and is
excluded from the Core threshold per the canonical tiering rule. External
literature and read-only source-checkout bytes are on-demand and excluded until
their process trigger applies.

## Core

| Bytes | Path | Rationale | Intake state |
|---:|---|---|---|
| 11927 | `AGENTS.md` | Root governance and scope | read |
| 20921 | `docs/codex_exec_plans.md` | Living ExecPlan requirements | read |
| 26367 | `docs/work-packages/AGENTS.md` | Package lifecycle and evidence rules | read |
| 392394 | `docs/work-packages/README.md` | Canonical catalog and lifecycle context | read in bounded chunks; catalog size drives WARN |
| 10508 | `docs/standards/prompt-wording-guidance.md` | Kickoff/delegation wording | read |
| 15309 | `docs/standards/kernel-work-package-preparation.md` | Critical package scaffold/gates | read |
| 18865 | `docs/work-packages/20260811-coupled-c3-forest-vegetation-model-stack-authority-001/package.md` | Active package authority | read after scaffold |
| 29208 | `docs/work-packages/20260808-rhessys-east-coast-coupled-vegetation-slice-001/package.md` | Held implementation successor | read |
| 5599 | `docs/specifications/science-contracts/AGENTS.md` | Canonical contract governance | read |
| 60482 | `docs/specifications/science-contracts/contracts/SC-VEGETATION-001.md` | Current vegetation authority | reading active before amendment |
| 4890 | `docs/decisions/0042-science-implementation-and-calibration-readiness.md` | Claim separation | read |
| 2240 | `docs/governance/reference-vendoring-policy.md` | Rights custody | read |
| 1478 | `references/README.md` | Corpus workflow | read |
| 176836 | `references/annotated_bibliography.md` | Existing source identities and annotations | reading active before authority use |
| 6245 | `docs/work-packages/20260808-rhessys-east-coast-code-literature-authority-audit-001/artifacts/authority-gap-register.md` | `AUTH-RHEC-*` intake population | read |
| 12576 | `docs/work-packages/20260809-native-forest-ecohydrology-authority-reframe-001/package.md` | Binding caller/native-flux reframe | read |
| **795845** | **Core total** | **Canonical threshold result** | **WARN** |

The two heavy files cannot move to On-demand because the user explicitly made
the complete work-package catalog and bibliography Core, and both are needed to
prevent duplicate lifecycle authority or use of stale/non-binding references.

## Conditional — Triggered Before Canonical/Test Edits

| Bytes | Path | Trigger |
|---:|---|---|
| 13715 | `docs/specifications/science-contract-authoring-procedure.md` | canonical amendment |
| 5792 | `docs/specifications/science-contracts/kernel-process-contract-profile.md` | canonical amendment |
| 11093 | `docs/specifications/science-contracts/index.md` | registry edit |
| 13749 | `docs/specifications/science-contract-spec.md` | contract/readiness schema |
| 12896 | `docs/specifications/unit-governance.md` | equation/unit authority |
| 11159 | `docs/specifications/correctness-authority-model.md` | comparator/adjudication |
| 22200 | `docs/standards/testing-and-gate-strategy.md` | gate selection |
| 15991 | `references/rights_classification_first_pass_2026-05-11.md` | reference custody |
| 4723 | `tests/AGENTS.md` | test edit |
| 19217 | `tests/integration/vegetation_boundary_authority_contract.rs` | contract-derived test edit |
| 13090 | `docs/work-packages/20260808-rhessys-east-coast-code-literature-authority-audit-001/artifacts/source-function-state-inventory.md` | RHESSys family mapping |
| 3424 | `docs/work-packages/20260808-rhessys-east-coast-code-literature-authority-audit-001/artifacts/literature-acquisition-log.md` | reference acquisition |
| 1824 | `docs/work-packages/20260809-native-forest-ecohydrology-authority-reframe-001/artifacts/primary-source-ledger.md` | carry-forward source identity |
| **148873** | **Conditional total** | **Tracked separately; all triggers apply** |

## On Demand

- `SC-LANDSURFACEENERGY-001`, `SC-EVAP-001`, `SC-WATBAL-001`,
  `SC-PLANT-001`, `SC-RESIDUE-001`, and `SC-SNOWFREEZE-001`: read before any
  corresponding ownership/handoff amendment.
- RHESSysEastCoast/GIS2RHESSys functions, format generators, and licenses: read
  by selected process family; never edited.
- Exact established-model definitions and primary literature: read after
  acquisition identity/rights/checksum entry and before using the family.
- Roadmap, backlog, authority registries, suites, and successor files: read
  before their lifecycle amendment.
- Existing canopy-phenology source/contracts: read before GSI/foliar-C/SLA/LAI
  ownership amendment.

## Measurement Command

Ran: a Bash array containing the 16 Core paths, `wc -c` for each file, and
integer summation. The package was 18,865 bytes at measurement time; later
living-plan edits do not retroactively change this frozen intake budget and are
captured by terminal diff/owned-file reconciliation.
