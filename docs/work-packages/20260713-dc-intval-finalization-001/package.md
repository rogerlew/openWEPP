# Finalize Integrated Validation Through Iterative Defect Closure

Status: `QUEUED`

Package ID: `20260713-dc-intval-finalization-001`

Campaign ID: `INTVAL-FINAL-001`

Starting defect: `INTVAL-AUTH-BIND-001`

Execution mode: `package-end-to-end`

This is the terminal DC-style campaign package. Do not create another
single-gate successor while an exposed blocker can be corrected within the
authority envelope below.

For this execution, its iterative defect-transition rules supersede the
original campaign's instruction to open a separate DC package for each semantic
failure. Every original scenario, consumer/conservation obligation, gate,
fixed-source restart rule, review, and verification requirement remains
binding; only the package-routing cadence changes.

## Purpose

Iteratively close release, required-authority, stability, and integrated-
validation defects until the original integrated campaign reaches a verified
`PASS-INTEGRATED-VALIDATION`. The package owns diagnosis, authority mapping,
regression, implementation, rerun, review, and final campaign restart as one
continuous closure loop.

## Progress

- [x] (2026-07-13 UTC) Scaffold the terminal iterative DC campaign from the
  accepted provenance correction and complete missing-binding inventory.
- [ ] Restore the seven required-suite bindings as one coherent first batch.
- [ ] Iterate exact release candidates until authority, binaries, lint, and
  both stability suites pass without skips.
- [ ] Freeze the passing candidate and restart every integrated-validation
  phase; return to the correction loop for any in-repository blocker.
- [ ] Complete final gates, dual review/disposition, dual verification, final
  assessment, roadmap/catalog closure, and commit.

## Surprises And Discoveries

- Release fixes exposed multiple stale layers in sequence: threaded libtest,
  incomplete authority provenance, then seven active suites bound to five
  deleted targets. This package replaces one-layer successor churn with one
  bounded iterative closure campaign.

## Decision Log

- Decision: use focused red/green evidence per defect and heavy release/full
  gates per coherent candidate batch.
  Rationale: this keeps fast iteration while preserving exact terminal gates.
  Date/Author: 2026-07-13 / Codex.
- Decision: keep newly exposed in-repository defects in this package when the
  seven-gate DC bar authorizes correction.
  Rationale: an intermediate nonzero gate is work to close, not grounds for a
  piecemeal successor.
  Date/Author: 2026-07-13 / Codex.

## Outcomes And Retrospective

Queued. At terminal disposition record every correction candidate, exact
release and integrated scenario result, consumer/conservation evidence, review
findings, and the exact PASS or legitimate external/authority HOLD boundary.

## Known Starting Inventory

Seven active required suites bind five targets deleted together by
`a381702beca580fa10e71456a897f1a6a705a968`:

- FC, WP, and WATBAL suites ->
  `auth05_level4_constitutive_authority_hardening_contract`;
- withdrawal soil-water cap -> `hphys0224_wb19_withdrawal_soilwater_cap_contract`;
- layer-pool withdrawal cap -> `hphys0225_wb19_layer_pool_withdrawal_cap_contract`;
- saturated-thickness response ->
  `hphys0226_wb19_lateral_saturated_thickness_response_contract`; and
- FC/WP + COCA water yield ->
  `hphys0227_wb19_fcwp_coca_watyld_authority_contract`.

The first coherent correction batch restores all five current-authority test
targets and adds a guard proving every active required registry path exists and
is registered. It may use deleted tests as assertion provenance but may not
revive the deleted symbol-map runtime.

## Iterative Correction Authority Envelope

The package may correct blockers exposed by the exact release command or the
restarted integrated campaign in these repository-owned surfaces:

- release, stability, authority-integrity, and local validation tooling;
- required-authority test targets, Cargo registrations, test-only fixtures and
  provenance metadata, suite bindings, and anti-evasion guards;
- integration tests, runner/orchestrator tests, and package evidence; and
- canonical contracts and production Rust only after a newly exposed semantic
  defect satisfies the seven-gate DC bar and the intended write set is amended
  before implementation.

For every newly exposed blocker, record reproduction, named mechanism,
ownership, proximate authority, safety, red regression, and measurable
acceptance in `artifacts/defect-ledger.md`. Fix it in this package when those
seven gates are satisfied. Do not hand off merely because another gate fails.

Protected across all iterations: no skip flags, threshold/tolerance loosening,
fixture-result editing, retry-until-green, surrogate physics, silent
canonicalization, compatibility wrappers around deleted runtime, production
fallbacks, authority-suite deactivation, or weaker failure posture. A semantic
kernel correction must be contract-first and baseline-authoritative; read
science-contract governance before expanding into that branch.

## Campaign Strategy

Use focused red/green tests for each correction. Batch related fixes before an
expensive run. Run the exact pinned-input release command once per coherent
candidate, not after every file edit. Preserve each nonzero log, add the defect
to the ledger, correct it in-envelope, and restart that exact command from the
beginning.

When exact release passes required authority, binaries, release lint, and both
stability suites, freeze the candidate commit and restart every phase of
`20260713-integrated-validation-campaign-001` from Phase 0. If that restart
exposes a new defect, return to the same correction loop, produce a new frozen
candidate, and restart the campaign again. Never combine evidence across
pre-fix and post-fix candidates.

Heavy full-workspace/release gates run at coherent candidate boundaries.
Focused tests and affected profiles carry local iteration. This replaces the
piecemeal one-gate/one-package cadence without weakening terminal acceptance.

## Dependencies And Required Reading

Read root, work-package, test/fixture, DC-ExecPlan, local-CI, and applicable
science-contract guidance before each owned surface. Read the original
integrated campaign plus `INTVAL-REL-001` and `INTVAL-AUTH-PROV-001` evidence
before the first batch. For each newly exposed mechanism, add its source,
contract, fixture, and applicable nested `AGENTS.md` files to the package
reading map before edits.

## Required Gates

Each candidate runs applicable focused tests, anti-evasion, AUTH11, and the
exact no-skip release command with the pinned cohort/watchlist inputs and suite
counts recorded by `INTVAL-REL-001`:

    bash tools/release/run_release_candidate_gates.sh \
      --cohort-seeds-csv /workdir/wepp-forest/docs/work-packages/20260503-wb05b-forest-hillslope-closure-sweep/artifacts/audits/_meta/defect_seeds.csv \
      --watchlist-csv /workdir/wepp-forest/docs/ablation/hillslope_watchlist.csv \
      --expect-suite wb05b_1166=1166 \
      --expect-suite release_gate_watchlist=19

Verify `/workdir/wepp-forest` commit
`375ccc296ed1ea491f599ff1b1a25b415d494a2a` and input SHA-256 values
`42b7d827d842ecbe75843175a80ab4f67a097784156658df8fb849161eb98958` and
`42214345a228d27a0536b771dd73068dc897d369f54cb8a197457dea675e26ab`
before each release candidate. The terminal candidate additionally runs:

    cargo fmt --check
    cargo clippy --workspace --all-targets -- -D warnings
    cargo nextest run --workspace --profile full
    cargo deny check
    markdown-doc lint --path docs/work-packages/20260713-dc-intval-finalization-001 --path docs/work-packages/20260713-integrated-validation-campaign-001 --path docs/work-packages/README.md --path docs/ROADMAP.md
    git diff --check

It then executes the complete integrated scenario matrix, real-consumer and
independent-reconstruction evidence, dual review/disposition, and dual
verification required by the original campaign.

## Terminal Outcomes And HOLD Rule

The intended terminal outcome is `PASS-INTEGRATED-VALIDATION`, with exact
release and every restarted campaign row/gate passing at one frozen source.

`HOLD` is allowed only for an external or authority boundary that cannot be
corrected in this package: unavailable external evidence, missing or
contradictory canonical authority, proven invalid upstream inputs, or ownership
outside openWEPP. Before HOLD, publish a legitimacy audit proving why the broad
in-repository correction envelope cannot close the defect. Effort, another
failed gate, a deleted test, or need for additional source reading is not a
legitimate HOLD.

## Review And Evidence

Maintain a cumulative defect ledger, intended-write-set revisions, command
ledger, candidate/source identities, fixture hashes, release/stability logs,
integrated scenario matrix, conservation/consumer evidence, and final
assessment. Disposition every review finding as accepted, rejected, deferred,
or follow-up; any current-scope deferred/follow-up finding prevents PASS.

Apply Rust line-count governance and source-level anti-evasion guards to all
authority-suite posture or binding work. Reviews must check no old-path,
symbol-map, wrapper, threshold, fixture, or skip evasion carries closure.

## Subagent Authorization

Subagent requirement: **REQUIRED**. This package explicitly authorizes
subagent spawning/delegation for parallel read-only authority inventory,
mechanism-specific implementation within parent-assigned write sets, heavy
release/stability/full/integrated runs, two independent reviewers, and two
independent verifiers. Runner/reviewer/verifier writes are limited to named
package artifacts; the parent owns source and scope expansion decisions.

## Security Impact Gate

Preserve fail-closed validation, argument arrays, hashes, authority posture,
release lint, and stability behavior. No network, credentials, dependency
fallbacks, arbitrary output paths, or weaker security/serialization behavior
are authorized.
