# CQR25 Behavior Equivalence

Status: complete.

Static: the public entry point remains:

```rust
pub fn execute_hillslope_run(
    request: &HillslopeRunRequest,
    argv: &[String],
) -> Result<HillslopeRunReport, HillslopeCliError>
```

Static: behavior-preserving decomposition moved existing orchestration into
private phase helpers for:

- Run directory and output directory setup.
- Runfile, soil, management, slope, and climate parsing.
- Output path resolution.
- Legacy and runfile sidecar resolution.
- Static runtime-surface assembly.
- Persistent multi-OFE and single-lane climate-day execution.
- WB13, pass, WAT, optional output, and manifest publication.

Static: protected surfaces were not intentionally changed:

- Public API signatures.
- Manifest schema identifiers and fields.
- Sidecar discovery policy.
- Parser compatibility modes.
- Runtime symbols and aliases.
- Units and output schemas.
- Typed guard/error IDs.
- Scheduler lifecycle source-shape guard behavior.

Ran: final focused source-shape guard passed:

```text
cargo test -p openwepp --test mofe01_per_ofe_state_contract mofe01_mi_multiofe_runner_lifecycle_is_mutually_exclusive_with_single_ofe_aggregate_path
```

Ran: final workspace tests passed:

```text
cargo test --workspace
```
