# Disposition

Status: HOLD

Evidence mode: ran

Static:

- HPHYS0309 is a diagnostic/state-lineage package.
- No production kernel edit was authorized or made.
- Downstream WB13/WB17/WB18/WB19/WB12 compensation remains prohibited.

Ran:

- Generated snow carry/depletion lineage ledger rows: `58`.
- Route counts:
  - `pre-day-carry-deficit-hold`: `45`
  - `prior-day-openwepp-meltout-hold`: `13`
- Production edit authorized rows: `0`.

## Rationale

Every HPHYS0308 baseline-extra melt-call key is explained by prior carry state:
openWEPP starts the key day with materially less carried snow than the fixed
comparator, or starts the key day snow-free while the fixed comparator still
carries snow. This keeps the branch-predicate and same-hour melt-term lanes in
`HOLD`; the next package should inspect prior-day/day-start snowpack carry-state
lineage before any producer or downstream water-balance edit.
