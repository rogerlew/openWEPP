# PERFARRAY02 Kickoff — WB11 Request/Accessor Authority Split + Integrated Floor

Execution mode: production hot-path pilot — flag-gated, bit-identity load-bearing; the package
that finally measures the integrated floor. Governed by proposed ADR-0023.

Autonomy: execute end-to-end (request/accessor seam → WB11 runoff pilot → perf-backed structural
proofs → bit-identity → integrated floor → determinism → gates → dual review/verification →
GO/NO-GO). **Stop and write NO-GO on any stop condition.** Do not start Stages C–F.

## Why you're here

PERFARRAY01 NO-GO'd: the kernel request reads scalars from logical `BTreeMap`s
(`core_types.rs:2453-2454`, built at `scheduler.rs:1606`), so a WB11 pilot there must export-per-
day or dual-write. Build the **array-capable request/accessor seam** so the kernel *reads* WB11
scalars from `ArrayHotState` by `SymbolId`, then run the WB11 runoff pilot and **measure the
integrated floor.** Your floor number is the real ≤10× / 5× answer and the basis the operator
ratifies ADR-0023 against.

## Builds on

PERFARRAY01 Stage A (landed, inert): `ArrayHotState` + `ArrayWritebackField/Payload` +
`evaluate_array_writeback`/`apply_array_writeback` + `export_btreemap_surfaces`. You wire the
*read* path to match the *write* path already shelled.

## Scope

1. Array-capable `HillslopeKernelRequest` (WB11 scalar reads from `ArrayHotState` by `SymbolId`;
   flag-gated; default stays logical + byte-identical).
2. WB11 runoff scalar accessor path (`state_access.rs`) reading from the array on the pilot path;
   logical names only on failure.
3. WB11 runoff pilot — the **real** runoff reconciliation daily flow on the array: real guards/
   conservation + scheduler apply (`apply_array_writeback`) + outlet/publication materialization.
4. Floor measurement (H2637, same machine as PERFIDX06) + the GO/NO-GO.

## The load-bearing design question (yours)

The pilot must execute WB11 runoff **array-native** to satisfy the proofs *during execution* —
but the scheduler still owns logical maps as the persistent lane authority (persisting the array
across days is Stage C, out of scope). So obtain the array as authority **for the piloted flow**
without a per-day logical↔array conversion *inside the kernel seam*: seed at the pilot boundary
and report the seed/materialize cost **separately** (as PERFARCH02 did with `export_once`), or a
scoped per-lane array authority for the pilot. The floor = array-native per-OFE-day execution,
extrapolated; the transitional boundary cost is measured apart (Stage C removes it later).

**If WB11 runoff cannot execute array-native without Stage-C-scale scheduler rework, STOP and
report that** — the migration may not be incrementally pilotable at this boundary. Do not quietly
expand into Stage C.

## Hard stops

1. **Honest measurement:** real WB11 runoff work on the array, not a stripped slice.
2. **The two structural proofs are make-or-break and must be perf-demonstrated** on the valid
   pilot path (no per-day seam export; no normal-path dual-write). If unmet → NO-GO.
3. **Bit-identity load-bearing:** flag-on piloted flow byte-identical to flag-off (HBP/loss/wat/
   plot; pass rows equal) on H2637 + OFE ladder. Flag off ⇒ byte-identical. Mismatch → STOP.
4. **Irrigation OUT.** Scope = WB11 runoff anchor only; no Stage C–F; don't bloat the large files.

## Floor + decision

Measure H2637 no-UI array-native per-OFE-day cost (same machine as PERFIDX06); extrapolate the
floor/ratio (budgets: ≤10× = 386 µs/OFE-day, 5× = 193 µs/OFE-day, legacy = 38.65 µs/OFE-day);
report the boundary seed/materialize separately. Disposition = GO / NO-GO / CONDITIONAL. **NO-GO**
if the proofs can't be met for a scoped pilot; or the array-native floor is above the ≤10× budget;
or conservation/guard/publication dominates such that ~73× is the honest floor; or the seam needs
Stage-C-scale or broad `SC-*` work. State whether 5× is demonstrated / plausible / out of reach.

## Constraints

- No `SC-*` change; no HBP/parquet schema change; no default-path behavior change.
- Determinism (`docs/numerics/`): frozen registry; no FP reorder; no per-OFE sequencing change.
- Rust gates: fmt; clippy `-D warnings`; `test --workspace`; deny; `git diff --check`; line-count.
- Truthfulness: proofs, bit-identity, floor, timing are empirical — label `Ran:`; the GO/NO-GO is
  a judgment from evidence. Do not deliver an optimistic floor as measured, and do not fabricate
  perf evidence for an invalid path (PERFARRAY01 set this precedent — honor it).

## Required reading

- `docs/work-packages/20260618-perfarray02-wb11-request-accessor-authority-split-001/package.md`
- `docs/work-packages/20260618-perfarray01-wb11-integrated-array-authoritative-pilot-001/artifacts/{perfarray01_disposition,perfarray01-structural-proofs,perfarray01-contract-shell,review-claude-independent}.md`
- `crates/openwepp-kernel-contract/src/lib_mod/array_hot_state.rs` (the landed Stage A shell)
- `docs/work-packages/20260618-perfarch02-array-authoritative-hot-path-state-redesign-001/artifacts/{perfarch02-proposed-adr,perfarch02-staged-migration-plan,perfarch02-contract-blast-radius,perfarch02-floor-prototype}.md`
- `docs/decisions/0022-indexed-runtime-surface-representation.md` (+ Amendment 1)
- `docs/work-packages/20260618-perfidx06-high-ofe-target-assessment-001/artifacts/{perfidx06-legacy-ratio,perfidx06-bottleneck-analysis}.md`
- `AGENTS.md`, `docs/work-packages/AGENTS.md`, `crates/AGENTS.md`, `docs/numerics/README.md`,
  `docs/standards/rust-scientific-coding-standard.md`
- The seam files: `core_types.rs` (`HillslopeKernelRequest`), `scheduler.rs:1606`,
  `state_access.rs`, `hydrology_phase_runoff_reconciliation.rs`.
