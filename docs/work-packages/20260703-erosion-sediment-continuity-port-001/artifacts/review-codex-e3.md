# Codex Review - Increment 2c / ROADMAP E.3 Multi-OFE Wave-1 Chaining

Reviewer: Codex, 2026-07-04. Branch/worktree reviewed:
`erosion-e3-multi-ofe-chaining` at `cf9f2c22`, plus the local dirty
worktree noted below.

Evidence class: **Static + Ran**. This was a focused implementation review,
not a full-suite re-run.

## Findings

1. **Medium - The reviewed branch head is not self-contained: the CLI03
   contract-derived expectation update is uncommitted.**

   Evidence: `git status --short` reports
   `M tests/integration/cli03_runner_contract_derived_tests.rs`; the local
   diff changes `cli03_mofe03_multiofe_runfile_executes_wave2_without_manual_symbol_injection`
   to the E.3 Wave-1-chain expectations (`erod14_wave2_enabled = false`,
   `erod14_qin_source_policy = wave1-hourly-sediment-coupled-handoff`,
   `erod14_qin_sediment_coupled = true`). That file is not in
   `git diff --name-only origin/main...HEAD`, so `cf9f2c22` alone still
   carries the pre-E.3 Wave-2 contract assertion.

   Failure mode: the package's "full suite green" evidence is not
   reproducible from the pushed branch head alone; it depends on a dirty
   worktree update to a contract-derived test. A clean checkout of the
   reviewed branch can retain the stale Wave-2 expectation while the
   implementation and contracts now publish the Wave-1 handoff.

2. **Medium - `INV-SED-016(f)` says skipped flux quanta have a surfaced
   `flux_refused_quanta` count, but the count is internal-only.**

   Evidence: `SC-SED-001` v44 requires that a quantum refused by the
   flux-consistency diagnostic "contributes zero sediment with a surfaced
   `flux_refused_quanta` count." The implementation increments and stores
   that field in `DirectWave1ContinuityState`
   (`crates/openwepp-hillslope-orchestrator/src/direct_runtime/erosion.rs:887`,
   `:900`, `:943`; state field at
   `crates/openwepp-hillslope-orchestrator/src/direct_runtime/erosion_continuity.rs:263`),
   but `rg flux_refused` shows no publication, manifest, pass/HBP, or
   runner surface consuming it.

   Failure mode: the production path can silently under-estimate sediment by
   zeroing refused quanta while no persisted artifact tells an operator or
   downstream reviewer that it happened. That weakens the contract's stated
   distinction between a named diagnostic under-estimate and an invisible
   dropped sediment quantum.

3. **Medium - The load-bearing `param.for:249-390` inter-OFE continuity
   rewrite lacks a direct regression or alias-separated effect test.**

   Evidence: `Wave1InterOfeContinuity` and `inter_ofe` are only found in the
   implementation paths (`erosion_continuity.rs`, `erosion_seed.rs`,
   re-exports); no test constructs an `inter_ofe` payload or asserts the
   rewritten shear/transport coefficients against a fixture or legacy-derived
   expected value. The p102 integration test proves that a 2-OFE chain runs,
   exports sediment, closes the HBP chain identity, and responds to OFE-2
   texture changes, but those assertions are not specific to the
   coefficient-continuity rewrite and can be satisfied by other parts of the
   handoff.

   Failure mode: `INV-SED-016(c)` could regress to discontinuous boundary
   shear/transport coefficients while mass closure, nonzero sediment, and
   composition-motion tests still pass. This is the highest-risk math in E.3
   and deserves a small direct test that fails if the rewrite is removed or
   if the receiver-side `strldn`/`sheart` continuity inputs are aliased to
   the wrong OFE basis.

## Readiness

Design revisions needed before merge. The overall architecture is sound:
the same-day upstream-to-downstream lane lifecycle is plausible, the HBP
EVENT is exit-scoped with chain totals, and the p102 fixture proves a real
multi-OFE sediment path. The findings above are about merge/package
truthfulness, an observability promise in the contract, and missing
load-bearing regression coverage.

## Ran

- `git diff --check origin/main...HEAD`
- `git diff --check`
- `cargo fmt --check`
- `cargo clippy -p openwepp-hillslope-orchestrator -p openwepp-runner --all-targets -- -D warnings`
- `cargo test --test erosion_multi_ofe_p102_chain -- --nocapture`
- `cargo test -p openwepp-hillslope-orchestrator direct_runtime_wave1_continuity -- --nocapture`
- `cargo test --test cli03_runner_contract_derived_tests cli03_mofe03 -- --nocapture` (ran on the dirty worktree that includes the uncommitted CLI03 expectation update)
- `cargo deny check`

Not run: full workspace `nextest --profile full`.

## Static Checks

- Line count: `erosion_continuity.rs` is 2223 lines, above the 2000-line WARN
  threshold and below the 3000-line BLOCK threshold. The package already
  records split intent around the natural E.3/E.4 solver seam; no additional
  line-count blocker found.
- Dirty worktree at review start/end: existing local modification in
  `tests/integration/cli03_runner_contract_derived_tests.rs` and untracked
  `papers/`. This review artifact is the only file added by Codex.
