# Kernel-Boundary CQR Burndown ExecPlan

Status: **active** — **CRAP-reduction** (code-quality) burndown following the symbol-map
kernel-boundary deletion; coverage is the secondary closure gate.
Dispatch surface: **main** (packages run on `main` directly, not in a worktree).
Owner: maintainers (Claude Code maintains this doc; Codex runs the CQR packages).
Last updated: 2026-07-01.

## Why this exists

The wholesale kernel-boundary deletion (`a381702b`, merged `c588023e`) reached the
single-authority terminal state but deleted 85 test files (~657 tests; the suite went
1,878 → 1,221) with zero replacements. That collapsed the coverage term on the modules
that lost their tests, **spiking CRAP** (`CC² · (1−cov)³ + CC`, ADR-0021, non-conforming
above 30) on exactly those modules. This burndown is a series of **CQR (Code Quality
Refactor)** packages that bring those modules back under the CRAP threshold.

## Closure model (primary = CRAP, secondary = coverage)

**Primary closure gate — CRAP reduction.** Every production function in the package's
owned scope with CRAP > 30 (ADR-0021) is brought to ≤ 30 by one or more of:
1. **cyclomatic-complexity refactor** — decompose, extract, table-drive, collapse
   error-variant construction (the central "code quality refactor" lever; lowers `CC`);
2. **coverage** — exercise the function (lowers the `(1−cov)³` term);
3. **explicit disposition** — for inherently-low-coverage error/guard/formatting code,
   an ADR-0021-style `complete-with-warnings` disposition with named rationale (do **not**
   manufacture tests for unreachable error-message variants just to move a number).

**Secondary closure gate — coverage.** The package restores the module's **ratified
contract assertions** that the deletion removed (the specific `SC-*#INV-*` / WB-identity /
conservation checks — e.g. FC/WP policy, BASECOND01 harmonic-`ssc`), stated at the stable
contract surface (typed boundary / contract, never the symbol surface). No net line-
coverage regression on any touched file versus the CRAP-before baseline.

> **CRAP ≠ contract coverage.** A function can score CRAP-clean on line coverage while its
> *specific* deleted contract assertion is gone (integration tests exercise the lines
> incidentally). The primary gate uses CRAP; the secondary gate independently verifies the
> named contract assertions exist. A module passing primary does **not** waive secondary.

## CRAP-before baseline (measured 2026-07-01 on `main` c540d1d6)

Ran: `cargo llvm-cov clean --workspace && cargo llvm-cov --workspace --ignore-run-fail
--lcov --output-path lcov.info` → `cargo crap --workspace --lcov lcov.info --min 0
--format json`. Production functions only (test files excluded). 141 source files in LCOV.

| # | Module (owned files) | funcs | mean cov% | max CRAP | mean CRAP | **#>30** | worst function (cc / cov% / CRAP) |
|---|---|---:|---:|---:|---:|---:|---|
| 4 | core-types + snow/frost/irrigation inputs (`runtime_inputs/{00_core_types,04_snow_frost_irrigation,05_projection_helpers,06_simimpl28,07_series_helpers}`) | 178 | 51.1 | 210 | **29.9** | **48** | `HillslopeRuntimeInputError::soil_c…` (14/0/210) |
| 7 | WB publication (`direct_runtime/01_publication` + runner `direct_publication/**`, `direct_seed_projections/**`) | 440 | 68.0 | 342 | 11.5 | **32** | `layered_snow_frost_insulation_dept…` (18/0/342) |
| 9 | direct_runtime kernel physics + typed boundary (`direct_runtime/{evapotranspiration,runoff,storage,00_core_frames,diagnostic_events}`, `support_helpers_mod/typed_boundary`) | 496 | 79.9 | **1260** | 13.6 | **28** | `DirectEvapotranspirationPmetCompute…` (35/0/1260) |
| 5 | hydrology WB kernel (`src/hydrology/**`) | 404 | 76.4 | 600 | 12.6 | **22** | `Wb11HydrologyKernelGuardError::code` (24/0/600) |
| 6 | growth + decomposition (`direct_runtime/growth`) | 78 | 88.8 | 403 | 15.3 | 4 | `DirectGrowthInputs::validate_sched…` (37/36/403) |
| 8 | per-OFE / MOFE (`direct_runtime/{subsurface,03_executor}`) | 166 | 83.2 | 62 | 7.4 | 4 | `maybe_write_r7h_percolation_trace` (8/6/62) |
| 3 | management + plant parser (`runtime_inputs/01_management`) | 112 | 77.1 | 31 | 6.5 | 2 | `project_primary_drain_controls` (9/36/31) |
| 1 | soil parser (`runtime_inputs/02_soil_slope`) | 58 | 78.8 | 28 | 7.7 | **0** | `project_typed_soil_wb11_runtime` (18/69/28) |
| 2 | climate parser (`runtime_inputs/03_climate`) | 12 | 93.3 | 5 | 2.2 | **0** | `HillslopeClimateRuntimeRequest::…` (4/60/5) |

**Read the numbers with two caveats:** (1) a large share of the `>30` count is
**error/guard-path CRAP** (`GuardError::code`, `RuntimeInputError::*` at 0% cov) —
disposition-eligible, not physics defects; (2) real **physics** hotspots also sit at 0%:
the ET kernel `evapotranspiration::compute` (1260), `layered_snow_frost_insulation_depth`
(342), `DirectGrowthInputs::validate_schedule` (403 @ 36%). Coverage did **not** uniformly
collapse — mean per-module coverage is 51–93% because surviving integration/contract tests
still exercise most code; the damage is concentrated.

## Sequencing (by measured CRAP burden)

Ordered by `#>30`, highest first (superseding the earlier "contract-bearing first" order):

- **Tier 1 (CRAP hotspots):** **#4** (48) → **#7** (32) → **#9** (28) → **#5** (22).
  Lead each with its monster function (row 9 → ET `compute` 1260; row 5 → guard 600).
- **Tier 2:** #6 (4, but the `validate_schedule` 403 is a real target) → #8 (4) → #3 (2).
- **Tier 3 (CRAP-clean; secondary-coverage only):** #1, #2 — primary gate already met;
  they owe only the ratified-contract assertions (see stash note below for #1).

Each row is an independent CQR package **dispatched on `main`**, with dual review, the
`.rs` line-count governance check, full gates (`fmt`/`clippy`/`nextest --profile full`/
`deny`/authority guards), and a re-measured CRAP-after in its artifacts proving every owned
`>30` function is ≤30 or dispositioned.

## Autonomous execution protocol

This burndown is run by Codex **autonomously, end to end, on `main`** — run the rows in the
sequenced order (tier 1 → tier 3), one CQR package at a time, committing each before the next.

**Per-package definition of done** (all required before moving on):
1. **Primary (CRAP):** every production function in the owned scope with CRAP > 30 is ≤ 30,
   or carries an explicit ADR-0021 `complete-with-warnings` disposition with named rationale
   (inherently-uncoverable error/guard/formatting variant). A **CRAP-after** re-measure (same
   `cargo llvm-cov` → `cargo crap` method) sits in the package artifacts, diffed against the
   baseline above.
2. **Secondary (coverage):** the module's deleted ratified-contract assertions are restored at
   the stable contract surface; no net line-coverage regression on any touched file.
3. **Behavior-preserving:** **H2637 byte/value output identity holds.** CRAP-reduction refactors
   must not change physics/publication output — this is the hard guard for the physics rows
   (5/6/7/9). A refactor that cannot preserve H2637 identity is a stop condition, not a commit.
4. **Gates:** `fmt` / `clippy -D warnings` / `nextest --workspace --profile full` / `deny` /
   authority guards green; `.rs` line-count governance dispositioned; dual review with every
   finding dispositioned.
5. Commit on `main` with the CRAP-before→after delta in the message.

**Continue autonomously** through all rows. **Stop and surface only on a genuine blocker:** a
CRAP target unreachable without a science/contract decision; an H2637 identity break a
behavior-preserving refactor cannot avoid; an unresolvable gate failure; or a genuine design
gap. Do **not** stop on scope, entanglement, or a large diff.

**Completion:** when every row is done, re-measure workspace CRAP (no production function > 30
remains un-dispositioned), then close the burndown — flip ROADMAP item K to closed and update
`docs/backlog/TRACKER.md`.

**Safety / rollback:** the regression net is the surviving suite (`--profile full`) + H2637
identity, per package, with git rollback. Restored coverage only strengthens it. Roll back a
package on a genuine regression rather than forcing it.

## Retired with justification (delete, do NOT restore)

Symbol-map-runtime tests, genuinely obsolete:
- `tests_mod/{writeback.rs (1875), day_frame.rs, boundaries.rs, phase.rs, schedule_export.rs}` — symbol writeback / scheduler phase / boundary export.
- `tests03/trace.rs`, `integration/kernel_writeback_contract.rs` — superseded by the typed events/boundary (row 9).
- `*_runtime_surface_projects_*` / `*_contains_canonical_state_symbols` cases — extract the *behavior* assertion into the owning row, drop the symbol-projection assertion.

## Preserved prior work

Codex's stopped **row-1 (soil) package** (a full WP + a 176-line
`tests/integration/infile_soil_parser_contract.rs` restoring FC/WP + harmonic-`ssc`
assertions) is in `git stash@{0}` ("codex stopped row-1 soil CQR…"). Soil is CRAP-clean, so
row 1 is Tier 3; that stashed test is the head-start for its **secondary-coverage** pass —
recover via `git stash apply` (or cherry-pick the test) rather than rewriting it.
