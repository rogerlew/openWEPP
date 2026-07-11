# WSHED-W11D Hourly Routing Numerical Defect Closure

Status: `EXECUTED-COMPLETE`

Package ID: `20260710-wshedw11d-hourly-routing-numerical-defect-closure-001`

Queue row: `WSHED-W11D`

Execution mode: `package-end-to-end`

Evidence mode: `Static + Ran`

## Objective

Close W11C defects `W11C-F001..F004` end-to-end: eliminate material negative
channel storage and generated terminal volume on hourly KW/static-MC routes;
adjudicate and correct passive-route MC peak amplification/timestep behavior;
publish true terminal volume/sediment for legacy event-scalar serial networks;
and reconcile canonical `chan.inp nchnum=0` parsing with its declared valid
output-disabled semantics.

This is a DC-ExecPlan. It diagnoses internally and lands contract-first
production corrections when the seven-gate bar is met. It may not relay an
in-envelope mechanism into another diagnostic-only package.

## Starting Evidence

WSHED-W11C produced the following real release-CLI observations:

- KW minimum storage `-65.192021 m3` and static-MC minimum storage
  `-210.400475 m3`, each exactly offset by terminal volume above external
  input;
- static-MC peak/input up to `1.152433` and variable-MC up to `1.549880`, with
  variable-MC early-spike peak changing `1.185839 -> 3.071519 m3/s` between
  3,600 and 600 second grids;
- legacy CREAMS 7,200 m3 input publishing 14,400 m3, element 1, and nonterminal
  sediment values in a two-channel serial network;
- the old three-line `nchnum=0` fixture compatibility-defaulting written
  `dtchr=600` to `60` seconds.

## Correction Authority Envelope

### Defects

1. `W11D-ROUTE-STORAGE-001` (`W11C-F001`): active hourly routing publishes
   material negative storage and terminal outflow above the only external input.
2. `W11D-MC-PEAK-001` (`W11C-F002`): passive static/variable MC routes publish
   material peak amplification and strong timestep divergence requiring
   authority-backed adjudication/correction.
3. `W11D-EVENT-PUBLICATION-001` (`W11C-F003`): non-interval serial networks
   publish sums of internal throughflows and first-channel identity instead of
   terminal-outlet volume/sediment.
4. `W11D-CHANINP-ZERO-001` (`W11C-F004`): parser line collection makes declared
   valid `nchnum=0` output-disabled payloads collapse to compatibility defaults,
   aliasing requested routing timestep evidence.

### In-scope authority and write set

- `SC-ROUTE-001`, `SC-SYSTEM-001`, and `SC-INFILE-CHANINP-001`.
- Pinned baseline `wshchr.for`, `wshpek.for`, `wshdrv.for`, and `wshinp.for` at
  `dac3c950d8b16cc73774bf5ce2e7e11f80baac70`.
- `crates/openwepp-watershed-orchestrator/src/lib_mod/kernel/hourly.rs` and
  module-local tests.
- `crates/openwepp-watershed-orchestrator/src/lib_mod/network_frame.rs`.
- `crates/openwepp-input-contract/src/parsers/chaninp.rs` and parser tests.
- `crates/openwepp-runner/tests/mt3_hbp_hourly_consumer_contract.rs` and
  watershed CLI behavior tests.
- The protected `tests/fixtures/watershed/p102-sediment-active/` wrapper's
  channel-routing selector, README provenance, and checksum manifest only, to
  replace its incidental now-inadmissible MC grid without changing the p102
  hillslope/HBP substrate or fixture acceptance purpose.
- This package, catalog, roadmap, and predecessor W11C handoff.

### Allowed correction classes

- Amend canonical invariants/guards and publication meaning before production
  edits.
- Correct KW/MC recurrence, state carry, grid-end storage accounting, or
  coefficient refresh only to canonical/pinned authority.
- Add typed failure if a configured recurrence is outside its authorized
  stability/domain envelope; do not silently clamp generated volume or peaks.
- Select actual terminal outlet channels for event-scalar publication and
  retain internal-throughflow diagnostics separately if required.
- Make `nchnum=0` canonical parsing explicit without fallback/default aliasing.

### Protected boundaries

- No empirical damping, peak clipping, negative-storage clamping, mass
  injection/removal, surrogate recurrence, or publication-only masking.
- No impoundment-hourly routing, HBP schema change, enriched particle timing,
  hillslope physics, wepppy orchestration, or unrelated output redesign.
- Missing or contradictory routing authority is a hold-for-authority boundary;
  implementation size is not.

## Conversion Rule

If a reproducible root cause lies inside this envelope and corrected behavior
is supported by canonical contracts, pinned baseline, or a contract-authorized
physical invariant, execute contract amendment, contract-derived failing tests,
pre-implementation gate, production correction, validation, dual review, and
disposition in this package. Do not stop at `HOLD` while those actions remain
possible in-envelope.

## Seven-Gate Bar

1. Reproduction: W11C release matrix and corrected sidecar bind every symptom.
2. Mechanism: localize each to recurrence/storage, publication selection, or
   parser record closure—not another variable name.
3. Ownership: mechanisms must lie in the declared contract/source write set.
4. Authority: confirm/amend canonical contract using pinned baseline and
   independent physical bounds before production edits.
5. Safety: no clamp, silent default, surrogate physics, or wrapper-only fix.
6. Testability: retain anti-alias cases where wrong internal-throughflow,
   compatibility-default, and negative-storage formulas differ numerically.
7. Validation: rebuild exact release CLI and rerun all W11C cases with
   independent terminal water/storage/sediment reconstruction.

## Acceptance Criteria

- Every valid W11C wave case has finite nonnegative storage within canonical
  roundoff tolerance and terminal volume does not exceed available external
  input plus authorized initial/baseflow sources.
- MC peak behavior is contract/pinned-authority consistent at 3,600 and 600
  seconds; any required stability rejection is typed and predeclared.
- Legacy `ipeak=2` two-channel output identifies channel 2 and publishes 7,200
  m3 terminal volume with independently closed sediment rather than summing
  serial throughflow.
- Canonical `nchnum=0` retains requested `dtchr`; malformed variants still fail
  or compatibility-default exactly as contract-authorized.
- Zero, uniform, early/late, spike/spread, protected W11B, full workspace,
  release, review, and verification gates pass.

## Phase Plan

1. Reproduce and complete operand/source/provenance maps.
2. Amend contracts and add failing contract-derived tests.
3. Record pre-implementation gate.
4. Implement direct production corrections.
5. Execute focused, release, comparator, conservation, and full gates.
6. Complete dual review, finding disposition, dual verification, and final
   disposition.

## Required Reading

Core: root `AGENTS.md`, `docs/codex_exec_plans.md`,
`docs/defect_closure_execplans.md`, `docs/work-packages/AGENTS.md`, package
catalog, and this package.

Conditional/on-demand: science-contract governance/profile/authoring procedure,
`SC-ROUTE-001`, `SC-SYSTEM-001`, `SC-INFILE-CHANINP-001`, ADR-0012, pinned
baseline files, W11C evidence, crate/test instructions, and local-CI guidance.
Exact tiers and bytes must be recorded before edits.

## Review and Subagent Authorization

Dual independent review and verification are mandatory. This package explicitly
authorizes subagent spawning/delegation to two bounded reviewer/verifier agents
and requires a `comparator_suite_runner` for heavy release/comparator/full gates.
Expected outputs are named package artifacts and compact log metrics; write
access is bounded to package artifacts for delegated roles.

## HOLD Legitimacy

Any HOLD must name missing/contradictory authority or a proven out-of-envelope
mechanism, cite evidence, list the in-envelope correction route considered, and
explain why it cannot close. Diagnostic uncertainty, effort, or a partial
publication wrapper are not legitimate boundaries.

## Security Impact

Expected `NONE`; parser and local CLI surfaces only. Preserve explicit paths,
typed validation, and no shell interpolation or external connectivity.

## Progress

- [x] Scaffolded from W11C defect evidence.
- [x] Required reading and source map complete.
- [x] Contract-first authority adjudicated.
- [x] Contract-derived tests fail before correction.
- [x] Production corrections landed.
- [x] Release/full gates and dual review/verification complete.
