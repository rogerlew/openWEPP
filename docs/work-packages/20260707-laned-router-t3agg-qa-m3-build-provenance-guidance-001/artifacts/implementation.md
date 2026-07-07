# QA-M3 implementation evidence

Status: **EXECUTED** (2026-07-07). Evidence mode: **Static**.

## Finding Closed

Parent finding: `20260706-laned-router-t3-aggressive-deficit-carry-001`
`artifacts/review-disposition.md`, QA-M3:

- The stale-binary near-miss was documented in the T3-AGG package.
- The H2637 timing recipe already received package-local mitigation.
- The remaining Codex-owned task was to promote the evidence-build rule into
  durable AGENTS.md guidance.

## Changes Made

- `docs/work-packages/AGENTS.md`: added a release-binary evidence provenance
  rule for packages that invoke release CLI binaries for timing, comparator, or
  acceptance evidence.
- `crates/AGENTS.md`: added the runner-CLI release-build rule to the Rust
  validation checklist.
- `tools/local_ci/README.md`: added copyable build/provenance commands for
  `openwepp-cli-hill` and related runner bins.
- `docs/work-packages/README.md`: added this executed package to the current
  package catalog.

## Closure Statement

QA-M3 is closed for the durable-guidance remainder: future package authors and
Rust/timing workers now encounter the rule in both the work-package playbook and
the Rust crate playbook, with operational commands in local-CI documentation.
