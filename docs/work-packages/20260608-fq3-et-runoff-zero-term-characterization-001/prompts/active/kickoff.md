# FQ3 Kickoff — ET / runoff zero-term characterization

Execution mode: package-end-to-end

Autonomy: execute end-to-end for the declared scope — run the 42 post-FQ1
hillslopes + the `wepp_260606` comparator, group by management, build the
per-term characterization ledger, classify defect-vs-config, route defect-shaped
follow-ons, dual review/verification, disposition — without asking for direction on
intermediate steps. Ask only if hard-blocked.

## Item 1 — characterize the zero-terms; do NOT fix

On `/wc1/runs/al/algebraic-radium` (post-FQ1, 42/43 runnable; `p11` excluded),
characterize the ET-partition / interception / runoff zero-terms and determine
which are real openWEPP defects vs expected/config, using `wepp_260606_hill` as the
flag. This is characterization only — emit defect-shaped DC-ExecPlan follow-ons;
make no production edits.

## Grounded starting facts (verify, then build on)

- The zero-terms are **management-specific**: Corn (annual crop) hillslopes give
  `Ep`=0, `Interception`=0 (all ET as soil evaporation `Es`); perennial/forest
  covers transpire+intercept (p1 `Tah_4899`: Ep=5511, Interception=643). `Q`=0 is
  universal (incl. p1). Verified via `duckdb` on `/tmp/fq1_after/outputs/*`.
- Leading hypothesis: openWEPP does not drive the annual-crop (Corn) plant-growth →
  canopy → transpiration/interception path; `Q`=0 is a separate universal thread.

## Method

1. Group the 42 by management/cover (Corn vs Tah_4899 vs bromegrass vs other).
2. Per group × term (`Ep`,`Es`,`Er`,`Interception`,`Q`/`QOFE`): openWEPP vs legacy
   full-run sums.
3. Classify each term×group defect vs config. **No "defect" label unless the
   comparator shows the term should be materially nonzero** (e.g., legacy corn
   transpires but openWEPP `Ep`=0 → defect; both ~0 → config).
4. Resolve the annual-crop ET/interception hypothesis and the `Q`=0 question.
5. Common-cause/grouping; emit defect-shaped DC-ExecPlan follow-on(s) (one per
   authority envelope, ADR-0018 grouping — likely a crop-ET/interception engagement
   defect, and separately a runoff `Q` question).

## Acceptance authority + constraints

- `wepp_260606` is a flag for the defect-vs-config question (ADR-0017), not an
  acceptance oracle; fail closed on missing terms; truthful evidence mode.
- No production edits; snow magnitude → Stage-2; MOFE/17-OFE deferred; `p11`
  percolation is `FQ1-P11`'s, not this package's.

## Required reading

- `docs/work-packages/20260608-fq3-et-runoff-zero-term-characterization-001/package.md`
- FROSTVAL01 `artifacts/frostval01-followon-queue.md` + my FROSTVAL01 review
  (the F2b zero-term table) + FQ-1 review.
- `docs/decisions/0011/0017`, `docs/defect_closure_execplans.md`, `AGENTS.md`
- `docs/specifications/science-contracts/contracts/SC-EVAP-001.md`,
  `SC-RUNOFFPART-001.md` (follow-on authority surfaces)
- Comparator: `/home/workdir/wepppy/wepp_runner/bin/wepp_260606_hill`
- Precedent: WBVAL01 / FROSTVAL01 validation/characterization shape;
  `tests/integration/cli01_runner_hillslope_integration.rs`.
