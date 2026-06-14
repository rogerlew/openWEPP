# Artifacts

Status: W-C executed-hold - package active, W-D queued

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
arboreal-dendrite CLI now emits all `14` watershed parquet outputs. W-D remains
for totalwatsed3 audit closure.
