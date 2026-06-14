# Worker Handoff

Status: T-A executed; T-B ready

Evidence mode: Static + Ran

## Current State

W-A characterized the watershed CLI and scoped the original routed-output work.
W-B cleared the no-impoundment parser seam. W-C cleared the WS10 channel guard
seam and published WAT-backed watershed outputs. W-D ran the totalwatsed3 audit
and fixed confirmed publication defects, but the W-D closure gate failed with
`closure_reconstructed_with_storage_total_mm=2950.498418`.

T-A applies the operator-directed architecture pivot: totalwatsed3 is
hillslope-only and must move to a dedicated openWEPP-native
`openwepp-cli-totalwatsed3`. It is not channel-routed watershed output and has
no channel loss/storage terms.

## T-A Scope Result

`totalwatsed3-cli-scope.md` is the controlling design artifact for T-B/T-C.
It records:

- authoritative semantics read from
  `/home/workdir/wepppy/wepppy/wepp/interchange/totalwatsed3.py` and
  `/home/workdir/wepppy/tools/totalwatsed3_daily_closure_audit.py`;
- the required hillslope inputs: PASS event rows, WAT rows, optional soil rows,
  optional element rows, and area/selector rules;
- PASS `runvol` as the independent `Runoff` operand;
- WAT flux/storage aggregation, including outlet-only MOFE `latqcc`;
- openWEPP-native output schema requirements with W-D unit/depth repairs;
- T-B red tests and T-C closure gates.

T-A sampled the arboreal-dendrite interchange schemas under
`/wc1/runs/ar/arboreal-dendrite/wepp/output/interchange/` and confirmed the
reference shape uses combined `H.pass.parquet`, `H.wat.parquet`,
`H.soil.parquet`, and `H.element.parquet` with `wepp_id`/`ofe_id` selectors.

## Live Implementation Gap

Current openWEPP hillslope output still requires a `.hbp` pass file and
optional WAT/soil/element parquet files. The HBP writer currently writes the
six event volume slots as zero, and the HBP parser consumes those slots without
exposing PASS `runvol`. The superseded watershed helper
`build_watershed_daily_rows_from_wat` uses pass filenames only to find sibling
WAT files, then derives runoff from WAT `Q`.

T-B must therefore create a real openWEPP-native PASS lineage surface before
totalwatsed3 closure can be claimed.

## Next Dispatch

```text
Execute increment T-B of docs/work-packages/20260613-wshed01-watershed-routed-outputs-totalwatsed3-closure-001/artifacts/watershed-staged-increment-plan.md end-to-end.
```

## T-B Requirements

- Add the dedicated `openwepp-cli-totalwatsed3` entrypoint.
- Add or expose an openWEPP-native PASS parquet/adapter surface containing
  canonical `runvol` and companion PASS metrics.
- Implement hillslope-only area-weighted aggregation from PASS + WAT +
  optional soil/element inputs.
- Preserve the W-D schema repairs: exact hydrology fields in `m^3`, depth
  aliases in `mm`, profile/interception fields, and outlet-only MOFE `latqcc`.
- Remove or relocate the superseded `build_watershed_daily_rows_from_wat`
  ownership from `openwepp-cli-watershed`.
- Keep wepppyo3 `wepp_interchange` out of the openWEPP implementation.
- Add red/green tests for PASS-vs-WAT runoff independence, MOFE outlet lateral
  collapse, required schema/typed errors, and real arboreal-dendrite emission.

## T-C Requirements

- Run `openwepp-cli-totalwatsed3` on arboreal-dendrite.
- Run the wepppy `totalwatsed3_daily_closure_audit.py` on the emitted parquet.
- Accept only independent nonzero-at-noise closure. Exact-zero closure on the
  real cohort is a tautology hold.
- On pass, update the package disposition, remove the ROADMAP deferral, and
  name `WATERSHED-CHANWB-ROUTED-OUTPUT` as the decoupled channel-output
  follow-on.

## Watchpoints

- `openwepp-cli-watershed.rs` is already above the 2000-line warning
  threshold. T-B should avoid adding totalwatsed3 logic there.
- `crates/openwepp-watershed-output/src/writers.rs` is above the 2000-line
  warning threshold. T-B should prefer a dedicated module/crate path instead
  of adding more writer logic there.
- No production wepppy edits are in scope. wepppy is semantic/audit evidence
  only unless a future package explicitly scopes cross-repo changes.
