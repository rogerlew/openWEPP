# H2637 34-Year Endpoint Recheck: Daily Timing Drift and Active-Router Fail-Closed — 2026-07-10

Status: Final
Last updated: 2026-07-10
Evidence mode: Executional
Scope: Wall/user/RSS timing of `openwepp-cli-hill` on the canonical 34-year
H2637 endpoint (daily path and Lane D active production default) plus a
paired legacy anchor, on this host, this session. Not in scope: root-causing
the active-router failure, attributing the daily-path drift, watershed tier.

## 1. Purpose

The dev-guide's §6.4 performance numbers mix two endpoints: the 34-year
daily H2637 run (32.8 s, 2026-07-01, pre-Lane-D) and the 2-year
`laned_shadow_h2637` active fixture (11.72 s post-Tier-1). This audit
answers: **what do the daily path and the Lane D active production default
actually cost today on the same canonical 34-year endpoint, against a
same-session legacy anchor?**

## 2. Method

All commands were run in this session (Ran) on the wshedperf01-class host
(dual Xeon E5-2697 v2 @ 2.70 GHz, 48 logical CPUs), pinned `taskset -c 4`,
release build, profiling off, ambient load avg ~2.7–4.4 (other sessions
active — see Caveats).

- Binary: `cargo build --release -p openwepp-runner --bin openwepp-cli-hill`
  from `main@0c1ae324` (worktree carried uncommitted changes confined to
  watershed-tier code (`chaninp.rs`, `openwepp-watershed-orchestrator`), a
  runner *test* file, and docs — no hillslope runtime surface; verified via
  `git status --short`). SHA-256
  `2fd65b10c60c2e43354e5675452247b092a7e031ae9771bea53bd11a2e195630`.
- Inputs: the surviving 2026-07-01 staging of the H2637 WB05A replay inputs
  (19 OFE × 12,419 days); all five input files verified `sha256`-identical
  to the canonical wepp-forest WB05A `with_wepp_ui/runs/` source. The
  native-management variant is the WB05A `p2637.man` (byte-identical to the
  committed 2-year fixture's) with the same patch the Tier-1 timing used:
  `ow-lanuse-1` datver, 19 `NativeCropland` landuses, 19
  `routing_coefficients 500.0 0.0 0.0 0.0 0.0` blocks (taken verbatim from
  the Tier-1 `/tmp` staging).
- Run protocol: exact 2026-07-01 shape — runfile with all five outputs on,
  `--policy compat --legacy-sidecar-discovery`, `wepp_ui = false` (the
  original `h2637.run` recorded that setting), `/usr/bin/time -v`, 3 reps
  per config.
- Configs: **A** legacy management, default env (router cannot activate —
  no coefficients); **B** native management,
  `OPENWEPP_LANED_ACTIVE_DISABLE=1`; **C** native management, default env
  (conditional default activation → router active); **D** = C with
  `wepp_ui = true`, single rep.
- Legacy anchor: `wepp_260430_hill < p2637.run` on the same staged inputs,
  same core. Two initial reps, then two reps **interleaved** with fresh
  config-A reps after the first legacy numbers looked contaminated.
- Activation probe: the committed 2-year fixture run with `wepp_ui = false`
  and `OPENWEPP_LANED_SHADOW_PROFILE=1` to confirm the router activates
  without `wepp_ui` and to compare solver counters against the Tier-1
  record.

## 3. Findings

### 3.1 Timing matrix (user seconds, `%M` max RSS)

| Config | Router | Rep user times | Median | Max RSS |
|---|---|---|---:|---:|
| A — daily, legacy man | inactive | 38.87 / 39.04 / 38.67 | **38.87 s** | 68,736–72,616 KiB |
| A — interleaved reps | inactive | 39.18 / 39.20 | 39.19 s | 68,716 KiB |
| B — daily, native man | disabled | 39.91 / 40.80 / 41.15 | **40.80 s** | 68,140–71,240 KiB |
| C — active, native man | **active** | **fail-closed** (53.88 / 49.56 / 50.17 to failure) | — | 47,632 KiB at failure |
| D — C with `wepp_ui=true` | **active** | **fail-closed** (1 rep) | — | — |
| legacy, first pair | — | 17.27 / 17.51 | *discarded* | 4,992 KiB |
| legacy, interleaved | — | **9.27 / 9.08** | **9.18 s** | 4,992 KiB |

The first legacy pair (17.3/17.5 s) is discarded as transient host
contamination: interleaved reps minutes later returned 9.1–9.3 s while
config-A runs bracketing them repeated at 39.2 s. Five config-A reps across
~12 minutes span 38.67–39.20 s — the openWEPP number is stable.

### 3.2 The active router fail-closes on the canonical endpoint

All three config-C reps and the config-D (`wepp_ui=true`) rep died
identically and deterministically:

```
CLIHILL-E-011 … HS-SIMPIPE-E-001 direct runtime day execution failed at
lane 8 day 2621: direct runtime kernel guard failed in laned_active_cascade:
lane 8 day 2621 routing failed: NegativeOutletBin
```

Day 2621 ≈ 1994-03-05 (±1 for index base): a **zero-precipitation,
hard-freeze day** (tmax 7.2 °C → tmin −5.0 °C) immediately following a
three-day ~46 mm warm rain spell (Mar 1–3, 1994) — a recession/melt-tail
regime. The 2-year committed fixture (1987–1988) never reaches this span,
so no prior active-router run had exercised it. **The production default
cannot currently complete the canonical 34-year H2637 endpoint.** There is
consequently no completed active-router timing for this endpoint; the
2-year post-Tier-1 numbers (11.72 s / ~21 MiB) remain the only completed
active measurements.

### 3.3 Same-session ratio and drift vs 2026-07-01

- Interleaved same-core pairing: **39.19 s vs 9.18 s ≈ 4.27× legacy** —
  inside the ≤5× aspirational budget, well inside ≤10× viability.
- Daily-path drift: 32.77 s (2026-07-01, WP-2 rubric record) → 38.87–39.20 s
  today = **≈ +19%**, with the legacy control unchanged (9.65 s then,
  9.18 s now). The drift is therefore real accumulated cost, not host
  noise. Five weeks of landings sit in the interval (erosion port E.1–E.5,
  residue/cover coupling, hourly-substrate seams, baseflow export, native
  landuse intake, …); attribution was **not** performed.
- RSS is essentially unchanged and run-length-flat: ~67–71 MiB (vs ~74–80
  MiB recorded on 2026-07-01; legacy ~4.9 MiB).

### 3.4 Secondary observations

- Native-management daily run (B) vs legacy-management (A): `loss.json`
  physics-identical (differs only in embedded `run_name`); B ran ~1.6–2 s
  (~4%) slower than A. Small and not adjudicated (native intake cost vs
  minute-scale noise). HBP/parquet outputs were not canonicalized for
  metadata, so byte-identity beyond `loss.json` was not adjudicated.
- 2-year activation probe: the router **does** activate with
  `wepp_ui = false`, and its solver counters
  (`solver_runs=11590`, `solver_steps=10016170`,
  `alpha_evaluations=100161700`) match the Tier-1/post-sweep records
  exactly — the routing workload on the 2-year fixture is unchanged since
  those measurements.

## 4. Caveats

- Ambient load was moderate and variable (other agent sessions active,
  including a concurrent test run); absolute times carry that noise. The
  interleaved legacy/openWEPP pairing bounds it for the ratio claim; the
  discarded 17 s legacy pair shows the pollution was real.
- Single host, single session, 3 reps per config, one fixture. No
  quiet-host confirmation run.
- The active configs use the synthetic `500.0 0.0 0.0 0.0 0.0` routing
  coefficients inherited from the Tier-1 fixture convention — the same
  authority every H2637 active measurement has used, but not a
  field-derived coefficient set. The `NegativeOutletBin` failure should be
  reproduced under ratified coefficients before any claim about real-member
  exposure.
- Per ADR-0037, H2637 is a synthetic stress case; these numbers
  characterize the canonical perf endpoint, not fleet behavior.
- The +19% daily drift is measured but unattributed; no bisect or profile
  was run in this audit.

## 5. Recommended follow-ups (not performed in this audit)

- **Defect closure for `NegativeOutletBin`** (Codex lane): deterministic
  repro is `openwepp-cli-hill --run-dir <staged runs> --run-file
  <native-man runfile> --output-dir <out> --policy compat
  --legacy-sidecar-discovery` on the staged 34-year inputs; fails in ~50 s
  at lane 8, day 2621. Candidate neighborhood given the rev-41 WA
  positivity-solver history: source-quiet/recession stepping positivity.
- Re-run C on real selected-cohort members' full climates to establish
  whether the failure class reaches non-synthetic inputs.
- Profile/bisect the +19% daily drift if it matters for the ≤5× budget
  (current headroom: 4.27× measured under ambient load).
- Quiet-host confirmation of the 39.2 s / 9.2 s pair.
