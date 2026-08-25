# Terminal bounded observation seam V11 parent-entry forwarding correction

Status: `REVIEW CANDIDATE / SOURCE EDITS FORBIDDEN`

Base: `2d34d1082ae0d6e324a33a2e2b9159f99f5f8ace`.

Findings: `CHILD1-TERM-EVIDENCE-009` —
`execute_covered_real_v11_parent` cannot choose `CaptureEvidence` or return
`CaptureState` after `BelowCarrierDomain`. `CHILD1-TERM-EVIDENCE-010` — the
compiled lower seam was rolled back and is absent at this base.

The exact write set is the V10 ten-file set plus
`crates/openwepp-hillslope-orchestrator/src/snow_stage3_v11_attachment.rs`.

The existing parent body moves unchanged into private
`execute_covered_real_v11_parent_with_evidence<M: TerminalEvidenceMode>(...,
evidence: &mut M::State)`. The existing production function retains its exact
signature, constructs `NoEvidence::State = ()`, delegates and returns the same
Result. A crate-private `cfg(test)` wrapper constructs `CaptureState`, invokes
the generic core without `?`, and returns `(physical_result, state)`.

The generic core passes the same `&mut M::State` to every
`try_actual_terminal_subslab_with_evidence<M>` call. Its production-signature
wrapper remains. Discovery and exact candidate calls both use the same state.
No public day/runner signature changes. Capture remains test-only; no callback,
flag, feature, environment input, global, thread-local or panic interception.

The real owning test adds
`interior_terminal_event_capture_reproduces_below_carrier_domain`; the existing
success-named test remains unfulfilled. All V3--V10 literal definitions and
corrections are reapplied exactly. Two fresh GO reviews are required.
