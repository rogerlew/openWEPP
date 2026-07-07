# LANED Hybrid Abandonment Removal

Status: EXECUTED-COMPLETE-ADR0037-REMOVAL

## Objective

Execute ADR-0037: remove the abandoned hybrid implicit-explicit stepper
subsystem — **code and contract** — from main, after archiving the final
working state on the branch `abandoned/hybrid-implicit-stepping`. Keep the
historical record (all work-package directories, revision histories, the
execution log, the ADR) on main. Extract the two named durable-knowledge
items into `docs/numerics/` before deletion. Prove the plain active path is
byte-identical before and after the strip.

This is a removal package, not a deprecation: no dormant selector, no
quarantined code, no `#[ignore]`d hybrid tests remain on main.

## Baseline At Package Start

- `SC-OFEROUTE-002` rev 5 (no-harm selector) is the final hybrid authority.
- The no-harm selector package
  `20260707-laned-router-d16-hybrid-noharm-selector-solvecost-hold-lift-001`
  is executed with gates green (`1442/1442`) and may still be uncommitted.
- `INV-OFEHYB-007` guarantees hybrid-off plain-path byte identity — the
  property that makes this strip provably safe.
- ADR-0037 records the abandonment decision, grounds, and keep-list.

## Scope

Included:
- Archive-branch creation and provenance recording.
- Knowledge extraction to `docs/numerics/` (Z-shaped equilibrium rating;
  selector-determinism input-class policy).
- Contract-first removal: `SC-OFEROUTE-001` removal revision;
  `SC-OFEROUTE-002` deleted from main; registry row set `withdrawn` with an
  ADR-0037 pointer.
- Full code and test strip of the hybrid subsystem (inventory derived from
  the `SC-OFEROUTE-002` guard map / test-vector obligations / BEI **before**
  the file is deleted).
- Explicit posture decision for a set `OPENWEPP_LANED_ACTIVE_IMPLICIT`
  after removal (recommended: typed startup rejection naming ADR-0037; the
  alternative, silently ignoring the variable, must be argued if chosen).
- Plain-path byte-identity evidence on the four selected-cohort members.
- Review, verification, gate evidence, disposition, execution-log entry.

Excluded:
- Deleting or rewriting any work-package directory, revision-history entry,
  or execution-log record (the historical record stays per ADR-0037).
- Any change to plain active Lane D semantics, the plain Case-4 oracle
  ladder (`INV-OFEROUTE-011`), the `ow-lanuse-1` consumer path, the
  `canhgt` publication, or explicit-path profile counters.
- Tier-2 mesh work of any kind (its re-scope to a Δx-target policy is a
  separate future package).
- WEPPpy-side changes (separate repository).

## Required Reading

Core:
- `AGENTS.md`
- `docs/work-packages/AGENTS.md`
- `docs/specifications/science-contracts/AGENTS.md`
- `crates/AGENTS.md`
- `docs/decisions/0037-abandon-hybrid-implicit-stepping.md`
- `docs/specifications/science-contracts/contracts/SC-OFEROUTE-002.md`
  (read BEFORE deletion — its guard map, test-vector obligations, and BEI
  are the authoritative strip inventory)
- `docs/specifications/science-contracts/contracts/SC-OFEROUTE-001.md`
  (hybrid pointer rows to remove; everything else untouched)
- `docs/work-packages/20260707-laned-router-d16-hybrid-viability-adjudication-001/artifacts/review-claude.md`
  (CL-M3 test-retirement obligation this package discharges)
- `docs/work-packages/20260707-laned-router-d16-hybrid-noharm-selector-solvecost-hold-lift-001/artifacts/selector-policy.md`
  (the selector surface being removed)
- Package-local `artifacts/required-reading-map.md`

Conditional:
- `docs/standards/local-ci-gate-selection.md` for narrowed iteration gates.
- `docs/specifications/unit-governance.md` only if a runtime
  symbol/metadata surface change exceeds pure removal.

On-demand (the strip surface):
- `crates/openwepp-hillslope-orchestrator/src/ofe_routing/implicit_recession.rs`
- `crates/openwepp-hillslope-orchestrator/src/ofe_routing/cascade.rs`
- `crates/openwepp-hillslope-orchestrator/src/ofe_routing/kinematic_wave.rs`
- `crates/openwepp-hillslope-orchestrator/src/ofe_routing/profile.rs`
- `crates/openwepp-hillslope-orchestrator/src/ofe_routing/friction.rs`
- `crates/openwepp-hillslope-orchestrator/src/ofe_routing/dval.rs`
- `crates/openwepp-hillslope-orchestrator/src/direct_runtime/laned_active.rs`
- `crates/openwepp-runner/src/hillslope/laned_active.rs`
- `crates/openwepp-runner/src/hillslope/00_runner_intake_and_lane_setup.rs`
- `crates/openwepp-runner/src/hillslope/05_runner_execution_and_outputs.rs`

Required-reading budget:
- local_required_bytes_total: ~240000 (Core list; contracts dominate)
- threshold_outcome: OK (`<=400000` bytes)
- map: `artifacts/required-reading-map.md`

## Write Set

Primary:
- `docs/work-packages/20260707-laned-router-hybrid-abandonment-removal-001/`
- `docs/work-packages/README.md` (closure entry only)
- `docs/specifications/science-contracts/contracts/SC-OFEROUTE-001.md`
  (removal revision)
- `docs/specifications/science-contracts/contracts/SC-OFEROUTE-002.md`
  (DELETE from main)
- `docs/specifications/science-contracts/index.md` (row -> `withdrawn`,
  notes point at ADR-0037 + archive branch)
- `docs/numerics/README.md` + a new
  `docs/numerics/kinematic-wave-equilibrium-rating-z-structure.md`
- `crates/openwepp-hillslope-orchestrator/src/ofe_routing/` (hybrid strip:
  `implicit_recession.rs` deleted; `cascade.rs`, `kinematic_wave.rs`,
  `profile.rs`, `mod.rs`, and any bare-skin-evaluator residence per the
  guard-map inventory)
- `crates/openwepp-hillslope-orchestrator/src/direct_runtime/laned_active.rs`
- `crates/openwepp-runner/src/hillslope/` (selector intake, manifest
  counters, `laned_active.rs`)
- Focused tests under the same crates (hybrid vectors removed).

Protected:
- All existing work-package directories other than this one (read-only —
  the historical record).
- `SC-OFEROUTE-001` revision-history entries (append the removal rev; never
  rewrite prior entries).
- Plain-path routing code semantics: every non-hybrid line the strip
  touches must be mechanical removal, not behavioral edit.

## Phase Plan

### Phase A - Archive And Baseline Provenance

If the no-harm selector package is still uncommitted, commit it as its own
commit first. Create branch `abandoned/hybrid-implicit-stepping` at the
commit containing that executed package (the final working hybrid state)
BEFORE any removal edit; record the branch name and tip hash in
`artifacts/branch-provenance.md`. Rebuild the exact release runner (QA-M3
recipe: `-p openwepp-runner --bins`, record path/mtime/sha256) and capture
pre-strip active-plain baselines for H2637, `mn_corn_h4`,
`n_idaho_forest_h1`, `wa_cascades_forest_h1`: HBP and pass-parquet hashes
into `artifacts/plain-identity-baseline.md`.

### Phase B - Knowledge Extraction

Author the two `docs/numerics/` items from ADR-0037 Decision item 5 while
`SC-OFEROUTE-002` still exists to cite: the Z-shaped-rating note (bistable
equilibrium rating from the `INV-OFEROUTE-002` regime dispatch; basin
structure; why history-dependent seeding silently leaks conservation) and
the selector-determinism input-class policy line. Each cites ADR-0037 and
the archive branch as provenance.

### Phase C - Contract-First Removal

Derive the authoritative strip inventory from `SC-OFEROUTE-002` (guard map
test names, test-vector obligations, BEI rows, selector/counter surfaces)
into `artifacts/strip-inventory.md`. Then: append the `SC-OFEROUTE-001`
removal revision (delete the hybrid Branch/Guard row, hybrid test-vector
and BEI pointer rows; changelog entry cites ADR-0037 and the archive
branch), delete `SC-OFEROUTE-002.md`, update the registry row to
`withdrawn`. Run contract/BEI/unit gates for the touched contract.

### Phase D - Code And Test Strip

Execute `artifacts/strip-inventory.md`: delete `implicit_recession.rs`;
remove the hybrid composition from `cascade.rs` (cooldown predicate,
deficit-carry `absorb_deficit`/`dispose_terminal_carry`, hour-partition
guard, hybrid tests); restore the single fail-closed `run_with_options` in
`kinematic_wave.rs` (remove the composition-scoped deficit-returning
variant and its tests); remove implicit profile counters, selector
env/intake plumbing, manifest requested/selected/fallback counters, and
the hybrid fields in both `laned_active.rs` files; delete all hybrid tests
including the Case-4 HYBRID ladder (the plain Case-4 oracle ladder stays).
Implement and record the `OPENWEPP_LANED_ACTIVE_IMPLICIT` posture decision.
No behavioral edit to any surviving line.

### Phase E - Identity And Gates

Rebuild the release runner (same provenance discipline), rerun the four
active-plain members, and prove hash identity against
`artifacts/plain-identity-baseline.md` in
`artifacts/plain-identity-after.md`. Run the full canonical gate set.

### Phase F - Review, Verification, And Disposition

Dual review, finding disposition, dual verification, gate results, final
disposition, execution-log closure entry in `docs/work-packages/README.md`,
and worker handoff naming the first follow-on (expected: the Tier-2
re-scope to a Δx-target mesh policy as a separately scaffolded package).

## Subagent Authorization

Subagent authorization: this package explicitly authorizes
spawning/delegation to `comparator_suite_runner`, `rust_code_reviewer`,
`rust_qa_reviewer`, `explorer`, and bounded `worker` subagents for release
builds, the four-member identity runs, full workspace gates, code review of
the strip diff, and bounded codebase questions. Expected outputs are
package-local `artifacts/review-*.md`, `artifacts/verification-*.md`,
identity/gate evidence with command lines and log paths. Review,
verification, comparator, and explorer agents are read-only; worker write
access must be bounded to named files from the strip inventory.

Subagent requirement: REQUIRED for heavy batch/closure runs (full workspace
`nextest`, release identity runs, `cargo deny check`), unless the subagent
tool is unavailable; if unavailable, record command-level evidence before
running locally.

## Required Artifacts

- `artifacts/required-reading-map.md`
- `artifacts/branch-provenance.md`
- `artifacts/plain-identity-baseline.md`
- `artifacts/strip-inventory.md`
- `artifacts/plain-identity-after.md`
- `artifacts/implementation.md` (including the env-var posture decision)
- `artifacts/gate-results.md`
- `artifacts/review-*.md`
- `artifacts/verification-*.md`
- `artifacts/disposition.md`
- `artifacts/final-disposition.md`
- `artifacts/worker-handoff.md`

## Required Gates

- Archive branch exists at the final hybrid commit BEFORE any removal edit
  (hash recorded)
- Plain-path byte identity: HBP + pass-parquet hashes for all four members
  identical pre/post strip, exact release-binary provenance both sides
- `git diff --check`
- Markdown/doc lint for touched docs
- Contract/profile/BEI checks for `SC-OFEROUTE-001`; registry consistency
  after the `SC-OFEROUTE-002` withdrawal
- SC unit compliance for the touched contract
- Focused `ofe_routing` / active-lane tests (post-strip)
- `cargo fmt --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo nextest run --workspace --profile full` (expected count DROPS from
  `1442`; record the delta and reconcile it against the strip inventory —
  every removed test is named)
- `cargo deny check`
- `.rs` line-count governance
- Authority anti-evasion guard (test-vector obligations are touched):
  `bash tools/release/check_authority_suite_antievasion.sh` and
  `cargo nextest run --test auth11_required_suite_obligation_guards_contract`

## Exit Criteria

Complete only if:
- The archive branch preserves the final working hybrid state and its hash
  is recorded.
- Main contains no hybrid code, no hybrid tests, no `SC-OFEROUTE-002`, and
  no dormant selector; the env-var posture is decided and tested.
- The four-member plain-path identity gate passes exactly.
- The two `docs/numerics/` knowledge items are landed.
- The historical record (work packages, revision histories, execution log)
  is untouched except for this package's own closure entry.
- All required gates `PASS` or are explicitly non-applicable with evidence.

Hold if:
- Plain-path identity fails (the strip touched behavior — stop and
  root-cause before proceeding; this gate is the package's reason to
  exist).
- The strip inventory cannot be made complete from the contract surfaces
  (record the gap, extend the inventory, do not strip blind).
