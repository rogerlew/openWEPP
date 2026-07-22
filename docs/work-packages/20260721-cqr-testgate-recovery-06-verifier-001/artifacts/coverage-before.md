# Coverage Before

Static: Attempt-15 function coverage for the target row is 0%. Exact current
module closure was measured once after scaffold review.

Ran: at exact clean HEAD `ce1da0a6`, production `verifier.rs` through line
1,676 measured 1,050/1,279 lines (82.0954%) and 1,646/2,019 deduplicated LLVM
regions (81.5255%). The one non-Linux `read_confined` row was not compiled in
this profile. Twelve compiled functions are below the binding 75% region floor:

- `verify_receipt_after_ready_audit`: 7/26 (26.9231%)
- `verify_prerequisite_results`: 26/35 (74.2857%)
- `verify_attempt`: 14/32 (43.75%)
- `verify_heavy_audit`: 21/30 (70%)
- `verify_envelope_artifacts`: 7/10 (70%)
- `equal`: 7/10 (70%)
- the six public `ReceiptVerdict`/`EnvelopeVerdict` identity and trust getters:
  0/3 each (0%)

Ran: 135 tests passed, 0 failed, and 2 were intentionally ignored in 231.60
seconds (255.71 seconds wall). Source and worktree identity were unchanged.
Raw evidence is retained at `/tmp/cqr-verifier-baseline-NLrOph`; the exact
631 MB disposable target was validated and pruned.
