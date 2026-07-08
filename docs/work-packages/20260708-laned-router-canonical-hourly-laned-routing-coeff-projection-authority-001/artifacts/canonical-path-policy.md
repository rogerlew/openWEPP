# Canonical Path Policy

Status: complete with projection hold.
Evidence class: Static.

## Policy

Hourly water balance plus Lane D active routing remains the preferred production
path when every scheduled lane has complete source-authorized static route
coefficients and the active/baseflow/watershed gates are otherwise satisfied.
The canonical input surface for new production physics should be
`ow-lanuse-1`, not a legacy datver plus an extra sidecar.

This package does not make Lane D active routing universal for coefficient-absent
legacy cropland. Because legacy-field projection was not ratified, the protected
legacy/off path remains the production behavior for no-coefficient runs.

## Post-Closure Consensus

After closure, the operator and Codex agreed that adding another runfile sidecar
or disturbed-class sidecar would increase operator-error risk. If a required
sidecar is forgotten, the same legacy management can run with different physics.
The preferred reduction in authority surfaces is:

- `ow-lanuse-1` becomes the canonical production datver for new openWEPP
  physics.
- WEPPpy is responsible for producing truthful native `ow-lanuse-1`
  management files, including embedded Lane D `routing_coefficients` from its
  Disturbed/native class coefficient table.
- openWEPP consumes explicit native operands from the management file and does
  not perform hidden legacy-field inference.
- Legacy datvers remain compatibility inputs and run through the legacy
  single/MOFE driver posture unless explicitly migrated to `ow-lanuse-1`.

## Retained Paths

The retained non-hourly, DC01-only, and non-Lane-D surfaces remain valid for:

- protected no-coefficient fallback;
- explicit disable/rollback selectors;
- legacy datver compatibility and legacy single/MOFE driver workflows;
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

Follow-on authority should shift from coefficient projection toward an
`ow-lanuse-1` canonicalization/migration package: ratify native datver
production eligibility, define legacy datver retention as compatibility only,
and queue WEPPpy producer migration plus openWEPP eligibility guards.
