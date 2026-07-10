# Required-Reading Map

Status: `EXECUTED`

Evidence mode: `Ran` (all listed files read/grepped in-session on 2026-07-10;
partial reads are labeled with the sections read).

Executor: Claude Code (operator-directed package-end-to-end execution,
2026-07-10: "I think it makes sense for you to complete the work-package and
handoff to codex for review").

## find-agents instruction chain

Ran `tools/agents/find-agents --for docs/specifications/science-contracts/contracts/SC-ROUTE-001.md docs/ROADMAP.md docs/work-packages/README.md docs/work-packages/20260710-wshedw11a-channel-hourly-sediment-authority-001/`:

- `SC-ROUTE-001.md` → `AGENTS.md`, `docs/specifications/science-contracts/AGENTS.md`
- `docs/ROADMAP.md` → `AGENTS.md`
- `docs/work-packages/README.md` → `AGENTS.md`, `docs/work-packages/AGENTS.md`
- package directory → `AGENTS.md`, `docs/work-packages/AGENTS.md`

## Core tier

| Path | Bytes | Read scope | Rationale |
|---|---|---|---|
| `AGENTS.md` | 10,624 | head (Purpose through Strategy/Provenance) | Root routing, contract-first + no-surrogate directives |
| `docs/work-packages/AGENTS.md` | 19,044 | first ~80 lines (mission, workflow, gate-evidence non-deferral, consumer-path rule) | Package governance for gates/holds |
| `docs/work-packages/README.md` | 275,044 | rows 30-45 (W11A/W11 state entries) | Queue-state rows in write set |
| `docs/specifications/science-contracts/AGENTS.md` | 5,599 | full | Contract authoring rules |
| `docs/standards/AGENTS.md` | 3,328 | full | Standards routing (ASCII rule, prompt wording pointers) |
| `docs/codex_exec_plans.md` | 20,708 | headings skim | Package execution posture (this is an authority/docs package; ExecPlan code-milestone machinery not triggered) |
| `package.md` (this WP) | — | full | Scope/gates/deliverables |
| W11 hold artifacts (`hold-legitimacy-audit.md`, `baseline-source-map.md`, `worker-handoff.md`) | — | full | The hold being lifted; verified source map with pinned-commit commands |

## Conditional tier

| Path | Bytes | Read scope | Rationale |
|---|---|---|---|
| `docs/specifications/science-contract-authoring-procedure.md` | 12,423 | full | Amendment workflow, dual-review/verification gate formats |
| `docs/specifications/science-contracts/contracts/SC-ROUTE-001.md` | 80,433 | full (v50) | Primary amendment target |
| `docs/specifications/science-contracts/contracts/SC-SED-001.md` | 92,568 | targeted (hourly surfaces, INV-SED-011/013/014/016/017, GAP-SED-007/008/009, revision tail) | Producer-side S_h semantics; class-fraction timing state |
| `docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md` | 145,739 | targeted (REF-SYSTEM-CH13-SEDCONT, INV-SYSTEM-001/009/028, GAP-SYSTEM-003/008) | System-level channel sediment continuity rows |
| Pinned-baseline correctness model | — | via science-contracts AGENTS.md §Physics Authority Rules | Baseline pin `dac3c950` |

## On-demand tier

| Path | Read scope | Rationale |
|---|---|---|
| `references/50201000/chap13.pdf` | full (20 pp, rendered) | Primary lineage authority (§13.4-13.6) |
| `references/vendorable/creams/312-ch3.md` + scan pp. 54-55 | full md; widening-law page visually verified | CREAMS Ch. 3 primary source ([I-56], [I-128]-[I-143]) |
| `references/vendorable/kineros/703.md` | targeted (channel continuity, sediment mass balance §, references) | ARS-77 same-grid + Bennett restatement |
| `references/vendorable/HEC_RAS_1D_Sediment_Transport_UserManual_20260710.pdf` | targeted (quasi-unsteady passages via text dump) | Quasi-steady-sequence class authority |
| `references/50201000/chap14.pdf` | first page + §14.1-14.2 (via prior read + agent report) | Internal per-time-step sediment precedent |
| `docs/decisions/0036-hydrograph-resolved-sediment-transport-and-routing.md` | full | D1-D5 decisions this extends |
| Pinned baseline `chnrt/chnero/dcap/detach/chncon/cgully.inc/wshdrv/wshchr/wshinp` | delegated forensic subagent read with file:line citations (recorded in `authority-matrix.md` evidence-mode block) | Legacy state-carry semantics |
| `wepp-forest/docs/jimf-wepp-2023-diff-audit.md` | delegated sweep (r1305 fix rows) | Maintainer-intent carry evidence |

## Budget disposition

Full reads were spent on the four decision-load-bearing documents (Chapter 13,
CREAMS Ch. 3 conversion, SC-ROUTE-001, authoring procedure). The two large
companion contracts (SC-SED-001, SC-SYSTEM-001) were read by targeted grep +
row reads because this package amends neither's producer semantics (recorded
in `contract-disposition.md`); the 275 KB work-packages README was read only at
its queue rows. Baseline Fortran forensics were delegated to a bounded
subagent whose report carries file:line citations; its load-bearing claims
about `/gully/` state carry were cross-checked against Chapter 13 §13.5.1 text
read directly.
