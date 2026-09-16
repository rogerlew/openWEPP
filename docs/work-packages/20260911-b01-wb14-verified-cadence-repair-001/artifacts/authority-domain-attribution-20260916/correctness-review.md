Attribution: `/root/event_correctness_attribution`, rust_code_reviewer role,
independent of the author/extractor/orchestrator and QA reviewer. Original
finding and same-reviewer final response follow.

---

Review finding for GSI row (medium precision/authority): proposed 'authenticate the prior committed receipt against the independently established beginning GSI/cursor/configuration/next-day tuple' is incomplete if read literally. Q `restart_authority_validate_native_provider_receipt` accessors.rs:1263-75 compares against `NativeProviderExpectedPositionV1` whose receipt digest is computed by `restart_authority_native_provider_prefix` (:1238-61) from authenticated bootstrap plus repository climate; the tuple alone cannot authenticate e1ca receipt preimage. Your text correctly marks day3 preimage missing, so make prospective correction explicitly conditional on independent prefix replay/source evidence and keep GSI join unresolved until that evidence exists. This is a claim correction, not an instruction to run physical work. Minor citation: ordinary V10 temporary receipt update is v9_real_consumer_shadow.rs:2731-43, not :2743 alone.

---

**Static:** I reviewed the final three-row attribution against frozen Q/R source, the cited contracts, both extraction scripts and their outputs. **Ran:** I independently read the retained resident and deferred members; they show resident transaction 5762 ending at `385200000000000 ns` and a separate 12-group unpublished continuation ending at `385920000000000 ns`. I ran no Rust tests or physics.

**Findings:** No remaining blocker to the bounded attribution decision. The earlier GSI finding is fixed: the row now requires independent provider-prefix or receipt-preimage evidence and explicitly leaves the `e1ca…` receipt join unresolved. The position row keeps the physical WB14 counter separate from outer parent and child/slab counts. The soil row preserves the current constructor refusal and identifies deferred-context restoration as an authority gap without claiming producer invalidity or successful restart.

**Residual risk and missing tests:** The day-3 GSI receipt preimage has not been authenticated. No supported restoration of the deferred soil composite has been demonstrated; the future admission invariant, semantic poison tests and successful inverse remain pending. The native six-owner projection and production qualification remain untested.

**Verdict:** No correctness blocker to recording *attribution complete* with native restoration and production on **HOLD**.

---

**Static addendum:** The added Q citations support both claims. `prepare_v11_intervals_from_repository` projects the prepared GSI receipt through a cloned candidate, and the successor receives the resulting interval; the live receipt is unchanged. The soil finalizer explicitly retains the beginning resident while keeping the selected trial in separate deferred custody.

No new correctness finding. The prior bounded approval stands: attribution may be recorded complete; deferred restoration and production remain **HOLD**. I ran no new tests.
