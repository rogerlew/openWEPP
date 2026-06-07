# FDHP01 — Frost Depth Heat-Flow Parity (single-OFE)

Status: scaffolded

Package type: Defect-Closure ExecPlan (DC-ExecPlan)

## Objective

Close defect `FDHP01-FROST-DEPTH-HEATFLOW-001`: openWEPP computes frost **depth** with a
freeze-index proxy (`frdp = 0.20·clamp(−mean_temp/6)`, hard-capped 0.20 m) instead of the
energy-balance **heat-flow** model the contract already mandates
(`SC-SNOWFREEZE-001#INV-SNOWFREEZE-006`/`-012`; legacy `frostn` lineage; CRM Ch. 3.8
Eq. [3.8.1]–[3.8.4]; Dun et al. 2010 ASABE 53(5):1399–1411). Replace the proxy with the
heat-flow depth model on the frost-active single-OFE substrate so frost depth and
frozen-soil duration track the heat-flow authority, and **close `GAP-SNOWFREEZE-002`** —
with the rung-1 + frost conservation closure still holding.

This package owns correction inside the frost depth-model envelope. The contract authority
(`INV-SNOWFREEZE-006`/`-012`, hard-fail) **already exists**; this is "make the
implementation match the contract," so if the root cause is in-envelope it must land.

## Why now (the re-sequence) and why single-OFE

Re-sequenced ahead of MOFE (2026-06-07, Roger): frost is a per-column **vertical**
mechanism, and the ladder settles vertical mechanisms on single-OFE **before** routing so
their error is not aliased into routing error. FQ-4 settled frost *activation* that way;
this settles the frost *depth model* the same way, before MOFE (rung-3). Building MOFE on
the proxy and fixing depth later would force a re-validation of MOFE under frost; doing it
now means MOFE is built once on a faithful frost foundation, and the heat-flow physics is
debugged in isolation — one column, no routing to confound it.

The gap is sized, not assumed:
[FDMC01](../20260608-fdmc01-frost-depth-comparator-characterization-001/) verdict
**materially off** — openWEPP depth capped 200 mm vs legacy 240–503 mm (43/43 exceed the
cap), depth-series median correlation **0.13**, and frozen-water duration **+258 days**
(the proxy ratchets via `frdp = max(prior, …)`, thawing only when `tmin>0`, so it
over-persists). The duration error is the more consequential one: the conductivity bite is
near-total whenever frost exists, so the proxy holds the frozen runoff-generation window
open ~34% longer than legacy.

## Rationale / authority

- `SC-SNOWFREEZE-001` `INV-SNOWFREEZE-006` (frost heat-flow `Qsrf`/`Quf` + harmonic-mean
  layered conductivity, Eq. [3.8.1]–[3.8.4]) and `INV-SNOWFREEZE-012` (`winter → frostN →
  frzng → frznw` dispatch chain) are **hard-fail and already in the contract**;
  `GAP-SNOWFREEZE-002` records the implementation/parity as open. This DC implements to
  the existing authority and closes that gap.
- External authority: CRM Ch. 3.8; Dun et al. 2010 (improved WEPP frost subroutines,
  fine-layer discretization).
- Legacy `frostn.for`/`frzng.for`/`frznw.for`/`frsoil.for` are the **corroborating
  reference implementation** of that authority (ADR-0017: flag/reference, not acceptance
  target). The comparator (`wepp_260606_hill`) is a flag that depth/duration should match
  the heat-flow envelope, not a number to tune to.

## Correction Authority Envelope

### Defect IDs and Observed Violations

- `FDHP01-FROST-DEPTH-HEATFLOW-001`
  - Observable: `frdp_m` capped 0.20 m (proxy) vs legacy heat-flow 0.24–0.50 m; depth
    series correlation 0.13; frozen-duration +258 days. Evidence:
    `../20260608-fdmc01-frost-depth-comparator-characterization-001/artifacts/`
    (metrics CSV, summary JSON, ledger, verdict).

### In-Scope Contracts and Source Files

- Contracts:
  - `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
    (**primary** — implement to `INV-SNOWFREEZE-006`/`-012`, tighten with the
    implementation spec as needed, **close `GAP-SNOWFREEZE-002`**).
  - `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md` (downstream:
    closure must still hold with the new `frozwt` magnitude/timing).
- Production/test files:
  - `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_00_support_helpers.rs`
    (the freeze-index proxy `:3290`–`:3335`, `frdp_m`/`dfrost`/`ws_frz`, the `qsrf`/`quf`
    heat-flux + resistance block `:3455`–`:3490`, `kfactor` selection — the proxy is
    replaced by the heat-flow model here).
  - `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_01_kernel_phases.rs`
    (frost outcome → soil conductivity `:345`/`:3925`/`:4672`, harmonic-mean conductivity
    `:2112`).
  - `crates/openwepp-hillslope-orchestrator/src/constants.rs` (`WB14_FROST_MAX_DEPTH_M`,
    `FROST_RUNTIME_FREEZE_INDEX_SCALE_C`, fine-layer/conductivity constants — retire the
    proxy constants, add heat-flow constants with provenance).
  - `crates/openwepp-runner/src/hillslope/mod.rs` (frost publication — **publish `frdp`**
    to the WAT/output surface per the FDMC01 caveat so future comparisons read actual
    runtime state, not a reconstruction).
  - `tests/integration/**frost**.rs`, `**snowfreeze**.rs`, `**watbal**.rs`,
    `clim06_frost_frozen_soil_kernel_contract.rs`.
  - `docs/work-packages/20260608-fdhp01-frost-depth-heat-flow-parity-closure-001/**`,
    `docs/ROADMAP.md`, `docs/work-packages/README.md`,
    `docs/backlog/20260607-frost-depth-model-heat-flow-parity.md`.

### Allowed Edit Classes

- Amend `SC-SNOWFREEZE-001` to specify the heat-flow depth implementation and close
  `GAP-SNOWFREEZE-002` **before** production code.
- Implement the energy-balance layered heat-flow frost-depth model (fine-layer
  discretization per Dun 2008; depth permitted to the physical range, retiring the 0.20 m
  cap) in place of the freeze-index proxy.
- Publish `frdp` to the WAT/output surface.
- Add contract-derived tests (heat-flow depth on a known-cold profile; frozen-duration
  without the ratchet over-persistence; warm-day/non-freezing non-regression; activation
  non-regression).
- Add bounded diagnostics for the depth/energy-balance state.

### Protected Boundaries (do not cross)

- **No comparator-match tuning.** `wepp_260606_hill` is a flag that depth/duration should
  fall in the heat-flow envelope (ADR-0017); acceptance is contract-correct heat-flow
  behavior, not matching legacy `frdp` to the millimetre.
- **Do not regress frost activation** (FQ-4) — the `ksflag`/`wintRed` activation gate
  stays; this changes only how deep/how long, not whether frost engages.
- **kfactor conductivity magnitude is legacy-faithful — do not change it.** This DC fixes
  the depth/duration that *drives* the bite, not the bite coefficient.
- **Forest `ksatadj`** is a separate path — out of scope.
- **Conservation must close** — the new `frozwt` magnitude/timing must keep the rung-1 +
  frost closure identity (with `frozwt` in storage) closed; no balance break.
- Snow magnitude (Stage-2), ET/runoff (closed), `p11` percolation (FQ1-P11) — do not
  touch.
- **Single-OFE only.** MOFE / the 17-OFE hillslope is rung-3 (item 2) — out of scope.

### Acceptance Criteria

- Frost depth and frozen-soil duration on the `algebraic-radium` single-OFE cohort track
  the `INV-SNOWFREEZE-006`/`-012` heat-flow authority (comparator as flag): the FDMC01
  gap materially closes — depth reaches the physical range (cap retired), depth-series
  correlation rises substantially, and the frozen-duration over-persistence (+258 days /
  ratchet) is eliminated.
- `GAP-SNOWFREEZE-002` is closed (or its residual scope explicitly re-stated if a phase
  boundary is declared — see Branch-out).
- Water-balance closure (rung-1 identity incl. `frozwt`, + totalwatsed3 audit) still
  closes after the depth model changes.
- Frost activation (FQ-4) is non-regressed; `frdp` is published to the output surface.
- Contract-derived red/green tests; pre-implementation failing evidence; post-fix
  validation over the cohort + non-regression on warm/non-freezing days.
- No conservation break, comparator-target tuning, kfactor change, activation regression,
  or downstream compensation.

### Branch-out Boundaries

- The full legacy heat-flow chain (layered energy balance + hourly `Qsrf`/`Quf` + fine
  sublayers) is substantial. If Milestone 1 shows the complete port exceeds one package,
  a **phased boundary** is legitimate **only if** the landed phase still closes the FDMC01
  depth+duration gap (a faithful energy-balance depth model retiring the cap and the
  ratchet) — deferring, e.g., hourly `Qsrf`/`Quf` sub-family *publication* detail with the
  residual re-stated in `GAP-SNOWFREEZE-002`. A cosmetic tweak to the proxy is **not** a
  legitimate phase.

## Conversion Rule

Reproducible in-envelope root cause (the proxy depth model) + canonical authority
(`INV-SNOWFREEZE-006`/`-012` + CRM Ch. 3.8 / Dun 2010) ⇒ proceed through contract
amendment → tests → pre-implementation gate → production correction → validation →
disposition. May not close `HOLD` because the heat-flow port is large — scope it (phased
boundary) and land the gap-closing phase.

## Symptom-Existence + Ownership Gate (Milestone 1, first)

1. Reproduce the FDMC01 baseline on single-OFE (proxy depth capped 200 mm, duration
   over-persisting) vs legacy heat-flow.
2. **Scope the implementation**: confirm the heat-flow depth model extent to port (full
   `frostn`/`frzng`/`frznw` layered energy balance with Dun-2008 fine sublayers vs a
   faithful energy-balance subset that closes the gap), and localize the proxy-replacement
   seam in `03_kernel_support_00`. Declare a phased boundary here if warranted (per
   Branch-out).
3. Ownership: contract `INV-SNOWFREEZE-006`/`-012` already mandate heat-flow; the proxy is
   the openWEPP divergence (`GAP-SNOWFREEZE-002`). openWEPP defect, in-envelope.

## Legitimate HOLD Conditions

- Heat-flow authority genuinely under-specified in the contract for an implementation
  decision (amend the contract, do not stall).
- Required heat-flow inputs (e.g. an energy-balance forcing) unavailable on the substrate
  (document with evidence).
- A declared phased boundary (per Branch-out) — lands the gap-closing phase, re-states the
  residual; this is a scoped close, not a grind-HOLD.

Grind-HOLD (forbidden): "tune the freeze-index scale," "raise the 0.20 m cap and stop,"
"port one more subroutine then defer."

## Milestones

1. Symptom-existence + ownership + implementation-scope gate (above).
2. Contract: amend `SC-SNOWFREEZE-001` (heat-flow depth implementation spec; close/restate
   `GAP-SNOWFREEZE-002`).
3. Contract-derived red tests (heat-flow depth/duration; ratchet eliminated; activation +
   warm-day non-regression).
4. Pre-implementation gate evidence.
5. Production correction: heat-flow depth model replaces the proxy; publish `frdp`.
6. Validation: FDMC01 gap closed on the cohort; conservation still closes; activation
   non-regressed.
7. Dual review, finding disposition, dual verification, defect-shaped handoff (naming MOFE
   as the next ROADMAP item).

## Deliverables

- `artifacts/frost-depth-heatflow-localization.md` (M1 scope + seam + ownership).
- `artifacts/fdhp01-frost-depth-validation-ledger.md` (post-fix depth/duration vs legacy +
  FDMC01-metric improvement + conservation preservation + activation non-regression).
- Standard contract, gate, dual-review, verification, disposition, handoff.

## Dependencies

- `docs/ROADMAP.md` (queue item 1), `docs/backlog/20260607-frost-depth-model-heat-flow-parity.md`
- FDMC01 package + artifacts (the sized gap + metrics this must close)
- FQ-4 package (activation — must stay non-regressed)
- `AGENTS.md`, `docs/defect_closure_execplans.md`, ADR-0011/0017/0018
- `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`, `SC-WATBAL-001.md`
- Legacy reference: `/workdir/wepp-forest_260430_baseline/src/frostn.for`, `frzng.for`,
  `frznw.for`, `frsoil.for`; CRM Ch. 3.8; Dun et al. 2010
- Comparator `/home/workdir/wepppy/wepp_runner/bin/wepp_260606_hill`; substrate
  `/wc1/runs/al/algebraic-radium/wepp/runs/` (single-OFE, `ksflag=1`)

## Autonomy

Execute end-to-end for the declared single-OFE scope — M1 scope/ownership, contract
amendment, red tests, pre-impl gate, heat-flow implementation, validation, dual
review/verification, disposition, handoff — without asking for direction on intermediate
steps. A phased boundary (per Branch-out) is a permitted scoped close; a proxy tweak is
not. Ask only if the heat-flow authority is under-specified for a needed implementation
decision (then amend the contract).
