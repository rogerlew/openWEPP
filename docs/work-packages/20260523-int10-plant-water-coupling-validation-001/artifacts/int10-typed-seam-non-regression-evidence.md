# INT10 Typed Seam Non-Regression Evidence

Status: `complete`
Evidence mode: `Static + Ran`

## ARCH15/ARCH21 Posture Check

Static:

- INT10 package preserves typed seam behavior at scheduler-to-kernel boundaries.
- Coupled ordering and state-transfer checks are expressed through typed
  contexts, typed status IDs, and writeback-surface continuity checks.
- No silent fallback/clamp/default behavior was introduced.

Ran:

```bash
cargo test -p openwepp-hillslope-orchestrator annual_growth_phase_emits_typed_growth_context -- --nocapture
cargo test -p openwepp-hillslope-orchestrator perennial_growth_phase_emits_typed_growth_context -- --nocapture
cargo test --test parser_runtime_seam_integration management_rotation_fixture_projects_schedule_growth_and_decomp_runtime_surface_families -- --nocapture
cargo test --workspace
```

Result:

- Growth typed-context checks: `1 passed` each targeted test.
- Parser/runtime PL surface projection coverage check: `1 passed`.
- Workspace-wide integration/unit/doc tests: `ok`.
