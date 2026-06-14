# Artifacts

Status: T-A executed - package active, T-B queued

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
