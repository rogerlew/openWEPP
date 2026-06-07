# FQ-4 Kickoff — ksflag frost activation + closure (the rung-2 target)

Execution mode: package-end-to-end

Autonomy: execute end-to-end — FQ-2 ledger fix + M1 localization/ownership on the
repaired population, contract amendment, contract-derived red/green tests,
pre-implementation gate, production correction, validation, dual review/verification,
disposition, defect-shaped handoff — without asking for direction on intermediate
steps. Ask only if hard-blocked.

## Item 1 — close defect `FQ4-FROST-KSFLAG-ACTIVATION-001`

Standard-WEPP `ksflag` frost does not engage in openWEPP on
`/wc1/runs/al/algebraic-radium`: frost depth (`frost.runtime_frdp_m`/`dfrost`) and
frozen water (`frost.runtime_ws_frz`/`frozwt`) stay 0, so the frozen-soil
conductivity gate (`frost.runtime_infcap_frz`) never bites and ksflag on/off paired
runs are identical — at a freezing agricultural site (PRESTON MN, all `.man`
`lanuse=1` → `ksflag=1`, frost-enabled per `SC-SNOWFREEZE-001#INV-SNOWFREEZE-014`).
Make the standard ksflag frost gate activate on cold days per `SC-SNOWFREEZE-001`, OR
prove with evidence frost legitimately should not fire (legacy also inactive / temps
never trigger). Closure must still hold under frost.

This is the **rung-2 target**, on the substrate the prior packages repaired (FQ-1
soil `1faf0be`, runoff DC `11c4e40`, corn-ET DC `ab809e8`). Do NOT touch ET, runoff,
or p11 percolation — they are landed/separate.

Primary surfaces: `03_kernel_support_00_support_helpers.rs` frost depth/freeze-index
(`:3290`–`:3335`) + `dfrost`/`ws_frz`/`infcap_frz`/`kfactor` (`:3335`–`:3530`);
`03_kernel_support_01_kernel_phases.rs` frost→conductivity (`:345`/`:3925`/`:4672`);
runner `mod.rs` frost publication + frozwt closure guard (`:5427`–`:5511`, `:6509`).

## Static lead (confirm, do not assume)

Frost depth grows only when daily MEAN temp < 0 °C:
`freeze_active = tmin<=0`; `freeze_index = clamp(-mean_temp/SCALE)`;
`if freeze_active { frdp_m = frdp_m.max(0.20 * freeze_index) }`. Conductivity:
`infcap_frz = soil_conductivity*(1 - ff + ff*kfactor)`, `ff = dfrost/0.20`. With
`dfrost=0` there is no bite — consistent with the measured on/off identity and
`frozwt=0`. At PRESTON MN winter mean temps ARE below 0, so the freeze path should
fire. Candidate mechanisms (M1 disambiguates, do not pre-decide): (a) `tmin`/`tmax`
not reaching the kernel (input wiring); (b) the freeze-index PROXY itself —
note it diverges from the SC-SNOWFREEZE heat-flow authority (INV-SNOWFREEZE-006/012,
`winter→frostN→frzng→frznw`, Qsrf/Quf, harmonic conductivity, Eq 3.8.x); (c)
`frdp_m` develops but `ws_frz` (frozen water) does not.

## Milestone 1 first (ledger fix → re-run → localize → ownership)

1. **Fix the FROSTVAL01 frost-closure ledger** (FQ-2 fold-in): consume the full WAT
   complete identity (incl. `Interception`, `frozwt`), not the ~1%-of-days subset
   that produced the bogus `frost-break ~134 mm`. Withdraw `frost-break`.
2. **Re-run the ksflag on/off paired activation pass on the repaired ~42-runnable
   population** (FROSTVAL01 only had 6). Confirm frost-inactive at population scale.
3. **Anti-false-pass:** confirm telemetry actually reads `frdp_m`/`ws_frz`/
   `infcap_frz`/`freeze_active`/`freeze_index`/`tmin`/`tmax`/`daily_mean_temp_c` on a
   known-cold day. Are temps correct-cold but `frdp_m=0` (depth/proxy defect), or are
   temps wrong/stale (input wiring)? Name the mechanism.
4. **Ownership:** run `wepp_260606_hill` on the same inputs — does legacy produce
   `frozwt`>0 / frost depth? Legacy-fires (or temps trigger) → openWEPP defect;
   legacy-also-inactive + temps never trigger → legitimate exclusion with evidence.

## Acceptance authority + constraints

- Conversion rule: root cause in-envelope + `SC-SNOWFREEZE-001`/physical freeze-law
  authority ⇒ MUST land the contract-first fix. The only non-grind close-without-fix
  is a comparator+temperature-proven legitimate frost-inactive exclusion.
- `wepp_260606` is a FLAG that frost should fire (ADR-0017), NOT a match target.
- **Conservation must still close** (rung-1 + corn-ET + runoff + totalwatsed3, incl.
  `frozwt` in the storage compartment) after frost engages.
- No comparator-match tuning, silent defaults, or downstream compensation.

## Hard constraints (protected boundaries)

- Snow MAGNITUDE → Stage-2 (escalate, don't patch, if the cause needs a snowpack
  magnitude change). Forest `ksatadj` is a SEPARATE concern from standard ksflag
  frost — do not edit/conflate. Do NOT touch ET (corn-ET DC) or runoff (runoff DC) or
  p11 percolation (FQ1-P11). 17-OFE MOFE out of scope (rung-3).

## Required reading

- `docs/work-packages/20260608-fq4-ksflag-frost-activation-closure-001/package.md`
- FROSTVAL01 `artifacts/frost-activation-ledger.md`, `frost-closure-ledger.md`,
  `frostval01-followon-queue.md`, and the Claude FROSTVAL01 review
- `docs/decisions/0011/0017/0018`, `docs/defect_closure_execplans.md`, `AGENTS.md`
- `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`,
  `SC-INFILE-FROST-001.md`, `SC-WATBAL-001.md`, `SC-CLIMATE-001.md`
- Comparator `/home/workdir/wepppy/wepp_runner/bin/wepp_260606_hill`; run inputs
  `/wc1/runs/al/algebraic-radium/wepp/runs/`.
