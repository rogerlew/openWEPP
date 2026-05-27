# WSHEDIMPL22 Disposition

Status: complete
Evidence mode: static+ran
Date: 2026-05-27

## Static
- Decision: `HOLD`
- Scope completion: complete for declared WS22 slice.
- Closed in this package:
  - WS21 opt-in lane no longer relies exclusively on unresolved fallback for
    positive-excess segments when required authority symbols are present.
  - Required `crfrac` class-fraction seam is now explicit and fail-closed.
  - Baseline-lineage `dcap` helper math is wired into WS21 detachment branch.
  - WS21 `case34` and `enddet` branch execution is active in WS21 opt-in path.
  - WS22 contract-derived vectors for `crfrac` failure and success lanes are
    active.
- Remaining blockers (still non-promotable):
  - `GAP-ROUTE-009`
  - `GAP-SED-006`
  - `GAP-SYSTEM-008`
  - Residual baseline-authoritative closure required for WS21 `case4 ->
    detach` iterative branch (`nt < cnpart`) before full
    `chnero/chnrt/detach` parity claims can be promoted.
  - Kernel-profile sequencing variance is documented in
    `wshedimpl22-kernel-profile-compliance-checklist.md` and
    `wshedimpl22-preimplementation-contract-gate.md` (contract-first order was
    not strictly in-order for this execution sequence).

## Ran
- Validation gates recorded in `gate-results.md`.
