# Required Reading Map

Status: `PASS`

Evidence mode: `Static`

## Reading Budget

- core_local_repo_bytes: `336472`
- triggered_local_repo_bytes: `756793`
- pinned_baseline_source_bytes: `95408`
- core_threshold_outcome: `OK` (`<=400000` bytes)
- triggered_threshold_outcome: `WARN` (`>400000` and
  `<=800000` bytes)
- measurement_method: `wc -c` over the exact path sets below
- measured_at_utc: `2026-07-11T04:42:09Z`

The WARN total is justified because this package changes three canonical
contracts and must adjudicate four coupled defects against the pinned routing
and parser source. The large `SC-ROUTE-001` and `SC-SYSTEM-001` files were
loaded phase-locally at the affected invariant, guard, WS11, gap, and revision
sections; unrelated historical addenda were not treated as pre-edit authority.

## Map

| Path | Tier | Bytes | Why required | Read timing |
|---|---|---:|---|---|
| `AGENTS.md` | Core | 10624 | Root governance and kernel gates | Pre-edit |
| `docs/codex_exec_plans.md` | Core | 20708 | Living ExecPlan requirements | Pre-edit |
| `docs/work-packages/AGENTS.md` | Core | 19044 | DC closure, evidence, review, and conservation rules | Pre-edit |
| `docs/work-packages/README.md` | Core | 277979 | Catalog/process context; active W11C/W11D rows loaded | Pre-edit |
| package `package.md` | Core | 8117 | Authority envelope and acceptance | Pre-edit |
| `docs/defect_closure_execplans.md` | Conditional | 24803 | This is a DC-ExecPlan | Pre-edit |
| science-contract `AGENTS.md` | Conditional | 5599 | Canonical contract-first governance | Pre-edit |
| `docs/specifications/science-contract-authoring-procedure.md` | Conditional | 12423 | Three canonical contracts change | Pre-edit |
| `docs/specifications/science-contracts/kernel-process-contract-profile.md` | Conditional | 5044 | Routing kernel semantics change | Pre-edit |
| `docs/specifications/science-contracts/index.md` | Conditional | 9024 | Contract lifecycle registry | Phase-local |
| `docs/standards/AGENTS.md` | Conditional | 3328 | Standards routing | Pre-edit |
| `docs/standards/kernel-work-package-preparation.md` | Conditional | 13488 | Kernel package and operand-lineage gates | Pre-edit |
| `docs/standards/local-ci-gate-selection.md` | Conditional | 3501 | Focused/full gate selection | Pre-edit |
| `crates/AGENTS.md` | Conditional | 5171 | Rust production rules | Pre-edit |
| `tests/AGENTS.md` | Conditional | 4534 | Contract/integration test rules | Pre-edit |
| `tests/fixtures/AGENTS.md` | Conditional | 9631 | Protected p102 wrapper correction, checksum, provenance, and anti-evasion rules | Post-review before fixture edit |
| `SC-ROUTE-001.md` | On-demand | 128112 | Wave storage, MC recurrence, and routing publication authority | Phase-local before contract edit |
| `SC-SYSTEM-001.md` | On-demand | 145739 | Terminal consumer/publication authority | Phase-local before contract edit |
| `SC-INFILE-CHANINP-001.md` | On-demand | 23734 | Conditional record closure for `nchnum=0` | Phase-local before contract edit |
| ADR-0012 | On-demand | 4212 | Pinned legacy provenance anchor | Phase-local |
| W11C package and six named evidence/handoff artifacts | On-demand | 21978 | Reproduction, operands, and named mechanisms | Phase-local |
| pinned `wshchr.for`, `wshpek.for`, `wshdrv.for`, `wshinp.for` | On-demand external baseline | 95408 | Recurrence, storage, publication, and parser provenance | Phase-local before contract edit |

## Instruction Chains

Ran: `tools/agents/find-agents --for` resolves root plus `crates/AGENTS.md` for
all Rust crate/test paths; root plus `tests/AGENTS.md` and
`tests/fixtures/AGENTS.md` for the protected p102 wrapper; root plus
science-contract `AGENTS.md` for each `SC-*` edit; and root plus
`docs/work-packages/AGENTS.md` for package/catalog artifacts.

All Core and triggered Conditional governance was read before repository edits.
The affected On-demand authority and W11C evidence were read before canonical
contract amendment; remaining source slices may be reread during implementation
without changing the budget classification.
