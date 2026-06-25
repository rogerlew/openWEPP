# Line-Count Governance

Static:

```text
  937 crates/openwepp-runner/src/hillslope/snowbench_physics_bulk.rs
  199 tools/snowfreeze_observed/physics_bulk_snotel_profile.py
   87 tests/integration/snowdensity03_physics_bulk_offline_contract.rs
   94 crates/openwepp-runner/src/bin/openwepp-snowbench.rs
   21 crates/openwepp-runner/src/hillslope/mod.rs
   44 crates/openwepp-runner/src/lib.rs
   98 docs/work-packages/20260625-snowdensity-03-offline-physics-core-001/package.md
 1480 total
```

Assessment:

- The new Rust module is intentionally self-contained because the package
  boundary requires offline candidate physics, CSV/JSON/Markdown evidence
  publication, and internal unit tests without production runtime coupling.
- The CLI edits are scoped to adding one explicit subcommand while preserving
  existing `export-pysnobal` behavior.
- The Python harness is a thin profile runner that reuses the existing SNOTEL
  rubric machinery rather than copying scoring logic.
- No extraction is required before SNOWDENSITY-04. If the physics envelope grows
  beyond bulk state during adjudication, split the solver core from file
  publication before runtime coupling.
