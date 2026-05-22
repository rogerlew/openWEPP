# Gate Results — ARCH13

Evidence: Ran + Static
Date: 2026-05-22 UTC
Ran: shell command outputs captured during ARCH13 execution.
Static: scope applicability judgment for Rust workspace gates.

## Required Checks

| gate | command | result | notes |
| --- | --- | --- | --- |
| 1 | `rg -o 'W4DR-[0-9]{3}' wave4-hold-ratification-checklist.md \| sort -u \| wc -l` | pass | [RAN] Decision record coverage is `12`. |
| 2 | `rg -o 'W4DR-[0-9]{3}' wave4-kickoff-acceptance-criteria.md \| sort -u \| wc -l` | pass | [RAN] Per-decision criteria coverage is `12`. |
| 3 | `rg -o '(CHANINP\|TC\|TCR\|GWCOEFF\|PHOS\|LCWB)-GAP-[0-9]{3}' checklist -> rg contracts` | pass | [RAN] All referenced HOLD gap IDs were found in `docs/specifications/science-contracts/contracts`. |
| 4 | `rg -n 'cbase\|dtchr\|ichout\|lcwbflg' wave4-hold-ratification-checklist.md` | pass | [RAN] Canonical symbol continuity note and decision references are present. |
| 5 | `test -f` checks for all required ARCH13 artifact paths | pass | [RAN] Required artifact bundle is present in the package `artifacts/` directory. |
| 6 | Workspace Rust gates (`fmt`, `clippy`, `test`, `deny`) | n/a | [STATIC] ARCH13 is governance/docs-only and does not implement or modify Rust kernel code. |

## Command Evidence Excerpts

- [RAN] Checklist decision IDs: `W4DR-001`..`W4DR-012`; count `12`.
- [RAN] Criteria decision IDs: `W4DR-001`..`W4DR-012`; count `12`.
- [RAN] Gap traceability: `present` for all referenced `CHANINP/TC/TCR/GWCOEFF/PHOS/LCWB` HOLD IDs.

## Gate Verdict

- [INFERENCE] ARCH13 documentation integrity gate status is `PASS`.
- [DIRECT] Wave 4 kickoff authorization remains `HOLD` until decision statuses
  transition from `pending` to `ratified` with required evidence.
