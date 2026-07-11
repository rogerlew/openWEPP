# Owned File Manifest

Status: `EXECUTED-CURRENT`

Evidence mode: `Static`

The bounded write set is the W11C package tree, the queued W11D defect-closure
successor scaffold, catalog/roadmap linkage, and
`crates/openwepp-runner/tests/mt3_hbp_hourly_consumer_contract.rs`. Production
Rust is excluded.

The Rust test change adds the W11C matrix, NOEVENT builder reuse, test-only
release-binary selection, structured metrics, and explicit sidecar parsing. It
also corrects the protected W11B two-channel fixture from a compatibility-
defaulting three-line `nchnum=0` sidecar to a canonical four-line `nchnum=2`
sidecar, so its written `dtchr=600` is now the executed timestep.

Ran: `git status --short` showed no modified production Rust and no unrelated
tracked paths in the W11C change set.
