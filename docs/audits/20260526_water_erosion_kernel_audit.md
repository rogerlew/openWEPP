# Hydrology and Erosion Kernel Audit (Delta) — 2026-05-26

Status: Draft
Last updated: 2026-05-27
Evidence mode: Static (HEAD inspection + working-tree inspection; no kernel execution, no test run, no numerical diff against legacy)
Scope: Delta snapshot of openWEPP's hillslope erosion subsystem one day after [`20260525_water_erosion_kernel_audit.md`](20260525_water_erosion_kernel_audit.md), capturing the EROD16→EROD21 sequence that landed at HEAD today plus the concept-stage Hairsine-Rose backlog filing. Hydrology kernels outside the erosion-coupled path are out of scope.

## 1. Purpose

Snapshot the 2026-05-26 post-EROD21 state of openWEPP's sediment subsystem. The 2026-05-25 audit row 214 flagged the route-branch family ("MSHEAR computed-GOTO cases") as a `Partial` runtime gap requiring algorithmic comparison. Today an EROD16→EROD21 chain landed that authored contract authority, contract-derived tests, the topology ingress seam, the kernel migration itself, constant symbolization, and a parity rerun. This audit confirms what landed in code, identifies where contract documentation lags code, and records the post-migration state of the supporting symbols.

This audit complements rather than replaces the 2026-05-25 audit. The full kernel inventory (production-kernel surface, stub scan, per-kernel physics summary) from §3-§5 of yesterday's audit remains canonical for non-erosion kernels.

## 2. Method

Did:
- `git log ed3e8a8..HEAD` to enumerate commits since the 2026-05-25 snapshot.
- Read each EROD17/18/19/20/21 disposition artifact under `docs/work-packages/20260526-erod*`.
- `git show <hash> --stat` for each EROD commit to confirm scope.
- `git show HEAD:crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support.rs | grep -n "mshear\|xc1\|xc2\|fn run_erod"` to verify kernel-migration landing at HEAD.
- `git show HEAD:crates/openwepp-runner/src/hillslope/mod.rs | grep -n "MOFE03_WAVE2_DEFAULT_X"` to verify residual-constants state.
- Read [`SC-SED-001.md` GAP-SED-005 row, revision-history table, EROD16 addendum](../specifications/science-contracts/contracts/SC-SED-001.md) and [`SC-ROUTE-001.md` EROD16 addendum](../specifications/science-contracts/contracts/SC-ROUTE-001.md) at working-tree state.
- `git status` to identify uncommitted refactor005 modularization changes that affect file-path stability for kernel references.
- Read new [`docs/backlog/20260526-hairsine-rose-multiclass-sediment-model.md`](../backlog/20260526-hairsine-rose-multiclass-sediment-model.md) and [HRREF-01 WP scaffolding](../work-packages/20260526-hrref01-hairsine-rose-references-intake-001/).
- Post-audit static trace of MOFE03 route-topology seeding flow in [`crates/openwepp-runner/src/hillslope/mod.rs`](../../crates/openwepp-runner/src/hillslope/mod.rs) to disposition `.unwrap_or(...)` reachability in §4.3.
- Read REFACTOR005 disposition artifact [`refactor005_disposition.md`](../work-packages/20260526-refactor005-openwepp-hillslope-orchestrator-kernel-support-mechanical-modularization-001/artifacts/refactor005_disposition.md) and re-anchor §4.1 evidence paths to split kernel files.

Did not:
- Execute `cargo test`, `cargo check`, any kernel invocation, or any CLI fixture.
- Re-walk the full kernel inventory from yesterday's audit §3-§5.
- Numerically diff `run_erod14_wave2` output against `route.for` reference — EROD21 disposition records that this was performed by the WP; this audit only confirms the disposition's existence.
- Read every line of the 504-line EROD19 kernel diff; sampling confined to the `mshear`/`xc1`/`xc2` introduction site and the surrounding ~60 lines.
- Audit non-erosion subsystems (ET, snow, frost, impoundment, channel) — those carry forward from 2026-05-25 unchanged.

## 3. Repo-state snapshot

### 3.1 Commits landed since 2026-05-25 audit

`git log ed3e8a8..HEAD` (where `ed3e8a8` is the HR backlog commit authored earlier today):

| Commit | Title | Scope |
|---|---|---|
| `955c18f` | docs(erod16): codify route branch contract authority and scope partition | SC-SED-001 rev 12 (EROD16 addendum, REF-SED-LEGACY-CONTIN-ROUTE, REF-SED-LEGACY-RTPART, `GAP-SED-005` opened); SC-ROUTE-001 rev 13 (scope-partition addendum, `GAP-ROUTE-007` closed) |
| `fd79f71` | test(erod17): add route branch contract vectors and pre-migration gate | Contract-derived test vectors for the route-branch family |
| `ba7698a` | feat(erod18): add route topology ingress seam and guards | New topology-symbol family + typed ingress guards in `02_guard_errors.rs`; runner ingress projection wired. Dispositioned HOLD pending EROD19 |
| `cbda790` | feat(erod19): migrate route branch kernel and activate vectors | +504 lines to `03_kernel_support.rs`; `mshear`/`xc1`/`xc2` algorithm introduced inside `run_erod14_wave2` |
| `8e43ef3` | refactor(erod20): symbolize sediment routing constants | Magic-number elimination; constants moved to `constants.rs` |
| `07d361c` | Execute EROD21 route parity rerun and disposition | Parity rerun gate; HOLD lifted with `GO` decision |

### 3.2 Capture-time working-tree state (historical)

| Item | Capture-time state |
|---|---|
| Notes added to 2026-05-25 audit (`PURK fx`, `watbal_hourly`, `ksatadj` parser row) | uncommitted (modified) — authored by Codex during yesterday's review cycle, not yet pushed |
| `docs/work-packages/README.md` row addition | uncommitted (modified) — appears to be a refactor005 index addition |
| refactor005 mechanical-modularization split of `03_kernel_support.rs` into `03_kernel_support_00_support_helpers.rs` + `03_kernel_support_01_kernel_phases.rs` | untracked + modified |
| refactor005 WP directory | untracked |

Post-audit update: REFACTOR005 subsequently landed (`GO` disposition), and §4 references are now re-anchored to the split-file layout.

### 3.3 Erosion-subsystem contract revision counts

| Contract | Revision at HEAD | Last revision row |
|---|---|---|
| `SC-SED-001` | rev 13 | EROD21 closure amendment (2026-05-26): closed `GAP-SED-005` after EROD19 migration + parity rerun evidence |
| `SC-ROUTE-001` | rev 13 | EROD16 amendment (2026-05-26): closed `GAP-ROUTE-007` |

SC-SED-001 now records the EROD19/EROD21 migration closure. See §4.2.

## 4. Findings

### 4.1 Hillslope route-branch runtime migration has LANDED at HEAD

The 2026-05-25 audit row 214 finding ("Partial. `run_erod14_wave2` covers the case-1-4 routing and per-class transport, but the per-segment upper-end detach-vs-deposit branching of `route.for` (MSHEAR computed-GOTO cases) is not visible in the Rust impl. Needs algorithmic comparison.") has been closed by the EROD17→EROD21 chain.

Evidence:
- `git show HEAD:crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_01_kernel_phases.rs` shows `run_erod14_wave2` (entry at L2706) with `mshear` / `xc1` / `xc2` classification and dispatch logic (`erod19_xcrit_classification` at L3393+ and branch use at L3845+). The kernel computes `xc1` via `tauchk / b` where applicable and dispatches across the five MSHEAR cases.
- EROD18 ([`erod18_disposition.md`](../work-packages/20260526-erod18-route-runtime-segment-state-topology-and-ingress-closure-001/artifacts/erod18_disposition.md)) records "route topology symbol family and typed ingress guards are implemented; runner ingress projection is wired." The pre-EROD18 hard-coded-constant ingress posture is replaced by topology-symbol resolution.
- EROD21 ([`erod21_disposition.md`](../work-packages/20260526-erod21-route-parity-rerun-and-hold-lift-disposition-001/artifacts/erod21_disposition.md)) records "EROD21 complete. GO issued; sediment-routing HOLD carried from EROD19/EROD20 is lifted."

**Finding**: the audit-row-214 gap is closed by code at HEAD. The 2026-05-25 finding was correct on its date; today's snapshot supersedes it for the same surface.

### 4.2 SC-SED-001 `GAP-SED-005` closure is now recorded (post-audit sync)

The stale-contract issue identified at audit capture time has now been closed in documentation:
- [`SC-SED-001.md`](../specifications/science-contracts/contracts/SC-SED-001.md) now records revision 13 and dispositions `GAP-SED-005` to `closed` with EROD19 + EROD21 closure evidence.
- [`docs/specifications/science-contracts/index.md`](../specifications/science-contracts/index.md) now includes the corresponding SC-SED-001 row note update.

**Finding update**: the one-day contract/code drift identified in the original §4.2 narrative has been resolved by this documentation sync. Preserve the original narrative context as historical capture rationale.

### 4.3 MOFE03 default constants remain at HEAD; route-topology fallback branch is statically unreachable in current seeding path

The `MOFE03_WAVE2_DEFAULT_XTOP = 0.2`, `_XBOT = 0.5`, `_XDETST = 0.1` constants are still present in HEAD at [`hillslope/mod.rs:1852-1854`](../../crates/openwepp-runner/src/hillslope/mod.rs#L1852-L1854), and still used both as seed insertions (L2008, L2012, L2016) and as `.unwrap_or(...)` fallbacks (L2070, L2072, L2074).

What changed between 2026-05-25 and HEAD is their **semantic role**, not their presence. Pre-EROD18 they were the *authoritative* values the kernel actually used because no topology-ingress mechanism existed. Post-EROD18 the runner projects topology-resolved values into the same boundary symbols via the EROD18 ingress seam; the constants now act as **fallbacks** when topology resolution does not produce a runtime-supplied value.

Post-audit static trace of [`seed_mofe03_wave2_runtime_surface_inputs`](../../crates/openwepp-runner/src/hillslope/mod.rs#L1877) shows:
- `seed_mofe03_wave2_core_scalars` writes `erod14_xtop`, `erod14_xbot`, `erod14_xdetst`, `erod14_lddend`, `erod14_ainftc`, `erod14_binftc`, `erod14_cinftc` before route-topology ingestion.
- `seed_mofe03_wave2_route_topology_ingress` then reads those symbols with `.unwrap_or(...)`.

Because the producer writes happen in-sequence within the same function and there is no intervening mutation/removal path, those specific `.unwrap_or(...)` fallbacks are statically unreachable in the current call path when Wave-2 seeding runs.

**Finding update**: uncertainty is closed for this surface. The constants remain active as MOFE03 seeding defaults, but not as reachable fallback inputs at the route-topology ingress read site.

### 4.4 SC-ROUTE-001 scope-partitioning closure remains valid

EROD16's companion amendment to SC-ROUTE-001 ([rev 13, addendum at §410](../specifications/science-contracts/contracts/SC-ROUTE-001.md#L410); `GAP-ROUTE-007` closed at [§442](../specifications/science-contracts/contracts/SC-ROUTE-001.md#L442)) is unaffected by EROD17-21. The scope partitioning between watershed-side WS10 routing and hillslope-side `CONTIN → ROUTE` sediment routing remains contract-separable.

**Finding**: no new SC-ROUTE-001 gap introduced by today's chain. `GAP-ROUTE-005` (runtime workload guards for Chapter-13 applicability limits) remains the lone non-promotable row for SC-ROUTE-001.

### 4.5 Hairsine-Rose backlog filed earlier today — no contract or code impact

[`docs/backlog/20260526-hairsine-rose-multiclass-sediment-model.md`](../backlog/20260526-hairsine-rose-multiclass-sediment-model.md) and [HRREF-01 WP scaffolding](../work-packages/20260526-hrref01-hairsine-rose-references-intake-001/) are committed at `ed3e8a8`. Both are docs-only. The backlog item's static-analysis section confirms that `particle_class_count` is already variable-cardinality (`u16` in HBP, per-class-indexed in kernel-support) at HEAD — a structural advantage for any future HR adoption but not itself a code change.

**Finding**: HR adoption is concept-stage. No openWEPP contract is amended by the backlog filing; no kernel surface changes.

**Caveat**: this audit was authored by the same agent that filed the HR backlog earlier in the same session. The finding is conservative — HR has no contract or code impact *today* — and intentionally avoids any forward claim about whether HR adoption is the right direction.

### 4.6 Parser-without-consumer surfaces — unchanged

The 2026-05-25 audit §10.4 (with today's added notes on `soil.ksatadj/ksatfac/ksatrec`) remains the current snapshot of parser-without-consumer surfaces. EROD17-21 did not introduce a new such surface, and the HR backlog explicitly defers any input-parser changes to a downstream WP.

**Finding**: no new parser-without-consumer drift introduced today.

### 4.7 Confirmed-implemented physics list — extended

Adding to the 2026-05-25 audit §10.7 "Confirmed-implemented physics" list, as of HEAD today:

- Foster-Meyer rill detachment/deposition with **MSHEAR 1-5 segment-topology dispatch** and dynamic `xc1`/`xc2` shear-critical crossing resolution inside `run_erod14_wave2`.

This is the substantive code-state change since yesterday.

### 4.8 REFACTOR005 modularization has landed; references re-anchored

A `refactor005` mechanical-modularization stream split `03_kernel_support.rs` into `03_kernel_support_00_support_helpers.rs` + `03_kernel_support_01_kernel_phases.rs` and is now dispositioned `GO` in REFACTOR005 artifacts.

**Finding update (informational)**: not a physics-state change. Cross-references in this audit are now re-anchored to the split-file layout (`03_kernel_support_01_kernel_phases.rs`) and no longer rely on pre-split line anchors.

## 5. Caveats

- **HEAD-anchored snapshot**: findings cite current HEAD and include post-audit sync updates for SC-SED-001 closure and refactor005 path re-anchoring.
- **Audit author conflict-of-interest disclosure for §4.5**: the HR backlog and HRREF-01 WP were authored by the same Claude Code session earlier today. The §4.5 finding is intentionally narrow.
- **No numerical comparison performed**: the §4.1 claim that the MSHEAR migration has "landed" is a *structural* claim — the symbols are present, the disposition records GO. This audit does **not** verify per-OFE numerical parity against `route.for` reference output; that was performed by the EROD21 WP and is documented in its disposition artifacts, not re-verified here.
- **§4.3 scope note**: fallback reachability closure is static for the current seeding path; no additional dynamic instrumentation was run in this update.
- **Scope excludes non-erosion subsystems**: ET, snow, frost, impoundment, and channel kernel status carries forward from 2026-05-25 unchanged.
- **EROD19 diff sampling**: read approximately 60 lines around the `mshear`/`xc1`/`xc2` introduction site, not the full 504-line diff. Other algorithmic content of EROD19 (e.g. upper-end deposit-vs-detach state handling, post-detachment deposition closure) is referenced from the contract addendum text and the disposition narrative, not from line-by-line code reading.
- **Sampling discipline**: only the erosion-relevant subset of today's commits was inspected in detail. Codex's other concurrent work (e.g. frost lineage, refactor stream) was acknowledged but not read.

## 6. Follow-up disposition status

1. **Completed: `GAP-SED-005` documentation sync** in [`SC-SED-001.md`](../specifications/science-contracts/contracts/SC-SED-001.md) and [`science-contracts/index.md`](../specifications/science-contracts/index.md) now records EROD19/EROD21 closure and revision-history continuity.
2. **Completed: EROD18 ingress fallback reachability disposition** — static trace confirms the route-topology `.unwrap_or(...)` reads in MOFE03 seeding are unreachable in the current Wave-2 seeding path because required `erod14_*` symbols are written immediately beforehand in the same function.
3. **Completed: REFACTOR005 re-anchoring** — kernel cross-references in §4.1/§4.8 now point to split-file layout (`03_kernel_support_01_kernel_phases.rs`) and no longer depend on pre-split line anchors.
4. **HR backlog progression**: HRREF-01 (references intake) is scaffolded; subsequent WPs for `SC-SED-HR-001` authoring and `Customizable-class architecture` decision are listed in the backlog's `## Work-package linkage` section but not yet authored. Not authored by this audit.

## 7. Cross-references

- [`20260525_water_erosion_kernel_audit.md`](20260525_water_erosion_kernel_audit.md) — prior-day snapshot; remains canonical for kernel inventory (§3-§5) and the full cross-reference table (§10).
- [`docs/specifications/science-contracts/contracts/SC-SED-001.md`](../specifications/science-contracts/contracts/SC-SED-001.md) — contract authority (revision 13; `GAP-SED-005` closed per §4.2).
- [`docs/specifications/science-contracts/contracts/SC-ROUTE-001.md`](../specifications/science-contracts/contracts/SC-ROUTE-001.md) — companion routing authority (revision 13).
- EROD16-21 WP dispositions:
  - [`20260526-erod16-route-branch-contract-authority-and-routine-map-001/`](../work-packages/20260526-erod16-route-branch-contract-authority-and-routine-map-001/)
  - [`20260526-erod17-route-branch-contract-derived-tests-and-preimplementation-gate-001/`](../work-packages/20260526-erod17-route-branch-contract-derived-tests-and-preimplementation-gate-001/)
  - [`20260526-erod18-route-runtime-segment-state-topology-and-ingress-closure-001/`](../work-packages/20260526-erod18-route-runtime-segment-state-topology-and-ingress-closure-001/)
  - [`20260526-erod19-route-mshear-segment-kernel-migration-001/`](../work-packages/20260526-erod19-route-mshear-segment-kernel-migration-001/)
  - [`20260526-erod20-sediment-routing-constant-symbolization-and-magic-number-elimination-001/`](../work-packages/20260526-erod20-sediment-routing-constant-symbolization-and-magic-number-elimination-001/)
  - [`20260526-erod21-route-parity-rerun-and-hold-lift-disposition-001/`](../work-packages/20260526-erod21-route-parity-rerun-and-hold-lift-disposition-001/)
- [`docs/backlog/20260526-hairsine-rose-multiclass-sediment-model.md`](../backlog/20260526-hairsine-rose-multiclass-sediment-model.md) — HR concept backlog (committed at `ed3e8a8`).
- [`docs/work-packages/20260526-hrref01-hairsine-rose-references-intake-001/`](../work-packages/20260526-hrref01-hairsine-rose-references-intake-001/) — HRREF-01 WP scaffolding (committed at `ed3e8a8`).
- [`docs/work-packages/20260526-refactor005-openwepp-hillslope-orchestrator-kernel-support-mechanical-modularization-001/`](../work-packages/20260526-refactor005-openwepp-hillslope-orchestrator-kernel-support-mechanical-modularization-001/) — REFACTOR005 modularization disposition (`GO`) and split-file layout authority.
- [`docs/decisions/0011-architecture-first-top-down-science-contracts.md`](../decisions/0011-architecture-first-top-down-science-contracts.md) — contract-first sequencing authority; the §4.2 contract-trails-code drift is the inverse of the contract-first pattern.
