# Manifest Cutover Plan

Status: executed-hold.
Evidence mode: Static + Ran.

## Handoff Item 4

Replace the manifest production provenance/checksum path with typed direct
publication projection in `DirectPublicationFrameCutover` mode.

## Required Evidence

- Current manifest compatibility-read inventory.
- Direct manifest projection schema and field mapping.
- Input checksum, output checksum, warning, direct-runtime counter, output
  policy, and provenance parity checks.
- Anti-alias fixture proving manifest fields are not read from compatibility
  provenance structures after cutover.

## Gate

BLOCKED. Production manifest cutover remains current-scope acceptance, but the
typed direct publication frame is not authoritative enough to drive manifest
provenance/checksums yet.

Static: the production path remains `write_hillslope_run_manifest` to
`build_hillslope_run_manifest`, using compatibility execution/provenance
structures. `build_manifest_text_from_direct_publication` is a candidate helper
only and is not the production manifest writer.
