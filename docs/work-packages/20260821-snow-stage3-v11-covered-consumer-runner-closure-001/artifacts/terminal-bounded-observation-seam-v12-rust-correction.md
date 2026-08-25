# Terminal bounded observation seam V12 generic-bound correction

Status: `REVIEW CANDIDATE / SOURCE EDITS FORBIDDEN`

V12 incorporates V3--V11. Every V11 parent, subslab and candidate forwarding
generic uses the exact bound:

```rust
M: TerminalEvidenceMode<Option<CoveredTerminalJointTrialStateV1>>
```

No unparameterized `TerminalEvidenceMode` bound remains. This selects the sole
CaptureEvidence implementation for the live joint shape. Write set, signatures,
state custody and all other inherited requirements are unchanged.
