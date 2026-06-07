# FQ-4 ksflag Frost Activation + Closure (the rung-2 target)

Status: complete

Package type: Defect-Closure ExecPlan (DC-ExecPlan)

## Objective

Close defect `FQ4-FROST-KSFLAG-ACTIVATION-001`: standard-WEPP `ksflag` frost does
not engage in openWEPP on `/wc1/runs/al/algebraic-radium` — frost depth
(`frost.runtime_frdp_m`/`dfrost`) and frozen water (`frost.runtime_ws_frz`/`frozwt`)
stay `0`, so the frozen-soil conductivity gate (`frost.runtime_infcap_frz`) never
bites and `ksflag` on/off paired runs are numerically identical — at a freezing
agricultural site (PRESTON MN, all `.man` `lanuse=1` → `ksflag=1`, frost-enabled per
`SC-SNOWFREEZE-001#INV-SNOWFREEZE-014`). Make the standard `ksflag` frost gate
activate on cold days per `SC-SNOWFREEZE-001` **or** prove with evidence that frost
legitimately should not fire on this substrate (e.g. legacy `wepp_260606` is also
frost-inactive), validated on the now-repaired single-OFE population — **with the
rung-1 + corn-ET + runoff conservation closure still holding under frost.**

This is the **rung-2 target**. It owns correction inside the frost
activation/depth/frozen-soil-conductivity envelope. If the root cause is in-envelope
and authority-backed, it must land the contract-first fix.

## Run / sibling context — the substrate is now repaired

algebraic-radium was selected *for* frost; every step since FROSTVAL01 cleared a
confounder off the frost read:

| Step | Cleared | State | Commit |
|---|---|---|---|
| FQ-1 | soil corrected-layer coverage | 6→42/43 runnable | `1faf0be` |
| FQ3-DC-RUNOFFPART | runoff partition | Q engages, conserves | `11c4e40` |
| FQ3-DC-ET-CORN | annual-crop ET/canopy | Ep+Interception engage, conserves | `ab809e8` |

The two partition defects that would confound a frost read (non-engaging ET, absent
runoff) are closed on a conserving foundation. FQ-4 is now assessable in isolation.

**FROSTVAL01 measured frost-inactive on only 6/43** (the rest were `HS-RUNTIME-E-062`
blocked, since fixed by FQ-1). FQ-4 must **re-run the activation pass on the
now-repaired ~42-runnable population** — the prior 6/6 inactive verdict is necessary
but not population-scale.

**FQ-2 ledger fix folds in here (not a separate WP).** The FROSTVAL01 frost-closure
ledger summed flux terms over a wrong ~1%-of-days set while ΔStorage was full-scale,
producing a bogus `frost-break ~134 mm` artifact. Fix the ledger to consume the
**full WAT complete identity** (incl. `Interception` and `frozwt`) **before**
measuring closure-under-frost, and **withdraw the `frost-break` classification**.

## Rationale (FROSTVAL01 evidence + static kernel read)

FROSTVAL01 `frost-activation-ledger.md` (Codex `Ran`, ksflag on/off paired):

- 6/6 runnable prefixes (`p8,p13,p22,p23,p26,p28`): `max_frozwt_on=max_frozwt_off=0`,
  `frozwt_nonzero_days=0`, and `delta_dp = delta_latq = delta_q = 0` (ksflag on/off
  identical). Source soils carry `1 1` (`ntemp ksflag`); off-soils were a strict
  `1 1 → 1 0` replacement. Classified `frost-inactive`.

Static kernel read (Claude, `03_kernel_support_00_support_helpers.rs`):

- Frost depth driver (~`:3304`–`:3335`): `freeze_active = tmin <= 0`;
  `freeze_index = clamp(−mean_temp / FROST_RUNTIME_FREEZE_INDEX_SCALE_C, 0, 1)`;
  `if freeze_active { frdp_m = frdp_m.max(WB14_FROST_MAX_DEPTH_M * freeze_index) }`
  (`WB14_FROST_MAX_DEPTH_M = 0.20`). So frost depth grows **only when daily mean
  temp < 0 °C**.
- Conductivity gate (~`:3335`): `freeze_fraction = clamp(dfrost / 0.20)`;
  `infcap_frz = soil_conductivity * (1 − freeze_fraction + freeze_fraction * kfactor)`.
  With `dfrost = 0`, `infcap_frz = soil_conductivity` → **no bite**, consistent with
  the measured on/off identity and `frozwt = 0`.

At PRESTON MN winter mean temps are below 0 °C, so the freeze path *should* produce
depth. That it produces none localizes the candidate mechanism to one of:
(a) the temperature inputs (`tmin`/`tmax`) feeding `freeze_active`/`freeze_index` are
not reaching the frost kernel (stale/zero/wrong wiring from daily gridmet);
(b) the **freeze-index proxy** itself is the issue — note it diverges from the
`SC-SNOWFREEZE-001` heat-flow authority (`INV-SNOWFREEZE-006`/`012`: `winter →
frostN → frzng → frznw`, `Qsrf`/`Quf`, harmonic-mean layered conductivity, Eq.
[3.8.1]–[3.8.4]); (c) frost depth develops but the frozen-water (`ws_frz`)
derivation does not. M1 disambiguates — do not pre-decide.

Legacy runs the same daily gridmet at the same freezing site; comparator-flag
confirmation (does `wepp_260606_hill` produce `frozwt`>0 / frost depth on the
cohort?) establishes ownership (ADR-0017: flag, not target).

## Correction Authority Envelope

### Defect IDs and Observed Violations

- `FQ4-FROST-KSFLAG-ACTIVATION-001`
  - Observable: `frdp_m`/`dfrost`=0, `ws_frz`/`frozwt`=0, `infcap_frz`=`soil_conductivity`,
    ksflag on/off identical (`delta_dp=delta_latq=delta_q=0`) on a freezing
    `lanuse=1` site. Fixture `/wc1/runs/al/algebraic-radium/wepp/runs/`;
    FROSTVAL01 `frost-activation-ledger.md` + `activation_summary.csv`.

### In-Scope Contracts and Source Files

- Contracts:
  - `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
    (**primary** — `INV-SNOWFREEZE-006` frost heat-flow, `-009` winter activation,
    `-012` frost routine-chain dispatch, `-013` frozen-soil conductivity authority,
    `-014` agricultural-frost-enabled parity).
  - `docs/specifications/science-contracts/contracts/SC-INFILE-FROST-001.md` **only
    if** M1 proves the cause is the parsed frost-control → runtime handoff
    (`frost.txt` seed not reaching the kernel).
  - `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md` as the
    downstream consumer (closure must stay closed once frost engages; `frozwt` is in
    the storage compartment).
- Production/test files:
  - `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_00_support_helpers.rs`
    (frost depth/thaw/freeze-index `:3290`–`:3335`, `dfrost`/`ws_frz`/`infcap_frz`/
    `kfactor` `:3335`–`:3530`, `FROST_RUNTIME_FRDP_M`/`TFRDP_M`/`WS_FRZ` symbols).
  - `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_01_kernel_phases.rs`
    (frost outcome → soil conductivity at `:345`, `:3925`, `:4672`).
  - `crates/openwepp-runner/src/hillslope/mod.rs` (`frost.runtime_dfrost`/`ws_frz`/
    `infcap_frz`/`winter_active` publication + `frozwt` closure guard `:5427`–`:5511`,
    `:6509`).
  - the temperature-input wiring feeding `tmin`/`tmax` into the frost kernel **only
    if** M1 proves the inputs are the cause (then see Branch-out).
  - `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/04_snow_frost_irrigation.rs`
    (frost-control seed) **only if** the parse→runtime handoff is the proven cause.
  - The FROSTVAL01 frost-closure ledger tooling (FQ-2 fold-in).
  - `tests/integration/**frost**.rs`, `**snowfreeze**.rs`, `**watbal**.rs`.
  - `docs/work-packages/20260608-fq4-ksflag-frost-activation-closure-001/**`,
    `docs/work-packages/README.md`.

### Allowed Edit Classes

- Amend canonical `SC-SNOWFREEZE-001` (or `SC-INFILE-FROST-001`, if proven) for the
  corrected frost activation / depth / frozen-soil-conductivity behavior **before**
  production code.
- Correct the freeze-index/heat-flow/temperature-input/frozen-water path so frost
  engages on cold days when it should.
- Fix the frost-closure ledger to the full WAT complete identity (FQ-2).
- Add contract-derived tests (a known-cold day → `frdp>0`/`ws_frz>0`/`infcap_frz <
  soil_conductivity`; non-regression on warm/non-freezing days; the repaired-cohort
  activation pass).
- Add bounded diagnostics to localize where frost depth/frozen water is lost.

### Protected Boundaries (do not cross)

- **No comparator-match acceptance.** `wepp_260606` is a flag that frost should fire
  (ADR-0017); acceptance is contract-correct activation, not matching legacy frost
  depth/`frozwt` magnitude.
- **Snow magnitude remains a Stage-2 protected boundary.** Frost depth/frozen water
  is in scope, but if the proven cause requires a snowpack-*magnitude* change,
  escalate to Stage-2 — do not patch snow magnitude here.
- The **forest `ksatadj` sat-fraction conductivity model is a separate concern** from
  the standard `ksflag` frost gate validated here — do not conflate or edit it.
- Do not touch annual-crop ET (`FQ3-DC-ET-CORN`, landed `ab809e8`) or runoff
  partition (`FQ3-DC-RUNOFFPART`, landed `11c4e40`); do not touch `p11` percolation
  (`FQ1-P11`, separate lineage — still blocked; exclude with note).
- Conservation must stay closed — do not engage frost by breaking the water balance;
  the rung-1 + corn-ET + runoff closure (incl. `Interception`, `frozwt`, snow) must
  still hold.
- The 17-OFE hillslope (MOFE) is out of scope (rung-3).

### Acceptance Criteria

- The standard `ksflag` frost gate activates on cold days on the repaired single-OFE
  population (`frdp_m`>0, `ws_frz`>0, `infcap_frz < soil_conductivity` when frozen,
  ksflag on/off no longer identical), consistent with `SC-SNOWFREEZE-001` (legacy
  nonzero as a flag, not a target) — **OR** a documented legitimate exclusion (legacy
  also frost-inactive / temps genuinely never trigger) with typed evidence.
- The frost-closure ledger consumes the full WAT complete identity; the bogus
  `frost-break` classification is withdrawn; closure-under-frost is re-derived.
- The water-balance closure (rung-1 identity + corn-ET + runoff + totalwatsed3 audit)
  still closes with frost engaged (`frozwt` in the storage compartment).
- Contract-derived red/green tests; pre-implementation failing evidence; post-fix
  validation over the repaired population + non-regression on warm/non-freezing days.
- No conservation break, comparator-target tuning, silent default, or downstream
  compensation.

### Branch-out Boundaries

- If M1 proves the cause is the **freeze-index proxy vs the `SC-SNOWFREEZE-001`
  heat-flow authority** (`INV-SNOWFREEZE-006`/`012`): either amend the contract to
  sanction the proxy with explicit provenance/bounds, or implement the heat-flow
  chain — in-package if bounded; branch a defect-shaped frost-physics target if it is
  a broad re-implementation.
- If the cause is the **temperature-input wiring** (daily climate → frost kernel),
  amend `SC-SNOWFREEZE-001`/`SC-CLIMATE-001` in-package if it is the frost kernel's
  temperature input, else branch a defect-shaped climate-seam target.
- If frost legitimately should not fire on this substrate (legacy also inactive /
  temps never below 0 °C), exclude with evidence — this is a legitimate HOLD, not a
  forced fix.

## Conversion Rule

If a reproducible root cause is established inside the declared frost
activation/depth/conductivity envelope and the corrected behavior is supported by
canonical `SC-SNOWFREEZE-001` (or proven `SC-INFILE-FROST-001`/`SC-CLIMATE-001`)
authority, pinned-baseline provenance, or a contract-authorized physical invariant
(frost forms when the soil column freezes), the package must proceed through contract
amendment → tests → pre-implementation gate → production correction → validation →
disposition. It may not close `HOLD` because more investigation is possible. (The one
non-grind exception: M1 + comparator prove frost legitimately should not fire — a
documented exclusion, not a deferral.)

## Seven-Gate Bar

All seven true ⇒ `HOLD` invalid, must land the fix: (1) reproduce frost-inactive
(`frdp=frozwt=0`, ksflag on/off identical) on a repaired-cohort prefix on a cold day;
(2) named mechanism (temp-input wiring / freeze-index-proxy / heat-flow chain /
frozen-water derivation / conductivity gate), not "trace deeper"; (3) ownership
(legacy `wepp_260606_hill` produces frost on the same climate/site, or openWEPP's own
freeze path should fire by temperature); (4) authority `SC-SNOWFREEZE-001`/physical
freeze law, not comparator match; (5) safety — no conservation break, no snow-magnitude
patch, no `ksatadj` edit, no silent default; (6) testability — red/green on a known-cold
day; (7) validation — repaired-population frost activation + closure measurable
before/after.

## Symptom-Existence + Ownership Gate (Milestone 1, first)

1. **Fix the closure ledger first** (FQ-2) so closure-under-frost is measurable, then
   **re-run the ksflag on/off paired activation pass on the repaired ~42-runnable
   population** (not just the prior 6). Confirm frost-inactive at population scale.
2. **Anti-false-pass guard:** confirm the activation telemetry actually *reads* frost
   state (`frdp_m`, `ws_frz`, `infcap_frz`, `freeze_active`, `freeze_index`,
   `tmin`/`tmax`, `daily_mean_temp_c`) on a known-cold winter day — a stubbed/zero
   telemetry would "pass" trivially. Localize: are `tmin`/`tmax` correct cold values
   but `frdp_m` still 0 (depth-logic/proxy defect), or are the temps wrong/stale
   (input wiring)?
3. **Ownership:** run `wepp_260606_hill` on the same inputs — does legacy produce
   `frozwt`>0 / frost depth on the cohort? Legacy-frost-fires (or openWEPP's own
   temperature triggers) → openWEPP defect. Legacy-also-inactive + temps never trigger
   → legitimate exclusion.

## Legitimate HOLD Conditions

- Mechanism outside the declared frost envelope (branch with a defect-shaped target —
  broad heat-flow re-implementation or a climate-seam temperature defect).
- Frost legitimately should not fire (legacy also inactive / temps never below 0 °C) —
  document the exclusion with evidence.
- Canonical authority missing/contradictory.
- Required evidence cannot be generated in the environment.

Grind-HOLD (forbidden): "inspect the next frost variable," "trace frost depth one step
deeper," "root cause in the freeze-index but implementation deferred."

## Milestones

1. Symptom-existence + ownership gate (above) — ledger fixed, population re-run,
   anti-false-pass confirmed, mechanism named, comparator-flag ownership.
2. Contract: amend `SC-SNOWFREEZE-001` (or proven `SC-INFILE-FROST-001`/`SC-CLIMATE-001`).
3. Contract-derived red tests (cold day → `frdp`/`ws_frz`/conductivity bite;
   warm-day non-regression; repaired-cohort activation).
4. Pre-implementation gate evidence.
5. Production correction in the proven frost activation/depth/conductivity surface.
6. Validation: repaired-population frost activation; closure (rung-1 + corn-ET +
   runoff + totalwatsed3) still holds with frost engaged; ledger re-derived.
7. Dual review, finding disposition, dual verification, defect-shaped handoff.

## Deliverables

- `artifacts/frost-activation-localization.md` (M1 mechanism + ownership + the fixed
  ledger + population re-run).
- `artifacts/fq4-frost-validation-ledger.md` (before/after frost activation + closure
  preservation incl. `frozwt` + warm-day non-regression).
- Standard contract, gate, dual-review, verification, disposition, handoff.

## Dependencies

- `/workdir/openWEPP/AGENTS.md`, `docs/codex_exec_plans.md`,
  `docs/defect_closure_execplans.md`
- `docs/decisions/0011-...`, `0017-...`, `0018-...`
- `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`,
  `SC-INFILE-FROST-001.md`, `SC-WATBAL-001.md`, `SC-CLIMATE-001.md`
- FROSTVAL01 package + `artifacts/frost-activation-ledger.md`,
  `frost-closure-ledger.md`, `frostval01-followon-queue.md`, and the Claude review
- `reference_wepp_forest_frost_ksflag_ksatadj` lineage (standard ksflag vs forest
  ksatadj distinction)
- Comparator: `/home/workdir/wepppy/wepp_runner/bin/wepp_260606_hill`
- Run inputs: `/wc1/runs/al/algebraic-radium/wepp/runs/`

## Autonomy

Execute end-to-end for the declared scope — ledger fix + M1 localization/ownership on
the repaired population, contract amendment, red tests, pre-impl gate, production
correction, validation, dual review/verification, disposition, defect-shaped handoff —
without asking for direction on intermediate steps. Ask only if hard-blocked by a
proven boundary (broad heat-flow re-implementation, a climate-seam temperature defect,
or a legitimately frost-inactive substrate confirmed by comparator + temperature).
