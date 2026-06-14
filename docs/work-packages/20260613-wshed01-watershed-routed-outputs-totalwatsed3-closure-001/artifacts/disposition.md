# Disposition

Status: T-A executed; package active; T-B queued

Evidence mode: Ran + Static

W-A, W-B, W-C, the keepable W-D publication repairs, and T-A design scope are
complete for their scoped seams. The package remains active because
totalwatsed3 closure still requires the dedicated T-B/T-C CLI path.

Disposition:

- W-A baseline: fail-closed before watershed output writing at
  `CLIWAT-E-010`/`IMP-E-004`.
- `jpond=0`: fixed as a parser defect on valid no-impoundment input.
- Runfile contract: schema v1 still requires `inputs.pw0_imp`; an explicit
  supported `.imp` file with `jpond=0` is accepted only when `pw0_str` declares
  zero impoundments.
- W-B arboreal-dendrite behavior: proceeds past `CLIWAT-E-010`; next observed
  hard stop is `CLIWAT-E-020` / `WKERNEL-WS10-CHANNEL-E-003`; output file
  count remains `0`.
- W-C hard-stop classification: valid zero-sediment HBP payload and `nchnum=0`
  output-disabled channel state were rejected by over-strict WS10 guards.
- Routing/output result: arboreal-dendrite configured and legacy-discovery
  runs exit `0`, emit all `14` watershed parquet outputs, and produce `2192`
  `totalwatsed3.parquet` rows with non-placeholder WAT fields.
- totalwatsed3 contract: documented from openWEPP and wepppy sources.
- W-D publication repairs: exact volume fields now emit `m^3`, depth aliases
  remain mm, MOFE `latqcc` is outlet-only, optional profile/interception WAT
  fields are published, and profile audit violations are zero.
- W-D closure gate: FAIL/HOLD. Configured and legacy-discovery audits both
  report `closure_reconstructed_with_storage_total_mm=2950.498418`, so
  package closure is blocked on independent daily PASS `runvol` lineage.
- T-A architecture pivot: executed. `totalwatsed3-cli-scope.md` establishes
  `openwepp-cli-totalwatsed3` as a hillslope-only openWEPP-native CLI, with
  PASS `runvol` as `Runoff`, WAT storage/flux operands, outlet-only MOFE
  `latqcc`, no channel terms, and no wepppyo3 dependency.

Next required increment:

```text
Execute increment T-B of docs/work-packages/20260613-wshed01-watershed-routed-outputs-totalwatsed3-closure-001/artifacts/watershed-staged-increment-plan.md end-to-end.
```
