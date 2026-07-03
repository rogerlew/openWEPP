# Handoff Prompt — Erosion Port Increment-1 (single-OFE Wave-1 sediment-continuity solve)

> **This is a task prompt for a fresh executing agent.** It is self-contained;
> read the referenced repo artifacts, then implement. You are executing
> **Increment-1** of the ADR-0035 hillslope-erosion sediment-continuity port in
> `/home/workdir/openWEPP` (openWEPP, Rust). ADR-0035 names Claude Code as
> executor for this port (an operator-authorized exception to the
> Codex-authors-code default) — you are continuing that mandate. **Author the
> Rust code.**

## 0. Orient (read these first, in order)
1. `docs/decisions/0035-hillslope-erosion-sediment-continuity-port.md` — why this port exists, staging, guardrails.
2. `docs/work-packages/20260703-erosion-sediment-continuity-port-001/package.md` — the WP + increments.
3. `docs/work-packages/20260703-erosion-sediment-continuity-port-001/artifacts/entry-gate.md` — the **Increment-0 design you are implementing** (decision, algorithm map, operand lineage, recovery plan). This handoff embeds the exact equations that doc summarizes.
4. `docs/specifications/science-contracts/contracts/SC-SED-001.md` — the governing contract. Your acceptance authority is **INV-SED-001..007 + INV-SED-010** (not legacy magnitude). Read those invariant rows.
5. Root `AGENTS.md` + `crates/.../AGENTS.md` — conventions + validation gates. Legacy `.for` is **source-intent authority (ADR-0024), never a magnitude oracle (ADR-0017)**.

## 1. What you are building (and what already exists)
The direct runtime has the **pointwise** detachment/deposition flux physics
(`crates/openwepp-hillslope-orchestrator/src/direct_runtime/erosion.rs`:
`direct_erod13_fluxes`, `compute_direct_erod13`) but **no spatial driver**: it
evaluates fluxes at one point using a *supplied* load `G` and only *validates* a
supplied `dgdx`; it never evolves `G(x)` along the OFE. Wave-1 is hard-disabled
(`00_builders_and_authority.rs:942` `wave1_enabled = false`).

**Increment-1 = build the normalized-space spatial continuity solver** that
evolves `G` from OFE top to toe, finds detachment↔deposition transitions,
integrates total detachment/deposition, and emits exported sediment — for a
**single OFE** — then enable Wave-1 for single-OFE and close mass conservation.

The whole solve is in the legacy **nondimensional** space: `x ∈ [0,1]`, 101 grid
points `xinput(i) = (i−1)·0.01`; load `G` normalized by `tcend`; parameters
`η/τc/θ/φ`; shear/transport polynomials `(a,b,c)`/`(atc,btc,ctc)`. Recover
dimensional sediment at the OFE exit via `tcend`. **Do not** try to reuse the
dimensional `direct_erod13_fluxes` as the inner loop — the driver is
normalized-space, matching legacy.

## 2. Recover the scaffold (do this first — big head start)
A prior Rust port already implemented much of this before it was deleted with the
symbol-map lane. Recover it as a reference:
```
git show a381702b^:crates/openwepp-hillslope-orchestrator/src/hydrology/kernel_phases_mod/hydrology_phase_storage_erosion/hydrology_phase_erod19.rs
```
It **provides** (adapt these to current types — it uses deleted `crate::hydrology::*` symbol-map types): `erod19_shear`, `erod19_root`, `erod19_xcrit_classification` (all 5 `mshear` cases), `erod19_depc`, `erod19_depend`, and the `param.for` derivations (`erod19_route_theta/phi/tauc`, `erod19_transport_capacity`, `erod19_initial_deposition_rate`, `erod19_route_drivers/ldlast`).
It **lacks** (you must ADD, §3): `erod` (RK4 101-pt march), `runge` (RK4 step), `depos`/`depeqs` (analytic deposition-profile writer), and the `route` segment control flow.

Legacy source-intent to verify against (baseline `dac3c950`, `/workdir/wepp-forest_260430_baseline/src/`): `route.for`, `erod.for`, `runge.for`, `xcrit.for`, `depc.for`, `depeqs.for`, `depend.for`, `depos.for`, `param.for`, `xinflo.for`.

## 3. The exact algorithm you must implement (source-intent)

### 3a. Continuity ODE + RK4 (`erod.for`/`runge.for`) — the core new piece
```
detachment region (Tc > 0):  dG/dx = Dc·(1 − G/Tc) + θ
                (Tc ≤ 0):     dG/dx = θ
  where  Dc = η·(shr − τc), clamped ≥ 0        [detachment capacity]
         shr = (a·x² + b·x + c)^(2/3)          [flow shear τf]
         Tc  = (atc·x² + btc·x + ctc)·ktrato, clamped ≥ 0
```
Classic **RK4** step over `dx` (default 0.01; first sub-point of a segment uses `dx = xinput(i) − xb`):
```
k1 = dx·f(x,        G)
k2 = dx·f(x+dx/2,   G + k1/2)
k3 = dx·f(x+dx/2,   G + k2/2)     ← REUSE k2's Dc,Tc (same x+dx/2); only G changes
k4 = dx·f(x+dx,     G + k3)
G_new = G + (k1 + 2k2 + 2k3 + k4)/6
```
**Interrill floor (required):** `if G_new < G + θ·dx { G_new = G + θ·dx }`
(`runge.for:219` — prevents RK undershoot below the interrill contribution).
**`/detcom` memo:** legacy recomputes `Dc` only when the shear term or `η/τc`
changed. Reproduce the *values* (a stateless recompute is fine — it must be
numerically identical; the memo is a perf cache, not a semantics change).

### 3b. Detachment march regime + deposition onset (`erod.for`)
March `i` while `xinput(i) ≤ xe`. At each point after `runge`:
- `Tc > 0`: `ldrat = 1 − G/Tc`; `kflag=1`; `detach(i) = Dc·ldrat`. If `G > 0` also `ldrat2 = Tc/G − 1`; `kflag=2`.
- `Tc ≤ 0`: if `G > 0` → `ldrat2 = Tc/G − 1`; `kflag=2`. else `G=0`; `kflag=3`.
- **Deposition trigger:** `(kflag==2 && ldrat2<0) || (kflag==1 && ldrat<0)` → set `ndep=1`, stop the march; secant-solve onset `xdbeg` via `cross` (linear zero-crossing of the detach ratio) then hand to deposition (§3d).
- Case-4 (flow ends inside plane): `−1 < qostar < 0 && xinput(i) > −qostar` → `G=0`, `kflag=4`.

### 3c. Shear classification `xcrit` → `mshear` and sub-interval dispatch
`tauchk = τc^(1.5) − c` (**un-clamped** — the baseline `≥0` clamp is commented out at `xcrit.for:82`; the recovered `erod19` *added* a clamp — **reconcile to un-clamped**, or prove it's a no-op on the fixture). Roots of `a·x² + b·x − tauchk = 0` give crossings `xc1 ≤ xc2`. Cases: **1** below-crit throughout, **2** above-crit throughout, **3** rising cross (below→above), **4** falling cross (above→below), **5** double cross (up-then-down; downgrade to 1 if roots ∉ segment or coincident). Dispatch (detachment-at-top form): pass rill erodibility `η` to `erod` on **above-critical** sub-intervals and `0.0` on **below-critical** ones (so only interrill `θ` acts there):
| mshear | erod calls |
|---|---|
| 1 | `erod(xu,xl, 0.0)` |
| 2 | `erod(xu,xl, η)` |
| 3 | `erod(xu,xc1, 0.0)`; if ndep==0 `erod(xc1,xl, η)` |
| 4 | `erod(xu,xc1, η)`; if ndep==0 `erod(xc1,xl, 0.0)` |
| 5 | `erod(xu,xc1, 0.0)`; if ndep==0 `erod(xc1,xc2, η)`; if ndep==0 `erod(xc2,xl, 0.0)` |

### 3d. Deposition region (analytic — `depc`/`depeqs`/`depend`/`depos`)
Continuity `dG/dx = φ·(Tc − G)` integrates to `G = Tc − D(x)·(x + q*)/φ`.
- `depc` → integration constant `cdep` from the known deposition rate `du` at onset `xu` (`depc.for:42-47`).
- `depeqs` → deposition rate `D(x) = 2a·ktrato·φ·(x+q*)/(2+φ) + φ/(1+φ)·(b·ktrato − θ − 2a·ktrato·q*) + cdep·((xu+q*)/(x+q*))^(1+φ)` (`depeqs.for:60-67`).
- `depend` → where deposition ends `xdend` (Newton ≤10 iters; increasing-flow `q*≥0` vs decreasing `q*<0` branches, `depend.for`).
- `depos` → write `detach(i)=D(x)`, `Tc(i)`, `G(i)=Tc(i) − D·(x+q*)/φ` along the grid; monotonic guard `G(i)=min(G(i),G(i−1))` when `θ≤0`; clamp `G≥0` (`depos.for:110-117`).

### 3e. `route` control flow (per OFE)
Init `ldlast = strldn`; per segment `k`: classify with `xcrit`; compute upper-boundary deposition rate `dl` (`route.for:169-173`: `|qostar|<0.0011` → `dl = φ/(φ+1)·(ktrato·btc − θ)`, else `dl = φ/qostar·(ktrato·ctc − ldlast)`). If `dl < 0` → deposition-at-top (depc→depend→depos, then remaining detachment sub-intervals by mshear). Else → detachment (mshear dispatch of `erod`). After detachment, if `ndep≠0`, run the deposition tail from `xdbeg`. Carry `ldlast`/`dl` to the next segment.

### 3f. Normalization (`param.for`/`xinflo.for`) — build these operands per OFE-day
`η = cntlen·kr·kradjf·shrsol/tcend`; `τc = tcadjf·shcrit/shrsol`;
`θ = cntlen·detinr/tcend·(effdrr/effdrn)`; `φ = β·Vf/pkro` (β=0.5 with rain else 1.0; `Vf=falvel(spgeff,diaeff)`; `pkro=(qout−qin)/slplen`; clamp φ to ±100000);
`ktrato = kt2/kt`; `tcend = kt·shrsol^(1.5)`; `qostar` (`xinflo.for:152-171`: `qout≤0` → `−efflen/slplen`; else `qin≤0` → 0, else `qin/(qout−qin)`); `strldn` = inflow load (0 for single-OFE OFE-1). Shear polynomial coeffs `a,b,c` = `ainf,binf,cinf` (`xinflo.for:177-204`), transport `atc,btc,ctc`.
**These raw operands already exist on `DirectErod13Inputs`** (`kr_s_m, kradjf, tcadjf, shrsol_pa, tcend_kg_s_m, shcrit_pa, detinr_kg_s_m2, cntlen_m, effdrr_m, effdrn_m, beta, vf_m_s, pkro_m3_s, ...`) — the projection exists; you build the driver that consumes them across `x`. Any operand not available → typed fail-closed (no defaults).

### 3g. Glossary
`G`/`load` sediment load (nondim, ÷tcend); `Tc`/`tcap` transport cap; `Dc`/`dcap` detach cap `η(shr−τc)`; `detach(i)` rate (>0 detach, <0 deposit); `shr`/`τf` shear `(ax²+bx+c)^{2/3}`; `τc`/`tauc` critical shear; `η`/`eata` rill erodibility; `θ`/`theta` interrill source; `φ`/`phi` deposition param `βVf/q`; `Vf`/`veleff` fall velocity; `ktrato` normalized transport coeff; `qostar` nondim discharge; `qin/qout` discharge/width; `strldn` inflow load; `xu/xl` seg top/bottom; `xdbeg/xdend` deposition begin/end; `xc1/xc2` shear-critical crossings.

## 4. Integration + scope
- Put the solver in a focused module (e.g. `direct_runtime/erosion_continuity.rs` or under `support_helpers_mod/`); keep files <2000 lines (governance BLOCK at 3000).
- Wire **shadow-first**: the frame already has `erosion_shadow_projection`, `run_r7d6_erosion_span`, and `r7d8_erosion_inputs_with_runoff_authority` (which already supplies `q_runoff/peakro/watdur` from `peak_runoff_shadow_projection`). Validate the shadow to conservation **before** it becomes the production authority.
- Enable Wave-1 for **single-OFE** (`00_builders_and_authority.rs:942`, and the `direct_production_typed_erosion_authority`). Leave multi-OFE integration + particle classes to Increments 2/3 — but do not regress the existing EROD14 routing or its guards.
- **Scope discipline:** single OFE only. INV-SED-008 (strip routing) and INV-SED-012 (MOFE qin handoff) are Increment-2; per-class enrichment (INV-SED-009) is Increment-3. Emit **totals** (`total_detachment_kg`, `total_deposition_kg`) for INV-SED-010; per-class fields deferred.

## 5. Acceptance gate (hard)
- **Mass conservation:** `Σ detachment − Σ deposition = exported sediment at the OFE toe` (denormalized), within a named tolerance. Fail-closed typed guard.
- **SC-SED-001 pointwise invariants** hold: INV-SED-001 (continuity signs), -002 (detach branch, `Df=0` when `τf≤τc`), -003 (deposition `Df<0`, `q>0` domain), -006 (Tc, sandy `tcadjf≥0.30`), -007 (`η/τc/θ/φ` finite denominators).
- **Fixture:** a **runoff + sediment-generating** hillslope — Hortonian runoff on unfrozen ground, **McKenzie Bridge OR class**. Candidate: `tests/fixtures/disturbed_burn/forest_high_severity_clay_loam/` (single-OFE, McKenzie Bridge). **Do NOT validate on `forest_high_severity_loam`/p313 (MORAN WY)** — it is erosion-inert (no infiltration-excess runoff). Prove the fixture actually generates detachment (nonzero exported sediment on storm days) before asserting the gate.
- Unit tests for: the RK4 continuity on a constant-coefficient segment where the analytic solution is known (verify RK4 matches to ~1e-9); each `mshear` case; a detach→deposition transition; the interrill floor; a conservation round-trip.

## 6. Fidelity traps (from Increment-0)
1. RK4 `k3` reuses `k2`'s `Dc,Tc` (same `x+dx/2`) — do not recompute at k3.
2. Interrill floor `G_new ≥ G_old + θ·dx` is mandatory.
3. `tauchk` is **un-clamped** in the baseline; the recovered `erod19` added a clamp — reconcile (un-clamp, or prove no-op on the fixture).
4. mshear dispatch passes `η` only on above-critical sub-intervals, `0.0` below.
5. Work in **normalized x∈[0,1]** space throughout; denormalize only at the OFE exit.
6. `falvel`/effective fall velocity and `kt/kt2` (transport coefficients) must come from the soil/Chapter-7 projection — trace them; fail-closed if absent.

## 7. Workflow + validation gates (AGENTS.md)
- Fresh worktree off `main`; the fixture needs `.venv` symlinked for the full suite (`ln -s /home/workdir/openWEPP/.venv .venv` if untracked).
- Full gates before merge-ready: `cargo fmt --check`; `cargo clippy --workspace --all-targets -- -D warnings`; `cargo nextest run --workspace --profile full`; `cargo deny check`; `git diff --check`; `bash tools/release/check_authority_suite_antievasion.sh --base-ref main --head-ref HEAD`; SC-unit lint.
- **No provisional/proxy sediment math in the production path** — fail-closed typed guards, named tolerances, source-intent equations only.
- **Do NOT self-merge to `main`.** Land on a branch, push, and request a Codex review pass. Record the work in `artifacts/implementation.md` (evidence classes Static/Ran, matched to what you actually ran).
- Update `package.md` status to reflect Increment-1 completion.

## 8. Deliverables
1. The normalized single-OFE Wave-1 sediment-continuity solver (source-intent, gated).
2. Wave-1 enabled for single-OFE; conservation + INV-SED gate green on the McKenzie-class fixture.
3. Unit + integration tests (§5).
4. `artifacts/implementation.md` (+ line-count-governance if a file split was needed).
5. Branch pushed for Codex review; `package.md` updated. **Not merged to main.**
