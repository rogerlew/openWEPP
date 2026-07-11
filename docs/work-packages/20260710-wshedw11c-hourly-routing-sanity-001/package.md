# WSHED-W11C Hourly Routing Sanity Characterization

Status: `EXECUTED-HOLD-SANITY-FAIL`

Package ID: `20260710-wshedw11c-hourly-routing-sanity-001`

Queue row: `WSHED-W11C`

Execution mode: `package-end-to-end`

Evidence mode: `Static + Ran`

## Objective

Characterize the numerical behavior of the real `openwepp-cli-watershed`
hourly channel-routing path after WSHED-W11B. Execute a controlled two-channel
matrix across the legacy event-scalar CREAMS control (`ipeak=2`), kinematic wave
(`ipeak=3`), static Muskingum-Cunge (`ipeak=4`), and variable-coefficient
Muskingum-Cunge (`ipeak=5`) at aligned hourly and sub-hourly channel steps.

The package answers three bounded questions with current executable evidence:

1. Does each selected branch run through the real CLI with paired hourly HBP
   water and sediment inputs?
2. Do water, storage, sediment, timing sensitivity, and degenerate-zero results
   close and remain finite?
3. Are branch and timestep responses qualitatively plausible, and what numeric
   findings require follow-up before physical-validation claims?

This is a characterization package. It may identify a defect-shaped finding,
but it does not amend contracts or alter production routing physics.

## Rationale

WSHED-W11B proved production activation, exact closure, and downstream
same-grid consumption. Its release two-channel `ipeak=4` spike case also
published a `2.112214620827 m3/s` outlet peak for a `2.0 m3/s` source pulse.
That is not by itself proof of a defect, but it warrants a branch/timestep sweep
and direct inspection of the typed interval series before treating the new lane
as physically sanity-validated.

## Included Scope

- A deterministic two-channel, no-impoundment real-CLI fixture.
- Paired 24-bin HBP hourly water and sediment inputs.
- `ipeak=2,3,4,5` branch matrix.
- `dtchr=3600 s` and `dtchr=600 s` for wave branches.
- Zero, early spike, early spread, uniform, and late spike hydrographs.
- Black-box Parquet reconstruction of terminal water, storage, balance,
  sediment, and peak response.
- Test-only structured result logging sufficient for independent review.
- Release-binary rerun with exact binary provenance.
- Focused, workspace, documentation, review, verification, and disposition
  evidence.

## Excluded Scope

- Production kernel, parser, HBP, writer, schema, or contract changes.
- New physical thresholds presented as canonical authority.
- Impoundment-hourly routing, enriched hourly particle composition, real-world
  calibration, or validation against observed hydrographs.
- A production hourly-output surface; diagnostics remain test-only.
- Defect correction. A reproduced authority-backed defect requires a separate
  defect-closure package or an explicit pre-implementation scope amendment.

## Intended Write Set

- `crates/openwepp-runner/tests/mt3_hbp_hourly_consumer_contract.rs`
- `docs/work-packages/20260710-wshedw11c-hourly-routing-sanity-001/**`
- `docs/work-packages/20260710-wshedw11d-hourly-routing-numerical-defect-closure-001/**`
- `docs/work-packages/README.md`
- `docs/ROADMAP.md`

No production `.rs` file is in scope.

## Required Reading

### Core

- `/home/workdir/openWEPP/AGENTS.md`
- `/home/workdir/openWEPP/docs/codex_exec_plans.md`
- `/home/workdir/openWEPP/docs/work-packages/AGENTS.md`
- `/home/workdir/openWEPP/docs/work-packages/README.md`
- this `package.md`

### Conditional

- `crates/AGENTS.md` and `tests/AGENTS.md` before Rust test edits.
- `docs/specifications/science-contracts/AGENTS.md` if a result is classified
  against canonical routing authority.
- `docs/standards/local-ci-gate-selection.md` before selecting final gates.

### On-demand

- `docs/specifications/science-contracts/contracts/SC-ROUTE-001.md`
- WSHED-W11B package evidence and release result artifacts.
- `crates/openwepp-watershed-orchestrator/src/lib_mod/kernel/hourly.rs`
- `crates/openwepp-watershed-orchestrator/src/lib_mod/kernel/hourly_tests.rs`
- `crates/openwepp-input-contract/src/parsers/chaninp.rs`

The exact budget and instruction chains are recorded in
`artifacts/required-reading-map.md`.

## Operand and Observation Boundaries

| Quantity | Units | Source | Role |
|---|---|---|---|
| HBP hourly runoff | m3/bin | serialized `V_h` fixture | authoritative external water input |
| HBP hourly sediment | kg/bin | serialized `S_h` fixture | authoritative external sediment input |
| input peak | m3/s | `max(V_h / 3600 s)` | diagnostic source-rate comparator |
| terminal runoff | m3 | `ebe_pw0.runoff_volume` | authoritative terminal publication |
| terminal peak | m3/s | `ebe_pw0.peak_runoff` | authoritative terminal publication |
| ending storage | m3 | `chanwb.Storage` | authoritative routed network storage publication |
| water balance | m3 | `chanwb.Balance` | supporting writer diagnostic |
| sediment yield | kg | `ebe_pw0.sediment_yield` | authoritative terminal publication |

Serialized-input reconstruction reparses both written HBP payloads before the
CLI launch and uses those totals as external water and sediment authority. The
routed water ledger then compares that input with terminal runoff, ending
storage, and explicit losses for the zero-baseflow/zero-loss fixture. Because
the production network storage writer derives storage from routed inflow and
outflow, exact ledger closure is an algebraic diagnostic, not independent
anti-tautological conservation proof. The result is not accepted solely from
the published `Balance` column and cannot override a physical-sign failure.

## Scenario Matrix

| Scenario | Hourly water | Hourly sediment | Purpose |
|---|---|---|---|
| zero | all zero | all zero | exact degenerate-state behavior |
| early spike | 7200 m3 at hour 6 | 240 kg at hour 6 | peak and drain-down response |
| early spread | 1800 m3 at hours 5-8 | 60 kg at hours 5-8 | equal-total attenuation response |
| uniform | 300 m3 each hour | 10 kg each hour | steady forcing response |
| late spike | 7200 m3 at hour 23 | 240 kg at hour 23 | end-of-day storage carry |

Wave branches run at `dtchr=3600 s` and `600 s`. The `ipeak=2` CREAMS
event-scalar control runs once because `chan.inp` timestep is not applicable.

## Acceptance and Classification Rules

Required executable gates:

1. Every matrix case exits successfully through the real watershed CLI.
2. Zero inputs publish exact zero terminal water, storage, peak, and sediment.
3. All published quantities are finite; physical mass/volume outputs are
   nonnegative.
4. The serialized-input routed water-ledger residual is within `1e-9` relative
   tolerance and the published channel balance is within the same tolerance;
   this diagnostic does not independently prove physical conservation.
5. The configured non-eroding fixture closes terminal sediment to input within
   `1e-9` relative tolerance.
6. Hourly wave branches distinguish equal-total spike and spread shapes;
   CREAMS early/late shifted equal-shape inputs remain event-scalar identical.
7. Early versus late forcing demonstrates the expected end-of-grid storage
   distinction.
8. Peak/input-peak ratios and `600 s` versus `3600 s` deltas are recorded and
   reviewed. No unsupported universal threshold is used to bless or condemn a
   branch.
9. Exact release binary provenance and release rerun results are recorded.
10. Dual independent review, finding disposition, dual verification,
    line-count governance, and all declared final gates are complete.

Physical classification vocabulary:

- `SANITY-PASS`: closure plus qualitative relationships are plausible and no
  unexplained material anomaly remains.
- `SANITY-PASS-WITH-FINDING`: executable/closure behavior passes, but a bounded
  numeric behavior requires follow-up before validation claims.
- `SANITY-FAIL`: a required executable or invariant gate fails.

The package may close characterization with `SANITY-PASS-WITH-FINDING`; it may
not describe that finding as physically validated or silently defer a failed
required executable gate.

## Phase Plan

### Phase A - Scaffold and pre-run design

- Record required reading, owned files, security posture, operand lineage, and
  the matrix above.
- Confirm no production edit is needed to execute the matrix.

### Phase B - Test implementation

- Extend the existing real-CLI hourly consumer fixture with explicit `ipeak`,
  `dtchr`, scenario, and binary selection inputs.
- Add structured per-case diagnostics, serialized-input reconstruction, and
  black-box publication assertions.
- Preserve the existing W11B protected tests unchanged in meaning.

### Phase C - Execute and investigate

- Run the focused debug-binary matrix.
- Build the exact release watershed CLI and rerun the matrix against it.
- Record output metrics, branch/timestep deltas, and any anomaly mechanism
  localized by existing typed interval tests or source inspection.

### Phase D - Closure

- Run formatting, clippy, focused/domain/full tests, deny, and Markdown lint as
  declared in gate evidence.
- Complete dual independent reviews, disposition every finding, then complete
  dual verification.
- Finalize disposition and worker handoff.

## Review and Subagent Authorization

Dual independent reviews and dual verification are mandatory. Every finding
must be dispositioned as `accepted`, `rejected`, `deferred`, or `follow-up`.
Closure is blocked while any required gate or finding is undispositioned.

Subagent authorization: this package explicitly authorizes subagent
spawning/delegation to two read-only reviewer/verifier agents for code, result,
gate-legitimacy, line-count, and disposition review; expected outputs are
`artifacts/review_agent_{a,b}.md` and `artifacts/verification_agent_{a,b}.md`;
write access is bounded to those four package artifacts.

Subagent requirement: REQUIRED for heavy closure runs. This package explicitly
authorizes and requires spawning/delegation to a `comparator_suite_runner`
agent for workspace clippy, full nextest, erosion-profile nextest, deny, and
release-binary execution; expected outputs are compact command metrics and log
paths; write access is bounded to package `artifacts/logs/` and gate evidence.

## Final Gates

- `cargo fmt --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- focused real-CLI W11C selector
- `cargo nextest run --workspace --profile erosion`
- `cargo nextest run --workspace --profile full`
- `cargo deny check`
- package/catalog/roadmap Markdown lint
- `git diff --check`
- dual review, finding disposition, and dual verification

## Security Impact

Expected impact is `NONE`: local deterministic fixtures, subprocess invocation
of a repository-built CLI, and local Parquet reads only. Review must confirm no
network use, secret handling, shell interpolation, or production debug hook.

## Progress

- [x] Package authorized by direct user request.
- [x] Scope and characterization boundaries declared.
- [x] Required-reading and owned-file intake complete.
- [x] Matrix implemented.
- [x] Debug and release matrices executed.
- [ ] Full gates complete.
- [ ] Dual review and verification complete.
- [ ] Final disposition recorded.
