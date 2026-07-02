# MOFEFID-D01 — OFE-by-OFE Routing Scaffold + Friction Kernels

Status: **EXECUTED (D1+D2+D3) — REVIEW-READY** (2026-07-02)
Campaign: [MOFEFID](../../planning/mofe-fidelity-campaign-strategy.md) Lane D.
Owner: Claude Code. Worktree: `mofefid-d01`. Activation: **opt-in-validated
only** (operator decision 2026-07-02; default activation deferred).

## What landed

- **D1 (contracts/ADR):** ADR-0033 (Proposed) — equivalent-plane ->
  OFE-by-OFE routing, opt-in, F-A2/DC01 tie-in, SC-OFEROUTE-001 as the
  contract home. Ratification pending (authorizes the solver stages).
- **D2 (fixtures):** `artifacts/validation-cases.json` — the four
  Papanicolaou validation-case inputs (from R-63 Table 1 + 3.1 docx) with
  Ef targets; observed series referenced in Figure_4.xlsx (copyright
  governance: not duplicated).
- **D3 (friction kernels):** `crates/openwepp-hillslope-orchestrator/src/
  ofe_routing/friction.rs` — eqs. (2)-(7) as pure SI functions (skin
  regime-dispatch, form, wave Fr-ramp, vegetation Katul, additive f_eq,
  Chezy), shadow-first (no phase wiring), 6/6 unit tests grounded in the
  equations + case parameters.

## Frozen-library citation posture

eq. 2-3 constants (Shen & Li, Hirsch) and eq. 4 applicability bounds
(Abrahams 1998) cited SECONDARY via R-63; eq. 4/5/6 primaries (R-77/72/78)
in hand. Unit conventions confirmed empirically by D-val (not by an
un-acquired primary).

## Gates (D3)

- `ofe_routing::friction` 6/6 unit tests; module is shadow-first (opt-in,
  no default-path call — default H2637 byte-flat by construction).
- fmt/clippy clean.

## Next (post-ratification)

D4 single-OFE KWE/TVD solver (eqs. 8-14, CFL) validated on Cases 1/2/4
(Case 4 = Iwagaki shock capture); D5 OFE-by-OFE cascade (Case 3 vegetation
patchiness); D-val acceptance (Ef targets, Zone taxonomy, default byte-flat).
The A01/A02/DC01 runon re-infiltration seam is the cascade's re-infiltration
semantics — hourly-faithful vs DC01's daily approximation.
