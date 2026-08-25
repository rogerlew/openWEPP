# Terminal bounded observation seam V8 write-set correction

Status: `CANDIDATE / NO SOURCE AUTHORITY`

Add exactly this ninth file to the V7 eight-file set:

`crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_00_support_helpers.rs`

Its only permitted edit is a `pub(crate) use support_helpers_mod::{...}` of the
sealed `NoTerminalEvidence`, `TerminalEvidenceMode`, and, under `cfg(test)`,
`CaptureTerminalEvidence`/capture DTO. This file is the existing hydrology
facade; `support_helpers_mod` is private, so no sibling crate module can name
those items without this re-export. No public export or other edit is allowed.

All attempted source changes were removed before freezing this correction.
The worktree was clean at V8 freeze.
