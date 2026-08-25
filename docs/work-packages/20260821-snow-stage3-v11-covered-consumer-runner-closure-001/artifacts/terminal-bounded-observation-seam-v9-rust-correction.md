# Terminal bounded observation seam V9 literal facade correction

Status: `CANDIDATE / NO SOURCE AUTHORITY`

The V8 facade edit must re-export the inherited literal symbols exactly:

```rust
pub(crate) use support_helpers_mod::{NoEvidence, TerminalEvidenceMode};
#[cfg(test)]
pub(crate) use support_helpers_mod::{CaptureEvidence, CaptureState};
```

V8 names `NoTerminalEvidence`, `CaptureTerminalEvidence`, and capture DTO are
withdrawn. No aliases or new names are authorized. The nine-file set and all
other inherited declarations remain unchanged.
