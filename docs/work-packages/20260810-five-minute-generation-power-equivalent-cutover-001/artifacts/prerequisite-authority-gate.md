# Prerequisite Authority Gate

Status: `PASS — predecessor terminally closed`

Evidence mode: `Static + Ran`

## Required predecessor lifecycle

Static: the required predecessor
`20260809-hourly-peak-runoff-authority-closure-001` is terminally closed:

- its `package.md` says `Status: complete — Critical closure PASS`;
- its `artifacts/disposition.md` says `Status: PASS — terminally closed`;
- its reopened exact-source full workspace run passed 2,346/2,346 tests;
- its authority, source guard, anti-evasion, formatting, dependency, and
  documentation gates pass; and
- two fresh independent terminal verifiers returned `PASS`.

The source and decision authority themselves are now aligned. Current
ADR-0036, `SC-WATBAL-001`, `SC-SED-001`, and `SC-INFILE-HBP-001` state that
WB16/public peak is the maximum hourly mean, public area conversion occurs
once, HBP derives peak from hourly water, and no independent analytical peak
is native authority. This agreement does not substitute for the package's
explicit terminal-lifecycle prerequisite. The lifecycle evidence now does.

## Current focused execution

Ran from `/workdir/openWEPP` at source
`c9f28a7dbe7adf69d8e6d54ebd8da57568af5552` with a clean starting worktree:

    TMPDIR=/home/workdir/openwepp-task-tmp \
      cargo nextest run --test peak_hourly_authority_contract

Result: `PASS`, 4 tests passed, 0 skipped; nextest run ID
`5b418d4a-ae58-416d-baa3-a0bcbc9558d3`.

The predecessor's admitted exact-source closure receipts are recorded in its
`artifacts/gate-results.md`, including run ID
`64cd5e97-d253-4da1-a3cf-3c4e16f83d22` for the 2,346/2,346 full pass. Its
fresh verifier receipts are `artifacts/verification_agent_a.md` and
`artifacts/verification_agent_b.md`.

Disposition: `PASS`. The declared prerequisite is closed. Shared contract and
production edits within this package's authorized water-output write set may
begin after Milestone 1 baseline identities are frozen. This does not authorize
the prospectively rejected erosion candidate or any production erosion cutover.
