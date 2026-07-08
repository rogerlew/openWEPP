# Disposition

Status: scaffold review and YAML amendment disposition complete.

## Review Agent A

Source:
`artifacts/review-agent-a.md`

Verdict: GO-WITH-AMENDMENTS.

Disposition:

- Medium schema-finalization finding: accepted. Added an implementation-closure
  gate to the spec and package requiring final class-map, `--args-file`, and
  migration-report schemas before Rust closure. Added test obligations for
  class-map conflict detection, partial maps, and global-class admissibility.
- Low TOML example finding: accepted. Replaced the forest-looking example with
  a cropland rotation label so the example cannot imply legacy forest migration
  support.
- Low `--args-file` sidecar wording finding: accepted. Clarified that
  `--args-file` is a migration-time input, not a runtime sidecar, and is not
  needed after the output is written.

## YAML-Only Amendment

User direction after scaffold: freeze flat legacy management as an ingest-only
source reader, use canonical `ow-lanuse-1+` YAML for native output, carry no
obligation to support native flat management output, and add `--validate`.

Disposition:

- Accepted. The CLI spec now emits canonical management YAML only.
- Accepted. The package forbids native flat `.man` writing.
- Accepted. `--validate` is specified for native YAML and flat-source
  migratability.
- Accepted. A new YAML authorization package is now a hard implementation
  closure dependency.

## Extension And Crate Ownership Amendment

User direction after YAML scaffold: producers should emit only `.yaml`, default
flat management migration output should be `.man.yaml`, the `.man` portion is
informal, consumers should accept `.yml`, `.YML`, `.yaml`, and `.YAML`, and the
scaffold should assess crate ownership.

Disposition:

- Accepted. The CLI spec now derives omitted output paths by appending `.yaml`
  to the input path, yielding `.man.yaml` for flat `.man` inputs.
- Accepted. Producer output must end in lowercase `.yaml`; explicit `.yml`,
  `.YML`, or `.YAML` output paths fail closed.
- Accepted. The `.man` portion of `.man.yaml` is documented as informal
  provenance naming, not a consumer requirement.
- Accepted. Consumer extension acceptance is owned by the YAML authorization
  package.
- Accepted. Crate ownership assessment recommends a dedicated publishable
  `crates/openwepp-management-schema` crate shared by producer and consumer.

No unresolved scaffold findings remain.
