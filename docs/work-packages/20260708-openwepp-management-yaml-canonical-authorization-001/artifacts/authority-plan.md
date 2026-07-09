# Authority Plan

Status: executed.

Authority decisions landed:

1. Created sibling contract `SC-INFILE-MANAGEMENT-YAML-001`.
2. Ratified YAML as a first-class native management input document, not a
   sidecar.
3. Ratified extension policy:
   - producers emit lowercase `.yaml`;
   - default migrated flat sources append `.yaml`, giving `.man.yaml`;
   - consumers accept `.yaml`, `.YAML`, `.yml`, and `.YML` for dispatch.
4. Amended management-lanuse authority with `LANUSE-AUTH-8`: YAML is the
   canonical native producer document for `ow-lanuse-1+`; flat native `.man`
   remains a source/compatibility bridge.
5. Amended `SC-OFEROUTE-001` rev 50 to recognize YAML route coefficients as
   canonical explicit native management operands for Lane D authority.
6. Recorded that legacy flat `.man` remains source/compatibility input only for
   native producer evolution and that no native flat writer is required.

Canonical authority files:

- `docs/specifications/science-contracts/contracts/SC-INFILE-MANAGEMENT-YAML-001.md`
- `docs/contracts/openwepp-management-lanuse-authority-contract.md`
- `docs/specifications/science-contracts/contracts/SC-OFEROUTE-001.md`
- `docs/specifications/wepp-input-files/specs/management-yaml.spec.md`
- `docs/specifications/wepp-input-files/input-surface-registry.md`

No package-local artifact replaces canonical contract authority.
