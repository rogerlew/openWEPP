# Obligation-to-Test Map

| Obligation | Characterization |
| --- | --- |
| mirror configuration absent is a no-op | isolated `absent` child case |
| relative and ephemeral roots fail closed | isolated `ephemeral` cases |
| artifact/mirror aliasing fails closed | isolated `alias` case |
| symlinked path components fail closed | isolated child and direct component cases |
| parent and non-directory components fail closed | direct directory test |
| missing normal directories are created incrementally | success and direct cases |
| output ordering/paths preserve exact bytes | two-file success case |
| checkpoint publication is canonical and node-bound | success case exact canonical bytes/path |
| real executor/resume consumers remain unchanged | static consumer map plus deferred master qualification |

Static: no process-physics obligation applies. Because module tests materially
changed, ADR-0021 glue-tier aggregate and per-function region gates bind the
final changed-head measurement.
