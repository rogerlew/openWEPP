# POST-BASECOND01-H2637-MAGNITUDE-DISPOSITION Kickoff — Close the FARPOINT01 71% Arc

Execution mode: Stage-2 magnitude DISPOSITION — synthesis + flag resolution, **no fix, no new
adjudication, no re-opening ruled-out terms.**

Autonomy: execute end-to-end (synthesis → verdict → FARPOINT01 flag resolution → backlog note).
The investigation is done; formalize the verdict.

## Why you're here

The FARPOINT01 H2637 **71%** lateral-magnitude flag has been adjudicated across a full arc. Every
internal candidate is verified correct or ruled out as the driver. This package renders the final
disposition and closes the flag.

## The verified chain (synthesize, cite each — do NOT re-open)

| Term | Package | Result |
|---|---|---|
| transfer / area / export / conservation | MAGPARITY01 | machine-epsilon correct |
| WB19 equation + operands | STAGE2-LATQCC | machine-precision correct; operands bound-valid |
| `ksatadj` `sat_frac` | REFINTENT001 | defect fixed, but `ksatadj = 0` for H2637 → byte-inert; not the driver |
| vertical `ssc` normalization | BASECOND01 | defect fixed (arithmetic→harmonic), but H2637 aggregate-inert; not the driver |
| **lateral** lineage `ksat`→`ui_ssh`(arith)→`Ke`→`latqcc` | base-cond | **verified correct end-to-end** vs source intent + contracts |
| sensitivity | base-cond | raw `ksat` byte-live → the lateral conductivity IS the (correct) driver |

## The verdict

- **`CORRECT-BY-CONSTRUCTION`** — the 71% is the output of a fully-verified lateral lineage; **not a
  defect**; the correct forest-hydrology magnitude for this wet (~2825 mm/yr) forest hillslope.
- **Absolute magnitude = `CONTRACT-GAP`** — no external authority (field benchmark / validated forest
  model) exists to validate the absolute number. A **documented gap, not a blocker**; internal
  adjudication has exhausted what it can resolve.

## FARPOINT01 flag resolution

- Resolve the FARPOINT01 71% magnitude flag as **`NO DEFECT` / expected** (correct-by-construction).
  The flag's question ("defect or expected?") → **expected**, by a verified lineage. Record the
  closure.
- Author a **`docs/backlog/` note** for the optional absolute-magnitude external-authority suite
  (forest lateral-flow magnitude) — pursued only if external data becomes obtainable, **not a
  blocker, not started here.**

## Guardrails

- Do **not** re-open transfer / equation / operands / `ksatadj` / vertical `ssc` without new
  evidence contradicting their machine-precision/source-verified results.
- Do **not** re-edit vertical `ssc` or make `ui_ssh` harmonic to chase the 55.5% comparator.
- No legacy parity target; no fix; no new `SC-*` change.

## Constraints / truthfulness

- A confirmation re-run of the closed H2637 totals is sufficient evidence; no new physics. Label
  evidence Static / Ran. The verdict cites the per-term packages + the ADR-0017 taxonomy, not the
  legacy delta.

## Required reading

- `docs/work-packages/20260618-post-basecond01-h2637-magnitude-disposition-001/package.md`
- `docs/work-packages/20260613-mofe-farpoint01-high-ofe-routing-closure-demonstration-001/artifacts/fc-legacy-closure-contrast.md`
- `docs/work-packages/20260618-mofe-magparity01-per-ofe-runoff-magnitude-adjudication-001/artifacts/{magparity01-runoff-decomposition,magparity01-per-term-verdict}.md`
- `docs/work-packages/20260618-stage2-latqcc-h2637-magnitude-001/artifacts/{latqcc_disposition,latqcc-equation-correctness}.md`
- `docs/work-packages/20260618-refintent001-ksatadj-satfrac-defect-closure-001/artifacts/review-claude-independent.md`
- `docs/work-packages/20260618-stage2-base-conductivity-h2637-magnitude-001/artifacts/{base-cond-per-step-verdict,review-claude-independent}.md`
- `docs/work-packages/20260618-basecond01-ssc-harmonic-normalization-defect-closure-001/artifacts/basecond01_disposition.md`
- `docs/decisions/0017-...comparator-is-flag-not-target.md`, `0024-...intent-authority.md`
- `docs/specifications/correctness-authority-model.md`, `AGENTS.md`
