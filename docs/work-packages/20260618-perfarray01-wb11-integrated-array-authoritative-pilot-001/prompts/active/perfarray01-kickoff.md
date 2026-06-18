# PERFARRAY01 Kickoff — WB11 Integrated Array-Authoritative Pilot (Stage A + B)

Execution mode: production hot-path pilot — flag-gated, bit-identity load-bearing, floor
measurement + GO/NO-GO. First production code in the array-authoritative arc; governed by
proposed ADR-0023.

Autonomy: execute end-to-end (Stage A contract shell → Stage B WB11 pilot → two structural
proofs → bit-identity → floor measurement → determinism → gates → dual review/verification →
GO/NO-GO disposition). **Stop and write NO-GO on any stop condition.** Do not start Stages C–F.

## Why you're here — this measures the real floor

PERFARCH02 proved the *writeback/guard surface* is ~49.9× faster array-authoritative on a
**synthetic** surface — but it did not run the integrated WB11 flow, so the H2637 floor is
unmeasured and **5× is unproven**. PERFARRAY01 ports **one representative WB11 daily flow** to
array-authoritative state with **real** daily work and measures the actual per-OFE-day floor.
**Your floor number is the real answer to "can openWEPP reach ≤10× / 5×."**

## Governing decision

Proposed **ADR-0023** (Array-Authoritative Hot-Path State; supersedes the hot-path-authority
portion of ADR-0022, keeps the registry/`SymbolId`/sorted-order/logical-export foundation) is
**Proposed, not ratified**. Your measured floor is the evidence the operator ratifies it
against — ratify on the real floor, not blind. Flag-gated inert code may land for measurement;
the broad migration (Stages C–F) is gated on GO **and** ADR-0023 ratification.

## Stage A — Contract Shell (no production flip)

`ArrayHotState` (dense state/flux by `SymbolId`), `ArrayWritebackField`/`Payload` (id + value +
finite/range bounds), an id-backed finite/domain evaluator preserving the **current message-id
class** + fail-closed semantics (logical name resolved **only on failure**), and logical
materialization APIs. **Flag off ⇒ byte-identical to today.** Gates: finite/domain parity vs
`evaluate_kernel_writeback`; success-path export identity; failure-path lazy-subject parity; no
default flip.

## Stage B — WB11 Integrated Pilot (behind the flag)

Port one representative WB11 daily flow with **real** daily work — anchor on runoff
reconciliation (`hydrology_phase_runoff_reconciliation.rs`) + frost/snow state access if active
+ the **real** typed guards/conservation + the scheduler apply step + outlet/publication
materialization. **Not** a writeback-only slice.

## Two structural proofs (make-or-break — these killed the half-measures)

1. **No per-day full `BTreeMap` export at the kernel seam** (PERFIDX03 trap): the pilot kernel
   consumes/produces the dense array directly; logical maps materialize once at the
   publication/validation boundary, not per OFE-day. **Prove with perf evidence.**
2. **No normal-path logical + array dual-write** (PERFIDX05 trap): the array is the single
   mutable authority on the pilot path; no mirror maintenance / logical dual-write in normal
   timing. **Prove with perf evidence.**

If the pilot cannot satisfy both, that is a NO-GO signal — do not paper over it.

## Hard stops

1. **Honest-measurement (continues from PERFARCH02):** the pilot must do openWEPP's **real**
   per-step work (conservation, typed bounds, fail-closed guards, scheduler apply, publication)
   on array state. A stripped pilot measures a fictional floor and is worse than nothing.
2. **Bit-identity is load-bearing:** the piloted flow byte-identical (HBP/loss/wat/plot;
   pass-parquet rows equal) vs the current production path on H2637 + the OFE ladder. Flag off
   ⇒ byte-identical default path. Any mismatch → STOP + diagnose.
3. **Irrigation is OUT** (deferred/inert).
4. **Do not expand the already-large touched files** without extraction (`core_types.rs`,
   `scheduler.rs`, `scheduler_seed_and_runtime.rs`, `state_access.rs` are all >2000) — add new
   array-authoritative types in new modules.

## Floor + decision

Measure H2637 no-UI per-OFE-day cost on the **same machine** as PERFIDX06; extrapolate the
floor/ratio (budgets: ≤10× = 386 µs/OFE-day, ≤5× = 193 µs/OFE-day, legacy = 38.65 µs/OFE-day).
Disposition = GO / NO-GO / CONDITIONAL for Stages C–F. **NO-GO** if: the correct array path is
still above the ≤10× budget after removing dual-write/export; or conservation/guard/publication
dominates such that ~73× is the honest floor; or the pilot needs broad `SC-*` changes. State
whether 5× is now demonstrated / plausible / out of reach.

## Constraints

- No `SC-*` change; no HBP/parquet schema change; no default-path behavior change.
- Determinism (`docs/numerics/`): frozen registry; no FP-reduction reorder; no per-OFE
  sequencing change; pinned-seed reproducible.
- Rust gates: fmt; clippy `-D warnings`; `test --workspace`; deny; `git diff --check`;
  line-count.
- Truthfulness: bit-identity, structural proofs, floor, timing are empirical — label `Ran:`;
  the GO/NO-GO is a judgment from evidence. Do not deliver an optimistic floor as a measured one.

## Required reading

- `docs/work-packages/20260618-perfarray01-wb11-integrated-array-authoritative-pilot-001/package.md`
- `docs/work-packages/20260618-perfarch02-array-authoritative-hot-path-state-redesign-001/artifacts/{perfarch02-proposed-adr,perfarch02-staged-migration-plan,perfarch02-redesign-shape,perfarch02-contract-blast-radius,perfarch02-floor-prototype}.md`
  (+ the prototype harness under `…/artifacts/perfarch02-floor-prototype/` for the array
  writeback/guard pattern)
- `docs/decisions/0022-indexed-runtime-surface-representation.md` (+ Amendment 1)
- `docs/work-packages/20260618-perfidx06-high-ofe-target-assessment-001/artifacts/{perfidx06-legacy-ratio,perfidx06-bottleneck-analysis}.md`
- `docs/work-packages/20260617-perfidx05-writeback-guards-by-id-001/artifacts/perfidx05_disposition.md` (dual-write)
- `docs/work-packages/20260616-perfidx03-indexed-surface-authority-001/artifacts/perfidx03_disposition.md` (export seam)
- `AGENTS.md`, `docs/work-packages/AGENTS.md`, `crates/AGENTS.md`, `docs/numerics/README.md`,
  `docs/standards/rust-scientific-coding-standard.md`
- The blast-radius files: `hydrology_phase_runoff_reconciliation.rs`, scheduler
  `execute_with_kernel_indexed`, `00_runner_intake_and_lane_setup.rs` (seed point),
  `02_output_and_climate_helpers.rs` (publication), `writeback.rs`, `core_types.rs`.
