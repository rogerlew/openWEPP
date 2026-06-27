# No Production Wiring Scan

Status: complete
Evidence mode: Static + Ran

This package is crate-only. Record evidence that production snow/frost behavior,
selectors, schemas, and defaults did not change.

Suggested checks:

- Static: `git diff --name-only HEAD -- crates/openwepp-hillslope-orchestrator crates/openwepp-runner crates/openwepp-climate-runtime-adapter crates/openwepp-input-contract`
- Static: `rg -n "Harder|Pomeroy|psychrometric|openwepp_meteorology|openwepp-meteorology" crates/openwepp-hillslope-orchestrator crates/openwepp-runner crates/openwepp-climate-runtime-adapter crates/openwepp-input-contract`
- Static: review `Cargo.toml` dependency edges so production crates do not consume the new crate in this package.

## Commands

- Ran: `git diff --name-only HEAD -- crates/openwepp-hillslope-orchestrator crates/openwepp-runner crates/openwepp-climate-runtime-adapter crates/openwepp-input-contract crates/openwepp-hillslope-output crates/openwepp-watershed-output crates/openwepp-legacy-bridge`
- Ran result: no output.
- Ran: `rg -n "Harder|Pomeroy|psychrometric|openwepp_meteorology|openwepp-meteorology" crates/openwepp-hillslope-orchestrator crates/openwepp-runner crates/openwepp-climate-runtime-adapter crates/openwepp-input-contract crates/openwepp-hillslope-output crates/openwepp-watershed-output crates/openwepp-legacy-bridge || true`
- Ran result: no output.
- Ran: `cargo metadata --no-deps --format-version 1 | jq -r '.packages[] | select(.dependencies[]?.name == "openwepp-meteorology") | .name'`
- Ran result: no output; no package depends on `openwepp-meteorology`.
- Static: root `Cargo.toml` contains `crates/openwepp-meteorology` only as a
  workspace member. Existing production crate manifests do not reference it.

Gate table:

| Surface | Status | Evidence |
|---|---|---|
| Production runtime wiring unchanged | PASS | Ran: production diff-name scan returned no files; production rg scan returned no references. |
| Runtime/default selectors unchanged | PASS | Static: no parser/runfile/runtime selector files touched; no production crate dependency edge. |
| Public output schemas unchanged | PASS | Static: no output crate files touched; production diff-name scan returned no output files. |
| `RST` path unchanged | PASS | Static: no runner/orchestrator/input files touched; new crate is not invoked by production partitioning. |
