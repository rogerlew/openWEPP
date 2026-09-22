Static fix verification of the stable authority frame.

No blocking correctness finding remains in the revised dependency chain:

1. The immutable canonical payload binds cycle, sources, implementation, projection, records, and static source joins.
2. Its content digest enters the framed run identity, making the two cycles distinct without circularity.
3. Each private GSI-support receipt is derived afterward and binds the new run, cycle, exact 60-second support, `1.0` bits, and authenticated GSI source join.
4. The parent forcing receipt binds the full projection digest, exact 30-record slice, and corresponding ordered 30 GSI-support receipts.
5. The retained transaction-41 receipt is expressly excluded; only its authenticated constructor/output scalar is reused as prospectively prescribed input.

The retained source boundary is now truthful: `strict_v8_endpoint_tests::snow_forcing` supplies `1.0`, `endpoint_fixture` seals it, and `advance_m1_caller_support` consumes that exact field. The new support receipt establishes correspondence to the new run/support identities without a climate-provider claim or new physical scalar.

Two nonblocking wording corrections are advisable:

- At line 3446, replace “The two latter raw32 values” with “The `calendar_receipt` and `forcing_receipt` raw32 values” to avoid ambiguity among four provider domains.
- “Derived execution evidence” at line 3411 could be “derived support authority”; these receipts must be minted and validated before physical consumption and do not prove execution.

Residual implementation risk remains: final adapter paths/hashes, dependency manifests, canonical output hashes, known-answer digests, and mutation controls require review after implementation. No Rust implementation has been reviewed.

**Verdict: PASS-WITH-NOTES for the corrected authority cut.** The framing, source correspondence, cycle separation, support mapping, and GSI receipt construction are acyclic and complete enough to proceed to contract-derived controls and implementation.
