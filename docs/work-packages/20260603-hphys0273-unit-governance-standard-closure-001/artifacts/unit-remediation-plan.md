# Unit Remediation Plan

Status: completed
Evidence mode: static

Static:

This queue is intentionally ordered from governance authority to enforcement and
then publication alignment. Packages may execute autonomously, but they should
not invert this dependency order unless a package records a specific blocker and
handoff.

## Package Queue

1. `20260603-hphys0273-unit-governance-standard-closure-001`
   - Author canonical unit-governance standard and amend contract-profile
     requirements.
   - Establish the rule set for registry, typed boundaries, conversions,
     output metadata, lint gates, and scalar exceptions.
2. `20260603-hphys0274-boundary-symbol-unit-registry-closure-001`
   - Implement the machine-readable registry that all later remediation uses.
   - Add validation for missing or conflicting dimensional units.
3. `20260603-hphys0275-boundaryvalue-dimensional-typing-remediation-001`
   - Expand typed unit wrappers and migrate high-risk dimensional runtime
     seams away from raw scalar values.
   - Consume the HPHYS0274 registry for priority and unit labels.
4. `20260603-hphys0276-unit-conversion-helper-and-raw-literal-guard-001`
   - Centralize named directional conversions and add a guard against
     unauthorized raw dimensional conversion literals.
   - Cover conversions such as meters/millimeters, per-second/per-day,
     Langleys/MJ, and rates.
5. `20260603-hphys0277-climate-radiation-physical-flux-guard-001`
   - Add production fail-closed guard for finite impossible hourly radiation.
   - Must derive bounds from baseline/physics authority, not a heuristic
     fixed cutoff.
6. `20260603-hphys0278-output-unit-metadata-registry-alignment-001`
   - Align hillslope/watershed output metadata with registry authority.
   - Preserve legacy output names and values.
7. `20260603-hphys0279-sc-contract-unit-compliance-lint-001`
   - Add linting for `SC-*` unit sections, alias-map unit checks, and registry
     cross-links.
   - Record remaining contract gaps as explicit HOLDs.

## Acceptance Across The Queue

- New unit-bearing runtime surfaces must declare canonical units in contracts,
  registry, tests, and output metadata when published.
- Unit conversions must be named, directional, provenance-backed, and tested.
- Dimensional high-risk surfaces should use typed `BoundaryValue` variants or
  documented exceptions.
- Future work packages should fail their gates when they introduce an
  unregistered dimensional symbol, a raw conversion literal, or metadata drift.

Ran: not-run; this remediation queue is static governance evidence.
