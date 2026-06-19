# PERFDEEP05 - Lane-Dense Transfer Authority and Sync Removal

Status: scaffolded 2026-06-19 (queued; not executed).

Package type: performance implementation / bounded PERFDEEP04 follow-on.

## Objective

Remove the measured PERFDEEP03 lane-dense resynchronization hotspot by making
the PERFDEEP03 opt-in path apply transfer and hot carryover updates directly to
lane-owned dense state. Preserve output identity, keep default production
disabled, and remeasure the real H2637 endpoint before any further expansion.

## Rationale

PERFDEEP04 profiled the PERFDEEP03 no-go and found the dominant opt-in-only
hotspot:

```text
HillslopeLaneDenseState::sync_from_writeback_surface
33.49% inclusive, 14.19% self
```

The current opt-in path applies transfer input to logical/indexed surfaces,
then resynchronizes lane-dense state from those surfaces, then flushes dense
dirty slots back to logical/indexed surfaces. That is dense state nested inside
a compatibility loop. PERFDEEP05 removes that loop at the measured edge.

This package is not a whole-simulation dense rewrite and not a kernel-physics
rewrite. If sync removal does not make the opt-in path endpoint-flat or positive,
the next decision may be the larger kernel-body rewrite identified by the
PERFDEEP03 independent review; this package must not hide that result.

## Scope

In scope:

- remove `sync_from_writeback_surface` from the PERFDEEP03 opt-in H2637 daily
  hot loop;
- apply MOFE transfer input directly to `HillslopeLaneDenseState` when the
  lane-dense path is active;
- keep logical/indexed transfer materialization only for non-migrated consumers,
  diagnostics, output/publication, and default-disabled execution;
- cache hot state/flux slot metadata or precomputed symbol lists so the daily
  loop does not rebuild hot-symbol vectors through `HotSymbolTables`;
- prefer indexed/dense writeback application so dense updates do not call
  `SymbolRegistry::id_of` on hot writeback payloads;
- preserve PERFDEEP03 identity gates and fail-closed env-gate behavior.

Out of scope:

- default activation;
- whole-simulation dense array conversion;
- erosion/growth island expansion;
- output schema changes;
- science/numeric formula changes;
- deleting logical/indexed surfaces globally;
- rewriting hydrology kernel bodies to direct typed physics.

## Required Reading

- `AGENTS.md`
- `crates/AGENTS.md`
- `docs/work-packages/AGENTS.md`
- `docs/work-packages/20260619-perfdeep03-persistent-lane-owned-dense-state-001/package.md`
- `docs/work-packages/20260619-perfdeep03-persistent-lane-owned-dense-state-001/artifacts/perfdeep03_disposition.md`
- `docs/work-packages/20260619-perfdeep03-persistent-lane-owned-dense-state-001/artifacts/review-claude-independent.md`
- `docs/work-packages/20260619-perfdeep04-profile-perfdeep03-lane-dense-no-go-001/artifacts/perfdeep04-profile-results.md`
- `docs/work-packages/20260619-perfdeep04-profile-perfdeep03-lane-dense-no-go-001/artifacts/perfdeep04-next-package-recommendation.md`
- `docs/architecture/array-native-runtime-specification.md`
- `docs/ROADMAP.md`

## Intended Write Set

- `crates/openwepp-hillslope-orchestrator/src/day_frame.rs`
- `crates/openwepp-hillslope-orchestrator/src/scheduler.rs`
- `crates/openwepp-hillslope-orchestrator/src/scheduler/**`
- `crates/openwepp-hillslope-orchestrator/src/tests/**`
- `crates/openwepp-kernel-contract/src/lib_mod/core_types/00_symbol_registry_and_indexed_surfaces.rs`
- `crates/openwepp-kernel-contract/src/lib_mod/core_types/02_boundary_values_and_kernel_requests.rs`
- `crates/openwepp-runner/src/hillslope/**`
- `tests/integration/mofe01_per_ofe_state_contract.rs` only if structural path
  assertions need mechanical updates
- `docs/work-packages/20260619-perfdeep05-lane-dense-transfer-authority-sync-removal-001/**`
- `docs/work-packages/README.md`
- `docs/ROADMAP.md`
- `docs/architecture/array-native-runtime-specification.md`

## Phase Plan

1. Establish the static hot-loop proof: locate every current
   `sync_from_writeback_surface` call and classify whether it is hot-loop,
   initialization, diagnostic, or test-only.
2. Add dense transfer-application API on `HillslopeLaneDenseState` or adjacent
   scheduler helpers.
3. Route PERFDEEP03 opt-in MOFE transfer updates to dense state directly, while
   preserving logical/indexed materialization for required boundaries.
4. Cache hot symbol/slot metadata so hot state/flux symbol lists are not rebuilt
   in the daily loop.
5. Remove symbol-registry lookup from dense writeback apply where indexed
   payloads are available.
6. Add focused tests proving dense transfer mutation, dirty-slot behavior, and
   default-disabled behavior.
7. Run H2637 identity and endpoint gates with `OPENWEPP_PERFDEEP03_LANE_DENSE_STATE=1`.
8. Run a matched profile sufficient to show whether the PERFDEEP04 sync hotspot
   is gone.
9. Update package artifacts, roadmap/spec disposition, and line-count
   governance.

## Acceptance Criteria

- Static proof: `sync_from_writeback_surface` is not called in the PERFDEEP03
  opt-in H2637 daily hot loop.
- Focused tests pass for direct dense transfer update and dirty-slot flush.
- Default-disabled behavior remains unchanged and does not require lane-dense
  state.
- H2637 identity passes:
  - HBP byte identity;
  - WAT byte identity;
  - PASS Arrow equivalence;
  - roundtrip diagnostic remains zero-mismatch if run.
- Real opt-in H2637 endpoint and RSS are measured against the PERFDEEP01
  `669.97 s` reference.
- PERFDEEP04 hotspot is re-profiled or otherwise directly disproven:
  `sync_from_writeback_surface` must not remain a top opt-in cost.
- Full Rust gates pass:

```text
cargo fmt --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
cargo deny check
```

- Markdown lint passes for the package and touched docs.
- No default activation unless the measured endpoint beats `669.97 s` and
  identity/full gates pass.

## Deliverables

- `artifacts/perfdeep05-implementation.md`
- `artifacts/perfdeep05-static-hot-loop-proof.md`
- `artifacts/perfdeep05-identity.md`
- `artifacts/perfdeep05-endpoint.md`
- `artifacts/perfdeep05-profile.md`
- `artifacts/perfdeep05-gate-results.md`
- `artifacts/perfdeep05_disposition.md`

## Subagent Requirement

None required. A second independent review may be added if the implementation
changes the package disposition from `NO-GO` to `CONTINUE`.

## Autonomy

Execute end-to-end when triggered. Do not stop after a diagnostic-only pass.
Do not activate the path by default. Do not expand the island or start a kernel
body rewrite inside this package unless the package is explicitly amended before
implementation.
