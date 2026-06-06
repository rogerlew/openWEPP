# Worker Handoff

Status: complete

Evidence mode: static+ran

Current handoff: none. Execute package phases before creating follow-ons.

If package closes `HOLD`, the handoff must name:

- Defect or boundary ID.
- Observable failure and failing fixture.
- Suspected mechanism.
- In-scope write set for the owning follow-on.
- Correction authority.
- Acceptance target.
- Legitimate `HOLD` conditions.

Forbidden relay: no handoff may name only a next diagnostic step.

Static:

First actionable item:

1. Close defect `WBVAL05-J95-HKERNEL-WB11-PERC-E-003` by opening/executing a
   snow/runoff boundary closure for `HKERNEL-WB14-RUNOFF-E-003` on J-95
   `snow.runtime_swe=-0.006171157610042402` for p7/p11/p18/p20.

Context:

- WB18 percolation no longer owns the first failure. It now consumes published
  `wb12_infiltration` before optional WB14/WB12 recomputation.
- The remaining blocker is upstream negative runtime SWE. Temporary local
  attribution showed the exact value on p7; final validation confirms all four
  targets now fail at WB14 runoff.
- Evidence roots:
  `/tmp/wbval05_j95_perc_20260606T000000Z/final_status.tsv` and package
  artifacts in this directory.

Ran:

- See `wbval05-validation-ledger.md` for final commands and status.
