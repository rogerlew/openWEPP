# Endpoint Validation

Status: `EXECUTED`

Evidence mode: `Ran`

## Build Provenance

The post-correction endpoint binary was built with:

    cargo build --release -p openwepp-runner --bin openwepp-cli-hill

Binary: `/home/workdir/openWEPP/target/release/openwepp-cli-hill`, size
`10687800` bytes, SHA-256
`a822036fd327c2f54d877ab51dc6c2e9aae13accff2ad4a61c154cbd730a131d`.
The manifests record that exact path and hash. Source was the working tree
based on `main@9fa0a294a0b8cd2db2abdefa38e15a2d7da0d73f`; the contract and solver
changes were uncommitted package work, so the binary hash is the operative
build identity.

## Canonical 34-Year H2637 Results

Both runs used the same staged, hash-pinned 1987--2020 H2637 inputs, target
mesh `dx = 5 m`, maximum routing step `300 s`, production-default runtime,
compat policy, legacy sidecar discovery, and all five outputs.

| Run | Effective mode evidence | Exit | User | Wall | Max RSS |
| --- | --- | ---: | ---: | ---: | ---: |
| `wepp_ui = false` | requested/effective `0/0`, daily lane, no divergence | `0` | `268.38 s` | `4:28.52` | `71928 KiB` |
| `wepp_ui = true` | requested/effective `1/1`, hourly lane, no divergence | `0` | `269.48 s` | `4:29.65` | `70164 KiB` |

Each run saw `12419` days and routed `10744`. Their complete active closure
blocks are byte-for-value identical and remain at numerical scale; the exact
operands are recorded in `fidelity-and-byte-identity.md`. Direct runtime
counters report `skeleton_runs = 0`, `compatibility_edge_invocations = 0`,
and `publication_capture_runs = 1`.

The false-mode manifest is:
`/tmp/openwepp_laned_nob_001_post_a822036f/output/openwepp_hillslope_run_manifest.json`.
The effective true-mode manifest is:
`/tmp/openwepp_laned_nob_001_post_a822036f/output-ui-effective-manifest/openwepp_hillslope_run_manifest.json`.

Core output hashes common to both modes are:

| Output | SHA-256 |
| --- | --- |
| HBP | `83b6b3a653fd5c25693254047a6045ef5d8bc67326362a2d2ee3c11a3cfa8f4c` |
| pass parquet | `bccb28f7ef218cc83df60302831cdcdd1e79ae318dada079b3072f373dfab5b5` |
| water parquet | `70b96af83f4853a6376386e67ef873848aecac33bb88d1f45e338ab3af56642e` |

## Sidecar-Discovery Surprise

A first true-mode attempt set only TOML `wepp_ui = true` while using
`--legacy-sidecar-discovery`; it exited `0`, but its manifest truthfully
reported requested/effective `0/0`. It is not counted as acceptance evidence.
Adding the canonical empty `wepp_ui.txt` sidecar and rerunning produced the
required requested/effective `1/1` hourly execution above. This discovery
changes only evidence interpretation; no sidecar-selection code was edited.

## Before/After Disposition

The pre-correction binary deterministically failed at lane 8/day 2621 after
`48.35 s` user time with `NegativeOutletBin`. The post-correction binary
completes the full 34-year endpoint in both effective modes with all closure
hard-fails green. This closes the audit's missing completed-active timing
measurement without imposing a performance target.
