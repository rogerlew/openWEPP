# Snow / Frost Fidelity Strategy

Status: **active strategy** (2026-06-25). Primary gap: `GAP-SNOWFREEZE-002`.
Physics authority: Claude Code. Architecture/runtime: Codex.
Subordinate to: `SC-SNOWFREEZE-001` (science), the array-native runtime spec
(architecture), and ADR-0011 / 0017 / 0024 / 0026.

## 0. TL;DR (the one-paragraph map)

openWEPP under-predicts snow **density** — and legacy WEPP does so *identically*
(Δ ≤ 4 kg/m³), so it is a **shared WEPP-lineage defect, not an openWEPP
regression**. Root cause: WEPP's empirical settling caps density at ~250 kg/m³
and has **no overburden compaction**, while real snow reaches 340–495. Fix:
replace the empirical settling with a **physics-based snow sub-solver** that
adopts **Anderson-1976 densification** (temperature metamorphism + overburden
compaction + melt) and **temperature-dependent fresh-snow density** — universal
literature constants, **no site-specific parameterization**; per-site density
*emerges* from the forcing (SWE/overburden + temperature). Delivered **staged,
opt-in** (`snow_model = legacy_wepp | physics_bulk`), **offline-snowbench-first**,
**rubric-gated**. Then resume frost-depth fidelity, which is blocked behind snow
insulation. PySnobal is the **reference implementation / cross-check**, not the
runtime engine.

## 1. How we got here (the arc)

- R7H closed opt-in; frost-depth fidelity reopened under `GAP-SNOWFREEZE-002`.
- Frost-depth attribution is **blocked behind snow insulation** (snow depth +
  density is the dominant frost confound).
- **D/E:** openWEPP over-predicts snow depth; E adjudicated it real (not a
  depth-vs-SWE/timing artifact) and wrote `INV-SNOWFREEZE-048`.
- **F:** legacy WEPP fails the same way; openWEPP ≈ legacy → shared lineage.
- The **over-accumulation vs low-density fork** could not be resolved at the
  frost sites (no observed SWE there).
- **SNOTEL added** (paired SWE + depth → observed density) across five mountain
  snow climates. **H** ran the three-way (openWEPP / legacy / PySnobal vs
  observed).
- **H verdict:** all five route `STRUCTURAL` — the fork was a *false binary*
  (SWE is forcing-limited, density is too low, entangled); setting SSD to the
  observed climatological density does **not** close depth.
- **Legacy shares the density defect** (Δ ≤ 4 kg/m³ from openWEPP) — confirmed
  via H's three-way.
- **Archaeology:** WEPP settling is CRM Eq 3.7.1 (1995) shimmed daily-into-hourly
  (Dun 2007); it **caps at `ssd` (250)** via `setf=1` and lacks overburden
  compaction; flagged and **deferred ~19 years** (snow-code Stage-2 dossier,
  Shen 2011, the `Eq. 3.7.5` code-vs-doc divergence).
- **PySnobal source** implements **Anderson-1976 compaction** (`_time_compact`
  PTM+POC, `_h2o_compact`) — the exact missing physics, universal constants.

## 2. The decision

**Physics-based snow density, not parameter tuning.** Adopt the Anderson-1976
formulation (what PySnobal uses), **reimplemented natively in Rust**, with
universal constants. Site density is **emergent from forcing**, never fit
per-site. H already proved a per-site SSD value does not close the defect, so
parameterization is a dead end by evidence.

## 3. Goals

- **G1 (primary):** modeled density and the seasonal `ρ(t)` trajectory reproduce
  observed SNOTEL density across **all five climates with ONE universal constant
  set**, no per-site fitting — graded on the `INV-SNOWFREEZE-050` rubric's
  forcing-robust cells.
- **G2:** depth/SWE improve where forcing allows (forcing-limited; not the
  primary gate — DAYMET/lapse under-catch is a climate-input problem, not snow
  physics).
- **G3:** snow-insulation control (`TOL-SNOWFREEZE-009`) becomes passable →
  **unblocks frost-depth attribution**.
- **G4:** the fix ships **opt-in**, beats legacy on forcing-robust signatures,
  and is conservation-clean.

## 4. Discipline / guardrails

- **Observation-first** (SNOTEL density via `INV-SNOWFREEZE-048/049`, rubric
  `INV-SNOWFREEZE-050`); **external-authority physics** (Anderson 1976);
  ADR-0017 (legacy/PySnobal are flags, never targets).
- **Anti-tuning:** constants come from the literature, **not** fit to SNOTEL;
  SNOTEL *validates*.
- **Contract-first:** amend `SC-SNOWFREEZE-001` before any production code.
- **Conservation:** densification is **SWE-conserving** (mass constant, depth
  shrinks); density bounded (~550 kg/m³).
- **Deliberate-legacy-divergence:** this is openWEPP's **first intentional
  improvement *over* legacy physics** (legacy is wrong too). It needs a
  governance ADR; snow-influenced legacy-parity gates re-baseline.
- **Profile, not scalar:** the rubric grades per-cell with forcing-robustness
  tiering; verdicts are `R`-cell-driven.

## 5. The physics target (WHAT — Claude)

- **Fresh-snow density:** temperature-dependent (~50–200 kg/m³), replacing WEPP's
  fixed `100` (`snow.txt` `densnf`). Formula from a standard relationship
  (Anderson/SNOW-17 or Hedstrom–Pomeroy) — choice is an open item.
- **Densification (Anderson 1976; SNOBAL `_time_compact`/`_h2o_compact`):**
  - PTM (temperature metamorphism): `PTM = 0.01·K·exp(−0.04·(Tz−Ts))`,
    `K = exp(−0.046·(ρ−100))` (or 1 if ρ<100).
  - POC (**overburden compaction ∝ SWE**): `POC = 0.026·exp(−0.08·(Tz−Ts))·SWE·exp(−21·ρ)`.
  - `_h2o_compact` (melt/rain liquid-water densification).
  - `ρ_new = ρ + (PTM + POC)·ρ`, capped ~550.
- **Universal constants;** per-site density emerges from SWE (overburden) +
  temperature. **Remove the `snow.txt` `ssd` cap** (a *parameter deletion* —
  reduces the tuning surface).
- **Minimal thermal state:** a **bulk snow temperature / cold content** for the
  `T`-dependence — **not** a full two-layer surface-energy-balance SNOBAL.
- **PySnobal = reference / cross-check** in stable (established-pack) regimes
  only — thin-snow cells are PySnobal-unavailable (its `sati.c` instability; see
  the H CSS WY2017 disposition). **Not** a runtime dependency (Python, unstable).

## 6. The architecture (HOW — Codex)

- The snow physics core lands **inside the ADR-0026 winter-column sub-solver**
  (snow state is already typed there) — not a greenfield subsystem.
- **Opt-in mode:** `snow_model = legacy_wepp | physics_bulk` (mirrors the
  array-native compatibility/direct opt-in).
- **Offline Rust snowbench first:** a standalone physics solver driven by the
  existing forcing bridge, validated on the rubric **before** production runtime.
- **Native Rust port** of the Anderson equations — not a PySnobal embed.
- **Increment 1 = single-layer mass + bulk thermal + Anderson densification +
  T-dependent fresh snow.** Escalate to a fuller 1–2 layer energy balance **only
  if** Increment 1 fails to close the rubric ("smallest physics that closes it").

## 7. The sequence (goals / gates / owner)

| Stage | Goal | Gate | Owner |
|---|---|---|---|
| **DENSITY-01** | Localize the empirical failure; read Shen 2011; pin the `Eq. 3.7.5` code-vs-doc divergence; confirm missing-overburden via SNOTEL `ρ(t)` | diagnostic profile, no code | Claude (physics read) |
| **DENSITY-02** | Contract-first physics spec: Anderson PTM/POC/`h2o` + T-fresh-snow invariant in `SC-SNOWFREEZE-001`; remove `ssd`; red tests | contract + red tests, no production | Claude |
| **DENSITY-03** | Offline Rust snowbench; validate density/`ρ(t)` vs SNOTEL + PySnobal on the rubric | beats legacy on forcing-robust cells, offline | Codex (build) + Claude (rubric review) |
| **DENSITY-04** | Couple as opt-in `physics_bulk` in the winter-column; conservation hard-stop; rerun the three-way rubric; governance ADR for the legacy divergence | rubric profile + conservation + opt-in + ADR | Codex + Claude |
| **FROST-RESUME** | With snow-insulation control bounded, resume the original frost-depth fidelity question | `TOL-SNOWFREEZE-009` passable; frost attribution unblocked | — |

## 8. Current state / assets already built

- **SNOTEL fixtures + observation corpus + observed density** (5 climates):
  `tests/fixtures/snotel_observed/`.
- **Frost-depth fixtures:** `tests/fixtures/snowfreeze_observed/`.
- **The rubric:** `SC-SNOWFREEZE-001` `INV-SNOWFREEZE-050` + addendum +
  `TOL-SNOWFREEZE-011` (v74).
- **Correspondences/tolerances:** `INV-SNOWFREEZE-048` (depth), `049` (density),
  `TOL-SNOWFREEZE-009/010`.
- **Three-way harness:** `tools/snowfreeze_observed/` (`observed_harness`,
  `legacy_snow_compare`, `pysnobal_compare`, `snotel_density_three_way`).
- **H package** (three-way results, `STRUCTURAL` verdict, PySnobal disposition):
  `docs/work-packages/20260625-snowfrost-fidelity-h-snotel-density-three-way-001/`.
- **Snow archaeology:** `snowd.for` Eq 3.7.1 + the `ssd` cap + version
  divergence; Shen 2011 (`references/copyrighted/D_Shen_020312.pdf`); the
  deferred Stage-2 dossier (`docs/backlog/20260605-snow-code-deferred-science-review.md`).
- **Reference physics:** SNOBAL `libsnobal/_time_compact.c`, `_h2o_compact.c`,
  `_precip.c` (Anderson-1976 formulation).

## 9. Open decisions

- **Energy-balance escalation fork:** Increment 1 (minimal) vs a fuller energy
  balance — decided by whether Increment 1 closes the rubric.
- **Governance ADR** for the deliberate legacy divergence + the `ssd` parameter
  removal.
- **Fresh-snow-density relationship** choice (Anderson/SNOW-17 vs Hedstrom–Pomeroy).
- Whether to fold the **"PySnobal-unavailable in thin-snow regime"** rule into
  `INV-SNOWFREEZE-050`.
- The **FDHP01 F4 reconciliation** (F4 measured openWEPP density ~381 vs legacy
  ~250; H now shows openWEPP ≈ legacy ≈ 250 — likely the SNOWSCI-S1 conservation
  single-sourcing shifted it; confirm).

## 10. References / authority

- Anderson, E. A. (1976), *A point energy and mass balance model of a snow
  cover*, NOAA Tech. Report NWS-19 — the compaction physics (PTM/POC).
- Marks et al. (1999) SNOBAL, *Hydrol. Process.* 13:1935; Oleson et al. (2013)
  CLM (Anderson lineage); Lute et al. (2022) GMD 15:5045 (shallow-snow stability).
- Shen, D. (2011/2012) WSU MS thesis — WEPP snow distribution (R-25).
- WEPP CRM Ch 3.7 — the documented empirical settling (Eq 3.7.1–3.7.5).
- `SC-SNOWFREEZE-001` (`INV-SNOWFREEZE-047/048/049/050`, `TOL-SNOWFREEZE-007..011`);
  array-native runtime spec; ADR-0011/0017/0024/0026.
- `docs/backlog/20260605-snow-code-deferred-science-review.md` (Stage 2).
- SNOWFROST-FIDELITY A–H work packages.
