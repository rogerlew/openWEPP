# PL09A Precondition 2: Symbol Wiring Disposition (`lib.rs:33`)

Status: `complete`
Evidence mode: `Static + Ran`
Disposition: `cleared (clarified)`

Static:
- Concern: `PL_DECOMP_IMNGMT_SYMBOL` constant names a decomposition precondition
  but points at `pl_growth_slot_0001_crop_0001_imngmt`.

Ran:
- Audited symbol projection and dispatch usage in runtime/orchestrator code.

## Findings

1. `imngmt` is projected for slot/crop into schedule and growth channels, not
   into decomposition channel:
   - schedule projection: `pl_schedule_*_imngmt`
   - growth projection: `pl_growth_*_imngmt`
   - no `pl_decomp_*_imngmt` projection exists in current seam.
2. Decomposition dispatch uses `imngmt` only to derive management class
   selection for decomposition preconditions.
3. Therefore, current reference to growth-channel `imngmt` is consistent with
   current projection surfaces and is not a proven functional defect.

## Precondition Closure Decision

`cleared (clarified)`.

Action:
- Treat `lib.rs:33` as naming ambiguity, not a confirmed bug.
- Keep queue `PL10-active-slot-authority` responsible for replacing
  hard-coded `slot_0001/crop_0001` symbol coupling across growth/decomp.

## Residual Risk

- Naming ambiguity can mislead future edits; this is tracked under secondary
  finding acknowledgement as a valid concern.

## Evidence Links

- `/workdir/openWEPP/crates/openwepp-hillslope-orchestrator/src/lib.rs:33`
- `/workdir/openWEPP/crates/openwepp-hillslope-orchestrator/src/lib.rs:42`
- `/workdir/openWEPP/crates/openwepp-hillslope-orchestrator/src/lib.rs:532`
- `/workdir/openWEPP/crates/openwepp-hillslope-orchestrator/src/runtime_inputs.rs:984`
- `/workdir/openWEPP/crates/openwepp-hillslope-orchestrator/src/runtime_inputs.rs:993`
- `/workdir/openWEPP/docs/work-packages/20260522-pl09-openwepp-totality-and-pl08-hold-lift-discovery-001/artifacts/claude-pl09-pre-execution-review.md:261`
