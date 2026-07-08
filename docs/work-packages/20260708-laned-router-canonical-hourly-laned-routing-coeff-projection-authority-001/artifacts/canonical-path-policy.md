# Canonical Path Policy

Status: complete with projection hold.
Evidence class: Static.

## Policy

Hourly water balance plus Lane D active routing remains the preferred production
path when every scheduled lane has complete source-authorized static route
coefficients and the active/baseflow/watershed gates are otherwise satisfied.

This package does not make Lane D active routing universal for coefficient-absent
legacy cropland. Because legacy-field projection was not ratified, the protected
legacy/off path remains the production behavior for no-coefficient runs.

## Retained Paths

The retained non-hourly, DC01-only, and non-Lane-D surfaces remain valid for:

- protected no-coefficient fallback;
- explicit disable/rollback selectors;
- legacy validation and comparator evidence;
- regression diagnosis;
- workflows whose coefficient authority has not been authored.

They are not a license to add new downstream consumer claims that bypass the
hourly Lane D path when route-coefficient authority is available.

## Implementation Consequence

M-T2B may implement groundwater/baseflow for single-OFE and Lane D MOFE, but it
must not assume coefficient-absent legacy cropland will become active by
default. It should preserve the current default eligibility:

- coefficient-complete lanes can use active hourly Lane D routing;
- no-coefficient lanes remain legacy/off;
- mixed coefficient authority fails closed;
- groundwater/baseflow outputs must distinguish disabled/missing authority from
generated zero values.
