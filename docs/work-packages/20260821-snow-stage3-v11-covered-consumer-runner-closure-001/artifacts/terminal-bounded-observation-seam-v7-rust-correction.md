# Terminal bounded observation seam V7 literal Rust correction

Status: `CANDIDATE / NO SOURCE AUTHORITY`

V3 `AdmissionHook` is replaced exactly by:

```rust
pub(crate) struct AdmissionHook<'a> {
    pub event_ordinal: u64,
    pub chronology_ordinal: u64,
    pub start_s: f64,
    pub proposed_duration_s: f64,
    pub required_half_duration_s: f64,
    pub minimum_duration_s: f64,
    pub outcome: &'a SnowTerminalNumericsFailure,
    pub provider_calls_before: u64,
    pub provider_calls_after: u64,
}
```

All trait and implementation admission signatures use `AdmissionHook<'_>`.
Capture clones the borrowed outcome into its test-only primitive outcome tag;
NoEvidence receives only the reference. `AdmissionHook` has no `Clone` or
`Copy` derive. No other inherited declaration or validator changes.
