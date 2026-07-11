# WSHED-W11E Hourly Routing Sanity Rerun

Status: `EXECUTED-COMPLETE-SANITY-PASS-WITH-FINDING`

Package ID: `20260711-wshedw11e-hourly-routing-sanity-rerun-001`

Queue row: `WSHED-W11E`

Execution mode: `package-end-to-end`

Evidence mode: `Static + Ran`

This ExecPlan is a living document. `Progress`, `Surprises & Discoveries`,
`Decision Log`, and `Outcomes & Retrospective` remain current throughout
execution under `docs/codex_exec_plans.md`.

## Purpose / Big Picture

Repeat WSHED-W11C's real watershed-CLI numerical sanity characterization after
WSHED-W11D closed its four defects. The rerun preserves W11C as historical
before-correction evidence and produces a new current-tree verdict from the
actual downstream CLI and Parquet publications.

The observable result is a fresh debug and exact release execution of the
seven-test `mt3_hbp_hourly_consumer_contract` suite. It covers corrected KW and
CREAMS behavior, canonical zero-count `chan.inp`, admitted static/dynamic MC,
typed rejection of inadmissible MC grids, and the protected hourly HBP consumer.

## Objective

Determine whether current W11D behavior earns `SANITY-PASS`,
`SANITY-PASS-WITH-FINDING`, or `SANITY-FAIL` without changing production code,
contracts, fixtures, or test semantics. Compare current results with W11C only
as a before/after diagnostic; canonical W11D contracts remain the authority.

## Included Scope

The package runs the existing real-CLI suite in debug and against an explicitly
rebuilt release binary. It records KW at 3,600 and 600 seconds across zero,
early spike, early spread, uniform, and late spike forcing; CREAMS event-scalar
terminal publication; zero-count timestep retention; admitted 60-second static
and dynamic MC; typed rejection of active inadmissible 3,600/600-second MC; and
the protected hourly HBP consumer cases. It also runs focused, erosion-profile,
full-workspace, formatting, clippy, deny, documentation, review, and
verification gates.

## Excluded Scope

No production Rust, canonical contract, test, fixture, schema, routing
threshold, or output-format edit is authorized. No observed-hydrograph
calibration or universal physical validation claim is made. A newly reproduced
defect would produce `SANITY-FAIL` plus a defect-shaped handoff rather than a
silent test relaxation.

## Intended Write Set

- `docs/work-packages/20260711-wshedw11e-hourly-routing-sanity-rerun-001/**`
- `docs/work-packages/README.md`
- `docs/ROADMAP.md`
- the W11C and W11D handoff/disposition artifacts only if final cross-links
  require clarification

The existing `docs/dev-guide/06-history-and-performance.md` modification is
explicitly outside this package and remains untouched. No `.rs` file is in the
write set.

## Required Reading

Core reading is root `AGENTS.md`, `docs/codex_exec_plans.md`,
`docs/work-packages/AGENTS.md`, `docs/work-packages/README.md`, this package,
and the W11C/W11D package dispositions. Conditional reading is
`crates/AGENTS.md`, science-contract `AGENTS.md`, and
`docs/standards/local-ci-gate-selection.md` because the package executes Rust,
classifies canonical behavior, and selects heavy gates. On-demand authority is
`SC-ROUTE-001`, `SC-SYSTEM-001`, and `SC-INFILE-CHANINP-001` at their W11D
versions. Exact paths, triggers, bytes, and instruction chains are recorded in
`artifacts/required-reading-map.md`.

## Observation and Authority Boundaries

External HBP hourly water and sediment totals are the source operands. Terminal
runoff, peak, sediment, element identity, channel storage, and channel balance
are read from real CLI Parquet outputs. The writer's balance is supporting
self-consistency only. W11D's independent storage and final-slot vectors remain
the anti-tautological authority evidence; this rerun confirms the public
consumer continues to expose their corrected result.

Typed `WKERNEL-WS10-CHANNEL-E-003` rejection is the canonical successful
outcome for active MC configurations outside `INV-ROUTE-022`. Separate admitted
static and dynamic MC routes must execute, so the guard cannot pass vacuously.

## Acceptance and Classification

A `SANITY-PASS` requires all seven debug and exact-release tests to pass; all KW
and CREAMS public quantities to be finite and nonnegative; terminal element 2,
volume, sediment, and storage relationships to satisfy current assertions; the
printed KW/CREAMS zero rows to remain exact; all four MC zero controls to
execute with peak and outlet volume within `1e-12`; admitted MC to execute with
passive finite peaks and closed balance; every active inadmissible MC case to
retain typed E003 identity; and no unexplained material finding to remain.
Required root, erosion, documentation, review, and verification gates must also
pass.

`SANITY-PASS-WITH-FINDING` is allowed only for a bounded non-gate numeric
observation without a violated canonical invariant. Any executable, sign,
terminal-publication, typed-guard, consumer, or required closure failure is
`SANITY-FAIL`.

## Plan of Work

First scaffold this package and record the current authority/read map. Next run
the seven-test suite with the test-built CLI, capture W11C result/timestep rows,
and compare them with historical W11C values. Then delegate exact release,
erosion, full, clippy, and deny gates to the required heavy runner. Two bounded
reviewers independently assess numerical meaning, consumer legitimacy,
anti-tautology, security, line count, and gate non-deferral. After every finding
is dispositioned, the same agents reverify the final artifacts. Finally record
the verdict, remove the completed row from the forward-only roadmap, and update
the package catalog.

## Concrete Steps

From `/home/workdir/openWEPP`, run the focused suite with
`cargo nextest run -p openwepp-runner --test mt3_hbp_hourly_consumer_contract
--no-capture`. Build the exact binary with `cargo build --release -p
openwepp-runner --bin openwepp-cli-watershed`, then rerun the same suite with
`OPENWEPP_W11C_WATERSHED_CLI` set to its absolute path. Final commands are
`cargo fmt --check`, workspace clippy with warnings denied, erosion and full
nextest profiles, `cargo deny check`, scoped `markdown-doc lint`, and
`git diff --check`.

## Review and Subagent Authorization

Dual independent review and dual same-agent verification are mandatory. This
package explicitly authorizes subagent spawning/delegation to two bounded
reviewer/verifier agents for code/result/gate-legitimacy review; expected
outputs are `artifacts/review_agent_{a,b}.md` and
`artifacts/verification_agent_{a,b}.md`; write access is restricted to those
four files.

Subagent requirement: REQUIRED. This package explicitly authorizes and requires
spawning/delegation to `comparator_suite_runner` for the exact release suite,
workspace clippy, erosion profile, full profile, and deny. Expected outputs are
compact metrics plus `artifacts/gate-results.md`,
`artifacts/release-binary-provenance.md`, and bounded files under
`artifacts/logs/`. The parent must not substitute its own heavy run while the
authorized runner is available.

## Validation and Acceptance

All current-scope gates must be classified `PASS`, `FAIL`, `BLOCKED`, or
`NOT RUN`; any current `FAIL`, `BLOCKED`, or unjustified `NOT RUN` prevents
completion. Release provenance must bind path, hash, size, mtime, build command,
and consumer command. Review findings must be accepted, rejected, deferred, or
follow-up before verification. Completion requires no unresolved accepted
finding and a truthful terminal disposition.

## Idempotence and Recovery

All tests use generated temporary run directories and are safe to repeat. A
failed release build or test is rerun only after recording its mechanism. This
package never rewrites W11C history or relaxes W11D guards to obtain a pass.

## Progress

- [x] (2026-07-11 UTC) Direct user authorization received.
- [x] (2026-07-11 UTC) Fresh W11E package scaffolded without changing W11C.
- [x] (2026-07-11 UTC) Required-reading and authority map complete.
- [x] (2026-07-11 UTC) Fresh debug and exact-release sanity suites complete,
  both 7/7.
- [x] (2026-07-11 UTC) Heavy closure gates complete: erosion 319/319 and full
  1,693/1,693 on accepted reruns.
- [x] (2026-07-11 UTC) Dual review, finding disposition, and dual same-agent
  verification complete with no residual finding.
- [x] (2026-07-11 UTC) Final disposition and catalog/roadmap closeout complete.

## Surprises & Discoveries

- Observation: W11D already consolidated the post-closure consumer evidence
  into one seven-test binary, so no Rust test edit is necessary.
  Evidence: current test inventory contains W11C KW/CREAMS characterization plus
  four W11D zero-count, terminal CREAMS, rejected-MC, and admitted-MC tests and
  two protected hourly-consumer tests.
- Observation: the fresh debug suite emits no `W11C_FINDING`; every printed KW
  storage is nonnegative and every printed peak/input ratio is at most one.
  Evidence: 7/7 run `7431c048-2070-4ddd-bf50-1fc5d09f17c4`, 15 result rows.
- Observation: corrected KW remains materially timestep-sensitive for spike
  forcing even though every canonical sign, volume, passive, and closure gate
  passes.
  Evidence: early peak `0.999951840 -> 1.999993817 m3/s`; late storage
  `65.473952630 -> 110.260168180 m3` from 3,600 to 600 seconds.

## Decision Log

- Decision: create W11E rather than mutate or reopen W11C.
  Rationale: W11C's failed metrics are required before-correction provenance;
  a new package makes the post-closure verdict independently auditable.
  Date/Author: 2026-07-11 UTC / Codex.
- Decision: interpret typed E003 as a passing result only for contract-
  inadmissible active MC grids and require admitted MC execution separately.
  Rationale: this matches `INV-ROUTE-022` and prevents rejection-only vacuity.
  Date/Author: 2026-07-11 UTC / Codex.
- Decision: classify the redo as `SANITY-PASS-WITH-FINDING` when all remaining
  gates pass, due to W11E-F001 KW timestep sensitivity.
  Rationale: the response is material and deserves explicit future authority
  adjudication, but no current canonical invariant or acceptance gate fails.
  Date/Author: 2026-07-11 UTC / Codex.

## Outcomes & Retrospective

Fresh debug and exact release consumers both pass 7/7. The redo confirms W11D's
four defect closures through the public CLI without changing code, contracts,
tests, or fixtures. Heavy format/clippy/erosion/full/deny gates pass. Review
corrected the proposed classification to `SANITY-PASS-WITH-FINDING` because KW
spike responses remain materially timestep-sensitive despite satisfying every
current canonical invariant. Both same-agent verifications pass with no
residual finding, and lifecycle closeout is complete.

## Security Impact

Expected `NONE`. The package adds documentation and runs existing local tests
and repository binaries. It adds no network access, dependency, secret, shell
interpolation, path resolver, production debug hook, or fixture mutation.
