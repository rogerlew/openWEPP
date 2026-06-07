# FROSTVAL01 Follow-On Queue (corrected)

Status: queue (proposed)
Author: Claude Code, 2026-06-07

This is the corrected, ordered follow-on queue from FROSTVAL01, incorporating the
Claude review (`review_claude_frostval01_ledger_and_activation.md`). It supersedes
the queue in `rung3-frost-defect-handoff.md`, which (a) carried the invalid
`frost-break` verdict as if real, (b) missed that the closure ledger is broken, and
(c) missed the openWEPP zero-term anomalies (`Ep`/`Q`/`Er`/`Interception` = 0).

Verified facts the queue rests on (Claude, `duckdb`/`awk` on
`/tmp/frostval01/full/out/`):

- openWEPP precip output is correct and complete: WAT `P` = 911 mm/yr (6379.6 mm
  full climate, P>0 on 1149 days). The ledger's ~10 mm "inputs" is a ledger bug,
  not a precip defect.
- openWEPP's own term sums show real anomalies on the 6 runnable hillslopes:
  `Q`=0, `Ep`=0, `Er`=0, `Interception`=0 (all ET is soil evaporation, `Es`=698).
- 37/43 single-OFE hillslopes are blocked by `HS-RUNTIME-E-062` before hydrology.
- Frost telemetry on the 6 runnable: `frozwt`=0 and ksflag on/off deltas=0, with
  real water at a freezing site (PRESTON MN) → frost likely not activating.

## Queue

### FQ-1 — HS-RUNTIME-E-062 soil-coverage closure  (DC-ExecPlan, openWEPP) — TOP PRIORITY

- Defect: 37/43 algebraic-radium single-OFE hillslopes fail closed with
  `HS-RUNTIME-E-062` / `CLIHILL-E-011` ("runtime surface failure for soil";
  blocked taxonomy cites soil layer 4 / layer 6 coverage gaps) before any
  hydrology/frost output.
- Why first: only 6/43 run; the frost rung cannot be validated at population scale
  until this is unblocked.
- Acceptance: all 43 single-OFE hillslopes produce `H.wat.parquet` + `H.hbp`
  (no soil-runtime fail-closed on valid inputs), or any residual failure is
  reclassified as genuinely invalid input with typed evidence.
- Authority: SC-SOIL-001 + the soil parser/runtime; fixture
  `/wc1/runs/al/algebraic-radium/wepp/runs/`.

### FQ-2 — Frost-closure ledger fix  (validation-tool fix; fold into a FROSTVAL01 re-run, not a DC-ExecPlan)

- Defect: the FROSTVAL01 closure ledger sums flux terms over a wrong ~1%-of-days
  set (inputs ≈ 9 mm/yr, outputs ≈ 8 mm/yr vs openWEPP's actual P=911, Es=698,
  Dp=89, latqcc=93), while ΔStorage is full-scale — so residuals are meaningless
  and `frost-break` is an artifact.
- Action: fix the ledger to consume the full WAT term sums (the rung-1 complete
  identity incl. `Interception`, `frozwt`), re-derive closure on the runnable
  cohort. **Withdraw the `frost-break` classification.**
- Independent of FQ-1; can proceed immediately.

### FQ-3 — ET-partition / runoff zero-term characterization  (diagnostic-first → DC-ExecPlan if confirmed, openWEPP)

- Observable: on the 6 runnable hillslopes openWEPP publishes `Ep`=0 (zero
  transpiration on a Corn site, 7 yr), `Q`=0/`QOFE`=0 (zero runoff), `Er`=0,
  `Interception`=0; all ET is soil evaporation.
- First step (symptom-existence / ownership): run `wepp_260606` on the same inputs
  as the activation/behavior flag — does legacy produce nonzero `Ep`/`Q`/`Er`?
  Determine whether these are openWEPP defects (PMET/plant-transpiration and
  runoff paths not engaging on this gridmet/cropland/PMET config) or config
  artifacts. `Ep`=0 echoes the HPHYS WB17 `Ep` lineage.
- Why before FQ-4: a non-engaging ET/runoff path would confound any frost
  assessment (frost interacts with infiltration/runoff). Likely the more
  fundamental openWEPP gap this rung-2 substrate exposed that rung-1's did not.
- If confirmed defects → contract-first frost-independent DC-ExecPlan(s).

### FQ-4 — Frost activation closure  (DC-ExecPlan, openWEPP — the rung-2 target)

- Defect (candidate): standard-WEPP `ksflag` frost does not activate in openWEPP —
  `frozwt`=0 and ksflag on/off identical, with real water at a freezing site.
- Pre-conditions: a runnable population (FQ-1) and a clean ET/runoff substrate
  (FQ-3), so frost can be assessed in isolation.
- Confirm before closing: (a) activation telemetry is not itself a tool bug
  (`frozwt`/frost-depth actually read), (b) `wepp_260606` produces `frozwt`>0 on
  the cohort (frost *should* fire). If confirmed, close "ksflag frost does not
  activate" contract-first.
- Authority: legacy `infile.for`/`infpar.for`/`winter.for` frost lineage;
  SC-SNOWFREEZE-001 / frost contract surfaces; comparator as flag (ADR-0017).

## Ordering and dependencies

- FQ-1 (unblock 37/43) and FQ-2 (ledger fix) are prerequisites and can run in
  parallel; both are needed before any meaningful frost re-validation.
- FQ-3 (ET/runoff zero-terms) should be characterized before FQ-4 (frost), since a
  non-engaging ET/runoff path confounds frost assessment — and may be the larger
  openWEPP gap this rung-2 substrate exposed.
- FQ-4 (frost activation) is the actual rung-2 frost target, on the clean substrate
  FQ-1+FQ-3 produce.
- Protected boundaries carry over: snow magnitude → Stage-2 backlog; MOFE (the
  17-OFE hillslope) → rung-3; forest `ksatadj` path is a separate concern from the
  standard `ksflag` frost gate validated here.
