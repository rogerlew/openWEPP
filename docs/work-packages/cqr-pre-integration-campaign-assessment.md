# Pre-Integration CQR Campaign Assessment

Status: `ACTIVE-LOW-ASSESSMENT`
Owner: maintainers
Baseline date: 2026-07-11 UTC
Roadmap ID: `CQR-PREINT-20260711`

## Objective

Burn down medium-or-greater complexity risk before opening a broader integrated
testing and validation campaign. Preserve focused, conservation, and consumer-
path evidence at each module checkpoint, then run comparator and full-workspace
closure once per tranche. "Before integrated testing" does not waive evidence;
the revised strategy aggregates expensive closure at the tranche boundary.

This document is the campaign ledger and living risk assessment. Four child
ExecPlans perform the work:

1. `cqr-high-risk-a-execplan.md` — terminal-pass hillslope routing, projection,
   publication, and primary runner paths (10 modules).
2. `cqr-high-risk-b-execplan.md` — terminal-pass erosion, watershed routing, conservation,
   and final output publication (10 modules).
3. `cqr-medium-risk-execplan.md` — contract boundaries, authoritative tooling,
   configuration, and validation references (13 modules), terminal-pass.
4. `cqr-low-priority-assessment-execplan.md` — active 12-module low-priority
   modules, classification/disposition, eligible cleanup, and final rerank.

Only one child ExecPlan and one module package may be active at a time.

The fixed raw baseline is
`docs/work-packages/cqr-pre-integration-campaign-baseline.md`. The binding
status, evidence, scaffold, coverage, no-action, non-deferral, line-count,
heavy-run delegation, and defect-transition rules are in
`docs/work-packages/cqr-pre-integration-campaign-execution-contract.md`. Every
child plan and module package incorporates that contract without weakening it.

## Baseline Evidence

The fresh workspace measurement was produced at commit
`14dcb022a86aa2e8921ab1154a6b8335e9ef0c26`. Production Rust source under
`crates/` is byte-identical between that commit and planning commit
`e320ab69044f45c2f8e8898519ae532da268f58e`; only documentation changed.

| Artifact | SHA-256 |
| --- | --- |
| `/tmp/openwepp-cqr-followup-final.lcov` | `a8ef30b6c6b748cdee3e5239bf74cabcab281fa9fa166e51dbc96bec979943b1` |
| `/tmp/openwepp-cqr-followup-final.json` | `53e7740029043b290f4e3d26bbf60e204d3df8ccd2cca78fc2b9ea2a4aa759e0` |
| `/tmp/openwepp-cqr-followup-final-crap.json` | `bb67da1bf31bdfabcbba156c0f176a8365a2c3be4ec2f1a801644d71a6862c56` |

The raw scan found 45 production module paths with 67 deduplicated CRAP rows
above 30. This assessment provisionally assigns 20 high, 13 medium, and 12 low
priority modules. These are risk assignments, not accepted ADR-0021 eligibility
dispositions. Each child plan must refresh source identity and obtain the exact
dual-reviewed symbol classification before suppressing any row.

## Campaign Classification

| Risk | Modules | Treatment before broader integration |
| --- | ---: | --- |
| High A | 10 | hard blocker; execute first |
| High B | 10 | hard blocker; execute after High A |
| Medium | 13 | execute after both high tranches |
| Low/provisional | 12 | classify every row; close eligible work or accept exact no-action dispositions |

The 12 low/provisional paths are dominated by formatter or diagnostic surfaces:

- `crates/openwepp-input-contract/src/parsers/frost.rs`
- `crates/openwepp-input-contract/src/parsers/phosphorus.rs`
- `crates/openwepp-input-contract/src/parsers/pmetpara.rs`
- `crates/openwepp-input-contract/src/parsers/tcr.rs`
- `crates/openwepp-input-contract/src/parsers/wepp_ui.rs`
- `crates/openwepp-legacy-bridge/src/hbp.rs`
- `crates/openwepp-legacy-bridge/src/sidecar.rs`
- `crates/openwepp-meteorology/src/error.rs`
- `crates/openwepp-runner/src/hillslope/snowbench.rs`
- `crates/openwepp-runner/src/hillslope/snowbench_coe_melt.rs`
- `crates/openwepp-sim-contract/src/symbols.rs`
- `crates/openwepp-watershed-orchestrator/src/lib_mod/network_frame.rs`

No path is pre-approved for exclusion. In particular, a formatter that carries
machine-read codes, validation priority, state, or control behavior is eligible.
The low-priority plan must apply ADR-0021 categories at exact symbol/line
granularity.

## Campaign Rules

Each fixed module receives exactly one reviewed implementation checkpoint or
one committed, source-bound `DISPOSITIONED-NO-ACTION` record under the binding
execution contract. An actionable checkpoint is cover-first and
behavior-preserving: prove the applicable ADR-0021 tier, line/region threshold,
75% per-function region floor, and complete applicable A–H/named obligation map
before decomposition. If existing tests do not close those gates,
characterization tests land first. Then decompose each eligible CRAP row to at
most 30 without changing formulas, units, ordering, typed errors, schemas, or
consumer meaning.

Raw and actionable CRAP counts remain separate. `E-SCIENCE`, `E-PRODUCTION`,
and CRAP-above-30 `R-INFRASTRUCTURE` rows are actionable. Only dual-reviewed
`R-OBSERVABILITY`, `R-IRREDUCIBLE-CRAP`, or closed-list `X-*` rows may leave the
actionable set. Dirty overlap is a sequencing conflict, not an exclusion.

If characterization discovers a semantic defect, the active CQR package must
not hide it inside mechanical work. Follow the execution contract's explicitly
authorized defect-closure transition, then remeasure the module. A defect holds
the child plan only at a documented authority/write-set boundary that passes a
hold-legitimacy audit.

The 2026-07-11 execution-model revision makes each child tranche the expensive
closure unit. Modules use compact focused records and one review by default;
workspace coverage, the non-target ratchet, full Rust gates, dual review, and
dual verification run once at tranche final. Exceptions and production control-
flow/publication changes still receive a second module review.

## Final Assessment And Follow-Up

The low-priority ExecPlan owns the campaign's closing assessment. From the
terminal commit of all four tranches it must run fresh workspace LCOV, JSON, and
CRAP; publish raw and actionable rankings; compare every original module with
its baseline and terminal disposition; and classify every newly surfaced row.

The closing recommendation is exactly `GO-INTEGRATED-VALIDATION` or
`HOLD-CQR-FOLLOWUP`. `GO-INTEGRATED-VALIDATION` requires:

- every original high and medium module has zero eligible CRAP rows above 30;
- every executed module has current tier/coverage/floor and complete applicable
  A–H/named obligation evidence;
- every low-priority row is either closed or has a current dual-reviewed
  disposition;
- no unresolved defect, coverage regression, conservation/publication gap, or
  dirty overlap remains;
- full workspace formatting, Clippy, nextest, deny, and diff/doc gates pass;
- the final raw/actionable assessment is committed and the worktree is clean.

If the recommendation is `HOLD-CQR-FOLLOWUP`, update `docs/ROADMAP.md` with a
finite follow-up queue. Group new work by authority and write set, not merely by
rank. Do not automatically expand one child plan or begin broader integrated
validation while a high/medium blocker remains.

## Progress

- [x] (2026-07-11 UTC) Capture the clean-commit 45-module raw baseline.
- [x] (2026-07-11 UTC) Provisionally classify 20 high, 13 medium, and 12 low modules.
- [x] (2026-07-11 UTC) Author four serialized child ExecPlans.
- [x] (2026-07-11 UTC) Receive three independent authoring reviews; initial
  recommendation `HOLD` with accepted remediation findings.
- [x] (2026-07-11 UTC) Complete remediation, two verification rounds, and all
  finding dispositions; unanimous `GO — AUTHORING READY` recorded in
  `cqr-pre-integration-campaign-authoring-review.md`.
- [x] (2026-07-12 UTC) Execute High A through `TERMINAL-PASS`: all 13 fixed
  rows removed, zero new identity, zero touched-module row above 30, full gates
  and dual terminal verification PASS.
- [x] (2026-07-12 UTC) Execute High B through its terminal PASS transition.
- [x] (2026-07-13 UTC) Execute Medium through `TERMINAL-PASS`: all 19 fixed
  rows removed, zero new identity, final census 13 rows/12 modules, full gates
  and both terminal reviews/verifications PASS.
- [ ] Execute Low/Assessment and publish the final recommendation.

## Revision Note

2026-07-11: initial campaign assessment authored from the final follow-up CQR
rerank. It preserves the 45-module raw census, separates risk from eligibility,
and makes final reranking plus follow-up classification a campaign exit gate.
