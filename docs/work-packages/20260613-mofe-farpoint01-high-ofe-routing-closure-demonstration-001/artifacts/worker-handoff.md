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

1. **`watpdg` upper-overflow symmetry (conditional defect).** `watpdg` sits
   symmetrically to `watbtm` in the (now-amended) internal frost adjustment and
   is also present in the identity's outflow (`frost_upper_overflow_mm`). It did
   **not** reproduce on H2637 (both variants, full 34 yr — `watpdg=0` on the
   relevant days), so it was correctly left unmodified. *Reproduction required
   before any change*: a `watpdg>0` (top-thaw) fixture exhibiting the same
   residual ≡ `watpdg` signature. Note the open physical question — `watpdg`
   routes *upward to the surface* and may be recycled within the OFE, which could
   make its inflow-side treatment legitimate (unlike `watbtm`, a terminal
   downward deep-perc outflow). Same authority/write-set as F-B if it reproduces.

2. **F-C — legacy-vs-openWEPP >10-OFE closure contrast (demonstration depth).**
   The core differentiating result (openWEPP closes at 19 OFEs) is done. To fully
   satisfy the package's "measure legacy's own closure on the same substrate"
   criterion, run a like-for-like legacy `wepp_260606` water-balance closure
   audit on H2637 and contrast it with openWEPP's. Far-point signatures already
   recorded (QOFE/Q = OFE ordinal; OFE19 q-cap with/without-ui ΣQ 53k→123k).
   Comparator is a flag (ADR-0017), not a target.

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
