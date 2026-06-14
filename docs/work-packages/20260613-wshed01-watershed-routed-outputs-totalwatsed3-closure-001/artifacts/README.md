# Artifacts

Status: T-B2-REDO executed - package active, T-C queued

Evidence and disposition artifacts for
`20260613-wshed01-watershed-routed-outputs-totalwatsed3-closure-001`.

W-A recorded the current watershed CLI fail-closed path, classified the
arboreal-dendrite no-impoundment input as a parser defect, and scoped the
routing/output/totalwatsed3 closure surface.

W-B implemented explicit no-impoundment `.imp` handling. The parser now accepts
`jpond=0` only when structure declares zero impoundments, and the real
arboreal-dendrite CLI proceeds past `CLIWAT-E-010` to the next channel-routing
hard stop.

W-C classified and fixed the channel-routing hard stop, then replaced
placeholder watershed publication with WAT-backed multi-row output. The real
arboreal-dendrite CLI now emits all `14` watershed parquet outputs.

W-D ran the totalwatsed3 audit, fixed volume/depth publication, outlet-only
MOFE `latqcc`, profile fields, and interception publication, but remains held
on totalwatsed3 closure: `closure_reconstructed_with_storage_total_mm` is
`2950.498418`. Its independent daily PASS `runvol` finding remains the live
defect, but the fix path is now the T-arc dedicated CLI rather than
W-D-REDO-in-watershed-CLI.

T-A superseded the W-D-REDO watershed-CLI route with the operator-directed
architecture pivot: totalwatsed3 is a dedicated hillslope-only
`openwepp-cli-totalwatsed3`. The new `totalwatsed3-cli-scope.md` pins PASS
`runvol` as the independent `Runoff` operand, WAT storage/flux aggregation,
MOFE outlet-only `latqcc`, the openWEPP-native schema, and the T-B/T-C gates.

T-B implemented the dedicated CLI, native PASS/WAT aggregation, required
typed-error and lineage tests, and unit-registry lineage for
`watershed_totalwatsed3.Runoff`. The arboreal-dendrite run emits `2192`
readable rows and the wepppy audit consumes them without schema repair. The
package remains active for T-C because the current independent closure
residual is `57.409871 mm`.

T-B2 added openWEPP-owned runoff-delivery PASS parquet emission from the
hillslope runner, but review found its first MOFE `runvol` formula used
`QOFE * publication area` and over-scaled runoff. T-B2-REDO corrected PASS
`runvol` to the published `Q * Area` dual. The corrected arboreal-dendrite
rerun under `/tmp/openwepp_wshed01_tb2_redo_qarea` emitted `36`
`H*.pass.parquet` files alongside byte-identical HBP/WAT anchors; direct
parquet audit showed PASS `runvol` matches `Q * Area` exactly over `78912`
rows. The dedicated totalwatsed3 CLI consumed the corrected native
per-hillslope files and wrote `2192` rows. T-C still owns conservation audit
closure; the current corrected-output audit residual is `6948.564523 mm`.
