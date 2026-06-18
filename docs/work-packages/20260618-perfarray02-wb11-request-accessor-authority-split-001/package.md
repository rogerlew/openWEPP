# PERFARRAY02 - WB11 Request/Accessor Authority Split + Integrated Floor

Status: executed-NO-GO 2026-06-18. The request/accessor seam and flag-gated WB11 runoff
pilot landed, bit identity passed, and the floor was measured. H2637 array-native
runoff pilot cost was `817.810 us/OFE-day`, above the `386 us/OFE-day` <=10x budget;
boundary seed/materialize cost was `1685.023 us/OFE-day`. ADR-0023 should not be
ratified from this pilot. **Post-review closure (operator A):** the perf arc is closed —
all PERFARRAY02 pilot code discarded **and** the PERFARRAY01 Stage A `ArrayHotState`
shell reverted; production carries **zero array-authoritative code**; record is docs-only.
The perf program concludes at the 73.12× endpoint (PERFIDX04/06). See
`artifacts/perfarray02_disposition.md` (Post-review closure) and
`artifacts/review-claude-independent.md`.

Package type: **Production hot-path pilot — flag-gated, bit-identity load-bearing, the package
that finally measures the integrated floor.** Builds the array-capable kernel request +
WB11-runoff scalar accessor path on top of PERFARRAY01's landed Stage A contract shell, then
runs the WB11 runoff pilot and measures H2637. Governed by proposed ADR-0023.

## Why this package exists

PERFARRAY01 NO-GO'd because the kernel request reads scalars from logical `BTreeMap`s
(`core_types.rs:2453-2454`, built at `scheduler.rs:1606`), so a WB11 pilot on that seam must
either export-per-day (PERFIDX03 trap) or dual-write (PERFIDX05 trap). The prerequisite is an
**array-capable request/accessor seam**: make the kernel *read* WB11 scalars from
`ArrayHotState` by `SymbolId`, so the pilot can execute array-native and satisfy the two
structural proofs. PERFARRAY02 builds that seam for the WB11 runoff anchor and then measures
the integrated floor — **its floor number is the real answer to ≤10× / 5×, and the basis the
operator ratifies ADR-0023 against.**

## Builds on

PERFARRAY01 Stage A (landed, inert): `ArrayHotState`, `ArrayWritebackField/Payload`,
`evaluate_array_writeback` / `apply_array_writeback`, `export_btreemap_surfaces` in
`openwepp-kernel-contract`. PERFARRAY02 wires a *read* path to match the *write* path already
shelled.

## Scope

1. **Array-capable kernel request** — `HillslopeKernelRequest` can supply WB11 scalar reads
   from `ArrayHotState` by `SymbolId` (flag-gated; the default request stays logical-map-backed
   and byte-identical).
2. **WB11 runoff scalar accessor path** — the `state_access.rs` scalar accessors used by
   runoff reconciliation read from the array on the pilot path; logical names resolved only on
   the failure path.
3. **WB11 runoff pilot** — run the **real** runoff reconciliation daily flow on the array:
   real typed guards / conservation, the scheduler apply step (via `apply_array_writeback`),
   and outlet/publication materialization for the assertion. **Not** a stripped slice.
4. **The two structural proofs — now achievable, and they must be DEMONSTRATED (perf-backed),
   not just statically argued:** during kernel execution there is no per-day full `BTreeMap`
   export at the seam (proof 1) and no normal-path logical+array dual-write (proof 2).
5. **Bit-identity** of the piloted flow (flag on) vs the current path (flag off) on H2637 +
   the OFE ladder.
6. **Integrated floor measurement** — H2637 no-UI per-OFE-day array-native execution cost on
   the same machine as PERFIDX06 → extrapolated floor / ratio.
7. **Disposition** — GO / NO-GO / CONDITIONAL + the 5× / ≤10× verdict + ADR-0023 ratification
   input.

## The load-bearing design question (Codex's call — surface, don't prescribe)

For the pilot to satisfy the proofs **during execution**, WB11 runoff must execute array-native.
But the scheduler still owns logical maps as the persistent lane authority (persisting the
array across days is **Stage C**, out of scope). So the pilot must obtain the array as the
authority **for the piloted flow** without a per-day logical↔array conversion *inside the
kernel seam*. Options (Codex decides): seed the array at the pilot boundary and measure
array-native execution with the seed/materialize cost reported **separately** (as PERFARCH02
did with `export_once`); or a scoped per-lane array authority for the pilot flow. The floor is
the **array-native per-OFE-day execution** cost, extrapolated — the transitional boundary
seed/materialize is measured apart and is what Stage C later removes.

**If WB11 runoff cannot execute array-native without Stage-C-scale scheduler-authority rework**
(i.e. the seam cannot be scoped), that is itself a finding: STOP and report that the
array-authoritative migration is not incrementally pilotable at this boundary — do not quietly
expand into Stage C.

## Hard stops

1. **Honest measurement:** the pilot runs openWEPP's real WB11 runoff work (guards,
   conservation, apply, publication) on the array. A stripped pilot measures a fictional floor.
2. **The two structural proofs are make-or-break and must be perf-demonstrated** on the valid
   pilot path. If they can't be met, NO-GO — do not paper over.
3. **Bit-identity is load-bearing:** flag-on piloted flow byte-identical to flag-off (HBP / loss
   / wat / plot; pass-parquet rows equal) on H2637 + the OFE ladder. Flag off ⇒ byte-identical
   default. Any mismatch → STOP + diagnose.
4. **Irrigation is OUT** (deferred/inert).
5. **Scope discipline:** the WB11 runoff anchor only; **no Stage C–F** (persistent scheduler
   array authority, family expansion, publication cleanup, mirror removal). Don't expand the
   already-large files (`core_types.rs` 2671, `scheduler.rs` 2452, `state_access.rs` 2219,
   `scheduler_seed_and_runtime.rs` 2672) without extraction.

## Decision + stop conditions

Disposition = GO / NO-GO / CONDITIONAL for the broader migration, per the PERFARCH02 staged
plan. **NO-GO** if: the proofs can't be met for a scoped WB11 runoff pilot; or the array-native
floor extrapolates **above the ≤10× per-OFE-day budget** (386 µs); or conservation/guard/
publication dominates such that ~73× is the honest floor; or the seam needs Stage-C-scale work
or broad `SC-*` changes. A candid NO-GO is a **valid, successful** outcome. A GO states whether
5× is now demonstrated / plausible / out of reach and recommends Stage C.

## Acceptance Criteria

- **Array-capable request + WB11 accessor path** land flag-gated; flag off ⇒ byte-identical.
- **WB11 runoff pilot** runs the real flow array-native behind the flag.
- **Two structural proofs:** perf evidence — no per-day seam export, no normal-path dual-write —
  on the valid pilot path.
- **Bit-identity (load-bearing):** flag-on vs flag-off byte-identical (HBP/loss/wat/plot; pass
  rows equal) on H2637 + OFE ladder.
- **Integrated floor:** H2637 no-UI array-native per-OFE-day cost + extrapolated floor/ratio,
  same machine as PERFIDX06; boundary seed/materialize reported separately.
- **Determinism** (`docs/numerics/`): frozen registry; no FP-reduction reorder; no per-OFE
  sequencing change; pinned-seed reproducible.
- **Rust gates:** `cargo fmt --check`; `cargo clippy --workspace --all-targets -- -D warnings`;
  `cargo test --workspace`; `cargo deny check`; `git diff --check`; line-count governance.
- **Disposition:** GO/NO-GO + 5×/≤10× verdict + Stage-C recommendation + ADR-0023 ratification
  input.

## Deliverables

- `artifacts/perfarray02-request-accessor-seam.md` (the array-capable request + WB11 accessor)
- `artifacts/perfarray02-wb11-runoff-pilot.md` (the piloted flow; what real work it includes)
- `artifacts/perfarray02-structural-proofs.md` (perf-backed: no export, no dual-write)
- `artifacts/perfarray02-bit-identity-evidence.md`
- `artifacts/perfarray02-floor-measurement.md` (array-native per-OFE-day → floor/ratio)
- `artifacts/perfarray02-determinism-evidence.md`
- `artifacts/perfarray02-gate-results.md`
- `artifacts/perfarray02-line-count-governance.md`
- `artifacts/perfarray02-review-a.md`
- `artifacts/perfarray02-review-b.md`
- `artifacts/perfarray02-verification-a.md`
- `artifacts/perfarray02-verification-b.md`
- `artifacts/perfarray02-worker-handoff.md`
- `artifacts/perfarray02_disposition.md` (GO/NO-GO + 5×/≤10× + ADR-0023 input)

## Dependencies

- `docs/work-packages/20260618-perfarray01-wb11-integrated-array-authoritative-pilot-001/artifacts/{perfarray01_disposition,perfarray01-structural-proofs,perfarray01-contract-shell,review-claude-independent}.md`
- `crates/openwepp-kernel-contract/src/lib_mod/array_hot_state.rs` (the landed Stage A shell)
- `docs/work-packages/20260618-perfarch02-array-authoritative-hot-path-state-redesign-001/artifacts/{perfarch02-proposed-adr,perfarch02-staged-migration-plan,perfarch02-contract-blast-radius}.md`
- `docs/decisions/0022-indexed-runtime-surface-representation.md` (+ Amendment 1)
- `docs/work-packages/20260618-perfidx06-high-ofe-target-assessment-001/artifacts/{perfidx06-legacy-ratio,perfidx06-bottleneck-analysis}.md` (73.12× endpoint, same-machine method, budgets)
- The blast-radius seam files: `core_types.rs` (`HillslopeKernelRequest`), `scheduler.rs:1606`
  (request construction), `state_access.rs` (WB11 scalar accessors),
  `hydrology_phase_runoff_reconciliation.rs` (the anchor flow)
- `docs/numerics/README.md`; `AGENTS.md`; `docs/work-packages/AGENTS.md`; `crates/AGENTS.md`;
  `docs/standards/rust-scientific-coding-standard.md`

## Subagent Requirement

None required. If the operator authorizes subagents, the bit-identity anchor (OFE ladder +
H2637 variants) and the accessor call-site mapping are parallelizable. Run gates + timing
locally; record command evidence.

## Autonomy

Execute end-to-end through the request/accessor seam, the WB11 runoff pilot, the perf-backed
structural proofs, bit-identity, the integrated floor measurement, determinism, gates, dual
review, dual verification, line-count governance, and the GO/NO-GO disposition. **Stop and write
NO-GO** on any stop condition — including "the seam cannot be scoped without Stage-C-scale work"
or "73× is the honest floor." Do not start Stages C–F.
