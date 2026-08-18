# Gate Results

Status: `CONTAINMENT PASS / coupled oracle not yet run`

No completion gate is claimed before the canonical midnight and complete
48-interval provider day pass.

Containment gates on the exact restored runtime:

- `cargo check -p openwepp-vegetation -p openwepp-land-surface-energy -p openwepp-hillslope-orchestrator`: PASS.
- `cargo nextest run -p openwepp-vegetation -p openwepp-land-surface-energy --profile quick --no-fail-fast`: PASS, 315/315, run `342cadac-e12c-4e98-91da-946e9bad36ff`.
- `cargo fmt --all -- --check`: PASS.
- `git diff --check`: PASS.

The restored `Cargo.toml`, vegetation constitutive source, LSE solver, and
canonical V13/V3 contract blobs exactly match commit `7bda42a56`. The active
V10 registry definition and executable V10 integration target are absent.
