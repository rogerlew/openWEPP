# Pre-Implementation Intent Plan

Evidence class: `Static`

Base commit: `497d76d0c29d2f711f4b0ac3f63454960793fe97`

Boundary: documentation-only authority increment.

## Intended Change

1. Adopt ADR-0041 as the canonical decision separating blocking TESTGATE
   correctness admission from optional observational coverage/CRAP QA.
2. Amend ADR-0021, ADR-0039, and ADR-0040 only where they make quality metrics
   transition-blocking or require their automatic execution.
3. Align root/package instructions, testing strategy, quality/refactor
   standards, CQR guidance/templates, and operator documentation.
4. Prospectively amend conflicting active predecessor package acceptance while
   preserving every historical run and receipt verbatim.

The declared write set was prospectively expanded before authority edits to
include `prompt-wording-guidance.md`, `local-ci-gate-selection.md`, the original
workflow-qualification package, and incompatible-recovery package after the
initial authority audit found direct contradictions in those paths.

## Preserved Authority

- Canonical `SC-*`, A0/A1/A3, conservation, reconstruction, consumer-path,
  anti-evasion, typed-guard, and selected science correctness gates.
- ADR-0021 metric formulas, percentages, symbol taxonomy, registry integrity,
  and CQR/test-enhancement package-local closure.
- Forest1 `LOCAL_UNTRUSTED` receipt acceptance and hosted verification
  separation.

## Release Decision

Campaign and release transitions do not require coverage/CRAP execution, a
current quality report, or an empty actionable set. An operator may request a
quality observation and attach it to retained evidence; its debt status remains
non-blocking. A separately authorized future decision is required to promote
any quality observation back into a transition gate.

## Gate Plan

- Documentation lint for every changed Markdown path.
- Link/path and package-shape checks.
- Source scan proving no remaining normative ordinary/campaign/release
  quality-blocking clauses in the declared write set, except explicit historical
  narrative and package-local CQR/test-enhancement requirements.
- Dual independent review, finding disposition, and dual terminal verification.
- No Rust, workflow, planner, executor, coverage, CRAP, TESTGATE, or QA
  execution: this order changes authority only.

## Terminal Reconciliation

Before disposition, compare the exact diff with the declared write set and this
plan. Any executable file change or quality-metric threshold/taxonomy change is
unauthorized and blocks closure.
