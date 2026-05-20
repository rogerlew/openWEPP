# Subsystem Dependency Map

Status: complete
Date: 2026-05-20 UTC
Evidence mode: `Static`
Ran evidence: none in this kickoff execution

`A -> B` means subsystem `A` depends on subsystem `B`.

## Directed Dependencies

- `SS-02 -> SS-01` typed state cannot exist without validated input contracts (`[INFERENCE][Static]`).
- `SS-03 -> SS-01` hillslope execution depends on parsed run/input contracts (`[DIRECT][Static]`).
- `SS-03 -> SS-02` hillslope execution consumes typed state surfaces (`[DIRECT][Static]`).
- `SS-03 -> SS-05` orchestrator dispatches kernels via routine interface contract (`[DIRECT][Static]`).
- `SS-03 -> SS-06` hillslope surfaces must pass invariants/closure checks (`[INFERENCE][Static]`).
- `SS-03 -> SS-08` hillslope produces HBP/parquet interchange outputs (`[DIRECT][Static]`).
- `SS-03 -> SS-09` runner/release/error governance constrains launch/failure behavior (`[DIRECT][Static]`).
- `SS-04 -> SS-01` watershed execution depends on validated watershed/topology inputs (`[DIRECT][Static]`).
- `SS-04 -> SS-02` watershed execution consumes typed state surfaces (`[DIRECT][Static]`).
- `SS-04 -> SS-03` watershed routing consumes hillslope-generated HBP shards (`[DIRECT][Static]`).
- `SS-04 -> SS-05` watershed orchestrator dispatches routing kernels via routine interfaces (`[INFERENCE][Static]`).
- `SS-04 -> SS-06` watershed surfaces must pass invariant/closure checks (`[INFERENCE][Static]`).
- `SS-04 -> SS-08` watershed outputs are emitted via interchange contracts (`[DIRECT][Static]`).
- `SS-04 -> SS-09` runner/release/error governance constrains launch/failure behavior (`[DIRECT][Static]`).
- `SS-07 -> SS-05` replay/comparator depends on reusable kernel contracts for isolation reruns (`[INFERENCE][Static]`).
- `SS-07 -> SS-06` comparator interpretation is tier-routed by invariant/acceptance policy (`[DIRECT][Static]`).
- `SS-07 -> SS-08` replay/comparator consumes HBP/parquet evidence surfaces (`[DIRECT][Static]`).
- `SS-07 -> SS-09` comparator outcomes feed governed disposition metadata and failure posture (`[INFERENCE][Static]`).

## Interface Boundaries

- `SS-01 -> SS-02`: legacy stdin `.run` + `.txt` sidecar compatibility bridge
  and schema-mode inputs both normalize into one typed run/config model
  (`[DIRECT][Static]`, ADR-0011, `docs/contracts/README.md`).
- `SS-03 -> SS-04`: `H*.hbp` shard set crossing process boundary (`[DIRECT][Static]`, ADR-0004, ADR-0006).
- `SS-03/SS-04 -> SS-08`: parquet emission via wepppy/wepppyo3 interchange schema (`[DIRECT][Static]`, ADR-0005).
- `SS-09` gates launch/release across all executable roles with explicit engine selection and mandatory sidecars (`[DIRECT][Static]`, ADR-0007).

## Secondary Legacy Static Evidence (Non-Authority)

- `wepp-forest/src/wshpas.f90` shows `.hbp` pass path detection used before watershed routing (`[DIRECT][Static]`).
- `wepp-forest/src/hbp_mode2_bridge.f90` and `wepp-forest/src/hbp_legacy_bridge.f90` expose explicit HBP read/write bridge boundaries (`[DIRECT][Static]`).
- `wepp-forest/src/wshrun.f90` is called from `wshdrv`, showing watershed orchestration layering (`[DIRECT][Static]`).
