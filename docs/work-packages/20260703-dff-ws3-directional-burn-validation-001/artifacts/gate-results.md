# Gate Results

Evidence class: Static + Ran.

Status: `EXECUTED-HOLD-DFF-WS3-SEDIMENT-PRODUCTION`

## Static Evidence

- No production runtime files remain in the final WS-3 diff.
- Current direct erosion authority still disables Wave-1 and seeds
  `DirectErod13Inputs::zero()`.
- `SC-SED-001` remains the owner for Wave-1/Wave-2 sediment authority; this
  package does not amend it.
- The matrix fixture manifest records source provenance, known climate header
  mismatch, temporary dewpoint normalization, and the sediment hold.

## Ran

The final gate commands for this package are recorded after execution below.

| Gate | Result | Notes |
| --- | --- | --- |
| `sha256sum -c SHA256SUMS` in the matrix fixture | `PASS` | All matrix fixture files verified |
| `sha256sum -c SHA256SUMS` in the p4 anchor fixture | `PASS` | `p4.run.toml` checksum verified |
| `cargo fmt --check` | `PASS` | Workspace formatting clean after rustfmt |
| `cargo test --test dff_ws3_directional_burn_validation -- --nocapture` | `PASS` | 2 passed, 0 failed, finished in 25.71 s after fixture whitespace normalization |
| `git diff --check` | `PASS` | No whitespace errors |
| `markdown-doc lint --path docs/work-packages/20260703-dff-ws3-directional-burn-validation-001` | `PASS` | 5 files validated, 0 errors, 0 warnings |
| `markdown-doc lint --path docs/work-packages/20260703-dff-ws3a-wave1-wave2-sediment-production-001` | `PASS` | 3 files validated, 0 errors, 0 warnings |
| `markdown-doc lint --path docs/work-packages/README.md` | `PASS` | 1 file validated, 0 errors, 0 warnings |
| Full Rust closure loop | `NOT RUN` | No production Rust implementation; package is held before Wave-1/Wave-2 changes |

## Representative Values

Diagnostic representative p4 high-burn direct-runtime output:

- rows: `2192`
- total `runvol`: about `59079.736 m3`
- max daily `runvol`: about `925.331 m3`
- max `peakro`: about `8.238e-6 m3/s`
- sediment columns: zero under the current production path

Diagnostic representative p1 unburned direct-runtime output:

- rows: `2192`
- total `runvol`: about `57222.623 m3`
- max `peakro`: about `6.759e-6 m3/s`
- sediment columns: zero under the current production path

These values support the runoff/peak direction check and the sediment hold; they
are not sediment acceptance targets.
