# Stage 0 RSS Profile

Evidence class: Ran + Static

## Commands

- Ran `cargo build -p openwepp-runner --bin openwepp-cli-hill --release`.
- Ran H2637 current no-env direct production with `/usr/bin/time -v`:
  `target/release/openwepp-cli-hill --run-dir /tmp/perfho01/run-dirs/h2637
  --run-file /tmp/typed-direct-stage0/h2637/h2637.run --output-dir
  /tmp/typed-direct-stage0/h2637/output --manifest-path
  /tmp/typed-direct-stage0/h2637/output/manifest.json`.
- Ran the same H2637 case with a minimized runfile that requested only HBP and
  loss outputs:
  `/tmp/typed-direct-stage0/h2637_min/h2637_min.run`.
- Ran the small `tests/fixtures/cli01/hillslope_run_dir` fixture with the release
  CLI and `/usr/bin/time -v`.
- Ran a temporary out-of-repository type-size probe plus the existing
  `cargo test -p openwepp-hillslope-orchestrator
  r7b_constructor_type_size_layout_is_bounded -- --nocapture`.

## Measurements

| Case | Climate days | Contributor OFEs | Publication rows | Requested outputs | Elapsed | Max RSS |
| --- | ---: | ---: | ---: | --- | ---: | ---: |
| H2637 full | 12419 | 19 | 235961 | HBP, loss, WAT, PASS, plot | 1:09.18 | 1159672 KiB |
| H2637 minimized | 12419 | 19 | 235961 | HBP, loss only | 1:13.77 | 1159296 KiB |
| `cli01` | 2 | 1 | 2 | fixture defaults | 0:00.09 | 19584 KiB |
| PERFARCH03 floor | n/a | n/a | n/a | prototype floor | recorded prior | 3072 KiB |

The H2637 manifests confirm production direct execution in both H2637 runs:
`runtime_selection.selected=direct-production-executor`,
`execution_provenance.scheduler_kernel_executed=false`, and
`direct_runtime_counters.compatibility_edge_invocations=0`.

The H2637 minimized run has effectively the same RSS as the full-output run
(`-376 KiB`, within noise), so optional parquet/plot output files are not the
material RSS driver. The tiny `cli01` run is `19584 KiB`, so the observed
H2637 RSS is not explained by a fixed one-time setup allocation alone.

## Type-Size Evidence

Temporary probe results:

| Type | Size |
| --- | ---: |
| `DirectPublicationDayRow` | 544 bytes |
| `HillslopeWatRow` | 312 bytes |
| `HillslopePassRow` | 96 bytes |
| `DirectRunPublicationFrame` | 128 bytes |
| `DirectRunFrame` | 256 bytes |
| `DirectLaneFrame` | 1136 bytes |
| `DirectDayFrame` | 12400 bytes |

Existing layout test output:

- `DirectRunConstructorInputs=72`
- `DirectLaneConstructorInputs=1024`
- `DirectDayConstructorInputs=4040`
- `DirectRunFrame=256`
- `DirectLaneFrame=1136`
- `DirectDayFrame=12400`

For H2637, the obvious retained row lower bounds are:

- `DirectPublicationDayRow`: `235961 * 544 = 128362784 bytes`
  (`122.4 MiB`).
- `HillslopeWatRow`: `235961 * 312 = 73619832 bytes` (`70.2 MiB`).
- `HillslopePassRow`: `235961 * 96 = 22652256 bytes` (`21.6 MiB`).
- Those three visible vectors alone account for about `214.2 MiB`, before
  allocator overhead, cloning, `SimulationOwnedWb13Row`, manifest/provenance
  state, parquet writer materialization, or additional retained ledgers.

## Attribution

Stage 0 corrects the input hypothesis:

- The setup-time symbol-map `runtime_surface` is real and still violates the
  single-authority architecture, but it is not established as the dominant RSS
  allocation.
- The RSS is strongly associated with whole-run per-OFE-day retained publication
  state. H2637 builds `235961` day/OFE rows and stays near `1.16 GiB` even when
  optional WAT/PASS/plot outputs are not requested.
- The direct path also clones retained publication before building output
  artifacts, so the publication frame can be live in more than one owner during
  output assembly.

Conclusion: Stage 1 typed setup and Stage 2 carrier deletion remain valid
architecture work, but they are not the first RSS-reduction lever. The first RSS
lever is retained-publication streaming/drop work.
