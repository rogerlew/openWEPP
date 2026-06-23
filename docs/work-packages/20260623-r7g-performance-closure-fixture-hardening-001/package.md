# R7G Performance Closure And Fixture Hardening

Status: executed-held at
`HOLD-R7G-SURFACE-FREE-ACTIVE-SNOW-PARTITION-AUTHORITY-ABSENT`.

Package type: Performance-Closure ExecPlan with defect-closure continuation
rules.

This package follows `docs/codex_exec_plans.md`,
`docs/defect_closure_execplans.md`, `docs/work-packages/AGENTS.md`,
`docs/specifications/science-contracts/AGENTS.md`, and
`docs/architecture/array-native-runtime-specification.md`.

Subagent authorization: this package explicitly authorizes
spawning/delegating to read-only reviewer and verifier subagents for R7G
performance evidence review, fixture-matrix review, no-compatibility profile
review, operand-reconstruction review, and line-count governance review.
Expected outputs are compact Markdown findings summarized into
`artifacts/review-disposition.md` and `artifacts/verification.md`; subagents
may not edit files.

## Purpose

R7G closes the current array-native runtime against the architecture viability
target and hardens validation beyond the protected H2637 happy path. It starts
after R7F removed the counted production direct day-input hot-loop
compatibility edge, so fresh benchmark evidence is required.

The R7G gate is intentionally strict: production direct cannot be treated as a
default or release candidate until same-binary H2637 direct default reaches
`<=10x` legacy WEPP, protected outputs stay identity-clean, and fixture
coverage exposes the process families that can otherwise hide compatibility
aliases.

## Non-Negotiable Terminal-State Rule

This package has exactly two honest terminal states:

1. `COMPLETE-R7G-PERFORMANCE-CLOSURE-FIXTURE-HARDENING`: same-binary H2637
   direct default reaches `<=10x` legacy WEPP, explicit direct remains
   byte/Arrow/metadata identity-clean, rollback compatibility remains
   identity-clean, no compatibility authority appears in direct-mode hot-loop
   source scans/profiles/counters, independent reconstruction passes for
   conservation-sensitive output families in scope, and the fixture matrix is
   documented with pass/fail/residual risk.
2. `HOLD-R7G-<SPECIFIC-ARCHITECTURE-BLOCKER>`: the direct default misses
   `<=10x` after in-envelope measured blockers are remediated or rejected, and
   the remaining blocker is named with profile evidence, source location,
   measured cost class, protected-boundary rationale, and the first actionable
   next work-package objective.

The following are invalid terminal reasons by themselves:

- benchmark command setup was completed;
- one run mode passed;
- direct mode is faster than earlier direct packages but still above `<=10x`;
- output identity is green while performance is red;
- performance is red but no profile exists;
- profile exists but an in-envelope hot-loop compatibility/allocation/string
  blocker was not remediated or explicitly rejected with evidence;
- a fixture family is listed without pass/fail/residual-risk disposition;
- a known current-scope gate is deferred to R7H.

## Current Starting State

R7F completed at
`COMPLETE-R7F-DIRECT-DAY-INPUT-HOT-LOOP-ISOLATION`. Production direct now uses
`DirectProductionDayInputBuilder` in the day/OFE loop and manifests report
`compatibility_edge_invocations = 0` for the right reason.

R7E default-candidate mechanics are already present:

- default disabled resolves to compatibility rollback;
- `--direct-default-candidate` activates direct production through the default
  candidate policy;
- `--direct-production-executor` selects explicit direct production;
- `--compatibility-runtime` selects explicit rollback compatibility.

R7G must not reuse R7C/R7D/R7F timings as closure evidence. It must run fresh
same-binary H2637 benchmarks from current `main`.

## Correction Authority Envelope

In-scope mechanisms:

- direct production executor setup and hot-loop overhead;
- direct day-input construction, typed frame layout, allocation, string
  formatting, map/registry lookups, per-day/per-OFE publication capture,
  direct runtime counters, and no-compatibility source scans;
- default-candidate, explicit direct, and compatibility rollback mode
  selection evidence;
- benchmark scripts/commands and package-local artifacts;
- protected output parity checks for HBP/WAT/PASS/loss/plot/manifest;
- fixture matrix documentation and focused fixture/test additions that expose
  process-family aliases without changing physics;
- independent operand reconstruction for conservation-sensitive output
  families in scope;
- architecture/spec/catalog updates needed to reconcile R7F/R7G state.

Allowed production edit classes:

- remove measured direct-mode overhead that is clearly representational and
  in-envelope;
- preallocate or reuse direct hot-loop buffers when ownership is local and
  output identity is preserved;
- replace string formatting, symbol lookup, map/registry access, or
  compatibility adapter work in direct hot loops with typed direct fields;
- add fail-closed guards where a direct typed operand is missing or
  non-authoritative;
- add focused tests/source scans/fixtures for no-compatibility authority and
  anti-alias coverage;
- split files only when line-count governance requires it.

Protected boundaries:

- no physics, units, schema meaning, or process-order changes without
  contract-first authority;
- no default activation for release;
- no compatibility WB13 rows, writeback surfaces, runtime symbol maps,
  scheduler requests, dense refreshes, or dirty flushes as direct hot-loop
  authority;
- no silent fallback wrappers for missing direct operands;
- no relaxing protected-output byte/Arrow/metadata parity;
- no narrowing source scans/profiles to hide forbidden compatibility work.

## Required Iterative Loop

Repeat until a terminal state is reached:

1. Build the release binary once and record binary hashes.
2. Run same-binary H2637 default-disabled compatibility, explicit rollback
   compatibility, direct default candidate, and explicit direct production
   with wall seconds, RSS, us/OFE-day, and legacy multiplier.
3. Compare protected public outputs and manifests across default, rollback,
   direct default, and explicit direct.
4. If direct default misses `<=10x`, profile explicit/direct-default mode and
   record hot functions, allocation sources, string formatting, map/registry
   calls, compatibility symbols, and layout costs.
5. For every in-envelope blocker found by the profile/source scan, implement
   the correction, rerun focused validation, and return to step 2.
6. If a blocker is outside this package envelope, record a named architecture
   hold with profile evidence and a first actionable next package.
7. Harden or refresh the fixture matrix and record pass/fail/residual-risk for
   snow/frost, breakpoint climate, PMET, irrigation when enabled,
   multi-OFE transfer ratios, nonzero erosion, sidecar absence/presence, and
   management transitions.
8. Complete independent operand reconstruction for conservation-sensitive
   output families touched or relied on by R7G.
9. Run package validation, dual review/disposition, line-count governance, and
   final handoff.

## Acceptance Gates

- H2637 same-binary benchmark matrix records default-disabled compatibility,
  rollback compatibility, direct default candidate, and explicit direct
  production with seconds, RSS, us/OFE-day, and legacy multiplier.
- Direct default candidate reaches `<=10x` legacy WEPP, or this package holds
  with a named profiled architecture blocker and next package.
- Protected public outputs remain byte/Arrow/metadata identity-clean for
  current H2637 direct default and explicit direct runs.
- Direct manifests report `compatibility_edge_invocations = 0` and source
  scans show no direct hot-loop compatibility authority.
- Profile evidence exists for any direct run above `<=10x`, and every
  in-envelope measured blocker is dispositioned.
- Fixture matrix is documented with pass/fail/residual risk across required
  families.
- Independent operand reconstruction is recorded for conservation-sensitive
  output families used as closure evidence.
- Rust/doc closure gates and line-count governance pass, unless a legitimate
  architecture hold is reached before implementation closure.

## Deliverables

- Package artifacts:
  - `artifacts/required-reading.md`
  - `artifacts/performance.md`
  - `artifacts/profile-and-blockers.md`
  - `artifacts/output-parity.md`
  - `artifacts/fixture-matrix.md`
  - `artifacts/operand-reconstruction.md`
  - `artifacts/no-compatibility-proof.md`
  - `artifacts/verification.md`
  - `artifacts/line-count.md`
  - `artifacts/review-disposition.md`
  - `artifacts/worker-handoff.md`
- Active kickoff prompt in `prompts/active/`.
- Catalog and architecture updates reflecting R7F completion and R7G
  disposition.

## Security Impact

No secret handling or external I/O changes are intended. R7G touches runtime
selection, execution, performance evidence, and output validation only. It
must preserve fail-closed behavior, serialization safeguards, manifest
provenance, checksum validation, and rollback compatibility.

## Progress

- [x] Scaffold R7G package and artifacts.
- [x] Build current release binary and record hashes.
- [x] Run H2637 same-binary mode benchmark matrix.
- [x] Compare protected outputs and manifests where outputs were produced.
- [x] Profile and disposition direct-mode blockers when the direct default
      misses `<=10x`.
- [x] Remediate in-envelope blockers or prove named architecture hold.
- [x] Refresh fixture matrix and reconstruction evidence.
- [x] Run validation, review, line-count governance, and final disposition.

## Outcomes & Retrospective

Final disposition:
`HOLD-R7G-SURFACE-FREE-ACTIVE-SNOW-PARTITION-AUTHORITY-ABSENT`.

R7G scaffolded and executed the current same-binary H2637 matrix after R7F.
Default-disabled compatibility and explicit rollback compatibility both
completed and produced identical protected output checksum maps. Direct default
candidate and explicit direct production both failed closed before hot-loop
timing at the first full-H2637 active-snow gate:

`R7F typed production day-input path does not yet have surface-free active snow
partition authority for lane 1`

This is not a `<=10x` performance miss yet. It is a production direct
completeness blocker: the only existing snow partition helper still consumes
`HillslopeWritebackSurface` state/flux maps, while R7F correctly forbids that
path in the production direct hot loop. No output identity, profiling, or
release-readiness claim can close until typed direct snow partition state,
mutations, downstream operands, and publication projection exist.
