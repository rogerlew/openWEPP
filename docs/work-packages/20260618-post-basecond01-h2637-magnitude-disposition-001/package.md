# POST-BASECOND01-H2637-MAGNITUDE-DISPOSITION - Final FARPOINT01 71% Disposition

Status: complete 2026-06-18 (Stage-2 magnitude disposition; the capstone that closes the
FARPOINT01 71% lateral-magnitude arc)

Package type: **Stage-2 magnitude DISPOSITION — synthesis + flag resolution, no fix, no new
adjudication.** The investigation is done; this package formalizes the verdict and resolves the
FARPOINT01 flag. Per ADR-0017 / the closure-not-magnitude ordering, legacy stays a flag.

## Why this package exists

FARPOINT01 (2026-06-16) surfaced that H2637 routes **71.0% of precip** to the outlet vs legacy
without_ui 55.5%. A multi-package arc adjudicated every internal candidate; this package ties them
together and renders the final disposition.

## Execution result

The disposition is complete. FARPOINT01's H2637 71% magnitude flag is resolved as
`CORRECT-BY-CONSTRUCTION` / `NO DEFECT` for the verified openWEPP lateral
lineage. The remaining absolute physical magnitude question is a documented
external-authority `CONTRACT-GAP`, recorded in
`docs/backlog/20260618-forest-lateral-flow-absolute-magnitude-authority.md`.
No production code, science contract, or new defect-closure package is
authorized by this disposition.

## The verified evidence chain (synthesize — do not re-open)

Every internal step of the H2637 lateral magnitude has been verified correct or ruled out as the
driver:

| Term | Package | Result |
|---|---|---|
| Inter-OFE transfer / area / export / conservation | MAGPARITY01 | Correct at **machine epsilon** (`INV-RUNOFFPART-028`, duality, closure). |
| WB19 lateral-flow **equation + operands** | STAGE2-LATQCC | `latqcc` = Eq [6.2.4]/Dun at **machine precision**; operands bound-valid. |
| `ksatadj` saturation fraction | REFINTENT001 | Algorithm defect **fixed** (source-intent `sat_frac`) — but **`ksatadj = 0` for H2637**, byte-inert; not the driver. |
| Vertical `ssc` 200 mm normalization | BASECOND01 | Real defect **fixed** (arithmetic → harmonic) — but **H2637 aggregate-inert** (percolation layer-limited deeper); not the driver. |
| **Lateral** conductivity lineage (`ksat` → `ui_ssh` arithmetic → `Ke` → `latqcc`) | base-cond | **Verified correct end-to-end** against source intent + `SC-INFILE-SOIL-001`/`SC-SUBHYD-001`. |
| Sensitivity | base-cond | Raw `ksat` is **byte-live** — the lateral conductivity *is* the driver, and it is correct. |

**Conclusion of the chain:** the H2637 71% is the output of a **fully-verified** lateral
conductivity → lateral-flow lineage. No internal defect remains.

## The verdict to render

- **The 71% is `CORRECT-BY-CONSTRUCTION`** — every step (transfer, conservation, equation,
  operands, conductivity lineage) is verified against source intent + canonical contracts. It is
  **not a defect**; it is the correct forest-hydrology magnitude openWEPP's verified physics
  produces on this wet (~2825 mm/yr) forest hillslope.
- **The only open item is the ABSOLUTE magnitude** — whether 71% is the physically-correct runoff
  fraction for H2637's site. This is a **`CONTRACT-GAP`**: no external authority (field benchmark,
  validated forest model) exists to validate the absolute number. It is a **documented gap, not a
  blocker** — internal adjudication has exhausted what it can resolve.

## FARPOINT01 flag resolution

- **Resolve the FARPOINT01 71% magnitude flag as `NO DEFECT` / expected (correct-by-construction).**
  The flag asked "defect or expected Stage-2 divergence?" — the answer is **expected**, by a
  verified lineage. Legacy 55.5% remains an A6 flag, never a target (and legacy with_ui's 127.7% is
  non-conserving / disqualified).
- **The absolute-magnitude validation becomes an optional `docs/backlog/` item** (an external-authority
  suite for forest lateral-flow magnitude), pursued only if/when external data is obtainable —
  **not** a queue blocker. Author the backlog note; do not start the suite here.

## Guardrails (hold)

- **Do not re-open** the ruled-out terms (transfer, equation, operands, `ksatadj`, vertical `ssc`)
  without new evidence contradicting their machine-precision/source-verified results.
- **Do not re-edit** vertical `ssc` or make `ui_ssh` harmonic to chase the 55.5% comparator
  (the BASECOND01 guardrail).
- **No legacy parity target;** no fix; no new `SC-*` change.

## Scope

In scope: the evidence synthesis; the correct-by-construction / `CONTRACT-GAP` verdict; the
FARPOINT01 flag resolution; the optional absolute-magnitude backlog note.

Out of scope: no fix; no new adjudication; no re-opening ruled-out terms; no `SC-*` change; no
external-authority suite (backlog only). Irrigation deferred.

## Acceptance Criteria

- **Synthesis** of the verified chain (the table above), each row citing its package + result.
- **Verdict:** `CORRECT-BY-CONSTRUCTION` for the lineage + `CONTRACT-GAP` for the absolute magnitude,
  with the ADR-0017 taxonomy and no legacy-parity framing.
- **FARPOINT01 flag resolved** as no-defect / expected, recorded so the magnitude flag is closed.
- **Backlog note** for the optional absolute-magnitude external-authority suite (not started).
- Evidence labeled Static / Ran (a confirmation re-run of the closed H2637 totals is sufficient;
  no new physics). Markdown lint clean.

## Deliverables

- `artifacts/farpoint01-evidence-synthesis.md` (the verified chain, per-term citations)
- `artifacts/farpoint01-final-verdict.md` (correct-by-construction + CONTRACT-GAP)
- `artifacts/farpoint01-flag-resolution.md` (the FARPOINT01 magnitude flag closure)
- `docs/backlog/<date>-forest-lateral-flow-absolute-magnitude-authority.md` (the optional suite note)
- `artifacts/post-basecond01_disposition.md`

## Dependencies

- `docs/work-packages/20260613-mofe-farpoint01-high-ofe-routing-closure-demonstration-001/artifacts/fc-legacy-closure-contrast.md` (the original 71% finding)
- `docs/work-packages/20260618-mofe-magparity01-per-ofe-runoff-magnitude-adjudication-001/` (transfer/closure/export)
- `docs/work-packages/20260618-stage2-latqcc-h2637-magnitude-001/` (equation/operands; the prior CONTRACT-GAP)
- `docs/work-packages/20260618-refintent001-ksatadj-satfrac-defect-closure-001/artifacts/review-claude-independent.md` (ksatadj off / byte-inert)
- `docs/work-packages/20260618-stage2-base-conductivity-h2637-magnitude-001/artifacts/{base-cond-per-step-verdict,review-claude-independent}.md` (lateral lineage verified; vertical ssc defect)
- `docs/work-packages/20260618-basecond01-ssc-harmonic-normalization-defect-closure-001/artifacts/basecond01_disposition.md` (vertical ssc fixed, aggregate-inert)
- `docs/decisions/0017-...comparator-is-flag-not-target.md`, `0018-...`, `0024-...intent-authority.md`
- `docs/specifications/correctness-authority-model.md`; `docs/ROADMAP.md`; `AGENTS.md`

## Subagent Requirement

None required. This is a synthesis/disposition; run any confirmation totals locally.

## Autonomy

Execute end-to-end through the synthesis, the verdict, the FARPOINT01 flag resolution, and the
backlog note. **No fix, no re-opening ruled-out terms.** The deliverable closes the FARPOINT01 71%
magnitude arc as `CORRECT-BY-CONSTRUCTION` with the absolute magnitude a documented external-authority
gap — the honest end of a thorough investigation that produced two real defect fixes (ksatadj,
vertical `ssc`) and ADR-0024 along the way.
