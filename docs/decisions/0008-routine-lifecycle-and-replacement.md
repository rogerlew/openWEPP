# ADR-0008: Routine lifecycle states and replacement catalog

**Status:** Proposed  
**Date:** 2026-05-11  
**Deciders:** Roger Lew, Codex

## Context

openWEPP needs lifecycle flexibility for kernel evolution without destabilizing
run orchestration:

- add new routines without forked control flow,
- replace routines with explicit migration intent,
- deprecate/retire routines without deleting provenance.

Today, these lifecycle semantics are not yet pinned as architecture policy.

## Decision

1. openWEPP adopts explicit routine lifecycle states:
   `experimental`, `active`, `deprecated`, `retired`.
2. Every runnable routine has a stable `routine_id` and a machine-readable
   descriptor that declares contract version, capabilities, and lifecycle state.
3. Routine replacement is explicit via descriptor linkage:
   `replaces` and/or `replaced_by` metadata referencing `routine_id`.
4. Resolver behavior is deterministic:
   - default selection chooses `active` routines only;
   - `experimental` requires explicit opt-in;
   - `deprecated` is callable but emits lifecycle warnings;
   - `retired` is non-runnable.
5. Lifecycle transitions require evidence:
   - `experimental` -> `active`: contract completeness + regression/parity
     evidence;
   - `active` -> `deprecated`: replacement path documented;
   - `deprecated` -> `retired`: migration guidance and compatibility note.
6. Lifecycle metadata is retained after retirement for historical
   reproducibility and auditability.

## Consequences

- Routine evolution becomes data-driven instead of hard-coded in orchestration.
- Replacements and additions can be introduced without monolithic rewrites.
- Deprecation risk is surfaced early to operators and downstream integrators.
- Architecture remains open to future domain expansions (for example reservoir
  routing routines) through the same lifecycle/resolver mechanism.
