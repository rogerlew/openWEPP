# Wave 4 Kickoff Acceptance Criteria (HOLD Decision Closure)

Status: Draft (ARCH13)
Evidence: Static
Ran evidence: none

## Global Kickoff Gate

Wave 4 kickoff is authorized only when all criteria below are satisfied:

1. `W4DR-001`..`W4DR-012` status is `ratified`.
2. Each ratified decision has:
- selected option,
- decider,
- decision date,
- evidence citation set,
- downstream action list.
3. Each linked HOLD gap in the referenced `SC-INFILE-*` contracts is updated from
   `HOLD` to a closed/dispositioned state with traceable decision linkage.
4. No high-severity ambiguity remains on strict-vs-compat behavior or ownership
   boundaries for the listed surfaces.
5. A follow-on implementation queue exists for all required code/spec/test
   changes implied by ratified decisions.

If any criterion fails, kickoff status is `HOLD`.

## Per-Decision Acceptance Criteria

| decision_id | acceptance criteria |
| --- | --- |
| `W4DR-001` | Source-authority policy explicitly ratified and applied consistently across `TC`, `TCR`, `GWCOEFF`, `PHOSPHORUS`, `LCWB` contracts. |
| `W4DR-002` | Strict/compat open-error policy is unified in wording and branch behavior across `TC`, `TCR`, `GWCOEFF`, `LCWB` with explicit error/warning IDs. |
| `W4DR-003` | Ownership matrix clearly separates parser-input surfaces from derived/output-control semantics, with no overlap ambiguity. |
| `W4DR-004` | `ichout` accepted domain and compatibility behavior are ratified and reflected in guards/tests plan. |
| `W4DR-005` | `dtchr` normalization and error-path semantics are fixture-backed and deterministically documented. |
| `W4DR-006` | `cbase` semantics/units and mandatory guard policy are ratified with downstream consumer alignment. |
| `W4DR-007` | `gwcoeff` missing/default and present-malformed behavior is explicitly ratified with strict/compat separation. |
| `W4DR-008` | Namespace-separation policy between `gwcoeff` and `chan.inp` similarly named coefficients is ratified with alias guard strategy. |
| `W4DR-009` | `phosphorus` range policy and applicability scope are ratified and mapped to runtime/contract guards. |
| `W4DR-010` | `tcr` bounds/default policy and producer interoperability handling (including blank/newline edge case) are ratified. |
| `W4DR-011` | Active authoritative consumer for `lcwbflg` semantics is ratified and historical ambiguity resolved. |
| `W4DR-012` | `tc_out.txt` row grammar ownership boundary is ratified between parser and output contracts. |

## Severity Rule

- Any unresolved decision among `W4DR-001`..`W4DR-012` is treated as
  kickoff-blocking (`HOLD`) for affected surfaces.
- Deferred decisions require explicit risk acceptance and scoped exclusion from
  kickoff scope; otherwise kickoff remains `HOLD`.
