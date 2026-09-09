# Mechanical Refactor Authoring Guide

Status: Active. Behavior-preserving structural work uses the single package.md
record and review selection in ../work-packages/AGENTS.md.

## 1) Purpose
Move/split code safely with a small, reviewable record. Keep the authorized
package scope and cadence. DC work also follows ../defect_closure_execplans.md.

## 1.1 End-to-End Execution Requirement (Required)
Execute the seam change, applicable checks, corrections and disposition while
safe in-scope work remains. Required tests actually run; generic cost-saving
guidance does not substitute for applicable correctness evidence. Higher-priority
user/tool instructions still govern. Do not invent an ambient test-skip conflict.

## 2) What counts as a mechanical refactor
Runtime behavior, scientific formulas/constants/guards, floating-point operation
order and public surfaces stay equivalent unless an explicit authorized delta
changes the package classification. Scientific or fail-closed semantic changes
use contract-first governance and consequential review.

## 3) Authoring checklist for package.md and kickoff prompt
Record the seam, relevant source/destination modules, stable public surfaces,
selected validation and acceptance in package.md. No separate kickoff is required.
Adjust routine implementation paths inside the authorized boundary; reconcile
the actual diff. Avoid opportunistic cleanup. File length is a maintenance signal;
only explicitly owned size-reduction acceptance creates a size gate.

## 4) Tool usage guidance
Use rg for symbols/imports/test boundaries. Move cohesive blocks, preserving
signatures, visibility, comments, authority citations and module wiring.
Inspect imports and cycles. Run focused checks after meaningful moves; do not
repeat unchanged evidence after prose-only review responses.

## 5) Mechanical refactor patterns
Split by cohesive responsibility, keeping wiring small and exports stable.
Use a before/after public surface inventory when API parity is material.
For intra-function extraction preserve expression grouping, accumulation order,
short-circuit and error behavior. Metric work also uses
code-quality-refactor-authoring-guide.md. Put explanations in package.md;
machine inventories can remain separate evidence.

## 6) Compile and test execution strategy
Use focused cargo nextest tests and cargo check as appropriate during edits.
A build is not a behavior test. Reconcile the final diff and select applicable
formatting, lint, owned tests, consumers and manifest checks directly under
testing-and-gate-strategy.md. Critical changes and campaign/release boundaries
retain full correctness qualification. Coverage/CRAP is observational except for
explicit metric packages. The prospective bounded inherited-lint policy may
apply; never silently substitute it for an already-failed green requirement.

Record argv/cwd, source/input identity, result and evidence path. Missing required
evidence blocks completion; negative experiment results and inherited maintenance
debt do not erase actual measured facts. No automatic full-workspace run for an
ordinary bounded refactor, and no automatic runner agent for a long command.

## 7) Execution and review
Use the common guide: ordinarily one independent reviewer, two if consequential
semantics/authority are affected. The reviewer verifies accepted fixes. No fresh
terminal verification wave. State identity and actual checked evidence, meaningful
findings, corrections and limits in an attributable section of package.md.

## 8) Anti-patterns to avoid
Do not hide behavior changes in mechanical moves, mask dependencies with defaults,
claim tests not run, weaken failing scientific assertions, or refactor unrelated
code just to satisfy an inherited file-length threshold.

## 9) Required artifact set for mechanical refactor packages
The only maintained narrative is package.md: seam/public parity, implementation,
checks/results, independent findings/fixes, current state and disposition.
Retain raw evidence separately as needed. No line-count checklist, handoff,
prompt archive, separate gate report or review/verification placeholders.
Scientific obligations retain their evidence, not duplicate administration.

## 10) Acceptance criteria
The authorized seam is complete, public/numerical behavior is preserved, applicable
checks pass (or meet a prospectively legitimate bounded criterion), required
independent findings are resolved and no current invariant defect is hidden.
Record an actual out-of-scope/unavailable-evidence blocker when completion is
impossible; continue any safe in-scope correction.
