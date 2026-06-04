# WB11-ET-E-003 Failure Characterization (pre-followup)

Status: diagnostic / findings-only
Evidence mode: static + ran
Author: Claude Code (root-cause analysis; implementation deferred to Codex)

This artifact characterizes the pre-existing `HKERNEL-WB11-ET-E-003` workspace
failure that has held HPHYS0275 through HPHYS0280 at `completed/HOLD`. It is the
diagnostic basis for scoping the follow-up package. It does **not** prescribe an
implementation; it localizes the defect and names the governing contract
authority so Codex can own the fix design.

## 1. Reproduction (Ran)

Ran: `cargo test --test pl14s_tier_a_candidate_emission_and_replay_contract simimpl18`
on clean `HEAD` (working tree clean). Both tests fail identically:

- `simimpl18_contract_requires_cold_day_partition_zero_rm_and_runtime_snow_storage`
- `simimpl18_contract_requires_multi_day_storage_state_mutation`

Both panic at the fixture run, not the assertion. Captured scheduler outcome:

```
HS-SIMPIPE-E-001 scheduler lifecycle did not complete successfully
(outcome_class=phase_failure, status_class=Failure,
 boundary_class=DOMAIN_VIOLATION, message_id=HKERNEL-WB11-ET-E-003,
 last_phase=evapotranspiration, last_decision_outcome=Reject)
[sim_day_index=1, calendar_year=2000, julian_day=1]
```

The failure is a hard `Reject` on **sim-day 1 (1 Jan 2000)** in the
`evapotranspiration` phase. This is not introduced by HPHYS0280 — it reproduces
on the committed baseline.

Ran: re-ran the cold-partition test with `OPENWEPP_HPHYS0245_TRACE_PATH` set; the
EVAPPM diagnostic trace is flushed at end-of-run and is therefore **not**
produced when the run aborts on day 1. No JSONL was written. (This is itself a
diagnostic-observability gap for the follow-up to consider.)

## 2. Failure mechanism (Static)

The guard that fires is the WB11 ET-phase EVAPPM-branch range check on the
seeded soil-evaporation component:

- `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_01_kernel_phases.rs:535-540`
  range-checks `pmet.es_m` with floor `-WB11_ZERO_THRESHOLD` (`1.0e-12`,
  `crates/openwepp-hillslope-orchestrator/src/constants.rs:92`). A value more
  negative than that tolerance is rejected; within-tolerance negatives are
  snapped to zero (`normalize_non_negative_within_tolerance`).

So the kernel is rejecting a **materially negative** `pmet.es_m` published by the
producer. The guard is behaving as designed (HPHYS0263/0264 era).

### Why the producer emits a material-negative `es_m`

The EVAPPM PMET seed is computed in the runner:
`crates/openwepp-runner/src/hillslope/mod.rs:2010-2196`.

Both fixture tests rewrite the day-1 climate line to a cold day but leave the
dewpoint column unchanged (`tests/integration/pl14s_tier_a_candidate_emission_and_replay_contract.rs:330-345`):

```
1 1 2000 4.4 2.0 0.25 3.0 -1.6 -14.6 200.0 3.0 180.0 -1.0
                          TMAX  TMIN  RAD VWIND DIR  TDPT
```

Dewpoint `TDPT = -1.0 °C` now sits **above** `TMAX = -1.6 °C` — a supersaturated /
dew regime. The seed derives vapor pressures monotonically from temperature
(`mod.rs:2011-2014`):

- `ed = svp(tdpt = -1.0)`
- `emaxt = svp(tmax = -1.6)`, `emint = svp(tmin = -14.6)`, `ee = ½(emaxt+emint)`

Because `svp` is monotone increasing, `ed > emaxt > ee`, so:

1. `(ee − ed) < 0` → the FAO-56 aerodynamic term in `etorc_mm`
   (`mod.rs:2048-2049`) is negative, driving reference ET toward/through zero
   (condensation regime).
2. `rhd_pct = ed/emaxt·100 > 100%` (`mod.rs:2050`) → `kcbadj ≤ 0`
   (`mod.rs:2053-2057`) → `etke` falls to its `1.2` fallback (`mod.rs:2058-2061`).
3. With `etke > 0` and reference ET negative, `potes_m = etorc_mm·etke·0.001 < 0`
   (`mod.rs:2185`). Since `potes_m ≤ residue_interception`, the seed takes the
   else branch and publishes `es_m = potes_m` directly (`mod.rs:2191-2193`) — a
   material negative.

Static note on confidence: the *sign reversal of `(ee − ed)` and `rhd_pct > 100%`*
is proven directly from the inputs and the `svp` monotonicity. The exact numeric
`es_m` and the precise sign of `etorc_mm` were **reasoned, not measured** (the
trace cannot flush on a day-1 abort). The follow-up's first action should be a
diagnostic capture of `etorc_mm`/`potes_m`/`es_m` for this day to confirm the
magnitude. The *fact* of a material-negative `es_m` is proven by the Ran
reproduction.

## 3. Contract authority — the guard is correct; the producer is the defect

`docs/specifications/science-contracts/contracts/SC-EVAP-001.md` is explicit on
both sides of this seam:

- **REF-EVAP-LEGACY-PMET-SEAM (line 70):** baseline `evappm`, *"when
  `es - resint < 0`, returns `-xx` to top-layer storage rather than publishing a
  material negative `es`."* The baseline handles the condensation case by
  **returning the negative quantity to top-layer soil storage** (mass-conserving)
  and publishing a non-negative `es`.
- **BR-EVAP-WB17-PMET-COMPONENT-SEAM (line 178):** the WB17/WB11 consumer must
  *"derive `Es`/`Er` from non-negative `pmet.es_m`, **reject material negative
  `pmet.es_m`** while snapping only within-tolerance negative roundoff to zero."*
- **INV-EVAP-004 (line 190):** actual soil evaporation must remain non-negative.

Therefore:

- The WB11 kernel guard's hard reject is **contract-faithful** to
  BR-EVAP-WB17-PMET-COMPONENT-SEAM. It is not the defect.
- The EVAPPM seed producer (`mod.rs:2185-2193`) is **not** contract-faithful to
  REF-EVAP-LEGACY-PMET-SEAM: it omits the baseline `-xx`-to-top-layer-storage
  redistribution and publishes the raw negative `es_m`. **This is the root
  defect.**

## 4. Secondary finding — producer/consumer disagreement on negative EVAPPM Es

The two downstream consumers of EVAPPM `es` disagree, and only one is reached
before the run dies:

- **WB13 sim-out publication** (`mod.rs:6218-6231`) tolerates `Es` to a `-1e-12`
  hard floor and, on the EVAPPM branch, **clamps a negative `Es` to `0.0`**.
- **WB11 ET kernel guard** (`kernel_phases.rs:535-540`) **rejects** material
  negative `pmet.es_m`.

Note that clamp-to-zero (WB13 path) is *also* not faithful to
REF-EVAP-LEGACY-PMET-SEAM, which returns the mass to storage rather than
discarding it. The contract names one correct behavior (return-to-storage); two
runtime seams each implement a different non-faithful behavior. The follow-up
should reconcile both seams against the single contract clause, not pick one of
the two existing behaviors.

## 5. Scope hand-off for the follow-up package (findings, not prescription)

The follow-up owns design/disposition; the following are the localized facts:

1. Root defect: EVAPPM seed (`mod.rs:2185-2193`) must implement the
   REF-EVAP-LEGACY-PMET-SEAM negative-`es` handling so it never publishes a
   material-negative `pmet.es_m`. Contract authority already exists; this is a
   production-faithfulness gap, not a new-contract question.
2. Confirm magnitude first: capture `etorc_mm`/`potes_m`/`es_m` for H?/day-1 of
   the `simimpl18_*` fixtures (the trace needs to survive a same-day abort, or a
   targeted diagnostic run is needed).
3. Reconcile the WB13 clamp-to-zero (`mod.rs:6228`) against the same contract
   clause so producer and both consumers agree.
4. Consider whether `dewpoint > tmax` warrants any climate-boundary note. It is
   physically a dew/condensation case the baseline handles, so this is **not**
   argued to be an input-rejection defect — flagged only for completeness. The
   climate parser (`crates/openwepp-input-contract/src/parsers/climate.rs`) does
   not guard `tdpt` against `tmax`.

## 6. Why this matters beyond the HOLD

This failure sits in the WB11 ET kernel — the exact domain parked at HPHYS0265,
where Ep/SWU identities all closed and the residual was attributed upstream. The
HPHYS0272 re-entry condition for the Ep thread was "return to WB17 `Ep` only
where snowpack state no longer owns the residual." Clearing this defect both
promotes the 0272–0280 governance stack to a green workspace **and** re-opens the
ET kernel with the upstream radiation/snow/typed-boundary seams now cleaned up.
The condensation-handling gap is a concrete, contract-anchored ET-producer
correction — a natural first step back into the Ep work.
