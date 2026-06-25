# ADR Disposition

Static:

- Added `docs/decisions/0027-opt-in-physics-bulk-snow-model.md`.
- Added ADR-0027 to `docs/decisions/README.md`.

Decision:

- Accepted an opt-in deliberate-legacy-divergence snow-density remediation lane
  named `physics_bulk`.
- Preserved `legacy_wepp` as default production behavior and rollback.
- Required offline Rust snowbench implementation before any runtime opt-in.
- Prohibited site-specific tuning and SSD residual fitting.
- Kept PySnobal/SNOBAL and legacy WEPP as diagnostic flag profiles, not targets.

Non-decisions:

- No exact equations/constants selected.
- No runtime selector, parser surface, output schema, or default activation
  added.
- No production runtime physics implemented.

Disposition:

- Complete. ADR-0027 supplies the governance bridge from evidence package to
  offline physics-core work without authorizing premature runtime coupling.
