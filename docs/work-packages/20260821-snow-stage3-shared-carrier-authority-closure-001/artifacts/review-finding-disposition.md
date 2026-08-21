# Review finding disposition

Status: complete / accepted corrections applied

Evidence mode: Static + Ran

The two independent science reviews both returned HOLD. Their findings were
treated as correction requirements for this authority package, not as runtime
activation authority. The following table records every finding and its final
disposition.

| Finding | Severity | Disposition | Evidence/action | Owner |
| --- | --- | --- | --- | --- |
| SCI-A-001 longwave used a bulk shared-air temperature | high | amended/closed | Canonical equation and vector now use weighted leaf/stem component temperatures; the vector carries two components and the receipt schema requires component lineage. | Child 2C authority owner |
| SCI-A-002 oracle trusted caller-supplied candidate errors | blocker | accepted/closed | `reference_model.py` ignores `diagnostic_candidate_errors` and recomputes each candidate from immutable terminal state and mass/energy targets. | Child 2C authority owner |
| SCI-A-003 ticks were numeric and lacked wire/overflow coverage | blocker | accepted/closed | Event vectors and receipt schemas use canonical decimal strings; the Rust gate checks leading-zero poison and ordered candidates, while the oracle checks u128 range. | Child 2C authority owner |
| SCI-A-004 `ERR-CT-021` was absent from canonical error precedence | high | accepted/closed | Coupled-time branch table, precedence range, stable alias list, invariant table, and binding row now include `ERR-CT-021`. | Coupled-time contract owner |
| SCI-A-005 positive carrier lacked exposure receipt validation | high | accepted/closed | Positive vector and carrier schema require sealed provider, provider digest, transfer height, roughness, and projected wind. | Snow/transaction contract owners |
| SCI-A-006 support aggregation was not participant/receipt authenticated | high | accepted/closed | Carrier/event vectors use ordered participant objects and support receipt IDs; the oracle and Rust poison checks require exact participant-set equality, unique nonempty receipt IDs, and derive the maximum. | Coupled-time contract owner |
| SCI-A-007 tie score and zero-tolerance behavior were underdefined | high | accepted/closed | Contract fixes score order and exact-zero semantics; vectors cover lower-error, equal-error-earlier, and zero-tolerance cases. | Coupled-time contract owner |
| SCI-A-008 receipts were opaque and lacked lineage/residuals | high | accepted/closed | Carrier/event schemas now require typed fluxes, ledgers, tolerance policy, candidate evaluations, owner map/digests, and conditional event joins. | Child 2C authority owner |
| SCI-B-001 review/verification/exact-diff gates were incomplete | blocker | accepted/closed | Review disposition, exact-diff, terminal manifest, dual verifier artifacts, and handoff are being completed before package release. | Child 2C authority owner |
| SCI-B-002 receipt schemas did not close custody/ledger authority | blocker | accepted/closed | Closed schemas and valid/invalid fixtures are directly exercised by the Rust contract test, including forged, duplicate, empty, and omitted-participant support-receipt mutations. | Child 2C authority owner |
| SCI-B-003 oracle conservation and event selection were tautological | blocker | accepted/closed | Independent ledger reconstruction covers snow, liquid, vapor, energy, reciprocal longwave, and event time; diagnostic melt alias is explicitly consumed and rejected as authority; restart tests execute the reference oracle for their source event cases. | Child 2C authority owner |
| SCI-B-004 rollback/restart custody was sentinel-only | blocker | amended/closed | Rejected vectors carry nonempty immutable owner digests for every typed rejection; restart vectors link to oracle-selected event ticks and the test asserts resumed/uninterrupted owner and receipt equality. Actual runtime restart remains a later implementation gate. | Child 2C authority owner |
| SCI-B-005 typed failures were not integrated | high | accepted/closed | New carrier, regime, scope, longwave, vegetation, transaction, LSE, and coupled-time failures are in branch tables and binding maps; poison vectors cover the Child 2C variants. | Contract owners |
| SCI-B-006 profile variables/lifecycle evidence drifted | high | accepted/closed | Variables, aliases, unit maps, obligation/invariant rows, and body/frontmatter lifecycle statuses are synchronized for the amended surfaces. | Contract owners |
| SCI-B-007 Binding Exposure Index mappings were incomplete | high | accepted/closed | All five strict BIE checks pass; Child 2C rows use canonical `flagged-binding-addition` review-gate vocabulary. | Contract owners |
| SCI-B-008 lineage/calibration artifacts were overstated | medium | accepted/closed | Operand lineage is explicitly pre-implementation with physical basis/source/consumer fields; calibration is marked not applicable with no efficacy claim. | Child 2C authority owner |
| SCI-B-009 event populations/assertions were incomplete | medium | accepted/closed | Population is expanded to 11 event cases covering support orders, exact minimum, one-tick rejection, structural 1 ns, out-of-window, two tie rules, no-candidate retry, and proposed/accepted divergence; the focused test asserts group-specific outputs and receipt evaluations. | Child 2C authority owner |

No finding authorizes production Rust, a selector/default change, calibration,
efficacy, seasonal qualification, or CoE retirement. Any runtime gaps named in
the reviews remain explicit later-package obligations.
