# MOFEFID-D6 — Infiltration Coupling (Rainfall-to-Runoff)

Status: **EXECUTED — REVIEW-READY** (2026-07-02)
Campaign: [MOFEFID](../../planning/mofe-fidelity-campaign-strategy.md) Lane D
integration. Contract: `SC-OFEROUTE-001` (rev 4). Owner: Claude Code.
Worktree: `mofefid-d6`. Activation: **opt-in / shadow-first**.

## What landed

`crates/openwepp-hillslope-orchestrator/src/ofe_routing/infiltration.rs` —
the rainfall-to-runoff coupling that completes the Lane D routing mechanism:

- **Green-Ampt-Mein-Larsen infiltration kernel** (`green_ampt_step`,
  `infiltration_capacity_m_s`): capacity `fc(F) = Ks(1 + psi*dtheta/F)`, the
  unponded->ponded transition, and the implicit Green-Ampt integration
  `F - F0 - s*ln((F+s)/(F0+s)) = Ks*dt` by Newton iteration.
- **`green_ampt_excess_hyetograph`**: rainfall series -> rainfall-excess
  hyetograph (per OFE), sub-stepped, mass-conserving (rainfall = infiltration
  + excess).
- **`run_infiltrated_cascade`**: the full composition — per-OFE Green-Ampt
  turns each OFE's RAINFALL into excess, which the D5 cascade routes with the
  upstream hydrograph as a surface boundary (Papanicolaou assumption 2 + 1).

Shadow-first: `ofe_routing` (D3-D6) is **not referenced by production**
(grep-verified against `direct_runtime/` and `openwepp-runner/`); default
hillslope path byte-flat (`INV-OFEROUTE-010`).

## GAP-OFEROUTE-003 physics corrected (D5 -> D6)

The D5 resolution said "supersede-then-compose (re-infiltrate the routed
hydrograph)". That was a **misread** and D6 corrects it (contract rev 4):
faithful Papanicolaou is **SUPERSEDE** — infiltration acts on **rainfall**
(assumption 2); the upstream hydrograph is a **surface boundary condition**
(assumption 1) that is **NOT re-infiltrated**. The routing supersedes DC01's
daily-lump runon re-infiltration with hydraulic surface routing; there is no
second re-infiltration of the runon. Production wiring + DC01-disable is the
activation gate.

## Validation (9 tests, no copyrighted data)

Green-Ampt kernel (5): capacity diverges at dry / decays with F / asymptotes
to Ks; rainfall below Ks never ponds (zero excess); high rainfall ponds and
produces excess with decaying infiltration; impermeable -> all excess;
mass conservation across variable rainfall. Coupled cascade (4): Case-1-like
rainfall-to-runoff conserves end-to-end (rainfall = infiltration + outlet +
storage) with 0<RC<1 and CFL held; impermeable routes all rainfall;
two-OFE lower-Ks-downslope runs off more; config/soil domain fail-closed.

## D-val Ef status (INV-OFEROUTE-011)

The rainfall-to-runoff mechanism the formal Ef needs now EXISTS (this
package). The committed Ef-vs-observed acceptance still awaits: (a) the
Papanicolaou case soil/rainfall operands bound to these kernels, and (b) the
**digitized observed series** (the in-repo supplemental carries the paper's
model series, not cleanly-labeled observed data — copyrighted, not
vendored). The Nash-Sutcliffe harness (D4) + coupling (D6) are ready; the Ef
run is D-val with observed data.

## Gates

- `ofe_routing` 32/32 (23 D3-D5 + 9 D6); full orchestrator suite 180/180;
  clippy `-D warnings` 0; fmt clean; BEI PASS-DEFERRED; authority guards
  PASS; `ofe_routing` shadow-first (not called by production).

## Next

Production activation (separate gate): wire `run_infiltrated_cascade` into an
opt-in runtime path, disable DC01's daily-lump admission when active, then run
the D-val Ef against digitized observed series. This is the default-activation
decision with its own no-regression + endpoint evidence — distinct from the
shadow-first mechanism D6 lands.
