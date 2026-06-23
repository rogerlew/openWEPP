# R7G Iterative Completion

Status: HOLD-R7G-FROST-STATEFUL-SUBSOLVER-REQUIRED.

Current active blocker:
`HOLD-R7G-FROST-STATEFUL-SUBSOLVER-REQUIRED`.

Package type: Defect-Closure ExecPlan / iterative R7G performance and fixture
closure continuation.

Defect IDs:

- `HOLD-R7G-SURFACE-FREE-ACTIVE-SNOW-PARTITION-AUTHORITY-ABSENT`
- any next R7G direct-production blocker exposed by the required loop, when it
  stays inside the authority envelope below.

This package follows `docs/codex_exec_plans.md`,
`docs/defect_closure_execplans.md`, `docs/work-packages/AGENTS.md`,
`docs/specifications/science-contracts/AGENTS.md`, and
`docs/architecture/array-native-runtime-specification.md`.

Subagent authorization: this package explicitly authorizes read-only reviewer
and verifier subagents for snow/frost authority, no-compatibility proof,
performance/profile evidence, fixture coverage, publication parity,
line-count governance, and final HOLD legitimacy review. Subagents may not edit
files. Findings are summarized in `artifacts/review-disposition.md` and
`artifacts/verification.md`.

## Purpose

The prior R7G package stopped at a fail-closed direct-production gate on full
H2637: production direct mode could not proceed because active snow partition
authority was not surface-free. This continuation owns that defect and the next
in-envelope blockers required to finish R7G.

R7G is complete only when same-binary H2637 direct default candidate reaches the
architecture performance target, protected outputs remain identity-clean, direct
runtime counters and scans prove no compatibility hot-loop authority, and the
fixture matrix covers the process families that can hide aliases.

## Non-Negotiable Terminal-State Rule

This package has exactly two honest terminal states:

1. `COMPLETE-R7G-PERFORMANCE-CLOSURE-FIXTURE-HARDENING`: same-binary H2637
   direct default reaches `<=10x` legacy WEPP, explicit direct remains
   byte/Arrow/metadata identity-clean, rollback compatibility remains
   identity-clean, direct hot-loop counters/source scans/profiles contain no
   compatibility authority, independent operand reconstruction passes for
   conservation-sensitive output families in scope, and fixture-matrix risk is
   documented.
2. `HOLD-R7G-<SPECIFIC-BOUNDARY>`: a blocker is proven outside this package's
   declared authority envelope, with command evidence, reduced mechanism,
   source location, contract/architecture boundary, attempted in-envelope
   corrections, dual-review agreement, and the first actionable next defect.

The following are invalid terminal reasons:

- the inherited R7G hold was reproduced;
- a guard marker changed;
- direct production exits `0` but performance was not measured;
- output parity is green but the direct default exceeds `<=10x`;
- performance is red without profile evidence;
- profile evidence names an in-envelope hot-loop blocker that was not fixed or
  rejected with concrete safety/authority evidence;
- a newly exposed snow, frost, publication, fixture, or performance blocker is
  named but not reduced and corrected when it lies inside this envelope;
- a fixture family is listed without pass/fail/residual-risk disposition;
- progress was made.

## Correction Authority Envelope

Observed starting violation:

- Full H2637 direct default candidate and explicit direct production fail
  closed before endpoint timing at
  `HOLD-R7G-SURFACE-FREE-ACTIVE-SNOW-PARTITION-AUTHORITY-ABSENT`.

In-scope defect mechanisms:

- false activation of snow coupling from sidecar presence rather than runtime
  SWE or thermally active day with projected controls;
- missing typed snow controls, persistent lane snowpack state, hourly winter
  forcing/rain-snow partition inputs, direct snow partition compute, state
  mutation, downstream operands, and shadow projection;
- missing typed frost carry/authority when it blocks the same R7G H2637 direct
  loop;
- HBP/WAT/PASS/loss/manifest publication operands for snow/frost-sensitive
  fields including `RM`, `Snow-Water`, snow coupling, and related storage;
- direct day-input construction and hot-loop overhead after snow/frost blockers
  are removed;
- fixture matrix gaps for active snow, sidecar presence/absence, active frost,
  breakpoint climate, PMET, irrigation when enabled, multi-OFE transfer ratios,
  nonzero erosion, and management transitions;
- line-count governance for touched Rust files, including splitting
  `day_input_and_helpers.rs` before substantial Rust expansion.

In-scope write set:

- `docs/work-packages/20260623-r7g-iterative-completion-001/**`
- `docs/work-packages/README.md`
- `docs/architecture/array-native-runtime-specification.md` only for R7G state
  or acceptance wording updates required by execution evidence
- `crates/openwepp-runner/src/hillslope/**`
- `crates/openwepp-runner/tests/**`
- `crates/openwepp-hillslope-orchestrator/src/direct_runtime/**`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/**` only to expose
  typed, surface-free helpers that preserve existing contract-authorized
  process math
- `crates/openwepp-hillslope-orchestrator/src/tests/**`
- existing comparison, fixture, or release-sidecar tools used for R7G evidence
- focused fixture additions under existing test fixture locations.

Allowed production edit classes:

- correct active snow/frost gate logic to match `SC-SNOWFREEZE-001` without
  weakening fail-closed behavior;
- move snow/frost state and controls into typed direct lane/day frames;
- extract or wrap existing baseline-authoritative snow/frost process math behind
  typed direct inputs, without changing equations or units;
- add typed direct operands and fail-closed guards for missing or invalid
  snow/frost publication authority;
- remove measured direct-mode overhead from map/string/allocation/compatibility
  work in the direct hot loop;
- add focused tests, source scans, comparison helpers, fixtures, and
  reconstruction checks;
- split files for line-count governance when touched code grows.

Protected boundaries:

- no production physics, units, or schema-meaning change without canonical
  `SC-*` authority;
- no compatibility WB13 rows, writeback surfaces, runtime symbol maps, scheduler
  requests, dense refreshes, dirty flushes, or map-backed snow partition helpers
  as production direct hot-loop authority;
- no sidecar-file presence as active snow trigger by itself;
- no silent fallback wrappers or canonicalize-and-proceed behavior for invalid
  snow/frost state;
- no default activation for release;
- no relaxing protected output parity, manifest provenance, or checksum gates.

## Required Iterative Loop

Repeat until terminal:

1. Reproduce the next R7G blocker or prove it by static/source evidence tied to
   the prior run artifact.
2. Record it in `artifacts/blocker-ledger.md` with command, marker, reduced
   mechanism, source location, direct operand/producer/consumer, authority, and
   in/out-of-envelope disposition.
3. If the blocker is in-envelope, implement the correction, add or update a
   focused regression/source-scan/fixture, and run the smallest relevant gate.
4. Rerun H2637 direct default candidate and explicit direct until they produce
   endpoints; compare protected outputs where artifacts exist.
5. When direct endpoints exist, run the same-binary R7G matrix: default-disabled
   compatibility, rollback compatibility, direct default candidate, and explicit
   direct production with seconds, RSS, us/OFE-day, legacy multiplier, binary
   hashes, and manifest counters.
6. If direct default exceeds `<=10x`, profile direct mode and fix every
   in-envelope hot-loop compatibility/allocation/string/layout blocker before
   rerunning the matrix.
7. Maintain fixture, reconstruction, no-compatibility, line-count, verification,
   and review artifacts as living evidence.
8. Before any `HOLD`, complete the premature-stop audit in
   `artifacts/review-disposition.md`; if any audit item fails, continue.

## Acceptance Gates

- H2637 same-binary matrix records default-disabled compatibility, explicit
  rollback compatibility, direct default candidate, and explicit direct
  production.
- Direct default candidate reaches `<=10x` legacy WEPP.
- Protected public outputs remain byte/Arrow/metadata identity-clean for direct
  default and explicit direct runs where outputs are produced.
- Direct manifests report `compatibility_edge_invocations = 0`.
- Source scans prove `DirectProductionDayInputBuilder` and direct snow/frost
  execution do not use `HillslopeWritebackSurface`, compatibility WB13 rows, or
  the map-backed `direct_publication_snow_liquid_partition` helper.
- Active snow sidecar-only and thermally active fixtures have focused
  disposition.
- Independent operand reconstruction is recorded for snow/frost-sensitive
  conservation outputs used as closure evidence.
- `cargo fmt --check`, package-relevant Rust tests, package-relevant source
  scans, `git diff --check`, and scoped Markdown lint pass, or a legitimate
  architecture hold is reached before full closure.

## Deliverables

- `artifacts/required-reading.md`
- `artifacts/blocker-ledger.md`
- `artifacts/snow-frost-authority.md`
- `artifacts/performance.md`
- `artifacts/profile-and-blockers.md`
- `artifacts/output-parity.md`
- `artifacts/fixture-matrix.md`
- `artifacts/operand-reconstruction.md`
- `artifacts/no-compatibility-proof.md`
- `artifacts/line-count.md`
- `artifacts/verification.md`
- `artifacts/review-disposition.md`
- `artifacts/worker-handoff.md`
- active kickoff prompt under `prompts/active/`

## Progress

- [x] Scaffold continuation package and artifacts.
- [x] Reproduce/reduce inherited active-snow blocker.
- [x] Correct in-envelope active snow authority blocker.
- [x] Rerun direct H2637 endpoint loop and disposition next blockers.
- [x] Run R7G same-binary matrix and output parity gates.
- [x] Profile/remediate direct performance blockers until `<=10x`.
- [x] Close with HOLD and preserve current state for follow-up frost
      architecture migration.
- [ ] Finalize fixtures, reconstruction, no-compatibility proof, review, and
      handoff. Superseded by HOLD; remaining closure must occur in the
      follow-up frost sub-solver package.

## Surprises & Discoveries

- The inherited active-snow hold was not only a false sidecar activation. H2637
  later exposed actual wet active-snow days, requiring typed hourly winter
  forcing, persistent snow carry, direct snow partition compute, liquid-event
  hyetograph correction, and snow publication projection.
- Once direct reached endpoint timing, protected-output parity exposed active
  frost as the next real blocker. Active-frost production authority has now
  been installed enough to run full H2637 with zero compatibility edges, but
  frost projection parity remains red.
- Direct endpoint performance with active frost passed temporarily
  (`89.88 s` against a `91.2 s` budget) before fine-layer carry preservation
  exposed the cost of retrofitting frost through request/symbol surfaces. The
  latest measured direct endpoint before the final no-material consumer patch
  was `195.27 s`, so performance is no longer proven green.
- Direct default and explicit direct are checksum-identical to each other, but
  not to compatibility for HBP/WAT/PASS.
- The package did substantial Rust expansion before completing the required
  line-count split; this remains a governance cleanup item for the next
  package, not a completed acceptance gate.
- The direct frost model needs a coupled stateful sub-solver with rich
  persistent lane state. Continuing to patch frost through
  `DirectFrostRunoffSurface` and one-day partitions repeatedly trades one
  parity residual for another projection or performance regression.

## Decision Log

- Sidecar presence is not an active-snow trigger. Production direct now
  activates snow coupling from runtime SWE or a thermally active day with
  projected snow controls.
- Typed snow partition authority is in-envelope and was implemented rather
  than held: inputs, direct compute, carry mutation, downstream operands,
  hydrology projection, and focused regressions were added.
- Same-day downstream EROD14 `qin` may consume a committed typed zero upstream
  `qout`; publication authority and qout handoff authority are separate.
- WB14/WB16 liquid-event hyetographs must be projected from raw rainfall to
  post-winter rain, then canopy interception, then routed snowmelt. Applying
  the interception scale directly to raw rainfall was the lane 1 day 1097
  negative-runoff blocker.
- Active frost is not parity-complete: the current production path still uses
  `DirectFrostRunoffSurface`/symbol-map frost authority and under-projects
  frost depth/frozen water versus compatibility on H2637. R7G cannot honestly
  close until frost/snow projection parity is reduced to byte/Arrow identity or
  a narrower legitimate boundary is proven.

## Outcomes & Retrospective

R7G is held at `HOLD-R7G-FROST-STATEFUL-SUBSOLVER-REQUIRED`.

The direct production executor reaches full H2637 with
`compatibility_edge_invocations=0`, but R7G did not meet its terminal gates.
The active-frost remediation loop proved that endpoint execution and narrow
performance wins are possible, but the current frost architecture is not viable
for closure:

- fine-layer liquid/frozen state must persist even when current-day frost
  material is zero;
- fine/shadow carry must not be interpreted as coarse active-water projection;
- no-material partitions must not strip residual water from coarse storage;
- patching those rules into one-day request/symbol surfaces regressed H2637
  direct runtime from the retained green `89.88 s` run to the latest measured
  red `195.27 s` run before the final consumer safeguard was measured;
- HBP/WAT/PASS parity remains red.

Current protected-output parity residuals include:

- WAT differs in `frozwt`, `frdp`, `Total-Soil`, `SoilWaterTotal`,
  `Snow-Water`, `RM`, runoff, transfer, ET, and related downstream fields.
- PASS differs in `runvol`, `sbrunv`, and `peakro`.
- HBP differs; loss and plot outputs are byte-identical.

The next work must migrate frost to a coupled stateful sub-solver with rich
persistent lane state, then rerun the R7G acceptance gates from that new
architecture. The follow-up package must treat R7G's partial patches as
evidence, not as the final design.
