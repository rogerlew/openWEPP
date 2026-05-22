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
| 5 | `rg -c '\| \`ratified\` \|' wave4-hold-ratification-checklist.md` | pass | [RAN] Ratified decision count is `12`. |
| 6 | `rg 'RATIFIED-W4DR' + hold-row regex checks on linked SC-INFILE contracts` | pass | [RAN] Ratified gap-row counts: `CHANINP=4`, `TC=3`, `TCR=5`, `GWCOEFF=4`, `PHOS=3`, `LCWB=4`; HOLD-row counts for linked gaps are `0` in all six contracts. |
| 7 | `test -f` checks for all required ARCH13 artifact paths | pass | [RAN] Required artifact bundle is present in the package `artifacts/` directory. |
| 8 | Workspace Rust gates (`fmt`, `clippy`, `test`, `deny`) | n/a | [STATIC] ARCH13 is governance/docs-only and does not implement or modify Rust kernel code. |

## Command Evidence Excerpts

- [RAN] Checklist decision IDs: `W4DR-001`..`W4DR-012`; count `12`.
- [RAN] Criteria decision IDs: `W4DR-001`..`W4DR-012`; count `12`.
- [RAN] Ratified status coverage: `12` decision rows with `status = ratified`.
- [RAN] Linked contract disposition update: all referenced HOLD gaps now `RATIFIED-W4DR-* (2026-05-22)`.

## Gate Verdict

- [INFERENCE] ARCH13 documentation integrity gate status is `PASS`.
- [DIRECT] Wave 4 kickoff authorization for the 12 decision surfaces is `GO`
  because all `W4DR-001`..`W4DR-012` are ratified with linked contract HOLD
  disposition updates.
