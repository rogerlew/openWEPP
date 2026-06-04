# Unit Remediation Plan

Status: completed
Evidence mode: static

Static: HPHYS0279 intentionally stops at executable linting plus gap inventory.
The full `SC-*` contract set remains non-green and should be remediated in
larger contract-family work packages.

Recommended follow-up chunks:

1. `SC-INFILE-*` unit section remediation: add `Variables and Units` and
   `Symbol Alias Map` coverage for all 20 infile contracts.
2. Water-balance/snow/climate registry declaration and alias-map remediation:
   use the persisted inventory to add missing registered canonical symbols and
   registered boundary/publication alias rows for `SC-WATBAL-001`,
   `SC-SNOWFREEZE-001`, and `SC-CLIMATE-001`.
3. Soil and ET/subhyd registry declaration remediation: add missing registered
   symbols and aliases for `SC-SOIL-001`, `SC-EVAP-001`, and `SC-SUBHYD-001`.
4. Alias unit-check remediation: update rows flagged by `SCUNIT-E-008` so
   `Units check` text includes executable registry units.
5. Unit mismatch adjudication: resolve `SCUNIT-E-004` cases by either
   correcting contract units, correcting registry authority, or documenting
   separate canonical/publication symbols where the same legacy name is
   currently overloaded.

Ran: not applicable; plan derives from `tools/release/check_sc_unit_compliance.sh`
HOLD inventory.
