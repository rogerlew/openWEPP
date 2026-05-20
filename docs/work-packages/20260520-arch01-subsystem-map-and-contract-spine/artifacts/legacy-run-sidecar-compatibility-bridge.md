# Legacy `.run` + `.txt` Sidecar Compatibility Bridge

Status: draft
Date: 2026-05-20 UTC
Evidence mode: `Static`

## Objective
Define initial backward compatibility for legacy stdin-driven `.run` plus
`.txt` sidecar flags/inputs while preserving strict validation and no-silent-
fallback behavior.

## Bridge Rules
1. Compatibility mode is explicit and discoverable.
2. Legacy `.run` + sidecar ingestion normalizes into the same typed internal
   run/config model used by schema-mode runs.
3. Missing required sidecars are hard errors.
4. Ambiguous mixed input mode is a hard error.
5. No silent defaulting for unresolved legacy flags.
6. Comparator and invariant checks operate on normalized typed state, not raw
   legacy text surfaces.

## Initial Sidecar Surface (seed list)
- `frost.txt`
- `wepp_ui.txt`
- Additional sidecars discovered via static legacy input-path analysis.

## Acceptance Criteria
- Compatibility ingestion can reproduce required run configuration fields for
  Tier-A comparator lanes (single OFE + daily water balance).
- Failure cases emit typed diagnostics identifying missing/invalid legacy
  sidecar prerequisites.

## Follow-on
- Promote a dedicated `.run` contract doc that formalizes mode selection and
  deprecation path from compatibility bridge to schema-first ingestion.
