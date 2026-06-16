# Worker Handoff — FARPOINT01

Status: F-A complete; **F-B CLOSED** (frost `watbtm` double-count corrected
contract-first; H2637 19-OFE closure demonstrated). Evidence: `disposition.md`.

Evidence mode: Ran + Static.

## Current state

FARPOINT01 selected H2637 (19 OFEs, in-repo provenance), produced a clean
`wepp_260606` legacy baseline, and ran openWEPP on it. openWEPP fail-closed at
the per-element WB13 gate (OFE5, frost day) — a defect-shaped finding, closed in
F-B by removing the frost lower-overflow `watbtm` from the internal frost
adjustment (it is owned by the `Dp` outflow lineage). With the fix, openWEPP
runs the full 19-OFE × 34-yr H2637 to completion under the hard conservation
gates — the FARPOINT01 differentiating result (closure past the legacy ceiling).

## Follow-on items

These are **named candidate defects / scopes**, not diagnostic breadcrumbs. None
is required for F-B closure.

1. **`watpdg` upper-overflow symmetry — RESOLVED: validated non-defect.**
   `watpdg` sits symmetrically to `watbtm` in the internal frost adjustment and
   is also present in the identity's outflow (`frost_upper_overflow_mm`), so the
   F-B fix deliberately left it unchanged pending a `watpdg>0` reproduction.
   **Executed** via temporary instrumentation (an `eprintln` when `watpdg>0`,
   reverted before commit) on the full-34-yr H2637 run: `watpdg>0` occurred on
   **4 OFE-days** (max `3.0e-7 m` = `3e-4 mm`), and the per-element + hillslope-
   total gates **closed at <1e-11 mm on those rows**. Had `watpdg` been a
   net-egress double-count (like `watbtm`), those rows would have failed by
   ≈`+watpdg` (3e-4 mm ≫ 1e-11 mm). They did not. So `watpdg` **cancels
   exactly** on both sides (magnitude-independent) and the both-sides treatment
   conserves. Mechanism: unlike `watbtm` (deep-perc egress, debited from
   `Total-Soil`, so `storage_delta` carries `−watbtm` and breaks the
   cancellation), `watpdg` routes upward to the surface and is **not** a net
   soil-storage egress — `storage_delta` carries no `−watpdg` term, so the
   inflow/outflow `watpdg` pair cancels. No code/contract change warranted.

2. **F-C — legacy-vs-openWEPP >10-OFE closure contrast — COMPLETE**
   (`fc-legacy-closure-contrast.md`). Dispositive, assumption-light result
   (QOFE/Q duality verified exactly): legacy `wepp_260606` with `wepp_ui`
   produces outlet runoff = **127.7 % of precip** (runoff > precip, a physical
   conservation violation — the WB-05A q-cap, quantified); openWEPP is
   `wepp_ui`-invariant and runoff-bounded (71.0 %) and closes its three
   identities at <1e-11 on the same substrate. Legacy without_ui is bounded
   (55.5 %). Comparator is a flag (ADR-0017); the openWEPP-vs-legacy magnitude
   gap (71 % vs 55.5 %) is a Stage-2 `MOFE-MAGPARITY01` question, not closure.

3. **openWEPP high-OFE performance (characterization).** openWEPP ran H2637 in
   ~1020 s vs the legacy Fortran's ~10 s — ~100× on a 19-OFE × 34-yr hillslope.
   Worth attributing (per-OFE WB13 shadow-state / parquet accumulation cost?). A
   characterization pass, not a defect yet.

## Watchpoints

- `per_ofe_internal_wb13.rs` is 752 lines (under the 2000 WARN threshold).
- No production wepppy edits; H2637 inputs are read-only fixture provenance.
- The H2637 working fixture lives under `/tmp/openwepp_farpoint01_h2637/`; if a
  durable in-repo fixture is wanted, the inputs (~1.1 MB, climate-dominated)
  would need a checked-in home + a test harness (separate scope).
