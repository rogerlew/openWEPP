# Release Notes

Evidence class: `Static`.

- Frost direct production is now the no-env default hillslope runtime path for
  supported modern single-OFE runs.
- Current multi-OFE/Wave-2 and legacy sidecar-discovery runs fall back to
  compatibility with an explicit fallback reason until a separate package
  promotes those direct surfaces.
- Explicit `--compatibility-runtime` remains the rollback selector.
- The frost observation validation method is ratified under
  `INV-SNOWFREEZE-047/048/050`; the residual gap remains open but attributed and
  bounded rather than zero-closed.
- No Qwet implementation, new frost physics, fixture repoint, public schema
  change, or observation-harness default change is included.
