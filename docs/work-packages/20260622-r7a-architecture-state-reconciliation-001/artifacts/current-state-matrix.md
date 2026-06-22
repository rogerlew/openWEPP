# Current State Matrix

Evidence class: Static.

The architecture spec now includes two explicit current-state tables.

## Stage Matrix

Source: `docs/architecture/array-native-runtime-specification.md`, section
`8.2.2 Current Post-R6J State`.

| Family | Reconciled state |
|---|---|
| R0/R1 | Planning-only complete; production constructors remain open. |
| PERFDEEP09 | PERFDEEP07 hold lifted for R2+. |
| R2A | Direct runtime namespace/skeleton complete. |
| R3A-R3C | First complete direct spans complete. |
| R4A-R4P/Q/Z | Staged direct hydrology span coverage complete. |
| R5A-R5E | Canonical direct OFE-day lifecycle and 14-phase coverage complete. |
| R6A-R6I | Historical direct publication producer-authority blocker reduction. |
| R6J | Opt-in direct publication cutover complete. |
| R7 | R7A documentation-authority reconciliation complete; R7B-H open. |

## Runtime Mode Matrix

The spec now separates these modes:

- Compatibility mode: default API/CLI mode and rollback/replay authority.
- Shadow mode: validation mode where compatibility remains public-output
  authority.
- Direct publication cutover: explicit opt-in mode where direct publication
  artifacts write public outputs, but execution still originates from the
  compatibility simulation record.
- Production direct mode: future R7 target using parsed typed frames and
  `DirectFrameExecutor` without hot compatibility authority.

## Closure Disposition

The matrix prevents two false closure claims:

- R6J must not be treated as default activation.
- A direct publication object adapted from compatibility WB13 rows must not be
  treated as production direct-runtime authority for R7.
