# Final Disposition

Status: `EXECUTED-COMPLETE-DX5-PRODUCTION-MESH-POLICY`
Evidence mode: Static + Ran.

## Outcome

The active production Lane D mesh-policy default is now target `dx = 5.0 m`
under `SC-OFEROUTE-001` rev 45.

The production default is:

```text
target_dx_m = 5.0
min_cells = 10
max_cells = 4096
sample_dt_s = 900
max_dt_s = 300
```

The diagnostic target-`dx` selector remains explicit and fail-closed.
Shadow routing remains unchanged at fixed `10` cells.

## Evidence

The package-local promotion matrix records
`DX5_PRODUCTION_RATIFIED_BY_EVIDENCE` with `21` rows, `0` blockers, and `0`
missing annual replay rows.

The release runtime evidence proves that selected real-cohort active no-env
runs use the production `dx5` default and are byte-identical to explicit
`OPENWEPP_LANED_ACTIVE_MESH_TARGET_DX_M=5.0` runs across HBP, loss JSON, pass
parquet, WAT parquet, and active trace JSONL.

The protected off/default path remains byte-identical when the mesh diagnostic
env is present but active routing is disabled.

Active closure, DC01 no-double-feed posture, and routed-hydrograph erosion
consumer proof are recorded in package artifacts.

## Gates

Required gates passed:

- promotion matrix replay;
- exact release-binary provenance;
- selected-cohort active default/no-env dx5 runtime evidence;
- active default/no-env versus explicit dx5 output identity;
- protected default/off byte identity;
- active closure and `INV-OFEROUTE-012` proof;
- DC01-disable/no-double-feed proof;
- routed-hydrograph-to-erosion consumer proof;
- focused Lane D mesh-policy tests;
- `git diff --check`;
- markdown/doc lint;
- contract binding exposure, unit compliance, and unit registry checks;
- `cargo fmt --check`;
- `cargo clippy --workspace --all-targets -- -D warnings`;
- `cargo nextest run --workspace --profile full`;
- `cargo deny check`.

The authority anti-evasion guard was not triggered because this package did
not touch required-case bindings, cohort fixtures, or external-authority suite
posture.

## Follow-On State

The production mesh-policy decision is closed at current authority. Follow-on
work should treat dx5 as the active production default unless a future
contract-first package amends `SC-OFEROUTE-001`.

Known non-blocking follow-ons are recorded in `artifacts/worker-handoff.md`.
