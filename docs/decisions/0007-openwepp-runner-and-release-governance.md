# ADR-0007: openWEPP owns runner boundary and release metadata contract

**Status:** Accepted  
**Date:** 2026-05-11  
**Deciders:** Roger Lew, Codex

## Context

openWEPP currently defines high-level engine architecture but does not yet lock
the executable boundary contract needed for a clean operational break from
legacy `wepp_runner` behaviors (implicit fallback, legacy-first assumptions,
and mixed naming conventions).

The project needs:

- a dedicated openWEPP launcher boundary,
- mandatory sidecar metadata,
- deterministic binary naming and lint gates,
- explicit engine selection at caller boundaries.

## Decision

1. openWEPP owns its own launcher contract via `openwepp_runner`.
2. openWEPP release binaries follow `openwepp_YYMMDD*` naming with role
   suffixes for hillslope (`_hill`) and replay (`_replay`).
3. JSON sidecars are mandatory for every openWEPP binary artifact.
4. Sidecar validation is blocking in release gates.
5. Engine selection is explicit (`legacy_wepp`, `openwepp`) where both engines
   are available.
6. No silent fallback between engine families or pass-family contracts.

## Consequences

- openWEPP and legacy WEPP can coexist during transition without hidden
  contract drift.
- Missing/invalid sidecars are surfaced early instead of defaulting to legacy.
- Downstream integration work in wepppy is simplified to an explicit
  engine-selection boundary.
- Release governance gains deterministic, machine-checkable packaging criteria.
