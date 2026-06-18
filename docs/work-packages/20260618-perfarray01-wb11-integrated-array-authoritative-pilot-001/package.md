# PERFARRAY01 - WB11 Integrated Array-Authoritative Pilot (Stage A + B)

Status: scaffolded 2026-06-18 (first production code in the array-authoritative arc;
implements Stages A+B of the PERFARCH02 staged plan; governed by proposed ADR-0023)

Package type: **Production hot-path pilot — flag-gated, bit-identity load-bearing, floor
measurement + GO/NO-GO.** This is a step up in risk from the PERFIDX/PERFARCH scoping work:
it touches the kernel writeback contract and the scheduler. The default path stays
bit-identical (the pilot is behind a non-default flag); the pilot path must be proven
bit-identical *and* must measure the real integrated floor.

## Why this package exists — it answers the 5× question

PERFARCH02's floor prototype proved the *writeback/guard surface class* is ~49.9×
faster array-authoritative and identity-preserving — but it ran a **synthetic surface**, not
the integrated WB11 flow, so the H2637 floor was **not** measured and **5× stayed unproven**.
PERFARRAY01 closes that gap: port **one representative WB11 daily flow** to array-authoritative
state with **real** daily work, measure the actual per-OFE-day cost on H2637, and decide
whether ≤10× (and possibly 5×) is reachable before any broad migration. **This is the package
whose floor number is the real answer to "can we reach 5×."**

## Governing decision

Proposed **ADR-0023** (Array-Authoritative Hot-Path State, from PERFARCH02) governs this work
— it supersedes the *hot-path authority* portion of ADR-0022 while keeping the
`SymbolRegistry` / `SymbolId` / sorted-order / logical-export foundation. ADR-0023 is
**Proposed, not ratified**. PERFARRAY01's measured floor + identity evidence is the input the
operator ratifies ADR-0023 against — i.e. **ratify based on the real floor, not blind.** The
flag-gated, inert-by-default code may land for measurement; committing to the broad migration
(Stages C–F) is gated on the GO verdict **and** ADR-0023 ratification.

## Stage A — Contract Shell (no production flip)

Add the array-authoritative types **without** changing default execution:

- `ArrayHotState` (or equivalent): dense state/flux authority keyed by `SymbolId`.
- `ArrayWritebackField` / `ArrayWritebackPayload`: id + value + finite/range bounds.
- id-backed finite/domain evaluator preserving the **current message-id class** and
  fail-closed semantics; logical `BoundarySymbol` resolved **only on the failure path**.
- logical materialization APIs (`HillslopeWritebackSurface` view) for input / tests / debug /
  failure / publication boundaries.

Gates (Stage A): finite/domain parity unit tests vs `evaluate_kernel_writeback`; success-path
map-export identity; failure-path lazy-subject parity; **no production default flip** (flag
off ⇒ byte-identical to today).

## Stage B — WB11 Integrated Pilot (behind the flag)

Port **one representative WB11 daily flow** to array-authoritative execution — and it must do
the **real** daily work, not a writeback-only slice:

- runoff reconciliation (`hydrology_phase_runoff_reconciliation.rs`) as the anchor flow;
- frost/snow state access if active for the selected fixture;
- the **real typed guards / conservation checks** (not stripped);
- the scheduler apply step;
- outlet / publication materialization for the assertion.

### The two structural proofs (make-or-break — these are what killed the half-measures)

1. **No per-day full `BTreeMap` export at the kernel seam** (the PERFIDX03 trap). The pilot
   kernel must consume/produce the dense array directly; logical maps materialize only at the
   publication/validation boundary, once, not per OFE-day. **Prove it with perf evidence.**
2. **No normal-path logical + array dual-write** (the PERFIDX05 trap). The array is the single
   mutable authority on the pilot path; the `indexed_writeback_surface` mirror maintenance and
   logical dual-write are **not** on the normal pilot timing path. **Prove it with perf
   evidence.**

If the pilot cannot satisfy both, that is a **NO-GO signal**, not something to paper over.

## The floor measurement (the headline deliverable)

Measure the pilot's H2637 no-UI per-OFE-day cost on the **same machine** as PERFIDX06, and
extrapolate the implied floor / legacy ratio. The honest read:

- the PERFARCH02 surface (writeback/guard) was 0.657 µs/OFE-day — far below the 193 µs ≤5×
  budget. The integrated pilot now includes the **rest** (hydrology physics, conservation,
  scheduler, publication). The floor is what that integrated cost extrapolates to.
- ≤10× budget = 386 µs/OFE-day; ≤5× budget = 193 µs/OFE-day; legacy = 38.65 µs/OFE-day.

## Honest-measurement hard stop (continues from PERFARCH02)

The pilot must run openWEPP's **real** per-step work — actual conservation gates, typed
bounds, fail-closed guards, scheduler apply, publication — on array state. A pilot that strips
the guards/conservation to look fast measures a fictional floor and is worse than no
measurement. The pilot path's outputs must be **bit-identical** to the current path on the
piloted flow.

## Decision + stop conditions

Disposition = GO / NO-GO / CONDITIONAL for the broader migration (Stages C–F), per the
PERFARCH02 staged plan's stop conditions. **Write a NO-GO** if the integrated pilot shows any
of:

- the correct array path is **still above the ≤10× per-OFE-day budget** after removing
  dual-write and per-day export;
- conservation / guard / publication work **dominates** such that ~73× is the honest floor;
- the pilot **requires broad `SC-*` science-contract changes** rather than a representation-only
  migration.

A NO-GO is a **valid, successful** outcome — it stops the broad migration honestly. A GO must
state whether 5× is now demonstrated, plausible, or still out of reach, and recommend the
Stage-C scope.

## Scope

In scope: Stage A (contract shell, flag-gated) + Stage B (one WB11 integrated pilot flow) +
the two structural proofs + bit-identity + the floor measurement + the GO/NO-GO decision.

Out of scope:

- **Stages C–F** (scheduler authority flip, kernel-family expansion, publication-boundary
  cleanup, read-mirror removal) — downstream, gated on this package's GO + ADR-0023 ratification.
- **Irrigation** — deferred/inert (`docs/backlog/20260617-irrigation-management-gated-activation.md`).
- No `SC-*` science-contract change (representation/interface only; outputs bit-identical).
- No HBP / parquet output-schema change (ADR-0019/0020) beyond where/when materialized.
- No default-path behavior change (flag off ⇒ byte-identical to today).
- **Do not expand the already-large touched files** (`core_types.rs` 2671, `scheduler.rs`
  2452, `scheduler_seed_and_runtime.rs` 2672, `state_access.rs` 2219, etc.) without
  extraction — add new array-authoritative types in new modules.

## Acceptance Criteria

- **Stage A:** contract-shell types + id-backed evaluator land flag-gated; finite/domain +
  export-identity + failure-parity unit tests pass; flag off ⇒ byte-identical default path.
- **Stage B bit-identity (load-bearing):** the piloted WB11 flow is byte-identical (HBP / loss
  / wat / plot; pass-parquet rows equal) vs the current production path on H2637 (both
  variants if feasible) + the OFE ladder.
- **Two structural proofs:** perf evidence that the pilot path has no per-day full `BTreeMap`
  export and no normal-path dual-write.
- **Floor measurement:** H2637 no-UI per-OFE-day cost + extrapolated floor/ratio, same machine
  as PERFIDX06, method + variance stated.
- **Determinism** (`docs/numerics/`): frozen registry; no FP-reduction reorder; no per-OFE
  sequencing change; pinned-seed reproducible.
- **Rust gates:** `cargo fmt --check`; `cargo clippy --workspace --all-targets -- -D warnings`;
  `cargo test --workspace`; `cargo deny check`; `git diff --check`; line-count governance.
- **Disposition:** GO / NO-GO / CONDITIONAL with the 5× / ≤10× judgment + Stage-C recommendation;
  ADR-0023 ratification input.

## Deliverables

- `artifacts/perfarray01-contract-shell.md` (Stage A: types, evaluator, flag, parity tests)
- `artifacts/perfarray01-wb11-pilot.md` (Stage B: the piloted flow, what it includes)
- `artifacts/perfarray01-bit-identity-evidence.md`
- `artifacts/perfarray01-structural-proofs.md` (no per-day export; no dual-write — perf-backed)
- `artifacts/perfarray01-floor-measurement.md` (per-OFE-day cost → extrapolated floor/ratio)
- `artifacts/perfarray01-determinism-evidence.md`
- `artifacts/perfarray01-gate-results.md`
- `artifacts/perfarray01-line-count-governance.md`
- `artifacts/perfarray01-review-a.md`
- `artifacts/perfarray01-review-b.md`
- `artifacts/perfarray01-verification-a.md`
- `artifacts/perfarray01-verification-b.md`
- `artifacts/perfarray01-worker-handoff.md`
- `artifacts/perfarray01_disposition.md` (GO/NO-GO + 5×/≤10× verdict + ADR-0023 ratification input)

## Dependencies

- `docs/work-packages/20260618-perfarch02-array-authoritative-hot-path-state-redesign-001/artifacts/{perfarch02-proposed-adr,perfarch02-staged-migration-plan,perfarch02-redesign-shape,perfarch02-contract-blast-radius,perfarch02-floor-measurement,review-claude-independent}.md`
- `docs/decisions/0022-indexed-runtime-surface-representation.md` (+ Amendment 1; the design ADR-0023 supersedes in part)
- `docs/work-packages/20260618-perfidx06-high-ofe-target-assessment-001/artifacts/{perfidx06-legacy-ratio,perfidx06-bottleneck-analysis}.md` (the 73.12× endpoint + same-machine method)
- `docs/work-packages/20260617-perfidx05-writeback-guards-by-id-001/artifacts/perfidx05_disposition.md` (dual-write trap)
- `docs/work-packages/20260616-perfidx03-indexed-surface-authority-001/artifacts/perfidx03_disposition.md` (export-seam trap)
- The blast-radius files (kernel producers, scheduler `execute_with_kernel_indexed`, seed point, publication boundaries) — see `perfarch02-contract-blast-radius.md`
- `docs/numerics/README.md`; `AGENTS.md`; `docs/work-packages/AGENTS.md`; `crates/AGENTS.md`;
  `docs/standards/rust-scientific-coding-standard.md`

## Subagent Requirement

None required. If the operator authorizes subagents, the bit-identity anchor runs
(OFE ladder + H2637 variants) and the contract-blast-radius confirmation are parallelizable.
Run gates and timing locally; record command evidence.

## Autonomy

Execute end-to-end through Stage A contract shell, Stage B WB11 pilot, the two structural
proofs, bit-identity, the floor measurement, determinism, gates, dual review, dual
verification, line-count governance, and the GO/NO-GO disposition. **Stop and write NO-GO** on
any stop condition rather than continuing — a candid "73× is the honest floor" is a valid,
successful closure. Do **not** start Stages C–F in this package.
