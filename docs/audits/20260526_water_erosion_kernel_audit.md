# Hydrology and Erosion Kernel Audit (Delta) — 2026-05-26

Status: Draft
Last updated: 2026-05-26
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

### 3.2 Working-tree state (not yet committed)

| Item | State |
|---|---|
| Notes added to 2026-05-25 audit (`PURK fx`, `watbal_hourly`, `ksatadj` parser row) | uncommitted (modified) — authored by Codex during yesterday's review cycle, not yet pushed |
| `docs/work-packages/README.md` row addition | uncommitted (modified) — appears to be a refactor005 index addition |
| refactor005 mechanical-modularization split of `03_kernel_support.rs` into `03_kernel_support_00_support_helpers.rs` + `03_kernel_support_01_kernel_phases.rs` | untracked + modified |
| refactor005 WP directory | untracked |

Audit consequence: file-path references in §4 cite the **HEAD-committed** kernel layout (`03_kernel_support.rs` with `run_erod14_wave2` at L5902), not the in-flight refactor005 split. When refactor005 lands, the kernel will move to `03_kernel_support_01_kernel_phases.rs`; line numbers cited here will need re-anchoring then.

### 3.3 Erosion-subsystem contract revision counts

| Contract | Revision at HEAD | Last revision row |
|---|---|---|
| `SC-SED-001` | rev 12 | EROD16 amendment (2026-05-26): opened `GAP-SED-005` |
| `SC-ROUTE-001` | rev 13 | EROD16 amendment (2026-05-26): closed `GAP-ROUTE-007` |

Neither contract has a revision row recording the EROD19/EROD21 migration closure. See §4.2.

## 4. Findings

### 4.1 Hillslope route-branch runtime migration has LANDED at HEAD

The 2026-05-25 audit row 214 finding ("Partial. `run_erod14_wave2` covers the case-1-4 routing and per-class transport, but the per-segment upper-end detach-vs-deposit branching of `route.for` (MSHEAR computed-GOTO cases) is not visible in the Rust impl. Needs algorithmic comparison.") has been closed by the EROD17→EROD21 chain.

Evidence:
- `git show HEAD:crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support.rs` shows `mshear` / `xc1` / `xc2` symbols introduced at L6589-6647+ inside `run_erod14_wave2` (entry at L5902). The kernel now computes the crossing point `xc1 = tauchk / b` (L6603) and dispatches across the five MSHEAR cases (e.g. `mshear = 3.0` at L6608, `mshear = 4.0` at L6616, etc.).
- EROD18 ([`erod18_disposition.md`](../work-packages/20260526-erod18-route-runtime-segment-state-topology-and-ingress-closure-001/artifacts/erod18_disposition.md)) records "route topology symbol family and typed ingress guards are implemented; runner ingress projection is wired." The pre-EROD18 hard-coded-constant ingress posture is replaced by topology-symbol resolution.
- EROD21 ([`erod21_disposition.md`](../work-packages/20260526-erod21-route-parity-rerun-and-hold-lift-disposition-001/artifacts/erod21_disposition.md)) records "EROD21 complete. GO issued; sediment-routing HOLD carried from EROD19/EROD20 is lifted."

**Finding**: the audit-row-214 gap is closed by code at HEAD. The 2026-05-25 finding was correct on its date; today's snapshot supersedes it for the same surface.

### 4.2 SC-SED-001 `GAP-SED-005` row is STALE at HEAD — contract documentation lags code

Despite EROD19/EROD21 closing the runtime-migration gap, the SC-SED-001 gap register has not been updated. At [`SC-SED-001.md:541`](../specifications/science-contracts/contracts/SC-SED-001.md#L541):

> | GAP-SED-005 | Baseline `route.for` segment-level branch family (`mshear 1..5`, upper-end deposition/detachment trees, post-detachment deposition closure) is now canonicalized but **not yet migrated into openWEPP runtime kernels**. | Hillslope sediment-routing process parity remains incomplete **until EROD19 runtime migration lands** with contract-derived closure. | **non-promotable** | `[DIRECT][Static] + [INFERENCE][Static]` |

Both the impact wording ("not yet migrated", "until EROD19 runtime migration lands") and the status (`non-promotable`) are wrong as of HEAD — EROD19 landed (`cbda790`) and EROD21 (`07d361c`) confirmed parity. No SC-SED-001 revision-history row exists past revision 12 to record this closure.

The same staleness affects the [`docs/specifications/science-contracts/index.md` SC-SED-001 row](../specifications/science-contracts/index.md), which still describes EROD16 opening `GAP-SED-005` without recording its closure.

**Finding**: contract documentation lags code by approximately one work-day. This is the *inverse* of the divergence the 2026-05-25 audit row 214 identified — yesterday the contract was about to lead code; today it trails. Until the gap-register update lands, anyone reading SC-SED-001 alone would conclude that hillslope sediment-routing parity is non-promotable, when in fact EROD21 cleared the parity HOLD.

This is exactly the kind of "hidden-contract" drift that ADR-0011's contract-first sequencing is designed to prevent in the opposite direction; the closure path needs symmetric discipline.

### 4.3 MOFE03 default constants remain at HEAD but semantic role changed

The `MOFE03_WAVE2_DEFAULT_XTOP = 0.2`, `_XBOT = 0.5`, `_XDETST = 0.1` constants are still present in HEAD at [`hillslope/mod.rs:1852-1854`](../../crates/openwepp-runner/src/hillslope/mod.rs#L1852-L1854), and still used both as seed insertions (L2008, L2012, L2016) and as `.unwrap_or(...)` fallbacks (L2070, L2072, L2074).

What changed between 2026-05-25 and HEAD is their **semantic role**, not their presence. Pre-EROD18 they were the *authoritative* values the kernel actually used because no topology-ingress mechanism existed. Post-EROD18 the runner projects topology-resolved values into the same boundary symbols via the EROD18 ingress seam; the constants now act as **fallbacks** when topology resolution does not produce a runtime-supplied value.

**Finding (partial)**: the constants are not necessarily dead code. Whether they ever execute at runtime depends on whether EROD18's ingress projection always populates the topology symbols for all OFE/event combinations, or whether some code paths still fall through to the defaults. This audit does **not** trace every code path that could leave the topology symbols unset. That is a separate audit task — likely "EROD18 ingress completeness audit" or absorbed into the eventual MOFE03 cleanup WP.

**Conservative reading**: the constants are no longer the documented authoritative-source-of-truth they were yesterday. Whether they are reachable at runtime is unverified by this audit.

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

### 4.8 Working-tree refactor005 modularization in progress

A `refactor005` mechanical-modularization stream is splitting `03_kernel_support.rs` (3,196 + 4,387 lines combined post-split) into `03_kernel_support_00_support_helpers.rs` + `03_kernel_support_01_kernel_phases.rs`. The split is uncommitted at audit time. Once landed, the kernel-line references in §4.1 will need re-anchoring; the EROD19 MSHEAR logic moves to `03_kernel_support_01_kernel_phases.rs`.

**Finding (informational)**: not a physics-state change, but flagging it because cross-references from this audit will become stale-by-path once refactor005 lands. The `[DIRECT][Static]` citations in §4.1 are correct for HEAD on the audit date.

## 5. Caveats

- **HEAD-anchored, working-tree-aware**: §4.1-§4.7 cite HEAD; §4.8 notes the in-flight refactor005 working-tree state. Both views are honest but a reader pulling at a later time may see refactored paths or amended contracts.
- **Audit author conflict-of-interest disclosure for §4.5**: the HR backlog and HRREF-01 WP were authored by the same Claude Code session earlier today. The §4.5 finding is intentionally narrow.
- **No numerical comparison performed**: the §4.1 claim that the MSHEAR migration has "landed" is a *structural* claim — the symbols are present, the disposition records GO. This audit does **not** verify per-OFE numerical parity against `route.for` reference output; that was performed by the EROD21 WP and is documented in its disposition artifacts, not re-verified here.
- **§4.3 reachability uncertainty**: this audit does not trace every code path that could leave the EROD18 topology symbols unset and fall back to the MOFE03 constants. That is a separate audit-task or a §6 follow-up. Conservative position: the constants are no longer authoritative documented values, but their runtime reachability is unverified.
- **Scope excludes non-erosion subsystems**: ET, snow, frost, impoundment, and channel kernel status carries forward from 2026-05-25 unchanged.
- **EROD19 diff sampling**: read approximately 60 lines around the `mshear`/`xc1`/`xc2` introduction site, not the full 504-line diff. Other algorithmic content of EROD19 (e.g. upper-end deposit-vs-detach state handling, post-detachment deposition closure) is referenced from the contract addendum text and the disposition narrative, not from line-by-line code reading.
- **Sampling discipline**: only the erosion-relevant subset of today's commits was inspected in detail. Codex's other concurrent work (e.g. frost lineage, refactor stream) was acknowledged but not read.

## 6. Recommended follow-ups (not performed in this audit)

1. **Update `GAP-SED-005`** in [`SC-SED-001.md:541`](../specifications/science-contracts/contracts/SC-SED-001.md#L541) to record EROD19/EROD21 closure. Move the row to `closed` (or `closed-with-residual-watchpoint` if appropriate). Add a revision-history row past revision 12 capturing the closure. Same for the [`science-contracts/index.md`](../specifications/science-contracts/index.md) SC-SED-001 row. *Cross-reference: this is the symmetric move to ADR-0011's contract-first opening sequence — once code closes a gap, the contract must record it.*
2. **EROD18 ingress completeness audit**: confirm that the topology-symbol family introduced by EROD18 is always populated by the runner for every OFE/event combination that can reach `run_erod14_wave2`, and either (a) verify the MOFE03 default fallbacks are unreachable and remove them, or (b) document the runtime conditions under which they remain authoritative. *Not performed by this audit — see §4.3.*
3. **Refactor005 landing audit-update**: once refactor005 commits, the kernel cross-references in §4.1 and §4.8 of this audit will become stale-by-path. Either add a clarifying note here (per audits/CLAUDE.md "Exception: typo corrections, dead-link repair...") or write a small follow-up audit re-anchoring the kernel references to the split file layout.
4. **HR backlog progression**: HRREF-01 (references intake) is scaffolded; subsequent WPs for `SC-SED-HR-001` authoring and `Customizable-class architecture` decision are listed in the backlog's `## Work-package linkage` section but not yet authored. Not authored by this audit.

## 7. Cross-references

- [`20260525_water_erosion_kernel_audit.md`](20260525_water_erosion_kernel_audit.md) — prior-day snapshot; remains canonical for kernel inventory (§3-§5) and the full cross-reference table (§10).
- [`docs/specifications/science-contracts/contracts/SC-SED-001.md`](../specifications/science-contracts/contracts/SC-SED-001.md) — contract authority (revision 12; gap register STALE per §4.2).
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
- [`docs/decisions/0011-architecture-first-top-down-science-contracts.md`](../decisions/0011-architecture-first-top-down-science-contracts.md) — contract-first sequencing authority; the §4.2 contract-trails-code drift is the inverse of the contract-first pattern.
