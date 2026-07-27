# Finding Disposition

Status: `IMPLEMENTED / DUAL REVIEW REQUIRED`

Evidence class: `Ran + Static`

| ID | Finding | Implemented disposition |
|---|---|---|
| DC-F01 | Invalid predecessor authority | New prospective scaffold and reviewed amendment; canonical chain READY; predecessor bytes retained |
| DC-F02 | Capabilities consumed twice | Mutation-free pre-LIGHT verification; audit-only `NOREPLACE` consumption; immutable consumed-root proof |
| DC-F03 | HEAVY lifecycle inverted | STARTED precedes representable audit/preflight evaluation; one typed terminal balances failure |
| DC-F04 | Pathname-based recovery | Pinned descriptor-relative read/stage/replace/delete with root/ancestor race rejection |
| DC-F05 | Stale attestations accepted | Exact transaction and parent-dispatch bindings plus stale replay rejection |
| DC-F06 | Duplicate audit authority | One inventory reconstruction and one ledger admission; proof verification performs zero re-admission |
| DC-F07 | CSV headers not authoritative | Exact ordered header validation rejects unknown, missing, or reordered columns |
| DC-F08 | Heuristic error taxonomy | Typed receipt, trust, ledger, identity, policy, and I/O classifications with assertions |

Independent implementation review must confirm these dispositions against the
actual consumer paths before canonical heavy admission.
