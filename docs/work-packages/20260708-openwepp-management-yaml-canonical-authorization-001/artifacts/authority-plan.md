# Authority Plan

Status: scaffolded.

Authority steps:

1. Decide whether to create `SC-INFILE-MANAGEMENT-YAML-001` or amend
   `SC-INFILE-MANAGEMENT-001`.
2. Ratify YAML as a first-class native management input surface, not a sidecar.
3. Ratify extension policy: producers emit lowercase `.yaml`, default migrated
   flat sources to `.man.yaml` naming, consumers accept `.yaml`, `.YAML`,
   `.yml`, and `.YML`.
4. Amend management-lanuse authority if needed so `ow-lanuse-1+` can be carried
   by canonical YAML rather than only flat `.man`.
5. Amend `SC-OFEROUTE-001` only where needed to recognize YAML route
   coefficients as equivalent canonical explicit operands.
6. Record that legacy flat `.man` remains source-only for native producer
   evolution and that no native flat writer is required.

No package-local artifact may replace canonical contract authority.
