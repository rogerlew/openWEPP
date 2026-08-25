# Terminal bounded observation seam V10 two-hop facade correction

Status: `CANDIDATE / NO SOURCE AUTHORITY`

Add a tenth file:

`crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/mod.rs`

Its sole edit is to add the exact inherited symbols to its existing
`pub(crate) use runoff_reconciliation::{...}` block, with Capture symbols in a
separate `#[cfg(test)]` re-export. The already-reviewed outer facade then
re-exports those same symbols through `crate::hydrology`.

Exact two-hop names are `NoEvidence`, `TerminalEvidenceMode`, and under test
`CaptureEvidence`, `CaptureState`. Neither module nor any symbol becomes public
outside the crate. No other source or contract change is authorized. The
failed attempt was fully removed; only package artifacts remained at freeze.
