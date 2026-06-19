# PERFDEEP04 - Profile PERFDEEP03 Lane-Dense No-Go

Status: executed 2026-06-19. Verdict:
`PROFILED - cut PERFDEEP05 at lane-dense sync removal`.

Package type: performance characterization / next-package decision.

## Objective

Profile the current PERFDEEP03 opt-in lane-owned dense H2637 path after its
`1147.96 s` endpoint no-go, identify the dominant remaining runtime costs, and
produce a ranked follow-up recommendation. This package does not implement the
next optimization; it creates the evidence needed to decide it.

## Rationale

PERFDEEP03 corrected the PERFDEEP02 ownership bug but still failed the
load-bearing endpoint gate:

```text
PERFDEEP01 reference: 669.97 s
PERFDEEP03 opt-in:    1147.96 s, 229580 KB
```

The next step is profiling, not default activation, wholesale revert, or blind
island expansion. The key question is whether the remaining wall time is still
dominated by logical/indexed compatibility edges, dense fallback access, output
publication surfaces, or a different hotspot.

## Scope

In scope:

- scaffold this work package and make it discoverable;
- run real profiling on the current PERFDEEP03 opt-in H2637 path;
- capture profiler command lines, tool availability, elapsed/RSS context, and
  top-symbol evidence;
- classify the main cost centers;
- recommend the next implementation package boundary.

Out of scope:

- production code optimization;
- science/numeric changes;
- output schema changes;
- default activation of PERFDEEP03;
- deletion of logical/indexed hot-path surfaces.

## Required Reading

- `AGENTS.md`
- `docs/work-packages/AGENTS.md`
- `docs/work-packages/20260619-perfdeep03-persistent-lane-owned-dense-state-001/package.md`
- `docs/work-packages/20260619-perfdeep03-persistent-lane-owned-dense-state-001/artifacts/perfdeep03_disposition.md`
- `docs/architecture/array-native-runtime-specification.md`
- `docs/ROADMAP.md`

## Intended Write Set

- `docs/work-packages/20260619-perfdeep04-profile-perfdeep03-lane-dense-no-go-001/**`
- `docs/work-packages/README.md`
- `docs/ROADMAP.md`
- `docs/architecture/array-native-runtime-specification.md` only if the profile
  changes the binding perf-program direction.

No Rust production edits are in scope.

## Phase Plan

1. Scaffold package files, runfiles, and active prompt.
2. Confirm profiler availability and release binary status.
3. Run a real PERFDEEP03 opt-in H2637 sampling profile using `perf`.
4. Generate profiler summaries from the captured sample.
5. Classify hotspots by runtime mechanism and compare against PERFDEEP03's
   suspected edge/fallback explanation.
6. Record package gate results, final disposition, and the recommended next
   work-package boundary.
7. Run markdown lint over the package and touched indexes.

## Acceptance Criteria

- Package scaffold exists with `package.md`, `prompts/active/`, `artifacts/`,
  and runfiles.
- Profiler availability is recorded.
- PERFDEEP03 opt-in H2637 profile evidence is captured and summarized.
- The final artifact ranks the top runtime mechanisms and states what the next
  package should and should not do.
- If full profiling is blocked by tool permissions, disk, or runtime failure,
  the package closes `HOLD` with the exact blocker and a reproducible command.
- Markdown lint passes for the package and touched docs.

## Execution Result

PERFDEEP04 captured matched `perf record` profiles for the PERFDEEP03 opt-in
H2637 path and the default-disabled H2637 path.

Opt-in lane-dense profile:

```text
perfdeep04_h2637_optin_perf_record  1164.31  519160
61248 samples, 0 lost samples, 492.932 MB perf.data
```

Default-disabled comparison profile:

```text
perfdeep04_h2637_default_perf_record  704.82  320640
37051 samples, 0 lost samples, 298.207 MB perf.data
```

Primary finding: the PERFDEEP03 no-go is dominated by lane-dense
resynchronization, not by dense array arithmetic. The opt-in profile reports
`HillslopeLaneDenseState::sync_from_writeback_surface` at `33.49%` inclusive
(`14.19%` self), absent from the default-disabled profile. That sync path calls
`HotSymbolTables::hot_state_symbols`, allocates vectors, performs symbol/map
fallback lookups, clones slot id lists, and then repopulates slots from the
logical/indexed surfaces. It reintroduces the compatibility edge the lane-owned
state was supposed to remove.

Recommended next package: remove daily lane-dense full resync and make transfer
application/dense carryover update the dense state directly, with cached hot slot
metadata and no symbol-id lookup in dense writeback apply.

## Deliverables

- `artifacts/perfdeep04-profile-results.md`
- `artifacts/perfdeep04-next-package-recommendation.md`
- `artifacts/perfdeep04-gate-results.md`
- `artifacts/perfdeep04_disposition.md`
- `artifacts/raw/` profiler summaries and command outputs

## Subagent Requirement

None.

## Autonomy

Execute end-to-end. Do not stop after scaffolding. Do not patch production code
inside this package. If the sampled profile clearly identifies a follow-up
implementation boundary, record it as the package output rather than starting
that implementation here.
