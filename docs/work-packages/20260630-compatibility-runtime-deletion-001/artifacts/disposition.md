# Disposition

Evidence class: Ran

Result: `EXECUTED-COMPLETE-PARTIAL-DELETION`

## What Closed

- Removed obsolete skeleton, shadow, and cutover runtime selections from the
  runner API and CLI.
- Removed the old compatibility-shaped direct publication day-input builder and
  runtime-surface overlay helper family.
- Removed the retained direct-publication cutover adapter that reconstructed
  direct rows from compatibility/simulation-owned WB13 rows.
- Removed stale tests that existed only to preserve those transition modes.
- Added a source guard preventing reintroduction of the deleted selectors, flags,
  and helper names.
- Updated the architecture spec, roadmap, ADR index, and local work-package
  guidance to reflect ADR-0030: production direct is the no-env hillslope
  default, silent compatibility fallback is a defect, and compatibility bit-parity
  is no longer the frost acceptance target.

## What Remains

- `--compatibility-runtime` remains intentionally available as an explicit,
  deprecated replay/comparator seam.
- Symbol-map carrier types such as `HillslopeWritebackSurface` still exist
  outside the production direct hot loop for setup-time parsing, legacy/replay
  support, and non-hillslope surfaces. Their deletion needs a separate typed
  setup-carrier/full replay-seam deletion package.
- RSS/working-set reduction was not attempted; that remains the separate
  performance workstream.

## Gate Verdict

The package is acceptable for the scoped production-transition deletion:

- full Rust, deny, anti-evasion, and obligation gates pass;
- source guard and static scan prove the deleted transition modes are absent
  from runner production/API/CLI sources;
- full direct fixture tests prove default, legacy sidecar-discovery, multi-OFE,
  and Wave-2 paths stay on direct production and report zero compatibility-edge
  invocations.

The next full-deletion package must not inherit this package's output-identity
shortcut. If it removes the explicit replay seam or setup carriers, it must run
true before/after byte identity across H2637, multi-OFE, and Wave-2 fixtures.
