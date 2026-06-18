# ADR-0024: Reference-implementation intent authority

**Status:** Accepted
**Date:** 2026-06-18 UTC
**Deciders:** Roger Lew (operator ratification), Codex (draft)
**Builds on:** [ADR-0011](0011-architecture-first-top-down-science-contracts.md),
[ADR-0017](0017-re-pin-operational-distrust-comparator-is-flag-not-target.md),
[ADR-0018](0018-defect-closure-execplans-conversion-rule.md)
**Work package:**
`docs/work-packages/20260618-refimpl-intent-authority-ksatadj-subhyd-001/`

## Context

Some openWEPP process families include empirical or conceptual models that do
not have a closed-form derivation in the WEPP technical chapters, a curated
field benchmark, or an independent solver authority. The forest disturbed-soil
`ksatadj` conductivity adjustment is the first active case: H2637 lateral-flow
equation and operand checks localized the remaining absolute-magnitude question
to the `ksatadj` equivalent-conductivity model, but no external physical source
defined its expected absolute magnitude.

ADR-0017 remains correct: legacy binary behavior and legacy output columns are
flags, not acceptance targets. That rule does not answer how to govern a model
whose useful authority is the algorithm encoded by the reference
implementation, rather than its output stream.

## Decision

An `SC-*` contract may use reference-implementation source intent as an `A0`
authority anchor for an empirical or conceptual model when no stronger
external physical authority is available.

This source-intent anchor is the static algorithm expressed by a cited
reference implementation, with file, routine, and commit provenance. It is not
the legacy binary output, a replay trace, or a comparator delta. The contract
must extract and name:

- the input symbols and units,
- the branch conditions,
- the formula or state transition,
- the required domain guards,
- the runtime or governance enforcement path, and
- any legacy bug or output artifact that is explicitly non-authoritative.

The anchor is a provenance basis for existing `A0` contract authority, not a
new rank in the authority ladder. It can close an authority gap only after the
resulting `SC-*` invariant is written in canonical contract form. If the source
contains contradictory, dead, disabled, or non-conserving behavior, the contract
must encode the intended algorithm when that intent is clear, or hold if it is
not.

The first application is the forest `ksatadj` effective-conductivity model in
`SC-SUBHYD-001`. Future applications such as `qdry` or `ksflag` require their
own source-intent extraction and contract amendments.

## Consequences

- ADR-0017 is preserved. Legacy binary behavior remains `A6` investigation
  evidence only.
- A process implementation can be judged `CORRECT` against a source-intent
  invariant even when the legacy binary output differs, provided the
  implementation satisfies the canonical contract and required guards.
- A process implementation that diverges from a newly ratified source-intent
  invariant is `OPENWEPP-DEFECTIVE` and routes to a defect-closure package under
  ADR-0018.
- Contract review and verification duties still apply to every `SC-*`
  amendment that introduces or changes an invariant.
