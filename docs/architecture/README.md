# Process Architecture

## Data flow

```
inputs (.run + soil + management + climate + watershed)
    |
    v
+----------------------------+
| openwepp-cli-hill          |   one process per hillslope
| - parse inputs             |
| - run daily loop           |
| - emit HBP shard           |
| - emit parquet             |
+----------------------------+
    |
    v
HBP shards on disk: H<id>.hbp
parquet per hillslope
    |
    v
+----------------------------+
| openwepp-cli-watershed     |
| - load watershed structure |
| - load HBP shards          |
| - route channels           |
| - emit watershed parquet   |
+----------------------------+
    |
    v
watershed parquet outputs


debugging path:
HBP shard -> openwepp-replay -> trajectory diff / kernel isolation / window re-run
```

## Process model
Subprocess-per-hillslope. The watershed CLI spawns hillslope CLI subprocesses; inter-binary state crosses the filesystem as HBP shards. See [../decisions/0004-subprocess-hillslope-orchestration.md](../decisions/0004-subprocess-hillslope-orchestration.md).

wepppy invokes the openWEPP CLIs as subprocesses, matching its existing call shape against the legacy WEPP binary. No in-process linkage, no PyO3.

## Kernel boundary
Kernels are pure functions over typed state. Orchestrators own time-stepping and topology; kernels own physics. The producer/consumer trajectory-ownership rules from the wepp-palimpsest trajectory-ownership contract map onto Rust lifetimes and ownership transfer.

> **Hot-path runtime architecture (proposed re-architecture):** the symbol-keyed
> `BTreeMap<BoundarySymbol, BoundaryValue>` hot path is being replaced by a typed dense
> *HillslopeDayFrame* to meet the ≤10× (ideally ≤5×) viability gate. See
> [array-native-runtime-specification.md](array-native-runtime-specification.md) — the comprehensive
> design authority for the perf re-architecture (pending ADR-0025 ratification). It fulfils the "pure
> functions over typed state" boundary above; the string-keyed maps were a scaffolding compromise.

## State files
- **Inputs**: WEPP soil, management, climate (cligen), watershed structure, plus the formalized `.run`. See [../contracts/README.md](../contracts/README.md).
- **HBP shards**: per-hillslope binary pass files; format authoritative in wepp-palimpsest.
- **Parquet outputs**: per the wepppy / wepppyo3 interchange schemas.

## Not in this repo
- Run state management (wepppy NoDb)
- GIS preprocessing (wepppy)
- Climate generation (wepppy / cligen)
- Web UI, REST/WebSocket interfaces (wepppy)
- Sediment routing physics (deferred to wepp-palimpsest sediment kernelization program)
