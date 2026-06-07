# FQ3 ET / Runoff Zero-Term Characterization

Status: executed

Package type: validation/characterization work package

## Objective

Characterize openWEPP's ET-partition, interception, and runoff zero-terms on the
post-FQ1 `/wc1/runs/al/algebraic-radium` population, determine which are real
openWEPP defects versus expected/config behavior using `wepp_260606` as the flag,
and route defect-shaped DC-ExecPlan follow-ons. This is a diagnostic-first
characterization — it measures, classifies, and routes; it makes **no production
edits**.

## Run Under Validation

`/wc1/runs/al/algebraic-radium`, post-FQ1 (HS-RUNTIME-E-062 closed): 42/43
single-OFE hillslopes now emit WAT (`p11` excluded — owned by `FQ1-P11`
percolation). Comparator `/home/workdir/wepppy/wepp_runner/bin/wepp_260606_hill`
on the same inputs.

## Rationale

FROSTVAL01 surfaced openWEPP output anomalies (`Ep`/`Q`/`Er`/`Interception` = 0)
that FQ-1 then made assessable at population scale. Grounding the actual post-FQ1
WAT term sums (Claude, `duckdb`) shows these are **management-specific, not
universal**:

- **Corn (annual crop) hillslopes → `Ep`=0, `Interception`=0**; all ET is soil
  evaporation (`Es` ~ 4100–4800 mm over 7 yr). Verified on `p8,p10,p12..p19`.
- **Perennial/forest covers transpire and intercept normally** — `p1`
  (`Tah_4899`): `Ep`=5511 mm, `Interception`=643 mm, `Es`=20 mm.
- **`Q`=0 is universal** (every checked prefix, including transpiring `p1`).

Leading hypothesis (to confirm, not assume): openWEPP does not drive the
**annual-crop (Corn) plant-growth → canopy → transpiration/interception** path, so
Corn hillslopes (the majority — ~36/44) evaporate everything as soil evaporation
with zero plant uptake and zero canopy interception; perennial covers work. `Q`=0
is a separate, universal thread (runoff path not engaging, or all-infiltrating
high-Ksat soils — the comparator decides).

This matters for the frost rung: a non-engaging crop ET/interception path (and a
zero-runoff path) would confound any frost-gate assessment (FQ-4), so it is
characterized first.

## Included Scope

- Run the 42 post-FQ1 single-OFE hillslopes (record build hash) and the
  `wepp_260606_hill` comparator on the same inputs.
- Group hillslopes by management/cover (Corn vs `Tah_4899` vs bromegrass vs other).
- Per group × per term (`Ep`, `Es`, `Er`, `Interception`, `Q`/`QOFE`): tabulate
  openWEPP vs legacy full-run sums.
- Classify each term×group: **defect** (openWEPP = 0 where legacy is materially
  nonzero) vs **expected/config** (both ~0 / legitimately so).
- Confirm or refute the annual-crop hypothesis: does `wepp_260606` produce
  `Ep`/`Interception` for the Corn hillslopes (it should — corn transpires heavily)
  while openWEPP gives 0?
- Determine `Q`=0: does legacy produce runoff where openWEPP gives 0, or is it
  all-infiltrating on both?
- Common-cause/grouping analysis: is the Corn `Ep`+`Interception`=0 one root cause
  (the plant-growth/canopy driver not engaging)? Is `Q`=0 separate?
- Emit defect-shaped DC-ExecPlan follow-ons per confirmed root cause.

## Excluded Scope / Protected Boundaries

- No production Rust edits — characterization only; corrections are defect-shaped
  follow-on DC-ExecPlans (one per confirmed authority envelope per ADR-0018
  grouping).
- No comparator-match acceptance; `wepp_260606` is a **flag** for the
  defect-vs-config question (ADR-0017), not an acceptance oracle.
- Snow magnitude remains a Stage-2 protected boundary.
- The 17-OFE hillslope (MOFE) is observe-only (rung-3).
- `p11` percolation is owned by `FQ1-P11-HKERNEL-WB11-PERC-E-003-J162`, not this
  package.
- Do not "fix" any zero-term here; classify and route.

## Deliverables

- `artifacts/zero-term-characterization-ledger.md` — per management group × term,
  openWEPP vs `wepp_260606` sums, defect/config classification, and the
  annual-crop and `Q`=0 verdicts.
- `artifacts/run-manifest.md` — 42-prefix inventory, management grouping, exact
  invocations, binary hashes (evidence mode **Ran**).
- `artifacts/fq3-defect-handoff.md` — defect-shaped DC-ExecPlan follow-ons for the
  confirmed root cause(s), with authority envelopes.
- Standard gate, dual review, verification, disposition, worker-handoff artifacts.

## Acceptance / Exit Criteria

- A per-management × per-term ledger over the 42 hillslopes with the `wepp_260606`
  flag, with terms classified defect vs expected/config from evidence (no defect
  label without the comparator showing the term should be nonzero).
- The annual-crop ET/interception hypothesis and the `Q`=0 question each resolved
  to defect or config with evidence.
- Common-cause analysis and defect-shaped DC-ExecPlan handoff(s).
- Truthful evidence mode per artifact (**Ran** for actual runs; **Static** for
  reasoned classification).
- A defect population is the expected, valid output — this package routes it; it
  does not fix it. Package failure is an incomplete/untruthful ledger or a defect
  label asserted without the comparator flag.

## Dependencies

- `/workdir/openWEPP/AGENTS.md`, `docs/codex_exec_plans.md`,
  `docs/defect_closure_execplans.md`
- `docs/work-packages/README.md` (roadmap, rung-2 / FQ queue)
- `docs/decisions/0011-architecture-first-top-down-science-contracts.md`,
  `0017-re-pin-operational-distrust-comparator-is-flag-not-target.md`
- `docs/backlog/20260605-snow-code-deferred-science-review.md` (snow boundary)
- FROSTVAL01 + `artifacts/frostval01-followon-queue.md`; FQ-1 package (substrate
  now 42/43)
- Relevant contracts for follow-on authority: `SC-EVAP-001` (Ep/Es/Er),
  interception/canopy surfaces, `SC-RUNOFFPART-001` (Q)
- Comparator: `/home/workdir/wepppy/wepp_runner/bin/wepp_260606_hill`
- Run inputs: `/wc1/runs/al/algebraic-radium/wepp/runs/`
