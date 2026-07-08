# Final Disposition

Status: `EXECUTED-HOLD-PROJECTION-AUTHORITY`.

M-T2P executed and closed as a projection-authority hold.

The package rejected implicit legacy-cropland projection to Lane D route
coefficients. The audited baseline sources compute or publish roughness,
aggregate friction, erosion, plant-state, and diagnostic terms, but they do not
provide a bounded deterministic mapping for all five static Lane D operands.

Changes made:

- `SC-OFEROUTE-001` rev 48 records `GAP-OFEROUTE-008` and BEI
  `OFEROUTE-ROUTE-COEFF-PROJECTION-AUTHORITY`.
- `plant-file.spec.md` now tells end users that route coefficients are explicit
  and are not inferred from legacy cropland fields.
- ROADMAP/catalog entries now show M-T2P as executed-hold.
- Package artifacts record the source audit, projection hold, canonical path
  policy, future bridge fidelity envelope, review, verification, and gates.

No Rust implementation, default selector broadening, coefficient formula,
solver/mesh/tolerance change, or legacy path deletion was made.

Post-closure consensus addendum: the preferred next authority direction is not
a new runfile or disturbed-class sidecar. To reduce operator error and avoid
sidecar omission changing physics, `ow-lanuse-1` should become the canonical
production datver for new openWEPP physics. WEPPpy should embed Disturbed/native
route coefficients directly in native management files; legacy datvers remain
compatibility inputs routed through legacy single/MOFE driver behavior unless
explicitly converted.

Validation summary:

- BEI non-strict: `PASS-DEFERRED`.
- BEI strict: deferred-nonzero because existing SC-OFEROUTE
  `science-review-follow-on` rows are not consolidated; recorded as such.
- SC unit compliance: PASS.
- Markdown/doc lint: PASS.
- `git diff --check`: PASS.
