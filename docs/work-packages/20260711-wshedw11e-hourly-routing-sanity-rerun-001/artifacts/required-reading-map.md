# Required Reading Map

Status: `PASS`

Evidence mode: `Static`

## Reading Budget

- local required bytes total: `341601`
- threshold outcome: `OK`
- measurement: `wc -c` over the seven Core paths below
- measured at: `2026-07-11 UTC`

## Map

| Path | Tier | Trigger and use | Read timing |
|---|---|---|---|
| `AGENTS.md` | Core | root governance and user-provided instruction map | pre-scaffold |
| `docs/codex_exec_plans.md` | Core | living ExecPlan requirements | pre-execution |
| `docs/work-packages/AGENTS.md` | Core | package gates, consumer/conservation evidence, reviews | pre-scaffold |
| `docs/work-packages/README.md` | Core | catalog and forward-only roadmap policy | pre-scaffold |
| W11E `package.md` | Core | current objective, boundaries, gates | immediately after scaffold |
| W11C `artifacts/disposition.md` | Core | historical failed comparator | pre-execution |
| W11D `artifacts/disposition.md` | Core | correction and accepted fingerprint | pre-execution |
| `crates/AGENTS.md` | Conditional, triggered | release CLI/test execution rules | pre-run |
| science-contract `AGENTS.md` | Conditional, triggered | canonical result classification | pre-run |
| `docs/standards/AGENTS.md` and prompt guidance | Conditional, triggered | package kickoff/subagent wording | pre-scaffold |
| local-CI gate selection | Conditional, triggered | erosion/full and timing selection | pre-run |
| `SC-ROUTE-001` INV-ROUTE-021/022 | On-demand, triggered | KW storage/volume and MC admission verdict | pre-run |
| `SC-SYSTEM-001` INV-SYSTEM-036 | On-demand, triggered | terminal water/sediment publication verdict | pre-run |
| `SC-INFILE-CHANINP-001` v0.1.4 | On-demand, triggered | zero-count record/timestep verdict | pre-run |

## Instruction Chains

`tools/agents/find-agents` resolves root plus
`docs/work-packages/AGENTS.md` for package/catalog files and root alone for
`docs/ROADMAP.md`. No Rust file is edited. The existing dev-guide modification
is outside W11E and excluded from ownership.
